#!/usr/bin/env pwsh
#requires -Version 5.1
<#
.SYNOPSIS
    Build and run meal_manager as WebAssembly on Windows.

.DESCRIPTION
    This script compiles the meal_manager project to WebAssembly and starts a local HTTP server.
    It temporarily sets target-dir to project-local using CARGO_TARGET_DIR environment variable
    to work around cargo-makepad path issues on Windows.

.PARAMETER Port
    The port number for the HTTP server. Default is 8010 (unused, cargo-makepad uses its own port).

.EXAMPLE
    .\wasm_run.ps1
#>
param(
    [int]$Port = 8010
)

$ErrorActionPreference = "Stop"

$ROOT_DIR = Split-Path -Parent $PSScriptRoot

# Set local target directory for WASM build using environment variable
$LOCAL_TARGET_DIR = Join-Path $ROOT_DIR "target"
$env:CARGO_TARGET_DIR = $LOCAL_TARGET_DIR
Write-Host "Using local target directory: $LOCAL_TARGET_DIR" -ForegroundColor Gray

# Build and run the wasm app using cargo makepad
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Meal Manager WASM Build & Run" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Push-Location $ROOT_DIR
try
{
    & cargo makepad wasm --bindgen run -p meal_manager --release 2>&1 | ForEach-Object {
        Write-Host $_
    }

    if ($LASTEXITCODE -ne 0)
    {
        throw "Build/Run failed with exit code $LASTEXITCODE"
    }
} finally
{
    Pop-Location
}
