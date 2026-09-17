use super::Collection;
use std::process::Command;

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            POWERSHELL_COLLECTOR,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Windows collector failed: {}", stderr.trim()).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let payload: Collection = serde_json::from_str(stdout.trim())?;
    Ok(payload)
}

const POWERSHELL_COLLECTOR: &str = r#"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$ErrorActionPreference = 'SilentlyContinue'
$sections = @()
$warnings = @()

function To-Text($Value) {
    if ($null -eq $Value) { return $null }
    if ($Value -is [System.Array]) {
        return (($Value | ForEach-Object { [string]$_ }) -join ', ')
    }
    if ($Value -is [DateTime]) { return $Value.ToString('o') }
    return [string]$Value
}

function New-Record($Label, $Fields) {
    $clean = [ordered]@{}
    foreach ($entry in $Fields.GetEnumerator()) {
        $value = To-Text $entry.Value
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            $clean[[string]$entry.Key] = $value
        }
    }
    [pscustomobject]@{ label = [string]$Label; fields = [pscustomobject]$clean }
}

function Add-Section($Name, $Records) {
    $script:sections += [pscustomobject]@{ name = [string]$Name; records = @($Records) }
}

function Decode-CharArray($Value) {
    if ($null -eq $Value) { return $null }
    return -join ($Value | Where-Object { $_ -ne 0 } | ForEach-Object { [char]$_ })
}

$os = Get-CimInstance Win32_OperatingSystem | Select-Object -First 1
$cs = Get-CimInstance Win32_ComputerSystem | Select-Object -First 1
$systemRecords = @()
if ($os) {
    $systemRecords += New-Record 'Operating System' ([ordered]@{
        Caption = $os.Caption
        Version = $os.Version
        BuildNumber = $os.BuildNumber
        Architecture = $os.OSArchitecture
        InstallDate = $os.InstallDate
        LastBootUpTime = $os.LastBootUpTime
        WindowsDirectory = $os.WindowsDirectory
        SystemDirectory = $os.SystemDirectory
    })
}
if ($cs) {
    $systemRecords += New-Record 'Computer' ([ordered]@{
        Manufacturer = $cs.Manufacturer
        Model = $cs.Model
        SystemType = $cs.SystemType
        Name = $cs.Name
        Domain = $cs.Domain
        TotalPhysicalMemoryBytes = $cs.TotalPhysicalMemory
    })
}
Add-Section 'System' $systemRecords

$cpuRecords = @(Get-CimInstance Win32_Processor | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        Socket = $_.SocketDesignation
        Cores = $_.NumberOfCores
        LogicalProcessors = $_.NumberOfLogicalProcessors
        MaxClockMHz = $_.MaxClockSpeed
        CurrentClockMHz = $_.CurrentClockSpeed
        ProcessorId = $_.ProcessorId
        Revision = $_.Revision
        ArchitectureCode = $_.Architecture
    })
})
Add-Section 'CPU' $cpuRecords

$gpuRecords = @(Get-CimInstance Win32_VideoController | ForEach-Object {
    $resolution = $null
    if ($_.CurrentHorizontalResolution -and $_.CurrentVerticalResolution) {
        $resolution = "$($_.CurrentHorizontalResolution)x$($_.CurrentVerticalResolution)"
    }
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        VideoProcessor = $_.VideoProcessor
        AdapterCompatibility = $_.AdapterCompatibility
        AdapterRAMBytes = $_.AdapterRAM
        DriverVersion = $_.DriverVersion
        DriverDate = $_.DriverDate
        PNPDeviceID = $_.PNPDeviceID
        CurrentResolution = $resolution
        CurrentRefreshRateHz = $_.CurrentRefreshRate
        VideoModeDescription = $_.VideoModeDescription
    })
})
Add-Section 'GPU' $gpuRecords

$boardRecords = @(Get-CimInstance Win32_BaseBoard | ForEach-Object {
    New-Record 'Mainboard' ([ordered]@{
        Manufacturer = $_.Manufacturer
        Product = $_.Product
        Version = $_.Version
        SerialNumber = $_.SerialNumber
        Tag = $_.Tag
    })
})
Add-Section 'Mainboard' $boardRecords

