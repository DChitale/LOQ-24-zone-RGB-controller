; setup.iss
; Inno Setup Script for LOQ RGB Controller
; To compile: Download Inno Setup (https://jrsoftware.org/isinfo.php), open this file, and press Build.

#define AppName "LOQ RGB Controller"
#define AppVersion "3.1.2"
#define AppPublisher "DChitale"
#define AppExeName "RGBController.exe"

[Setup]
; Unique AppId (randomly generated GUID)
AppId={{C151B124-DF9C-43EE-8975-CE4E37A47D4C}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
; Output folder and setup file name
OutputDir=.
OutputBaseFilename=LOQ_RGB_Controller_Setup
SetupIconFile=RGBController\Assets\AppIcon.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
; Ensure the install dir is fully removed on uninstall
UninstallFilesDir={app}\uninstall
; Close the running app before uninstalling
CloseApplications=yes
CloseApplicationsFilter=*.exe

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; Packages the entire compiled output including net8.0 self-contained libraries, dlls, and assets
Source: "RGBController\bin\Release\net8.0-windows\win-x64\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{group}\{cm:UninstallProgram,{#AppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(AppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Remove any files created at runtime inside the install folder (logs, debug output, etc.)
Type: filesandordirs; Name: "{app}"

[UninstallRun]
; Remove the scheduled task created by the app
Filename: "schtasks"; Parameters: "/delete /tn ""SetWindowsLightingOnTop"" /f"; Flags: runhidden; RunOnceId: "RemoveScheduledTask"
; Remove the Registry Run key for launch-on-startup
Filename: "reg"; Parameters: "delete ""HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"" /v ""LOQ RGB Controller"" /f"; Flags: runhidden; RunOnceId: "RemoveRunKey"
; Remove the AppData\Roaming\LightingControl folder (settings.json, active_preset.txt, PowerShell script)
Filename: "cmd"; Parameters: "/c rmdir /s /q ""%APPDATA%\LightingControl"""; Flags: runhidden; RunOnceId: "RemoveAppData"
; Remove the stale PowerShell script from AppData root (created by installer.rs)
Filename: "cmd"; Parameters: "/c del /f /q ""%APPDATA%\SetWindowsLightingOnTop.ps1"""; Flags: runhidden; RunOnceId: "RemoveStartupScript"

