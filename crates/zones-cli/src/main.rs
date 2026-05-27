use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs,
    path::{Path, PathBuf},
};
use zones_core::{
    attach_geojson_geometries, build_offset_candidate_plan, compare_offset_candidate_plans,
    evaluate_offset_fit, evaluate_zone_plan, evaluate_zone_plan_evaluation_with_catalog,
    evaluate_zone_plan_input_with_manifest_and_catalog, rplan_context_intake_report, seed_fixture,
    seed_module_boundary_contract, seed_plan_input, seed_plan_input_with_map_points,
    seed_source_gate_policy, seed_source_limitation_matrix, seed_source_manifest,
    seed_temporal_dataset, seed_us_county_baseline_seed_plan_input,
    seed_us_county_baseline_smoke_plan_input, seed_us_county_seed_geometry_reconciliation,
    seed_us_county_seed_representative_points, seed_us_county_seed_rplan_context,
    seed_us_county_seed_time_zone_assignments, seed_us_county_smoke_representative_points,
    seed_us_county_smoke_rplan_context, seed_us_county_smoke_time_zone_assignments,
    seed_zone_catalog, zone_plan_source_ref_report, CountyGeometryReconciliationSet,
    CountyRepresentativePointSet, CountyTimeZoneAssignmentSet, GeometryJoinOptions,
    ModuleBoundaryContract, OffsetCandidateGrid, OffsetMapRenderOptions, OffsetMapView,
    SourceGatePolicy, SourceLimitationMatrix, SourceManifest, TemporalDataset, ZoneCatalog,
    ZonePlanInput,
};

#[derive(Debug, Parser)]
#[command(name = "zones")]
#[command(about = "Evaluate civic-boundary time-zone plans.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CandidateGridArg {
    WholeHour,
    HalfHour,
    QuarterHour,
}

