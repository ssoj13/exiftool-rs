#!/usr/bin/env pwsh
# Bootstrap script for exiftool-rs
#
# Usage:
#   .\bootstrap.ps1              # Show help
#   .\bootstrap.ps1 build        # Build all crates (release)
#   .\bootstrap.ps1 build --debug # Build all crates (debug)
#   .\bootstrap.ps1 python       # Build Python wheel (release)
#   .\bootstrap.ps1 python --debug # Build Python wheel (debug)
#   .\bootstrap.ps1 python dev   # Install Python module in dev mode

param(
    [Parameter(Position=0)]
    [string]$Command,
    
    [Parameter(Position=1, ValueFromRemainingArguments=$true)]
    [string[]]$Args
)

# Check cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host 'Error: Rust/Cargo not found!' -ForegroundColor Red
    Write-Host 'Install from: https://rustup.rs/'
    exit 1
}

switch ($Command) {
    'build' {
        Write-Host 'Building exiftool-rs...' -ForegroundColor Cyan
        Write-Host ''
        
        if ($Args -contains '--debug') {
            cargo build
        } else {
            cargo build --release
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ''
            Write-Host '[OK] Build complete!' -ForegroundColor Green
        }
        exit $LASTEXITCODE
    }
    
    'python' {
        Write-Host 'Building Python module (exiftool-py)...' -ForegroundColor Cyan
        Write-Host ''

        # Check maturin
        if (-not (Get-Command maturin -ErrorAction SilentlyContinue)) {
            Write-Host 'Installing maturin...'
            pip install maturin
            if ($LASTEXITCODE -ne 0) {
                Write-Host 'Error: Failed to install maturin' -ForegroundColor Red
                exit 1
            }
            Write-Host '[OK] maturin installed' -ForegroundColor Green
            Write-Host ''
        }

        $manifestPath = 'crates\exiftool-py\Cargo.toml'
        $pythonDir = 'crates\exiftool-py\python\exiftool_py'
        $subCmd = if ($Args.Count -gt 0) { $Args[0] } else { '' }

        # Clean .pyd files to avoid "already added" error
        Get-ChildItem -Path $pythonDir -Filter "*.pyd" -ErrorAction SilentlyContinue | Remove-Item -Force

        switch ($subCmd) {
            'dev' {
                Write-Host 'Installing in development mode...'
                maturin develop --manifest-path $manifestPath
            }
            { $_ -in 'install', '-i', '--install' } {
                Write-Host 'Building and installing wheel...'
                if ($Args -contains '--debug') {
                    maturin build --manifest-path $manifestPath
                } else {
                    maturin build --manifest-path $manifestPath --release
                }
                if ($LASTEXITCODE -eq 0) {
                    $wheel = Get-ChildItem -Path 'target\wheels' -Filter "exiftool_py*.whl" |
                             Sort-Object LastWriteTime -Descending | Select-Object -First 1
                    if ($wheel) {
                        Write-Host "Installing $($wheel.Name)..."
                        pip install $wheel.FullName --force-reinstall
                    }
                }
            }
            default {
                if ($Args -contains '--debug') {
                    Write-Host 'Building debug wheel...'
                    maturin build --manifest-path $manifestPath
                } else {
                    Write-Host 'Building release wheel...'
                    maturin build --manifest-path $manifestPath --release
                }
            }
        }

        if ($LASTEXITCODE -eq 0) {
            Write-Host ''
            Write-Host '[OK] Python module built!' -ForegroundColor Green
            Write-Host ''
            Write-Host 'Usage:'
            Write-Host '  import exiftool_py as exif'
            Write-Host '  img = exif.open("photo.jpg")'
            Write-Host '  print(img.make, img.model)'
        }
        exit $LASTEXITCODE
    }
    
    'test' {
        Write-Host 'Running tests...' -ForegroundColor Cyan
        cargo test @Args
        exit $LASTEXITCODE
    }
    
    'codegen' {
        Write-Host 'Regenerating tag tables...' -ForegroundColor Cyan
        cargo xtask codegen
        exit $LASTEXITCODE
    }
    
    default {
        Write-Host 'exiftool-rs bootstrap script' -ForegroundColor Cyan
        Write-Host ''
        Write-Host 'USAGE:'
        Write-Host '  .\bootstrap.ps1 [COMMAND] [OPTIONS]'
        Write-Host ''
        Write-Host 'COMMANDS:'
        Write-Host '  build              Build all Rust crates (release)'
        Write-Host '  build --debug      Build in debug mode'
        Write-Host '  python             Build Python wheel (release)'
        Write-Host '  python --debug     Build Python wheel (debug)'
        Write-Host '  python dev         Install in development mode (editable, debug)'
        Write-Host '  python install     Build and pip install (release)'
        Write-Host '  test               Run tests'
        Write-Host '  codegen            Regenerate tag tables from ExifTool'
        Write-Host ''
        Write-Host 'EXAMPLES:'
        Write-Host '  .\bootstrap.ps1 build'
        Write-Host '  .\bootstrap.ps1 build --debug'
        Write-Host '  .\bootstrap.ps1 python'
        Write-Host '  .\bootstrap.ps1 python --debug'
        Write-Host '  .\bootstrap.ps1 python dev'
        Write-Host '  .\bootstrap.ps1 test'
    }
}
