param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CodexArgs
)

$ErrorActionPreference = "Continue"

$scriptDir = Split-Path -Parent -Path $MyInvocation.MyCommand.Definition
if (-not $scriptDir) { $scriptDir = (Get-Location).Path }

if ((Get-Location).Path -match "(?i)system32") {
    Set-Location -Path $scriptDir
}

<#
Forces the local API configuration into standard CLI configuration files.
Variable interpolation bypasses Join-Path array evaluation conflicts.
#>
$configPaths = @(
    "$env:USERPROFILE\.codex\config.json",
    "$env:USERPROFILE\.openai\config.json",
    "$env:APPDATA\codex\config.json",
    "$env:APPDATA\Configstore\codex.json",
    "$env:APPDATA\npm\node_modules\@openai\codex\config.json"
)

foreach ($path in $configPaths) {
    if (Test-Path -Path $path) {
        try {
            $config = Get-Content -Path $path -Raw | ConvertFrom-Json
            $modified = $false
            
            if ($null -ne $config) {
                if ($null -ne $config.apiBase -and $config.apiBase -ne "http://localhost:1234/v1") { 
                    $config.apiBase = "http://localhost:1234/v1"
                    $modified = $true 
                }
                if ($null -ne $config.baseUrl -and $config.baseUrl -ne "http://localhost:1234/v1") { 
                    $config.baseUrl = "http://localhost:1234/v1"
                    $modified = $true 
                }
                if ($null -ne $config.model -and $config.model -notmatch "local") { 
                    $config.model = "local-model"
                    $modified = $true 
                }
                
                if ($modified) {
                    $config | ConvertTo-Json -Depth 10 | Set-Content -Path $path -Force
                }
            }
        } catch {
            continue
        }
    }
}

$logFile = Join-Path -Path $scriptDir -ChildPath "codex_error.log"
$tempTranscript = Join-Path -Path $env:TEMP -ChildPath "codex_temp_transcript_$([guid]::NewGuid()).txt"

$env:OPENAI_API_BASE = "http://localhost:1234/v1"
$env:OPENAI_BASE_URL = "http://localhost:1234/v1"
$env:OPENAI_API_KEY = "lm-studio"

try {
    Start-Transcript -Path $tempTranscript -Force | Out-Null
    
    codex $CodexArgs
    
    Stop-Transcript | Out-Null
    
    if (Test-Path -Path $tempTranscript) {
        $output = Get-Content -Path $tempTranscript -Raw
        Remove-Item -Path $tempTranscript -ErrorAction SilentlyContinue
        
        <#
        Regex targets standard API failure responses and rate limit warnings
        that third-party CLI wrappers frequently output directly to stdout 
        rather than triggering standard system error streams.
        #>
        $errorPattern = "(?i)(failed|error|exception|limit|unauthorized|econnrefused|rate limit)"
        $hasErrors = $output -match $errorPattern
        
        if ($LASTEXITCODE -ne 0 -or $hasErrors) {
            $timestamp = (Get-Date).ToString("yyyy-MM-dd HH:mm:ss")
            $logEntry = @"
==================================================
[$timestamp] Exit Code: $LASTEXITCODE
[Captured Execution Log]
$output
==================================================

"@
            Add-Content -Path $logFile -Value $logEntry
        }
    }
} catch {
    $timestamp = (Get-Date).ToString("yyyy-MM-dd HH:mm:ss")
    $exceptionLog = "[$timestamp] Script Host Exception: $($_.Exception.Message)`n"
    Add-Content -Path $logFile -Value $exceptionLog
}