impl From<CandidateGridArg> for OffsetCandidateGrid {
    fn from(value: CandidateGridArg) -> Self {
        match value {
            CandidateGridArg::WholeHour => Self::WholeHour,
            CandidateGridArg::HalfHour => Self::HalfHour,
            CandidateGridArg::QuarterHour => Self::QuarterHour,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Status,
    SeedReport,
    SeedPlanInput,
    SeedPlanInputMapPoints,
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
    OffsetFit {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
    },
    WriteOffsetFit {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
        #[arg(long, default_value = "target/zones/seed-offset-fit.json")]
        output: PathBuf,
        #[arg(long, default_value = "target/zones/seed-offset-fit-units.csv")]
        unit_scores_csv: PathBuf,
    },
    WriteOffsetMaps {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
        #[arg(long, default_value = "target/zones/maps")]
        output_dir: PathBuf,
    },
    WriteOffsetAtlas {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
        #[arg(long, default_value = "target/zones/offset-atlas")]
        output_dir: PathBuf,
    },
    WriteOffsetGeojson {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
        #[arg(long, default_value = "target/zones/seed-offset-fit.geojson")]
        output: PathBuf,
    },
    WriteOffsetCandidatePlan {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = CandidateGridArg::HalfHour)]
        grid: CandidateGridArg,
        #[arg(long, default_value = "target/zones/offset-candidate-plan.json")]
        output: PathBuf,
    },
    CompareOffsetCandidates {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "target/zones/offset-candidate-comparison.json")]
        output: PathBuf,
    },
    WriteOffsetCandidateMaps {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long)]
        geojson: Option<PathBuf>,
        #[arg(long, default_value = "unit_id")]
        unit_id_property: String,
        #[arg(long)]
        require_all_units: bool,
        #[arg(long, default_value_t = 60)]
        dst_delta_minutes: i32,
        #[arg(long, default_value = "target/zones/offset-candidate-maps")]
        output_dir: PathBuf,
    },
    AttachGeojsonGeometries {
        geojson: PathBuf,
        #[arg(long, default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "unit_id")]
        unit_id_property: String,
        #[arg(long)]
        require_all_units: bool,
        #[arg(long, default_value = "target/zones/plan-with-geometries.json")]
        output: PathBuf,
        #[arg(long, default_value = "target/zones/geometry-join-report.json")]
        report: PathBuf,
        #[arg(long, default_value = "target/zones/geometry-join-units.csv")]
        unit_status_csv: PathBuf,
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
    SourceRefReport {
        #[arg(default_value = "data/plan-inputs/seed-plan.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    SeedSourceGate,
    SourceGateReport {
        #[arg(default_value = "data/source-gates/us-foundation-source-gate.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    SeedCountyRplanContext,
    SeedCountySeedRplanContext,
    RplanContextReport {
        #[arg(default_value = "data/rplan-contexts/us-county-smoke-rplan-context.json")]
        path: PathBuf,
    },
    SeedCountyAssignments,
    SeedCountySeedAssignments,
    CountyAssignmentReport {
        #[arg(default_value = "data/legal-assignments/us-county-smoke-current-law.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    SeedCountyGeometryReconciliation,
    GeometryReconciliationReport {
        #[arg(
            default_value = "data/geometry-reconciliation/us-county-seed-dot-reconciliation.json"
        )]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    SeedRepresentativePoints,
    SeedCountySeedRepresentativePoints,
    RepresentativePointReport {
        #[arg(default_value = "data/representative-points/us-county-smoke-gazetteer.json")]
        path: PathBuf,
        #[arg(long, default_value = "data/source-manifests/us-foundation.json")]
        source_manifest: PathBuf,
    },
    SeedCountyBaselinePlanInput,
    SeedCountyBaselineSeedPlanInput,
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
        Command::SeedPlanInputMapPoints => {
            let input = seed_plan_input_with_map_points();
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
        Command::OffsetFit {
            path,
            dst_delta_minutes,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = evaluate_offset_fit(&input, dst_delta_minutes)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::WriteOffsetFit {
            path,
            dst_delta_minutes,
            output,
            unit_scores_csv,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = evaluate_offset_fit(&input, dst_delta_minutes)?;
            write_json(&output, &report)?;
            write_offset_fit_units_csv(&unit_scores_csv, &report.unit_scores)?;
            println!("{}", output.display());
            println!("{}", unit_scores_csv.display());
        }
        Command::WriteOffsetMaps {
            path,
            dst_delta_minutes,
            output_dir,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = evaluate_offset_fit(&input, dst_delta_minutes)?;
            fs::create_dir_all(&output_dir).with_context(|| {
                format!("failed to create output directory {}", output_dir.display())
            })?;
            for view in offset_map_views() {
                let svg = zones_core::render_offset_fit_svg(
                    &report,
                    view,
                    &OffsetMapRenderOptions::default(),
                );
                let path = output_dir.join(format!("{}.svg", view.slug()));
                fs::write(&path, svg)
                    .with_context(|| format!("failed to write {}", path.display()))?;
                println!("{}", path.display());
            }
        }
        Command::WriteOffsetAtlas {
            path,
            dst_delta_minutes,
            output_dir,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = evaluate_offset_fit(&input, dst_delta_minutes)?;
            fs::create_dir_all(&output_dir).with_context(|| {
                format!("failed to create output directory {}", output_dir.display())
            })?;
            let mut map_files = Vec::new();
            for view in offset_map_views() {
                let file_name = format!("{}.svg", view.slug());
                let svg = zones_core::render_offset_fit_svg(
                    &report,
                    view,
                    &OffsetMapRenderOptions::default(),
                );
                let path = output_dir.join(&file_name);
                fs::write(&path, svg)
                    .with_context(|| format!("failed to write {}", path.display()))?;
                map_files.push((view, file_name));
                println!("{}", path.display());
            }
            let index_path = output_dir.join("index.html");
            fs::write(&index_path, render_offset_atlas_html(&report, &map_files))
                .with_context(|| format!("failed to write {}", index_path.display()))?;
            println!("{}", index_path.display());
        }
        Command::WriteOffsetGeojson {
            path,
            dst_delta_minutes,
            output,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = evaluate_offset_fit(&input, dst_delta_minutes)?;
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create output directory {}", parent.display())
                })?;
            }
            fs::write(&output, zones_core::render_offset_fit_geojson(&report))
                .with_context(|| format!("failed to write {}", output.display()))?;
            println!("{}", output.display());
        }
        Command::WriteOffsetCandidatePlan { path, grid, output } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let candidate = build_offset_candidate_plan(&input, grid.into())?;
            write_json(&output, &candidate)?;
            println!("{}", output.display());
        }
        Command::CompareOffsetCandidates { path, output } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let report = compare_offset_candidate_plans(
                &input,
                &[
                    OffsetCandidateGrid::WholeHour,
                    OffsetCandidateGrid::HalfHour,
                    OffsetCandidateGrid::QuarterHour,
                ],
            )?;
            write_json(&output, &report)?;
            println!("{}", output.display());
        }
        Command::WriteOffsetCandidateMaps {
            path,
            geojson,
            unit_id_property,
            require_all_units,
            dst_delta_minutes,
            output_dir,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let mut input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            if let Some(geojson_path) = geojson {
                let geojson_text = fs::read_to_string(&geojson_path)
                    .with_context(|| format!("failed to read {}", geojson_path.display()))?;
                let join_report = attach_geojson_geometries(
                    &input,
                    &geojson_text,
                    &GeometryJoinOptions {
                        unit_id_property,
                        require_all_units,
                    },
                )?;
                write_json(&output_dir.join("geometry-join-report.json"), &join_report)?;
                input = join_report.input;
            }
            let grids = [
                OffsetCandidateGrid::WholeHour,
                OffsetCandidateGrid::HalfHour,
                OffsetCandidateGrid::QuarterHour,
            ];
            let comparison = compare_offset_candidate_plans(&input, &grids)?;
            write_json(&output_dir.join("candidate-comparison.json"), &comparison)?;

            let mut packet_links = Vec::new();
            write_offset_candidate_map_packet(
                &output_dir.join("current-law"),
                "Current-law baseline",
                &input,
                dst_delta_minutes,
            )?;
            packet_links.push((
                "Current-law baseline".to_string(),
                "current-law/atlas/index.html".to_string(),
                "current-law/offset-fit.geojson".to_string(),
            ));

            for grid in grids {
                let candidate = build_offset_candidate_plan(&input, grid)?;
                let label = format!("{} candidate grid", grid.label());
                let slug = grid.slug();
                write_offset_candidate_map_packet(
                    &output_dir.join(slug),
                    &label,
                    &candidate,
                    dst_delta_minutes,
                )?;
                packet_links.push((
                    label,
                    format!("{slug}/atlas/index.html"),
                    format!("{slug}/offset-fit.geojson"),
                ));
            }

            let index_path = output_dir.join("index.html");
            fs::write(
                &index_path,
                render_offset_candidate_maps_index_html(&input.input_id, &packet_links),
            )
            .with_context(|| format!("failed to write {}", index_path.display()))?;
            println!("{}", output_dir.display());
            println!("{}", index_path.display());
        }
        Command::AttachGeojsonGeometries {
            path,
            geojson,
            unit_id_property,
            require_all_units,
            output,
            report: report_path,
            unit_status_csv,
        } => {
            let bytes =
                fs::read(&path).with_context(|| format!("failed to read {}", path.display()))?;
            let input: ZonePlanInput = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            let geojson_text = fs::read_to_string(&geojson)
                .with_context(|| format!("failed to read {}", geojson.display()))?;
            let join_report = attach_geojson_geometries(
                &input,
                &geojson_text,
                &GeometryJoinOptions {
                    unit_id_property,
                    require_all_units,
                },
            )?;
            write_json(&output, &join_report.input)?;
            write_json(&report_path, &join_report)?;
            write_geometry_join_units_csv(&unit_status_csv, &join_report.unit_statuses)?;
            println!("{}", output.display());
            println!("{}", report_path.display());
            println!("{}", unit_status_csv.display());
            eprintln!(
                "matched {} units; {} unmatched units; {} unused features",
                join_report.matched_unit_count,
                join_report.unmatched_unit_ids.len(),
                join_report.unused_feature_unit_ids.len()
            );
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
        Command::SourceRefReport {
            path,
            source_manifest,
        } => {
            let (input, manifest) = read_plan_and_manifest(&path, &source_manifest)?;
            let report = zone_plan_source_ref_report(&input, &manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedSourceGate => {
            let policy = seed_source_gate_policy();
            println!("{}", serde_json::to_string_pretty(&policy)?);
        }
        Command::SourceGateReport {
            path,
            source_manifest,
        } => {
            let policy_bytes = fs::read(&path)
                .with_context(|| format!("failed to read source gate policy {}", path.display()))?;
            let policy: SourceGatePolicy =
                serde_json::from_slice(&policy_bytes).with_context(|| {
                    format!("failed to parse source gate policy {}", path.display())
                })?;
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
            let report = policy.report(&manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedCountyRplanContext => {
            let context = seed_us_county_smoke_rplan_context();
            println!("{}", serde_json::to_string_pretty(&context)?);
        }
        Command::SeedCountySeedRplanContext => {
            let context = seed_us_county_seed_rplan_context();
            println!("{}", serde_json::to_string_pretty(&context)?);
        }
        Command::RplanContextReport { path } => {
            let bytes = fs::read(&path)
                .with_context(|| format!("failed to read RPLAN context {}", path.display()))?;
            let context: rplan_core::RplanContext = serde_json::from_slice(&bytes)
                .with_context(|| format!("failed to parse RPLAN context {}", path.display()))?;
            let report = rplan_context_intake_report(&context)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedCountyAssignments => {
            let assignments = seed_us_county_smoke_time_zone_assignments();
            println!("{}", serde_json::to_string_pretty(&assignments)?);
        }
        Command::SeedCountySeedAssignments => {
            let assignments = seed_us_county_seed_time_zone_assignments();
            println!("{}", serde_json::to_string_pretty(&assignments)?);
        }
        Command::CountyAssignmentReport {
            path,
            source_manifest,
        } => {
            let assignment_bytes = fs::read(&path).with_context(|| {
                format!("failed to read county assignment set {}", path.display())
            })?;
            let assignments: CountyTimeZoneAssignmentSet =
                serde_json::from_slice(&assignment_bytes).with_context(|| {
                    format!("failed to parse county assignment set {}", path.display())
                })?;
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
            let report = assignments.report(&manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedCountyGeometryReconciliation => {
            let reconciliation = seed_us_county_seed_geometry_reconciliation();
            println!("{}", serde_json::to_string_pretty(&reconciliation)?);
        }
        Command::GeometryReconciliationReport {
            path,
            source_manifest,
        } => {
            let reconciliation_bytes = fs::read(&path).with_context(|| {
                format!("failed to read geometry reconciliation {}", path.display())
            })?;
            let reconciliation: CountyGeometryReconciliationSet =
                serde_json::from_slice(&reconciliation_bytes).with_context(|| {
                    format!("failed to parse geometry reconciliation {}", path.display())
                })?;
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
            let report = reconciliation.report(&manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedRepresentativePoints => {
            let points = seed_us_county_smoke_representative_points();
            println!("{}", serde_json::to_string_pretty(&points)?);
        }
        Command::SeedCountySeedRepresentativePoints => {
            let points = seed_us_county_seed_representative_points();
            println!("{}", serde_json::to_string_pretty(&points)?);
        }
        Command::RepresentativePointReport {
            path,
            source_manifest,
        } => {
            let point_bytes = fs::read(&path).with_context(|| {
                format!("failed to read representative points {}", path.display())
            })?;
            let points: CountyRepresentativePointSet = serde_json::from_slice(&point_bytes)
                .with_context(|| {
                    format!("failed to parse representative points {}", path.display())
                })?;
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
            let report = points.report(&manifest)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::SeedCountyBaselinePlanInput => {
            let input = seed_us_county_baseline_smoke_plan_input();
            println!("{}", serde_json::to_string_pretty(&input)?);
        }
        Command::SeedCountyBaselineSeedPlanInput => {
            let input = seed_us_county_baseline_seed_plan_input();
            println!("{}", serde_json::to_string_pretty(&input)?);
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
    let (input, manifest) = read_plan_and_manifest(path, source_manifest)?;
    let catalog_bytes = fs::read(zone_catalog)
        .with_context(|| format!("failed to read zone catalog {}", zone_catalog.display()))?;
    let catalog: ZoneCatalog = serde_json::from_slice(&catalog_bytes)
        .with_context(|| format!("failed to parse zone catalog {}", zone_catalog.display()))?;
    Ok((input, manifest, catalog))
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

fn offset_map_views() -> [OffsetMapView; 5] {
    [
        OffsetMapView::CurrentStandard,
        OffsetMapView::CurrentDst,
        OffsetMapView::BestWholeHour,
        OffsetMapView::BestHalfHour,
        OffsetMapView::BestQuarterHour,
    ]
}

fn render_offset_atlas_html(
    report: &zones_core::OffsetFitReport,
    map_files: &[(OffsetMapView, String)],
) -> String {
    let mut cards = String::new();
    for (view, file_name) in map_files {
        cards.push_str(&format!(
            "<section class=\"map-card\"><h2>{}</h2><object data=\"{}\" type=\"image/svg+xml\"></object></section>\n",
            html_escape(view.title()),
            html_escape(file_name)
        ));
    }
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>ZONES Offset Atlas</title>
<style>
:root {{
  color-scheme: light;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #f8fafc;
  color: #0f172a;
}}
body {{
  margin: 0;
}}
header {{
  padding: 28px 32px 20px;
  border-bottom: 1px solid #cbd5e1;
  background: #ffffff;
}}
h1 {{
  margin: 0 0 8px;
  font-size: 28px;
  line-height: 1.15;
}}
.summary {{
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 18px;
}}
.metric {{
  border: 1px solid #cbd5e1;
  border-radius: 6px;
  padding: 8px 10px;
  background: #f8fafc;
}}
.metric strong {{
  display: block;
  font-size: 18px;
}}
.metric span {{
  color: #475569;
  font-size: 12px;
}}
main {{
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(420px, 1fr));
  gap: 18px;
  padding: 20px;
}}
.map-card {{
  border: 1px solid #cbd5e1;
  border-radius: 8px;
  background: #ffffff;
  overflow: hidden;
}}
.map-card h2 {{
  margin: 0;
  padding: 14px 16px;
  font-size: 16px;
  border-bottom: 1px solid #e2e8f0;
}}
object {{
  width: 100%;
  min-height: 320px;
  display: block;
}}
</style>
</head>
<body>
<header>
<h1>ZONES Offset Atlas</h1>
<div>Input <code>{}</code>. Schematic maps compare current assigned offsets with candidate offset grids.</div>
<div class="summary">
<div class="metric"><strong>{:.1}</strong><span>current standard mean error</span></div>
<div class="metric"><strong>{:.1}</strong><span>DST-period mean error</span></div>
<div class="metric"><strong>{:.1}</strong><span>best whole-hour mean error</span></div>
<div class="metric"><strong>{:.1}</strong><span>best half-hour mean error</span></div>
<div class="metric"><strong>{:.1}</strong><span>best quarter-hour mean error</span></div>
</div>
</header>
<main>
{}
</main>
</body>
</html>
"#,
        html_escape(&report.input_id),
        report.current_weighted_mean_standard_error_minutes,
        report.current_weighted_mean_dst_error_minutes,
        report.best_whole_hour_weighted_mean_error_minutes,
        report.best_half_hour_weighted_mean_error_minutes,
        report.best_quarter_hour_weighted_mean_error_minutes,
        cards
    )
}

fn write_offset_candidate_map_packet(
    output_dir: &Path,
    label: &str,
    input: &ZonePlanInput,
    dst_delta_minutes: i32,
) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create output directory {}", output_dir.display()))?;
    write_json(&output_dir.join("plan-input.json"), input)?;
    let report = evaluate_offset_fit(input, dst_delta_minutes)?;
    write_json(&output_dir.join("offset-fit.json"), &report)?;
    let geojson_path = output_dir.join("offset-fit.geojson");
    fs::write(
        &geojson_path,
        zones_core::render_offset_fit_geojson(&report),
    )
    .with_context(|| format!("failed to write {}", geojson_path.display()))?;

    let maps_dir = output_dir.join("maps");
    fs::create_dir_all(&maps_dir)
        .with_context(|| format!("failed to create output directory {}", maps_dir.display()))?;
    let atlas_dir = output_dir.join("atlas");
    fs::create_dir_all(&atlas_dir)
        .with_context(|| format!("failed to create output directory {}", atlas_dir.display()))?;

    let mut map_files = Vec::new();
    for view in offset_map_views() {
        let file_name = format!("{}.svg", view.slug());
        let svg =
            zones_core::render_offset_fit_svg(&report, view, &OffsetMapRenderOptions::default());
        let map_path = maps_dir.join(&file_name);
        fs::write(&map_path, &svg)
            .with_context(|| format!("failed to write {}", map_path.display()))?;
        let atlas_path = atlas_dir.join(&file_name);
        fs::write(&atlas_path, svg)
            .with_context(|| format!("failed to write {}", atlas_path.display()))?;
        map_files.push((view, file_name));
    }

    let index_path = atlas_dir.join("index.html");
    fs::write(&index_path, render_offset_atlas_html(&report, &map_files))
        .with_context(|| format!("failed to write {}", index_path.display()))?;
    println!("{}: {}", label, output_dir.display());
    Ok(())
}

fn render_offset_candidate_maps_index_html(
    input_id: &str,
    packet_links: &[(String, String, String)],
) -> String {
    let mut cards = String::new();
    for (label, atlas_path, geojson_path) in packet_links {
        cards.push_str(&format!(
            "<li><strong>{}</strong>: <a href=\"{}\">atlas</a> | <a href=\"{}\">GeoJSON</a></li>\n",
            html_escape(label),
            html_escape(atlas_path),
            html_escape(geojson_path)
        ));
    }
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>ZONES Candidate Map Packet</title>
<style>
:root {{
  color-scheme: light;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #f8fafc;
  color: #0f172a;
}}
body {{
  margin: 0;
  padding: 32px;
}}
main {{
  max-width: 900px;
  margin: 0 auto;
  border: 1px solid #cbd5e1;
  border-radius: 10px;
  background: #ffffff;
  padding: 24px 28px;
}}
.gate {{
  border-left: 4px solid #b45309;
  background: #fffbeb;
  padding: 12px 14px;
  margin: 18px 0;
}}
li {{
  margin: 10px 0;
}}
</style>
</head>
<body>
<main>
<h1>ZONES Candidate Map Packet</h1>
<p>Input <code>{}</code>. These maps compare current-law and generated offset-grid counterfactuals for internal measurement.</p>
<div class="gate"><strong>Recommendation gate closed.</strong> Candidate maps are not preferred maps, enactment advice, or publication-ready national claims.</div>
<p><a href="candidate-comparison.json">Candidate comparison JSON</a></p>
<ul>
{}
</ul>
<p>Each packet includes <code>plan-input.json</code>, <code>offset-fit.json</code>, <code>offset-fit.geojson</code>, SVG maps, and a local atlas page.</p>
</main>
</body>
</html>
"#,
        html_escape(input_id),
        cards
    )
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
        "unit_id,unit_name,zone_id,time_zone_assignment_source_id,unit_source_caveats,reference_zone_id,moved_from_reference,population,solar_offset_minutes,zone_utc_offset_minutes,absolute_error_minutes\n",
    );
    for score in scores {
        let assignment_source_id = score
            .source_refs
            .as_ref()
            .and_then(|source_refs| source_refs.time_zone_assignment_source_id.as_deref())
            .unwrap_or("");
        let unit_source_caveats = score
            .source_refs
            .as_ref()
            .map(|source_refs| source_refs.caveats.join("; "))
            .unwrap_or_default();
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&score.unit_id),
            csv_cell(&score.unit_name),
            csv_cell(&score.zone_id),
            csv_cell(assignment_source_id),
            csv_cell(&unit_source_caveats),
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

fn write_offset_fit_units_csv(
    path: &PathBuf,
    scores: &[zones_core::OffsetFitUnitScore],
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }
    let mut ranked_scores = scores.to_vec();
    ranked_scores.sort_by(|left, right| {
        right
            .current_standard_error_minutes
            .partial_cmp(&left.current_standard_error_minutes)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| right.population.cmp(&left.population))
            .then_with(|| left.unit_id.cmp(&right.unit_id))
    });
    let mut csv = String::from(
        "rank,unit_id,unit_name,population,solar_offset_minutes,current_zone_id,current_standard_offset_minutes,current_standard_error_minutes,current_dst_offset_minutes,current_dst_error_minutes,best_whole_hour_offset_minutes,best_whole_hour_error_minutes,best_half_hour_offset_minutes,best_half_hour_error_minutes,best_quarter_hour_offset_minutes,best_quarter_hour_error_minutes\n",
    );
    for (rank, score) in ranked_scores.iter().enumerate() {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            rank + 1,
            csv_cell(&score.unit_id),
            csv_cell(&score.unit_name),
            score.population,
            score.solar_offset_minutes,
            csv_cell(&score.current_zone_id),
            score.current_standard_offset_minutes,
            score.current_standard_error_minutes,
            score.current_dst_offset_minutes,
            score.current_dst_error_minutes,
            score.best_whole_hour_offset_minutes,
            score.best_whole_hour_error_minutes,
            score.best_half_hour_offset_minutes,
            score.best_half_hour_error_minutes,
            score.best_quarter_hour_offset_minutes,
            score.best_quarter_hour_error_minutes,
        ));
    }
    fs::write(path, csv).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn write_geometry_join_units_csv(
    path: &PathBuf,
    statuses: &[zones_core::GeometryJoinUnitStatus],
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }
    let mut csv = String::from("unit_id,matched,geometry_type\n");
    for status in statuses {
        csv.push_str(&format!(
            "{},{},{}\n",
            csv_cell(&status.unit_id),
            status.matched,
            csv_cell(status.geometry_type.as_deref().unwrap_or(""))
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
