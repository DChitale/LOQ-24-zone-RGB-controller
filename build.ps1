# build.ps1
# Build script for Lenovo 24-Zone RGB Controller (WinUI 3 + Rust DLL)

Write-Host "1/2. Building Rust backend DLL..." -ForegroundColor Cyan
Push-Location rust-backend
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Rust compilation failed!"
    Pop-Location
    exit $LASTEXITCODE
}
Pop-Location

Write-Host "2/2. Building WinForms C# application..." -ForegroundColor Cyan
& "C:\Program Files\dotnet\dotnet.exe" build RGBController\RGBController.csproj --configuration Release -r win-x64 --self-contained true
if ($LASTEXITCODE -ne 0) {
    Write-Error "C# compilation failed!"
    exit $LASTEXITCODE
}

Write-Host "Build complete! To run the application, launch the executable in: RGBController\bin\Release\net8.0-windows10.0.19041.0\win-x64\RGBController.exe" -ForegroundColor Green
