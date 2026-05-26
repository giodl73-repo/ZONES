use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use zones_core::{
    evaluate_zone_plan, evaluate_zone_plan_evaluation, evaluate_zone_plan_input_with_manifest,
    seed_fixture, seed_plan_input, seed_source_manifest, SourceManifest, ZonePlanInput,
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
    },
    EvaluatePlanDetail {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    WriteEvaluation {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
        #[arg(long, default_value = "target/zones/seed-evaluation.json")]
        output: PathBuf,
    },
    SeedSources,
    SourceReport {
        #[arg(default_value = "data/source-manifests/us-foundation.json")]
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
        } => {
            let (input, manifest) = read_plan_and_manifest(&path, &source_manifest)?;
            let report = evaluate_zone_plan_input_with_manifest(&input, &manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::EvaluatePlanDetail {
            path,
            source_manifest,
        } => {
            let (input, manifest) = read_plan_and_manifest(&path, &source_manifest)?;
            let evaluation = evaluate_zone_plan_evaluation(&input, &manifest)?;
            println!("{}", serde_json::to_string_pretty(&evaluation)?);
        }
        Command::WriteEvaluation {
            path,
            source_manifest,
            output,
        } => {
            let (input, manifest) = read_plan_and_manifest(&path, &source_manifest)?;
            let evaluation = evaluate_zone_plan_evaluation(&input, &manifest)?;
            write_json(&output, &evaluation)?;
            println!("{}", output.display());
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
    }
    Ok(())
}

fn read_plan_and_manifest(
    path: &PathBuf,
    source_manifest: &PathBuf,
) -> Result<(ZonePlanInput, SourceManifest)> {
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
    Ok((input, manifest))
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