$biosRecords = @(Get-CimInstance Win32_BIOS | ForEach-Object {
    New-Record 'BIOS / UEFI' ([ordered]@{
        Manufacturer = $_.Manufacturer
        SMBIOSBIOSVersion = $_.SMBIOSBIOSVersion
        Version = $_.Version
        ReleaseDate = $_.ReleaseDate
        SerialNumber = $_.SerialNumber
        SMBIOSMajorVersion = $_.SMBIOSMajorVersion
        SMBIOSMinorVersion = $_.SMBIOSMinorVersion
    })
})
Add-Section 'BIOS / UEFI' $biosRecords

$memoryRecords = @(Get-CimInstance Win32_PhysicalMemory | ForEach-Object {
    $label = 'Memory Module'
    if ($_.DeviceLocator) { $label = $_.DeviceLocator }
    elseif ($_.BankLabel) { $label = $_.BankLabel }
    New-Record $label ([ordered]@{
        BankLabel = $_.BankLabel
        Manufacturer = $_.Manufacturer
        PartNumber = $_.PartNumber
        SerialNumber = $_.SerialNumber
        CapacityBytes = $_.Capacity
        SpeedMHz = $_.Speed
        ConfiguredClockMHz = $_.ConfiguredClockSpeed
        DataWidthBits = $_.DataWidth
        TotalWidthBits = $_.TotalWidth
        MemoryTypeCode = $_.MemoryType
        SMBIOSMemoryTypeCode = $_.SMBIOSMemoryType
        FormFactorCode = $_.FormFactor
    })
})
Add-Section 'Memory' $memoryRecords

$diskRecords = @(Get-CimInstance Win32_DiskDrive | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Model = $_.Model
        Manufacturer = $_.Manufacturer
        InterfaceType = $_.InterfaceType
        MediaType = $_.MediaType
        SizeBytes = $_.Size
        SerialNumber = $_.SerialNumber
        FirmwareRevision = $_.FirmwareRevision
        PNPDeviceID = $_.PNPDeviceID
        Partitions = $_.Partitions
        Status = $_.Status
    })
})
Add-Section 'Storage' $diskRecords

if (Get-Command Get-PhysicalDisk -ErrorAction SilentlyContinue) {
    $physicalRecords = @(Get-PhysicalDisk | ForEach-Object {
        New-Record $_.FriendlyName ([ordered]@{
            FriendlyName = $_.FriendlyName
            Manufacturer = $_.Manufacturer
            Model = $_.Model
            SerialNumber = $_.SerialNumber
            FirmwareVersion = $_.FirmwareVersion
            MediaType = $_.MediaType
            BusType = $_.BusType
            SizeBytes = $_.Size
            HealthStatus = $_.HealthStatus
            OperationalStatus = $_.OperationalStatus
        })
    })
    Add-Section 'Physical Disks' $physicalRecords
}

$volumeRecords = @()
if (Get-Command Get-Volume -ErrorAction SilentlyContinue) {
    $volumeRecords = @(Get-Volume | ForEach-Object {
        $label = 'Volume'
        if ($_.DriveLetter) { $label = "$($_.DriveLetter):" }
        elseif ($_.FileSystemLabel) { $label = $_.FileSystemLabel }
        New-Record $label ([ordered]@{
            DriveLetter = $_.DriveLetter
            FileSystemLabel = $_.FileSystemLabel
            FileSystem = $_.FileSystem
            SizeBytes = $_.Size
            SizeRemainingBytes = $_.SizeRemaining
            HealthStatus = $_.HealthStatus
            DriveType = $_.DriveType
            Path = $_.Path
        })
    })
}
Add-Section 'Volumes' $volumeRecords

$monitorRecords = @(Get-CimInstance Win32_DesktopMonitor | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.MonitorManufacturer
        Type = $_.MonitorType
        PNPDeviceID = $_.PNPDeviceID
        ScreenWidth = $_.ScreenWidth
        ScreenHeight = $_.ScreenHeight
        Status = $_.Status
    })
})
Add-Section 'Monitors' $monitorRecords

$edidRecords = @(Get-CimInstance -Namespace root\wmi -ClassName WmiMonitorID | ForEach-Object {
    New-Record $_.InstanceName ([ordered]@{
        FriendlyName = (Decode-CharArray $_.UserFriendlyName)
        ManufacturerName = (Decode-CharArray $_.ManufacturerName)
        ProductCodeID = (Decode-CharArray $_.ProductCodeID)
        SerialNumber = (Decode-CharArray $_.SerialNumberID)
        WeekOfManufacture = $_.WeekOfManufacture
        YearOfManufacture = $_.YearOfManufacture
        Active = $_.Active
    })
})
Add-Section 'Monitor EDID' $edidRecords

