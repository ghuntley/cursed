# CURSED Programming Language - Chocolatey Installation Script
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch

$ErrorActionPreference = 'Stop'

# Package information
$packageName = 'cursed'
$version = '1.0.0'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = Join-Path $env:ChocolateyInstall "lib\$packageName\tools"

# URLs for download (will be updated by release automation)
$url64 = 'https://github.com/ghuntley/cursed/releases/download/v1.0.0/cursed-1.0.0-x86_64-pc-windows-gnu.zip'
$checksum64 = '0000000000000000000000000000000000000000000000000000000000000000'

Write-Host "Installing CURSED Programming Language v$version..." -ForegroundColor Green

# Download and extract package
$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  url64bit       = $url64
  checksum64     = $checksum64
  checksumType64 = 'sha256'
  validExitCodes = @(0)
}

Install-ChocolateyZipPackage @packageArgs

# Verify installation directory structure
$binDir = Join-Path $toolsDir "bin"
if (-not (Test-Path $binDir)) {
    throw "Installation failed: bin directory not found at $binDir"
}

# Verify core binaries exist
$requiredBinaries = @(
    'cursed.exe',
    'cursed-lsp.exe', 
    'cursed-fmt.exe',
    'cursed-lint.exe',
    'cursed-doc.exe',
    'cursed-pkg.exe'
)

foreach ($binary in $requiredBinaries) {
    $binaryPath = Join-Path $binDir $binary
    if (-not (Test-Path $binaryPath)) {
        Write-Warning "Binary not found: $binary at $binaryPath"
    } else {
        Write-Host "✓ Found: $binary" -ForegroundColor Green
    }
}

# Create shims for main executables
$shimConfigs = @(
    @{ Name = 'cursed'; Path = Join-Path $binDir 'cursed.exe'; Description = 'CURSED Compiler' },
    @{ Name = 'cursed-lsp'; Path = Join-Path $binDir 'cursed-lsp.exe'; Description = 'CURSED Language Server' },
    @{ Name = 'cursed-fmt'; Path = Join-Path $binDir 'cursed-fmt.exe'; Description = 'CURSED Formatter' },
    @{ Name = 'cursed-lint'; Path = Join-Path $bindir 'cursed-lint.exe'; Description = 'CURSED Linter' },
    @{ Name = 'cursed-doc'; Path = Join-Path $binDir 'cursed-doc.exe'; Description = 'CURSED Documentation Generator' },
    @{ Name = 'cursed-pkg'; Path = Join-Path $binDir 'cursed-pkg.exe'; Description = 'CURSED Package Manager' }
)

foreach ($config in $shimConfigs) {
    if (Test-Path $config.Path) {
        Install-ChocolateyShim -Name $config.Name -Path $config.Path
        Write-Host "✓ Created shim for $($config.Description)" -ForegroundColor Green
    } else {
        Write-Warning "Could not create shim for $($config.Name): binary not found at $($config.Path)"
    }
}

# Set up environment variables
$cursedHome = $toolsDir
$stdlibPath = Join-Path $cursedHome "stdlib"

# Set machine-level environment variables for system-wide access
Install-ChocolateyEnvironmentVariable -VariableName 'CURSED_HOME' -VariableValue $cursedHome -VariableType 'Machine'

if (Test-Path $stdlibPath) {
    Install-ChocolateyEnvironmentVariable -VariableName 'CURSED_STDLIB_PATH' -VariableValue $stdlibPath -VariableType 'Machine'
    Write-Host "✓ Set CURSED_STDLIB_PATH to $stdlibPath" -ForegroundColor Green
} else {
    Write-Warning "Standard library not found at $stdlibPath"
}

# Create desktop shortcuts (optional)
$createDesktopShortcuts = $env:ChocolateyPackageParameters -match 'desktop'
if ($createDesktopShortcuts) {
    $desktop = [Environment]::GetFolderPath("Desktop")
    $shortcutPath = Join-Path $desktop "CURSED Compiler.lnk"
    $cursedExe = Join-Path $binDir "cursed.exe"
    
    if (Test-Path $cursedExe) {
        Install-ChocolateyShortcut -ShortcutFilePath $shortcutPath -TargetPath $cursedExe -Description "CURSED Programming Language Compiler"
        Write-Host "✓ Created desktop shortcut" -ForegroundColor Green
    }
}

# Verify installation by running version check
try {
    $cursedExe = Join-Path $binDir "cursed.exe"
    if (Test-Path $cursedExe) {
        $versionOutput = & $cursedExe --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ Installation verified: $versionOutput" -ForegroundColor Green
        } else {
            Write-Warning "Version check returned non-zero exit code: $LASTEXITCODE"
        }
    }
} catch {
    Write-Warning "Could not verify installation: $($_.Exception.Message)"
}

# Display installation success message
Write-Host ""
Write-Host "🎉 CURSED Programming Language v$version installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Quick Start:" -ForegroundColor Yellow
Write-Host "  1. Create a new file: hello.💀" -ForegroundColor White
Write-Host "  2. Add content: yeet `"vibez`"; vibez.spill(`"Hello, Windows!`");" -ForegroundColor White
Write-Host "  3. Run it: cursed hello.💀" -ForegroundColor White
Write-Host ""
Write-Host "Available Commands:" -ForegroundColor Yellow
Write-Host "  cursed        - CURSED compiler" -ForegroundColor White
Write-Host "  cursed-lsp    - Language Server Protocol for IDEs" -ForegroundColor White  
Write-Host "  cursed-fmt    - Code formatter" -ForegroundColor White
Write-Host "  cursed-lint   - Static analyzer and linter" -ForegroundColor White
Write-Host "  cursed-doc    - Documentation generator" -ForegroundColor White
Write-Host "  cursed-pkg    - Package manager" -ForegroundColor White
Write-Host ""
Write-Host "IDE Integration:" -ForegroundColor Yellow
Write-Host "  VS Code: Install 'CURSED Language Support' extension" -ForegroundColor White
Write-Host "  Other editors: Use cursed-lsp for Language Server Protocol support" -ForegroundColor White
Write-Host ""
Write-Host "Documentation:" -ForegroundColor Yellow
Write-Host "  Website: https://cursedlang.org" -ForegroundColor White
Write-Host "  Docs: https://docs.cursedlang.org" -ForegroundColor White
Write-Host "  Community: https://discord.gg/cursedlang" -ForegroundColor White
Write-Host ""
Write-Host "Happy coding with CURSED! 🚀" -ForegroundColor Green
Write-Host ""

# Log installation details
Write-Host "Installation Details:" -ForegroundColor Yellow
Write-Host "  Install Directory: $toolsDir" -ForegroundColor White
Write-Host "  Binary Directory: $binDir" -ForegroundColor White
Write-Host "  CURSED_HOME: $cursedHome" -ForegroundColor White
if (Test-Path $stdlibPath) {
    Write-Host "  Standard Library: $stdlibPath" -ForegroundColor White
}
Write-Host "  Version: $version" -ForegroundColor White
