param(
  [string]$SourcePath = 'SOURCES.md',
  [string]$OutputPath = 'audit/source-manifest.jsonl',
  [ValidateRange(1, 50)] [int]$BatchSize = 25,
  [int]$Offset = 0
)

$ErrorActionPreference = 'Stop'

function Get-SourceType([string]$Url) {
  if ($Url -match '^https://github\.com/') { return 'github' }
  if ($Url -match '^https://arxiv\.org/') { return 'paper' }
  if ($Url -match '^https://aclanthology\.org/') { return 'paper' }
  if ($Url -match '^https://doi\.org/') { return 'doi' }
  if ($Url -match '^https://developer\.android\.com|^https://ai\.google\.dev|^https://modelcontextprotocol\.io') { return 'official_docs' }
  return 'web_reference'
}

function Get-GitHubRepository([string]$Url) {
  if ($Url -match '^https://github\.com/([^/?#]+)/([^/?#]+)') {
    $owner = $Matches[1]
    $repo = $Matches[2]
    if ($repo -notin @('blob', 'tree', 'pull', 'issues')) { return "$owner/$repo" }
  }
  return $null
}

$raw = Get-Content -Raw -LiteralPath $SourcePath
$urls = [regex]::Matches($raw, 'https?://[^\s)>]+') |
  ForEach-Object { $_.Value.TrimEnd('.', ',', ';') } |
  Sort-Object -Unique
$batch = @($urls | Select-Object -Skip $Offset -First $BatchSize)
if ($batch.Count -eq 0) { throw 'Aucune URL dans ce lot.' }

$directory = Split-Path -Parent $OutputPath
if ($directory) { New-Item -ItemType Directory -Force -Path $directory | Out-Null }
$headers = @{ 'User-Agent' = 'op15-adaptive-harness-source-audit' }
if ($env:GH_TOKEN) { $headers.Authorization = "Bearer $env:GH_TOKEN" }

$records = foreach ($url in $batch) {
  $record = [ordered]@{
    url = $url; canonical_url = $url; source_type = Get-SourceType $url
    checked_at = (Get-Date).ToUniversalTime().ToString('o')
    collection = 'metadata_only'; http_status = $null; primary_evidence = $null
    licence = $null; archived = $null; language = $null; activity = $null
    decision = 'revalidate'; rationale = 'Collecte initiale : lecture primaire non encore faite.'
  }
  $repo = Get-GitHubRepository $url
  try {
    if ($repo) {
      if (-not $env:GH_TOKEN) {
        $record.primary_evidence = "https://github.com/$repo"
        $record.rationale = 'Métadonnées GitHub non collectées sans GH_TOKEN ; lecture primaire requise.'
      } else {
        $metadata = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo" -Headers $headers -TimeoutSec 20
        $record.http_status = 200; $record.canonical_url = $metadata.html_url
        $record.primary_evidence = $metadata.html_url; $record.licence = $metadata.license.spdx_id
        $record.archived = $metadata.archived; $record.language = $metadata.language
        $record.activity = $metadata.updated_at
      }
    } else {
      $response = Invoke-WebRequest -Uri $url -Method Head -MaximumRedirection 3 -Headers $headers -TimeoutSec 20
      $record.http_status = [int]$response.StatusCode
      $resolved = $response.BaseResponse.ResponseUri.AbsoluteUri
      if (-not $resolved) { $resolved = $url }
      $record.canonical_url = $resolved
      $record.primary_evidence = $resolved
    }
  } catch {
    $record.rationale = "Collecte incomplète : $($_.Exception.Message)"
  }
  [pscustomobject]$record
}

$records | ConvertTo-Json -Compress | Add-Content -LiteralPath $OutputPath
"Lot $Offset..$($Offset + $batch.Count - 1) collecte : $($batch.Count) URL. Sortie : $OutputPath"
