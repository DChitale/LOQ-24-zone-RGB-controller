#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Sets Windows Dynamic Lighting as the top priority controller
/// by swapping provider order in registry

pub fn enable_windows_lighting() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        $lightingPath = "HKCU:\Software\Microsoft\Lighting"

        if (-not (Test-path $lightingPath)) {
            New-Item -Path $lightingPath -Force | Out-Null
        }

        Set-ItemProperty -Path $lightingPath -Name "IsEnabled" -Value 1 -Type DWord -Force

        Write-Output "Enabled"
    "#;

    let output = Command::new("powershell")
        .args([
            "-ExecutionPloicy",
            "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-Command", script
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }

    Ok(())
}

pub fn disable_windows_lighting() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        $lightingPath = "HKCU:\Software\Microsoft\Lighting"
        
        if (Test-Path $lightingPath) {
            Set-ItemProperty -Path $lightingPath -Name "IsEnabled" -Value 0 -Type DWord -Force
        }
        
        Write-Output "Disabled"
    "#;

    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy",
            "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }

    Ok(())
}

pub fn is_windows_lighting_enabled() -> bool {
    let script = r#"
        $lightingPath = "HKCU:\Software\Microsoft\Lighting"
        if (Test-Path $lightingPath) {
            $props = Get-ItemProperty -Path $lightingPath -ErrorAction SilentlyContinue
            if ($props.IsEnabled -eq 1) {
                Write-Output "enabled"
            }
        }
    "#;

    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy",
            "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if let Ok(output) = output {
        let result = String::from_utf8_lossy(&output.stdout);
        return result.trim() == "enabled";
    }

    false
}

pub fn set_windows_lighting_on_top() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        # Set Windows Dynamic Lighting as top controller
        
        function Set-WindowsOnTop {
            param($path)
            
            if (Test-Path $path) {
                $props = Get-ItemProperty -Path $path -ErrorAction SilentlyContinue
                if ($props) {
                    $provider1 = $props.'1'
                    $provider2 = $props.'2'
                    
                    if ($provider1 -ne "WindowsLighting" -and $provider2 -eq "WindowsLighting") {
                        Set-ItemProperty -Path $path -Name "1" -Value $provider2 -Force
                        Set-ItemProperty -Path $path -Name "2" -Value $provider1 -Force
                        return $true
                    }
                }
            }
            return $false
        }
        
        $providersPath = "HKCU:\Software\Microsoft\Lighting\Providers"
        $devicesPath = "HKCU:\Software\Microsoft\Lighting\Devices"
        
        Set-WindowsOnTop -path $providersPath | Out-Null
        
        if (Test-Path $devicesPath) {
            Get-ChildItem -Path $devicesPath -Recurse -ErrorAction SilentlyContinue | 
                Where-Object { $_.PSChildName -eq "Providers" } | 
                ForEach-Object {
                    Set-WindowsOnTop -path $_.PSPath | Out-Null
                }
        }
    "#;

    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy",
            "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }

    Ok(())
}
