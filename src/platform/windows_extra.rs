use super::Collection;
use crate::command;
use std::{process::Command, time::Duration};

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    let powershell = command::windows_powershell()?;
    let mut process = Command::new(powershell);
    process.args([
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
        POWERSHELL_COLLECTOR,
    ]);

    let output = command::run_capture(&mut process, Duration::from_secs(90))?;
    if output.timed_out {
        return Err("Windows supplemental collector timed out after 90 seconds".into());
    }
    if !output.status.success() {
        return Err(format!(
            "Windows supplemental collector failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(serde_json::from_str(stdout.trim())?)
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

$productRecords = @(Get-CimInstance Win32_ComputerSystemProduct | ForEach-Object {
    New-Record 'Computer System Product' ([ordered]@{
        Vendor = $_.Vendor
        Name = $_.Name
        Version = $_.Version
        IdentifyingNumber = $_.IdentifyingNumber
        UUID = $_.UUID
        SKUNumber = $_.SKUNumber
        Caption = $_.Caption
    })
})
Add-Section 'System Product / UUID' $productRecords

$enclosureRecords = @(Get-CimInstance Win32_SystemEnclosure | ForEach-Object {
    New-Record 'System Enclosure' ([ordered]@{
        Manufacturer = $_.Manufacturer
        Model = $_.Model
        Version = $_.Version
        SerialNumber = $_.SerialNumber
        SMBIOSAssetTag = $_.SMBIOSAssetTag
        ChassisTypes = $_.ChassisTypes
        LockPresent = $_.LockPresent
        SecurityStatusCode = $_.SecurityStatus
        PowerSupplyStateCode = $_.PowerSupplyState
        ThermalStateCode = $_.ThermalState
    })
})
Add-Section 'Chassis / Enclosure' $enclosureRecords

$memoryArrayRecords = @(Get-CimInstance Win32_PhysicalMemoryArray | ForEach-Object {
    New-Record $_.Tag ([ordered]@{
        LocationCode = $_.Location
        UseCode = $_.Use
        MemoryErrorCorrectionCode = $_.MemoryErrorCorrection
        MaximumCapacityKB = $_.MaxCapacity
        MaximumCapacityBytes = $_.MaxCapacityEx
        NumberOfMemoryDevices = $_.MemoryDevices
    })
})
Add-Section 'Memory Arrays' $memoryArrayRecords

$cpuFeatureRecords = @(Get-CimInstance Win32_Processor | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        DataWidthBits = $_.DataWidth
        AddressWidthBits = $_.AddressWidth
        L2CacheSizeKB = $_.L2CacheSize
        L3CacheSizeKB = $_.L3CacheSize
        VirtualizationFirmwareEnabled = $_.VirtualizationFirmwareEnabled
        VMMonitorModeExtensions = $_.VMMonitorModeExtensions
        SecondLevelAddressTranslationExtensions = $_.SecondLevelAddressTranslationExtensions
        CurrentVoltageCode = $_.CurrentVoltage
        ExtClockMHz = $_.ExtClock
    })
})
Add-Section 'CPU Features / Cache' $cpuFeatureRecords

$reliabilityRecords = @()
if ((Get-Command Get-PhysicalDisk -ErrorAction SilentlyContinue) -and (Get-Command Get-StorageReliabilityCounter -ErrorAction SilentlyContinue)) {
    Get-PhysicalDisk | ForEach-Object {
        $disk = $_
        $counter = $disk | Get-StorageReliabilityCounter
        if ($counter) {
            $reliabilityRecords += New-Record $disk.FriendlyName ([ordered]@{
                DeviceId = $disk.DeviceId
                SerialNumber = $disk.SerialNumber
                TemperatureCelsius = $counter.Temperature
                TemperatureMaxCelsius = $counter.TemperatureMax
                WearPercent = $counter.Wear
                PowerOnHours = $counter.PowerOnHours
                ReadErrorsTotal = $counter.ReadErrorsTotal
                ReadErrorsUncorrected = $counter.ReadErrorsUncorrected
                WriteErrorsTotal = $counter.WriteErrorsTotal
                WriteErrorsUncorrected = $counter.WriteErrorsUncorrected
                ReadLatencyMax = $counter.ReadLatencyMax
                WriteLatencyMax = $counter.WriteLatencyMax
                FlushLatencyMax = $counter.FlushLatencyMax
                StartStopCycleCount = $counter.StartStopCycleCount
                LoadUnloadCycleCount = $counter.LoadUnloadCycleCount
            })
        }
    }
}
Add-Section 'Storage Reliability / SMART-like Counters' $reliabilityRecords

$firmwareRecords = @(Get-CimInstance Win32_PnPEntity | Where-Object { $_.PNPClass -eq 'Firmware' } | ForEach-Object {
    New-Record $_.DeviceID ([ordered]@{
        Name = $_.Name
        Manufacturer = $_.Manufacturer
        PNPClass = $_.PNPClass
        PNPDeviceID = $_.PNPDeviceID
        Service = $_.Service
        Status = $_.Status
    })
})
Add-Section 'Firmware Devices' $firmwareRecords

[pscustomobject]@{ sections = $sections; warnings = $warnings } | ConvertTo-Json -Depth 8 -Compress
"#;
