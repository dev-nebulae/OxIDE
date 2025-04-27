$ErrorActionPreference = "Stop"

$ProjectDir = Get-Location

Write-Host "Starting build in $ProjectDir..."

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Error: cargo is not installed. Please install Rust and Cargo first."
    exit 1
}

if (-not (Test-Path "$ProjectDir\Cargo.toml")) {
    Write-Error "Error: Cargo.toml not found. Are you in the Rust project root?"
    exit 1
}

cargo build --release

Write-Host "`u2705 Successfully built the project!"

