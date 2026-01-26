# Install spec-extract binary for Windows
# Usage: .\install-spec-extract.ps1 [version]

param(
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"

$Repo = "jackal-lch/opensdd"
$InstallDir = "$env:USERPROFILE\.local\bin"

Write-Host "[INFO] Installing spec-extract for Windows..." -ForegroundColor Green

# Construct download URL
if ($Version -eq "latest") {
    $DownloadUrl = "https://github.com/$Repo/releases/latest/download/spec-extract-windows-x64.zip"
} else {
    $DownloadUrl = "https://github.com/$Repo/releases/download/$Version/spec-extract-windows-x64.zip"
}

Write-Host "[INFO] Downloading from: $DownloadUrl" -ForegroundColor Green

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Download and extract
$TempFile = [System.IO.Path]::GetTempFileName() + ".zip"
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile
    Expand-Archive -Path $TempFile -DestinationPath $InstallDir -Force
} finally {
    Remove-Item -Path $TempFile -ErrorAction SilentlyContinue
}

Write-Host "[INFO] Installed spec-extract to $InstallDir\spec-extract.exe" -ForegroundColor Green

# Check if in PATH
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "[WARN] spec-extract installed but not in PATH" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Run this command to add to PATH (requires restart of terminal):"
    Write-Host ""
    Write-Host "  [Environment]::SetEnvironmentVariable('PATH', `$env:PATH + ';$InstallDir', 'User')"
    Write-Host ""
} else {
    Write-Host "[INFO] spec-extract is ready to use!" -ForegroundColor Green
    & "$InstallDir\spec-extract.exe" --version
}
