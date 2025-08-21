# CURSED Programming Language - Chocolatey Uninstallation Script
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch

$ErrorActionPreference = 'Stop'

$packageName = 'cursed'
$version = '1.0.0'

Write-Host "Uninstalling CURSED Programming Language v$version..." -ForegroundColor Yellow

# Remove shims
$shimsToRemove = @('cursed', 'cursed-lsp', 'cursed-fmt', 'cursed-lint', 'cursed-doc', 'cursed-pkg')

foreach ($shim in $shimsToRemove) {
    try {
        Uninstall-ChocolateyShim -Name $shim
        Write-Host "✓ Removed shim: $shim" -ForegroundColor Green
    } catch {
        Write-Warning "Could not remove shim '$shim': $($_.Exception.Message)"
    }
}

# Remove environment variables
try {
    Uninstall-ChocolateyEnvironmentVariable -VariableName 'CURSED_HOME' -VariableType 'Machine'
    Write-Host "✓ Removed CURSED_HOME environment variable" -ForegroundColor Green
} catch {
    Write-Warning "Could not remove CURSED_HOME environment variable: $($_.Exception.Message)"
}

try {
    Uninstall-ChocolateyEnvironmentVariable -VariableName 'CURSED_STDLIB_PATH' -VariableType 'Machine'
    Write-Host "✓ Removed CURSED_STDLIB_PATH environment variable" -ForegroundColor Green
} catch {
    Write-Warning "Could not remove CURSED_STDLIB_PATH environment variable: $($_.Exception.Message)"
}

# Remove desktop shortcut if it exists
$desktop = [Environment]::GetFolderPath("Desktop")
$shortcutPath = Join-Path $desktop "CURSED Compiler.lnk"
if (Test-Path $shortcutPath) {
    Remove-Item $shortcutPath -Force
    Write-Host "✓ Removed desktop shortcut" -ForegroundColor Green
}

Write-Host ""
Write-Host "✅ CURSED Programming Language v$version uninstalled successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Thank you for using CURSED! 👋" -ForegroundColor Yellow
Write-Host ""
Write-Host "If you encountered any issues, please report them:" -ForegroundColor White
Write-Host "  GitHub Issues: https://github.com/ghuntley/cursed/issues" -ForegroundColor White
Write-Host "  Discord: https://discord.gg/cursedlang" -ForegroundColor White
Write-Host ""
