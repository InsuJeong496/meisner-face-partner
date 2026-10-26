[CmdletBinding()]
param(
    [string]$Change,
    [switch]$NativeBuild
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$repoRoot = Split-Path -Parent $PSScriptRoot
$appRoot = Join-Path $repoRoot '02_Program/meisner-face-app'
$startedAt = [DateTime]::UtcNow
$runId = $startedAt.ToString('yyyyMMddTHHmmssfffZ') + '-' + $PID
$outputRoot = Join-Path $repoRoot "artifacts/verification/$runId"
New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null

$specArguments = if ($Change) { @('validate', $Change) } else { @('validate', '--all') }
$specArguments += @('--strict', '--no-interactive', '--json')
$checks = @(
    @{ name = 'openspec'; command = 'openspec'; arguments = $specArguments },
    @{ name = 'frontend-types'; command = 'pnpm'; arguments = @('--dir', $appRoot, 'run', 'check') },
    @{ name = 'web-build'; command = 'pnpm'; arguments = @('--dir', $appRoot, 'run', 'build') },
    @{ name = 'rust-tests'; command = 'cargo'; arguments = @('test', '--manifest-path', (Join-Path $appRoot 'src-tauri/Cargo.toml'), '--locked') }
)
if ($NativeBuild) {
    $checks += @{ name = 'native-build'; command = 'pnpm'; arguments = @('--dir', $appRoot, 'exec', 'tauri', 'build', '--no-bundle') }
}
$steps = @($checks | ForEach-Object {
    [ordered]@{
        name = $_.name
        command = $_.command
        arguments = $_.arguments
        status = 'skipped'
        exitCode = $null
        startedAt = $null
        finishedAt = $null
        log = Join-Path $outputRoot ($_.name + '.log')
    }
})
$report = [ordered]@{
    startedAt = $startedAt.ToString('o')
    finishedAt = $null
    root = $repoRoot
    commit = $null
    dirty = $null
    workingTreeStatus = @()
    change = $Change
    status = 'running'
    checks = $steps
    notVerified = @(
        'Real Korean audio output, microphone permissions and recognition require Windows device verification.'
        'Web build and Rust unit tests do not establish native end-to-end user-flow correctness.'
        'Muse API integration is not part of this verification.'
    )
}
$failed = $false
Push-Location $repoRoot
try {
    $report.commit = (& git rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0) { throw 'Unable to identify the Git revision.' }
    $report.workingTreeStatus = @(& git status --porcelain)
    if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect Git working-tree state.' }
    $report.dirty = $report.workingTreeStatus.Count -gt 0

    foreach ($step in $steps) {
        $step.startedAt = [DateTime]::UtcNow.ToString('o')
        try {
            $tool = Get-Command $step.command -ErrorAction Stop
            $stepArguments = $step.arguments
            Write-Host "Checking $($step.name)..."
            & $tool.Source @stepArguments 2>&1 | Tee-Object -FilePath $step.log
            $step.exitCode = $LASTEXITCODE
            if ($step.exitCode -ne 0) { throw "$($step.name) exited with code $($step.exitCode)." }
            $step.status = 'passed'
        }
        catch {
            $step.status = 'failed'
            if ($null -eq $step.exitCode) { $step.exitCode = 1 }
            $_.ToString() | Out-File -LiteralPath $step.log -Append -Encoding utf8
            Write-Warning $_.ToString()
            $failed = $true
        }
        finally {
            $step.finishedAt = [DateTime]::UtcNow.ToString('o')
        }
        if ($failed) { break }
    }
}
catch {
    $failed = $true
    $report['error'] = $_.ToString()
    Write-Warning $_.ToString()
}
finally {
    Pop-Location
    $report.finishedAt = [DateTime]::UtcNow.ToString('o')
    $report.status = if ($failed) { 'failed' } else { 'passed' }
    $reportPath = Join-Path $outputRoot 'report.json'
    $report | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $reportPath -Encoding utf8
    Write-Host "Verification $($report.status): $reportPath"
}
if ($failed) { exit 1 }
exit 0
