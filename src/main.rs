mod edid;
mod elevation;
mod model;
mod output;
mod platform;
mod report;

use chrono::Local;
use elevation::{ElevationOutcome, ElevationState};
use model::{ReportMeta, SystemReport};
use std::{env, path::PathBuf};

#[derive(Debug, Default)]
struct Cli {
    output: Option<PathBuf>,
    no_elevate: bool,
    relaunch_marker: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("HowAmI failed / 실행 실패: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_args()?;

    let elevation = elevation::ensure_elevated(cli.no_elevate, cli.relaunch_marker)?;
    let elevation_state = match elevation {
        ElevationOutcome::Relaunched => return Ok(()),
        ElevationOutcome::Continue(state) => state,
    };

    let mut warnings = Vec::new();
    if let Some(warning) = elevation_state.warning.clone() {
        warnings.push(warning);
    }

    let collection = match platform::collect() {
        Ok(collection) => collection,
        Err(error) => {
            warnings.push(format!(
                "Collector error: {error} / 시스템 정보 수집 오류: {error}"
            ));
            platform::Collection::default()
        }
    };
    warnings.extend(collection.warnings);

    let report = build_report(elevation_state, collection.sections, warnings);
    let output_dir = output::resolve_output_dir(cli.output);
    let (txt_path, json_path) = output::write_reports(&report, &output_dir)?;

    println!("HowAmI report created / 리포트 생성 완료");
    println!("TXT : {}", txt_path.display());
    println!("JSON: {}", json_path.display());
    println!("Review the report before sharing it. / 외부 공유 전 개인정보를 확인하세요.");

    Ok(())
}

fn build_report(
    elevation_state: ElevationState,
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
            elevated: elevation_state.elevated,
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
            "--elevated" => cli.relaunch_marker = true,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("Unknown argument: {other}").into()),
        }
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
