param(
    [string]$OutputDir = "target\zones\us-national-exploratory",
    [double]$MaxAllowableOffset = 0.05,
    [int]$GeometryPrecision = 2,
    [int]$DstDeltaMinutes = 60
)

$ErrorActionPreference = "Stop"

$outputPath = New-Item -ItemType Directory -Force -Path $OutputDir
$boundaryPath = Join-Path $outputPath.FullName "county-boundaries.geojson"
$planPath = Join-Path $outputPath.FullName "plan-input.json"
$packetPath = Join-Path $outputPath.FullName "candidate-maps"
if (Test-Path $packetPath) {
    Remove-Item -Recurse -Force $packetPath
}

$query = @{
    where              = "1=1"
    outFields          = "GEOID,NAME,STATE,COUNTY,INTPTLAT,INTPTLON"
    f                  = "geojson"
    outSR              = "4326"
    geometryPrecision  = $GeometryPrecision
    maxAllowableOffset = $MaxAllowableOffset
    returnGeometry     = "true"
}

$baseUri = "https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/State_County/MapServer/1/query"
$uri = $baseUri + "?" + (($query.GetEnumerator() | ForEach-Object {
            "{0}={1}" -f [uri]::EscapeDataString($_.Key), [uri]::EscapeDataString([string]$_.Value)
        }) -join "&")

$geojson = Invoke-RestMethod -Uri $uri
$features = @($geojson.features | Sort-Object { $_.properties.GEOID })
if ($features.Count -lt 3000) {
    throw "Expected national county features, got $($features.Count)."
}

foreach ($feature in $features) {
    $geoid = ([string]$feature.properties.GEOID).PadLeft(5, "0")
    $feature | Add-Member -NotePropertyName id -NotePropertyValue $geoid -Force
    $feature.properties | Add-Member -NotePropertyName unit_id -NotePropertyValue $geoid -Force
    $feature.properties | Add-Member -NotePropertyName source -NotePropertyValue "Census TIGERweb county layer, generalized with maxAllowableOffset=$MaxAllowableOffset and geometryPrecision=$GeometryPrecision for ignored local national exploratory maps" -Force
}

$boundary = [ordered]@{
    type     = "FeatureCollection"
    name     = "zones_us_national_county_boundaries_exploratory"
    features = $features
}
$boundary | ConvertTo-Json -Depth 100 -Compress | Set-Content -Path $boundaryPath -Encoding utf8

$zoneOffsets = for ($minutes = -660; $minutes -le 660; $minutes += 60) { $minutes }
$zones = @($zoneOffsets | ForEach-Object {
        $prefix = if ($_ -ge 0) { "utc-plus" } else { "utc-minus" }
        [ordered]@{
            id                 = "{0}-{1:00}-00" -f $prefix, [math]::Abs($_ / 60)
            utc_offset_minutes = $_
        }
    })
$zoneIndexByOffset = @{}
for ($index = 0; $index -lt $zones.Count; $index++) {
    $zoneIndexByOffset[[int]$zones[$index].utc_offset_minutes] = $index
}

$units = New-Object System.Collections.Generic.List[object]
$assignment = New-Object System.Collections.Generic.List[int]
$adjacency = New-Object System.Collections.Generic.List[object]
foreach ($feature in $features) {
    $properties = $feature.properties
    $geoid = ([string]$properties.GEOID).PadLeft(5, "0")
    $latitude = [double]$properties.INTPTLAT
    $longitude = [double]$properties.INTPTLON
    $solarOffset = $longitude * 4.0
    $nearestOffset = [int]([math]::Round($solarOffset / 60.0) * 60)
    $nearestOffset = [math]::Max([math]::Min($nearestOffset, ($zoneOffsets | Select-Object -Last 1)), ($zoneOffsets | Select-Object -First 1))

    $units.Add([ordered]@{
            id                   = $geoid
            name                 = $properties.NAME
            solar_offset_minutes = $solarOffset
            population           = 1
            map_point            = [ordered]@{
                latitude  = $latitude
                longitude = $longitude
                source_id = "census-tigerweb-counties"
            }
            source_refs          = [ordered]@{
                boundary_source_id             = "census-tiger-counties-2024"
                representative_point_source_id = "census-tigerweb-counties"
                population_source_id           = "exploratory-unit-weight"
                time_zone_assignment_source_id = "analytic-longitude-nearest-whole-hour"
                time_zone_geometry_source_id   = "not-current-law"
                caveats                        = @(
                    "Exploratory national map unit generated from Census TIGERweb internal point and boundary geometry.",
                    "Population is set to 1 for equal-unit visual inspection; this is not a population-weighted national scorecard.",
                    "Assigned offset is nearest whole-hour longitude-derived analytic offset, not current law."
                )
            }
        })
    $assignment.Add([int]$zoneIndexByOffset[$nearestOffset])
    $adjacency.Add([object[]]@())
}

$plan = [ordered]@{
    input_id             = "zones-us-national-county-exploratory-plan-input"
    source_manifest_id   = "zones-us-foundation-sources"
    scenario             = [ordered]@{
        scenario_id = "us-national-county-longitude-exploratory"
        kind        = "analytic-counterfactual"
        label       = "US national county exploratory longitude-offset map"
    }
    units                = $units
    adjacency            = $adjacency
    plan                 = [ordered]@{
        name       = "us-national-county-nearest-whole-hour-exploratory"
        zones      = $zones
        assignment = $assignment
    }
    reference_assignment = $assignment
    caveats              = @(
        "Ignored local national map artifact for visual inspection only.",
        "Not current law, not legal advice, not a recommendation, and not a publication-ready national scorecard.",
        "County boundaries are generalized from Census TIGERweb for browser/SVG performance.",
        "Population is equal-unit placeholder; national source-derived population weighting remains future work.",
        "Adjacency is intentionally empty in this first national map packet; contiguity scores are not interpretable."
    )
}
$plan | ConvertTo-Json -Depth 100 | Set-Content -Path $planPath -Encoding utf8

cargo run -p zones-cli -- write-offset-candidate-maps $planPath --geojson $boundaryPath --require-all-units --dst-delta-minutes $DstDeltaMinutes --output-dir $packetPath
if ($LASTEXITCODE -ne 0) {
    throw "zones-cli write-offset-candidate-maps failed with exit code $LASTEXITCODE."
}

$svgPath = Join-Path $packetPath "baseline\maps\current-standard.svg"
$svg = Get-Content $svgPath -Raw
if ($svg -notmatch "<path" -or $svg -notmatch "fill-rule=`"evenodd`"") {
    throw "National exploratory SVG did not render boundary paths."
}

$joinReport = Get-Content (Join-Path $packetPath "geometry-join-report.json") -Raw | ConvertFrom-Json
if ($joinReport.matched_unit_count -ne $features.Count) {
    throw "Expected $($features.Count) joined units, got $($joinReport.matched_unit_count)."
}

[pscustomobject]@{
    units          = $features.Count
    plan           = $planPath
    boundaries     = $boundaryPath
    packet_index   = Join-Path $packetPath "index.html"
    standard_svg   = $svgPath
    recommendation = "closed"
}