$networkRecords = @(Get-CimInstance Win32_NetworkAdapter | Where-Object { $_.PhysicalAdapter -eq $true } | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        ProductName = $_.ProductName
        Manufacturer = $_.Manufacturer
        MACAddress = $_.MACAddress
        AdapterType = $_.AdapterType
        SpeedBitsPerSecond = $_.Speed
        NetConnectionID = $_.NetConnectionID
        NetEnabled = $_.NetEnabled
        PNPDeviceID = $_.PNPDeviceID
    })
})
Add-Section 'Network Adapters' $networkRecords

$audioRecords = @(Get-CimInstance Win32_SoundDevice | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        ProductName = $_.ProductName
        PNPDeviceID = $_.PNPDeviceID
        Status = $_.Status
    })
})
Add-Section 'Audio Devices' $audioRecords

$usbRecords = @(Get-CimInstance Win32_PnPEntity | Where-Object { $_.PNPDeviceID -like 'USB\*' } | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        PNPClass = $_.PNPClass
        PNPDeviceID = $_.PNPDeviceID
        Service = $_.Service
        Status = $_.Status
    })
})
Add-Section 'USB Devices' $usbRecords

$bluetoothRecords = @(Get-CimInstance Win32_PnPEntity | Where-Object { $_.PNPClass -eq 'Bluetooth' -or $_.PNPDeviceID -like 'BTH*' } | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        PNPClass = $_.PNPClass
        PNPDeviceID = $_.PNPDeviceID
        Service = $_.Service
        Status = $_.Status
    })
})
Add-Section 'Bluetooth Devices' $bluetoothRecords

$batteryRecords = @(Get-CimInstance Win32_Battery | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        ChemistryCode = $_.Chemistry
        DesignCapacity = $_.DesignCapacity
        FullChargeCapacity = $_.FullChargeCapacity
        EstimatedChargeRemainingPercent = $_.EstimatedChargeRemaining
        EstimatedRunTimeMinutes = $_.EstimatedRunTime
        BatteryStatusCode = $_.BatteryStatus
        Status = $_.Status
    })
})
Add-Section 'Battery' $batteryRecords

if (Get-Command Get-PnpDevice -ErrorAction SilentlyContinue) {
    $pnpRecords = @(Get-PnpDevice -PresentOnly | Sort-Object Class, FriendlyName | ForEach-Object {
        $label = $_.InstanceId
        if ($_.FriendlyName) { $label = $_.FriendlyName }
        New-Record $label ([ordered]@{
            Class = $_.Class
            FriendlyName = $_.FriendlyName
            InstanceId = $_.InstanceId
            Status = $_.Status
            ProblemCode = $_.Problem
        })
    })
    Add-Section 'Present PnP Devices' $pnpRecords
} else {
    $warnings += 'Get-PnpDevice is unavailable; present-device enumeration was skipped.'
}

$driverRecords = @(Get-CimInstance Win32_PnPSignedDriver | Where-Object { $_.DeviceName } | Sort-Object DeviceClass, DeviceName | ForEach-Object {
    New-Record $_.DeviceName ([ordered]@{
        DeviceClass = $_.DeviceClass
        Manufacturer = $_.Manufacturer
        DriverProviderName = $_.DriverProviderName
        DriverVersion = $_.DriverVersion
        DriverDate = $_.DriverDate
        InfName = $_.InfName
        DeviceID = $_.DeviceID
        IsSigned = $_.IsSigned
        Signer = $_.Signer
    })
})
Add-Section 'Drivers' $driverRecords

$securityRecords = @()
if (Get-Command Get-Tpm -ErrorAction SilentlyContinue) {
    $tpm = Get-Tpm
    if ($tpm) {
        $securityRecords += New-Record 'TPM' ([ordered]@{
            TpmPresent = $tpm.TpmPresent
            TpmReady = $tpm.TpmReady
            TpmEnabled = $tpm.TpmEnabled
            TpmActivated = $tpm.TpmActivated
            ManufacturerIdTxt = $tpm.ManufacturerIdTxt
            ManufacturerVersion = $tpm.ManufacturerVersion
        })
    }
}
try {
    $secureBoot = Confirm-SecureBootUEFI
    $securityRecords += New-Record 'Secure Boot' ([ordered]@{ Enabled = $secureBoot })
} catch {}
Add-Section 'Security Firmware' $securityRecords

[pscustomobject]@{ sections = $sections; warnings = $warnings } | ConvertTo-Json -Depth 8 -Compress
"#;
