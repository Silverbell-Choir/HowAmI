#[cfg(target_os = "linux")]
mod edid;
mod command;
mod elevation;
mod model;
mod output;
mod platform;
mod report;

use chrono::Local;
use model::{ReportMeta, SystemReport};
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
struct Cli {
    output: Option<PathBuf>,
    no_elevate: bool,
    elevated_handoff: Option<PathBuf>,
    elevated_token: Option<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("HowAmI failed / 실행 실패: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_args()?;

    if let Some(handoff) = cli.elevated_handoff.as_deref() {
        let token = cli
            .elevated_token
            .as_deref()
            .ok_or("internal elevated child token is missing")?;
        return run_elevated_child(handoff, token);
    }

    let mut warnings = Vec::new();
    let already_elevated = elevation::is_elevated();

    let (collection, collection_elevated) = if already_elevated {
        (collect_with_fallback(), true)
    } else if cli.no_elevate {
        warnings.push(
            "Elevation was disabled; some hardware data may be unavailable. / 관리자 권한 상승이 비활성화되어 일부 정보가 누락될 수 있습니다."
                .into(),
        );
        (collect_with_fallback(), false)
    } else {
        match collect_via_elevated_child() {
            Ok(collection) => (collection, true),
            Err(error) => {
                warnings.push(format!(
                    "Administrator/root collection was unavailable: {error}. Continuing with standard-user access. / 관리자/root 권한 수집을 사용할 수 없습니다: {error}. 일반 권한으로 계속합니다."
                ));
                (collect_with_fallback(), false)
            }
        }
    };

    warnings.extend(collection.warnings);

    let report = build_report(collection_elevated, collection.sections, warnings);
    let output_dir = output::resolve_output_dir(cli.output);
    let (txt_path, json_path) = output::write_reports(&report, &output_dir)?;

    println!("HowAmI report created / 리포트 생성 완료");
    println!("TXT : {}", txt_path.display());
    println!("JSON: {}", json_path.display());
    println!("Review the report before sharing it. / 외부 공유 전 개인정보를 확인하세요.");

    Ok(())
}

fn collect_with_fallback() -> platform::Collection {
    match platform::collect() {
        Ok(collection) => collection,
        Err(error) => platform::Collection {
            sections: Vec::new(),
            warnings: vec![format!(
                "Collector error: {error} / 시스템 정보 수집 오류: {error}"
            )],
        },
    }
}

fn collect_via_elevated_child() -> Result<platform::Collection, Box<dyn std::error::Error>> {
    let (handoff, token) = create_handoff_file()?;
    let result = elevation::run_elevated_child(&env::current_exe()?, &handoff, &token);

    if let Err(error) = result {
        let _ = fs::remove_file(&handoff);
        return Err(error);
    }

    let bytes = fs::read(&handoff);
    let _ = fs::remove_file(&handoff);
    let bytes = bytes?;
    if bytes.is_empty() {
        return Err("elevated collector returned no data".into());
    }

    Ok(serde_json::from_slice(&bytes)?)
}

fn create_handoff_file() -> Result<(PathBuf, String), Box<dyn std::error::Error>> {
    let temp = env::temp_dir();
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();

    for attempt in 0u32..32 {
        let token = format!("HOWAMI_HANDOFF_V1:{pid}:{now}:{attempt}");
        let path = temp.join(format!("HowAmI_{pid}_{now}_{attempt}.handoff"));
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);

        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }

        match options.open(&path) {
            Ok(mut file) => {
                file.write_all(token.as_bytes())?;
                file.sync_all()?;
                return Ok((path, token));
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }

    Err("could not create a unique elevation handoff file".into())
}

fn run_elevated_child(
    handoff: &Path,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !elevation::is_elevated() {
        return Err("elevated child marker was supplied without Administrator/root privileges".into());
    }

    let metadata = fs::symlink_metadata(handoff)?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("invalid elevation handoff target".into());
    }

    let marker = fs::read_to_string(handoff)?;
    if marker != token || !token.starts_with("HOWAMI_HANDOFF_V1:") {
        return Err("elevation handoff authentication failed".into());
    }

    let collection = collect_with_fallback();
    let payload = serde_json::to_vec(&collection)?;

    let mut file = OpenOptions::new().write(true).truncate(true).open(handoff)?;
    file.write_all(&payload)?;
    file.sync_all()?;
    Ok(())
}

fn build_report(
    collection_elevated: bool,
    sections: Vec<model::Section>,
    warnings: Vec<String>,
) -> SystemReport {
    SystemReport {
        meta: ReportMeta {
            app_name: "HowAmI".into(),
            app_version: env!("CARGO_PKG_VERSION").into(),
            generated_at: Local::now().to_rfc3339(),
            os: env::consts::OS.into(),
            architecture: env::consts::ARCH.into(),
            elevated: collection_elevated,
            privacy_notice: "This report may contain hardware serial numbers, UUIDs, MAC addresses, host names, device IDs, and other unique identifiers. It is generated locally and is not uploaded by HowAmI. / 이 리포트에는 하드웨어 시리얼, UUID, MAC 주소, 호스트명, 장치 ID 등 고유 식별 정보가 포함될 수 있습니다. HowAmI는 로컬에서만 생성하며 외부로 업로드하지 않습니다.".into(),
        },
        sections,
        warnings,
    }
}

fn parse_args() -> Result<Cli, Box<dyn std::error::Error>> {
    let mut cli = Cli::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let value = args.next().ok_or("--output requires a directory path")?;
                cli.output = Some(PathBuf::from(value));
            }
            "--no-elevate" => cli.no_elevate = true,
            "--elevated-child" => {
                let path = args
                    .next()
                    .ok_or("--elevated-child requires a handoff path")?;
                let token = args
                    .next()
                    .ok_or("--elevated-child requires a handoff token")?;
                cli.elevated_handoff = Some(PathBuf::from(path));
                cli.elevated_token = Some(token);
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}").into()),
        }
    }

    if cli.elevated_handoff.is_some() && (cli.output.is_some() || cli.no_elevate) {
        return Err("internal elevated-child mode cannot be combined with user options".into());
    }
    if cli.elevated_handoff.is_some() != cli.elevated_token.is_some() {
        return Err("incomplete internal elevated-child arguments".into());
    }

    Ok(cli)
}

fn print_help() {
    println!("HowAmI {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: HowAmI [--output <directory>] [--no-elevate]");
    println!();
    println!("  --output <directory>  Write reports to a specific directory.");
    println!("  --no-elevate          Do not request Administrator/root privileges.");
    println!("  -h, --help            Show this help.");
}
