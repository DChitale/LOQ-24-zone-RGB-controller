use std::process::Command;
use std::env;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const TASK_NAME: &str = "SetWindowsLightingOnTop";
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn create_startup_task(delay_seconds: u32) -> Result<(), Box<dyn std::error::Error>> {
    let script_content = r#"
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
            }
        }
    }
}

$providersPath = "HKCU:\Software\Microsoft\Lighting\Providers"
$devicesPath = "HKCU:\Software\Microsoft\Lighting\Devices"

Set-WindowsOnTop -path $providersPath

if (Test-Path $devicesPath) {
    Get-ChildItem -Path $devicesPath -Recurse -ErrorAction SilentlyContinue | 
        Where-Object { $_.PSChildName -eq "Providers" } | 
        ForEach-Object {
            Set-WindowsOnTop -path $_.PSPath
        }
}
"#;

    // Get AppData path
    let appdata = env::var("APPDATA")?;
    let script_path = format!("{}\\SetWindowsLightingOnTop.ps1", appdata);
    
    // Save the script file
    std::fs::write(&script_path, script_content)?;
    
    // Create scheduled task with configurable delay
    let task_script = format!(r#"
        $taskName = "{}"
        $scriptPath = "{}"
        $delaySeconds = {}
        
        # Delete existing task if it exists
        Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction SilentlyContinue
        
        # Create new task
        $action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument "-ExecutionPolicy Bypass -NoProfile -WindowStyle Hidden -File `"$scriptPath`""
        
        $trigger = New-ScheduledTaskTrigger -AtLogOn
        
        # Add user-configured delay
        $trigger.Delay = "PT$($delaySeconds)S"
        
        $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -Hidden
        
        # Register the task
        Register-ScheduledTask -TaskName $taskName -Action $action -Trigger $trigger -Settings $settings -Force | Out-Null
        
        Write-Output "Success"
    "#, TASK_NAME, script_path.replace("\\", "\\\\"), delay_seconds);
    
    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy", "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle", "Hidden",
            "-Command", &task_script
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }
    
    Ok(())
}


pub fn remove_startup_task() -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(r#"
        Unregister-ScheduledTask -TaskName "{}" -Confirm:$false -ErrorAction SilentlyContinue
        
        $scriptPath = "$env:APPDATA\SetWindowsLightingOnTop.ps1"
        if (Test-Path $scriptPath) {{
            Remove-Item $scriptPath -Force
        }}
        
        Write-Output "Success"
    "#, TASK_NAME);
    
    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy", "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle", "Hidden",
            "-Command", &script
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.into());
    }
    
    Ok(())
}

pub fn is_startup_task_installed() -> bool {
    let script = format!(r#"
        $task = Get-ScheduledTask -TaskName "{}" -ErrorAction SilentlyContinue
        if ($task) {{
            Write-Output "exists"
        }}
    "#, TASK_NAME);
    
    let output = Command::new("powershell")
        .args([
            "-ExecutionPolicy", "Bypass",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle", "Hidden",
            "-Command", &script
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    
    if let Ok(output) = output {
        let result = String::from_utf8_lossy(&output.stdout);
        return result.trim() == "exists";
    }
    
    false
}