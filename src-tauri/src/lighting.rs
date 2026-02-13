#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
/// Sets Windows Dynamic Lighting as the top priority controller
/// by swapping provider order in registry
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
            "-ExecutionPolicy", "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle", "Hidden",
            "-Command", script
        ])
        .creation_flags(0x08000000)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }
    
    Ok(())
}