use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use zones_core::{
    evaluate_zone_plan, evaluate_zone_plan_evaluation_with_catalog,
    evaluate_zone_plan_input_with_manifest_and_catalog, seed_fixture,
    seed_module_boundary_contract, seed_plan_input, seed_source_limitation_matrix,
    seed_source_manifest, seed_temporal_dataset, seed_zone_catalog, ModuleBoundaryContract,
    SourceLimitationMatrix, SourceManifest, TemporalDataset, ZoneCatalog, ZonePlanInput,
};

#[derive(Debug, Parser)]
#[command(name = "zones")]
#[command(about = "Evaluate civic-boundary time-zone plans.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Status,
    SeedReport,
    SeedPlanInput,
    EvaluatePlan {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
        #[arg(long, default_value = "data/zone-catalogs/seed-offsets.json")]
        zone_catalog: PathBuf,
    },
    EvaluatePlanDetail {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
        #[arg(long, default_value = "data/zone-catalogs/seed-offsets.json")]
        zone_catalog: PathBuf,
    },
    WriteEvaluation {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
        #[arg(long, default_value = "data/zone-catalogs/seed-offsets.json")]
        zone_catalog: PathBuf,
        #[arg(long, default_value = "target/zones/seed-evaluation.json")]
        output: PathBuf,
        #[arg(long, default_value = "target/zones/seed-unit-scores.csv")]
        unit_scores_csv: PathBuf,
        #[arg(long, default_value = "target/zones/seed-zone-summaries.csv")]
        zone_summaries_csv: PathBuf,
    },
    SeedSources,
    SourceReport {
        #[arg(default_value = "data/source-manifests/us-foundation.json")]
        path: PathBuf,
    },
    SeedZoneCatalog,
    ZoneCatalogReport {
        #[arg(default_value = "data/zone-catalogs/seed-offsets.json")]
        path: PathBuf,
    },
    SeedTemporalDataset,
    TemporalDatasetReport {
        #[arg(default_value = "data/temporal-fixtures/non-us-pilot.json")]
        path: PathBuf,
    },
    SeedSourceLimitationMatrix,
    SourceLimitationReport {
        #[arg(default_value = "data/source-limitation-matrix/global-source-claims.json")]
        path: PathBuf,
    },
    SeedModuleBoundaries,
    ModuleBoundaryReport {
        #[arg(default_value = "data/module-boundaries/zones-rplan-rline.json")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Status => {
            println!("ZONES foundation workspace ready");
        }
        Command::SeedReport => {
            let (units, adjacency, plan) = seed_fixture();
            let report = evaluate_zone_plan(&units, &adjacency, &plan)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedPlanInput => {
            let input = seed_plan_input();
            println!("{}", serde_json::to_string_pretty(&input)?);
        }
        Command::EvaluatePlan {
            path,
            source_manifest,
            zone_catalog,
        } => {
            let (input, manifest, catalog) =
                read_plan_manifest_and_catalog(&path, &source_manifest, &zone_catalog)?;
            let report =
                evaluate_zone_plan_input_with_manifest_and_catalog(&input, &manifest, &catalog)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::EvaluatePlanDetail {
            path,
            source_manifest,
            zone_catalog,
        } => {
            let (input, manifest, catalog) =
                read_plan_manifest_and_catalog(&path, &source_manifest, &zone_catalog)?;
            let evaluation =
                evaluate_zone_plan_evaluation_with_catalog(&input, &manifest, &catalog)?;
            println!("{}", serde_json::to_string_pretty(&evaluation)?);
        }
        Command::WriteEvaluation {
            path,
            source_manifest,
            zone_catalog,
            output,
            unit_scores_csv,
            zone_summaries_csv,
        } => {
            let (input, manifest, catalog) =
                read_plan_manifest_and_catalog(&path, &source_manifest, &zone_catalog)?;
            let evaluation =
                evaluate_zone_plan_evaluation_with_catalog(&input, &manifest, &catalog)?;
            write_json(&output, &evaluation)?;
            write_unit_scores_csv(&unit_scores_csv, &evaluation.unit_scores)?;
            write_zone_summaries_csv(&zone_summaries_csv, &evaluation.zone_summaries)?;
            println!("{}", output.display());
            println!("{}", unit_scores_csv.display());
            println!("{}", zone_summaries_csv.display());
        }
        Command::SeedSources => {
            let manifest = seed_source_manifest();
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        Command::SourceReport { path } => {
            let bytes = fs::read(&path)
                .with_context(|| format!("failed to read source manifest {}", path.display()))?;
            let manifest: SourceManifest = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse source manifest {}", path.display()))?;
            let report = manifest.report()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedZoneCatalog => {
            let catalog = seed_zone_catalog();
            println!("{}", serde_json::to_string_pretty(&catalog)?);
        }
        Command::ZoneCatalogReport { path } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let catalog: ZoneCatalog = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = catalog.report()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedTemporalDataset => {
            let dataset = seed_temporal_dataset();
            println!("{}", serde_json::to_string_pretty(&dataset)?);
        }
        Command::TemporalDatasetReport { path } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let dataset: TemporalDataset = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = dataset.report()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedSourceLimitationMatrix => {
            let matrix = seed_source_limitation_matrix();
            println!("{}", serde_json::to_string_pretty(&matrix)?);
        }
        Command::SourceLimitationReport { path } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let matrix: SourceLimitationMatrix = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = matrix.report()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedModuleBoundaries => {
            let contract = seed_module_boundary_contract();
            println!("{}", serde_json::to_string_pretty(&contract)?);
        }
        Command::ModuleBoundaryReport { path } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let contract: ModuleBoundaryContract = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = contract.report()?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn read_plan_manifest_and_catalog(
    path: &PathBuf,
    source_manifest: &PathBuf,
    zone_catalog: &PathBuf,
) -> Result<(ZonePlanInput, SourceManifest, ZoneCatalog)> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let input: ZonePlanInput = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    let source_bytes = fs::read(source_manifest).with_context(|| {
        format!(
            "failed to read source manifest {}",
            source_manifest.display()
        )
    })?;
    let manifest: SourceManifest = serde_json::from_slice(&source_bytes).with_context(|| {
        format!(
            "failed to parse source manifest {}",
            source_manifest.display()
        )
    })?;
    let catalog_bytes = fs::read(zone_catalog)
        .with_context(|| format!("failed to read zone catalog {}", zone_catalog.display()))?;
    let catalog: ZoneCatalog = serde_json::from_slice(&catalog_bytes)
        .with_context(|| format!("failed to parse zone catalog {}", zone_catalog.display()))?;
    Ok((input, manifest, catalog))
}

fn write_json<T: serde::Serialize>(path: &PathBuf, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{json}\n"))
        .with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn write_unit_scores_csv(path: &PathBuf, scores: &[zones_core::ZoneUnitScore]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }
    let mut csv = String::from(
        "unit_id,unit_name,zone_id,reference_zone_id,moved_from_reference,population,solar_offset_minutes,zone_utc_offset_minutes,absolute_error_minutes\n",
    );
    for score in scores {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&score.unit_id),
            csv_cell(&score.unit_name),
            csv_cell(&score.zone_id),
            csv_cell(score.reference_zone_id.as_deref().unwrap_or("")),
            score
                .moved_from_reference
                .map(|moved| moved.to_string())
                .unwrap_or_default(),
            score.population,
            score.solar_offset_minutes,
            score.zone_utc_offset_minutes,
            score.absolute_error_minutes
        ));
    }
    fs::write(path, csv).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn write_zone_summaries_csv(path: &PathBuf, summaries: &[zones_core::ZoneSummary]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }
    let mut csv = String::from(
        "zone_id,unit_count,population,moved_unit_count,moved_population,weighted_mean_absolute_error_minutes,max_absolute_error_minutes\n",
    );
    for summary in summaries {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            csv_cell(&summary.zone_id),
            summary.unit_count,
            summary.population,
            summary.moved_unit_count,
            summary.moved_population,
            summary.weighted_mean_absolute_error_minutes,
            summary.max_absolute_error_minutes
        ));
    }
    fs::write(path, csv).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn csv_cell(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
