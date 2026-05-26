use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use zones_core::{
    evaluate_zone_plan, evaluate_zone_plan_input_with_manifest, seed_fixture, seed_plan_input,
    seed_source_manifest, SourceManifest, ZonePlanInput,
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
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let source_bytes = fs::read(&source_manifest).with_context(|| {
                format!(
                    "failed to read source manifest {}",
                    source_manifest.display()
                )
            })?;
            let manifest: SourceManifest =
                serde_json::from_slice(&source_bytes).with_context(|| {
                    format!(
                        "failed to parse source manifest {}",
                        source_manifest.display()
                    )
                })?;
            let report = evaluate_zone_plan_input_with_manifest(&input, &manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
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
