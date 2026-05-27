use rgraph_core::{assignment_label_connected, undirected_edge_cut, EdgeCutError};
use rplan_core::{
    CanonicalOrder, EdgeKind, EdgeSemantics, GeometryContext, PlanUnitIndex, RplanContext,
    SourceHashes, UnitEdge, UnitGraph, UnitKind, RCTX_VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceKind {
    LegalText,
    GeospatialBoundary,
    TimeRuleDatabase,
    Population,
    RepresentativePoint,
    DerivedManifest,
    ResearchNote,
    Imported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceCitation {
    pub source_id: String,
    pub title: String,
    pub kind: SourceKind,
    pub url: String,
    pub retrieved_on: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vintage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceManifest {
    pub manifest_id: String,
    pub generated_on: String,
    pub sources: Vec<SourceCitation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceManifestReport {
    pub manifest_id: String,
    pub source_count: usize,
    pub caveated_source_count: usize,
    pub legal_text_count: usize,
    pub geospatial_boundary_count: usize,
    pub time_rule_database_count: usize,
    pub population_count: usize,
    pub representative_point_count: usize,
    pub derived_manifest_count: usize,
    pub research_note_count: usize,
    pub imported_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceAcquisitionMode {
    ManualReference,
    FletchCandidate,
    LocalDerived,
    SyntheticFixture,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceCachePolicy {
    ReferenceOnly,
    IgnoredLocalCache,
    DerivedMetadataOnly,
    CommittedFixture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceGateEntry {
    pub source_id: String,
    pub acquisition_mode: SourceAcquisitionMode,
    pub cache_policy: SourceCachePolicy,
    pub rights_posture: String,
    pub expected_artifact: String,
    pub hash_required: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gate_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceGatePolicy {
    pub policy_id: String,
    pub source_manifest_id: String,
    pub generated_on: String,
    pub entries: Vec<SourceGateEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceGateReport {
    pub policy_id: String,
    pub source_manifest_id: String,
    pub source_count: usize,
    pub policy_entry_count: usize,
    pub covered_source_count: usize,
    pub missing_source_ids: Vec<String>,
    pub extra_entry_source_ids: Vec<String>,
    pub manual_reference_count: usize,
    pub fletch_candidate_count: usize,
    pub local_derived_count: usize,
    pub synthetic_fixture_count: usize,
    pub imported_count: usize,
    pub reference_only_count: usize,
    pub ignored_local_cache_count: usize,
    pub derived_metadata_only_count: usize,
    pub committed_fixture_count: usize,
    pub hash_required_count: usize,
    pub gate_note_count: usize,
    pub source_gate_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RplanContextIntakeReport {
    pub context_hash: String,
    pub computed_context_hash: String,
    pub context_hash_matches: bool,
    pub unit_kind: String,
    pub canonical_order: String,
    pub unit_count: usize,
    pub has_graph: bool,
    pub graph_edge_count: usize,
    pub has_populations: bool,
    pub population_count: usize,
    pub has_geometry_context: bool,
    pub source_hash_count: usize,
    pub rplan_context_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CountyAssignmentStatus {
    Placeholder,
    Reconciled,
    SplitCounty,
    Uncertain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyTimeZoneAssignment {
    pub unit_id: String,
    pub zone_id: String,
    pub legal_source_id: String,
    pub legal_clause: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_source_id: Option<String>,
    pub status: CountyAssignmentStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyTimeZoneAssignmentSet {
    pub assignment_id: String,
    pub source_manifest_id: String,
    pub generated_on: String,
    pub scenario_id: String,
    pub assignments: Vec<CountyTimeZoneAssignment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyTimeZoneAssignmentReport {
    pub assignment_id: String,
    pub source_manifest_id: String,
    pub assignment_count: usize,
    pub legal_source_ref_count: usize,
    pub legal_clause_count: usize,
    pub geometry_source_ref_count: usize,
    pub placeholder_count: usize,
    pub reconciled_count: usize,
    pub split_county_count: usize,
    pub uncertain_count: usize,
    pub caveated_assignment_count: usize,
    pub caveat_count: usize,
    pub assignment_evidence_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountyRepresentativePointRecord {
    pub unit_id: String,
    pub point: RepresentativePoint,
    pub solar_offset_minutes: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountyRepresentativePointSet {
    pub point_set_id: String,
    pub source_manifest_id: String,
    pub generated_on: String,
    pub records: Vec<CountyRepresentativePointRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountyRepresentativePointReport {
    pub point_set_id: String,
    pub source_manifest_id: String,
    pub record_count: usize,
    pub internal_point_count: usize,
    pub population_center_count: usize,
    pub geometry_centroid_count: usize,
    pub source_provided_count: usize,
    pub imported_count: usize,
    pub caveated_record_count: usize,
    pub caveat_count: usize,
    pub max_solar_offset_delta_minutes: f64,
    pub exploratory_point_method: bool,
    pub strong_claim_point_method_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GeometryReconciliationStatus {
    Pending,
    RepresentativePointMatched,
    Reconciled,
    SplitCounty,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyGeometryReconciliation {
    pub unit_id: String,
    pub assignment_zone_id: String,
    pub legal_source_id: String,
    pub geometry_source_id: String,
    pub status: GeometryReconciliationStatus,
    pub evidence_note: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyGeometryReconciliationSet {
    pub reconciliation_id: String,
    pub source_manifest_id: String,
    pub generated_on: String,
    pub rows: Vec<CountyGeometryReconciliation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountyGeometryReconciliationReport {
    pub reconciliation_id: String,
    pub source_manifest_id: String,
    pub row_count: usize,
    pub pending_count: usize,
    pub representative_point_matched_count: usize,
    pub reconciled_count: usize,
    pub split_county_count: usize,
    pub mismatch_count: usize,
    pub caveated_row_count: usize,
    pub caveat_count: usize,
    pub geometry_reconciliation_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZonePlanSourceRefReport {
    pub input_id: String,
    pub source_manifest_id: String,
    pub unit_count: usize,
    pub units_with_source_refs: usize,
    pub units_with_complete_source_refs: usize,
    pub units_missing_source_refs: usize,
    pub boundary_source_ref_count: usize,
    pub missing_boundary_source_ref_count: usize,
    pub representative_point_source_ref_count: usize,
    pub missing_representative_point_source_ref_count: usize,
    pub population_source_ref_count: usize,
    pub missing_population_source_ref_count: usize,
    pub time_zone_assignment_source_ref_count: usize,
    pub missing_time_zone_assignment_source_ref_count: usize,
    pub time_zone_geometry_source_ref_count: usize,
    pub missing_time_zone_geometry_source_ref_count: usize,
    pub units_with_source_caveats: usize,
    pub units_missing_source_caveats: usize,
    pub unit_source_caveat_count: usize,
    pub publishable_source_ref_coverage: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceClaim {
    OffsetRuleHistory,
    CurrentLegalOffset,
    LegalBoundaryGeometry,
    AdministrativeBoundaryGeometry,
    RepresentativePoint,
    PopulationWeights,
    DisplayMetadata,
    HistoricalReconstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceSupportLevel {
    Supports,
    Partial,
    NotSupported,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceClaimAssessment {
    pub claim: SourceClaim,
    pub support: SourceSupportLevel,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLimitationEntry {
    pub source_id: String,
    pub source_kind: SourceKind,
    pub title: String,
    pub assessments: Vec<SourceClaimAssessment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLimitationMatrix {
    pub matrix_id: String,
    pub generated_on: String,
    pub entries: Vec<SourceLimitationEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLimitationMatrixReport {
    pub matrix_id: String,
    pub entry_count: usize,
    pub assessment_count: usize,
    pub supports_count: usize,
    pub partial_count: usize,
    pub not_supported_count: usize,
    pub unknown_count: usize,
    pub caveated_entry_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleBoundaryEntry {
    pub module_id: String,
    pub owns: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_not_own: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upstream_dependencies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downstream_consumers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleBoundaryContract {
    pub contract_id: String,
    pub generated_on: String,
    pub entries: Vec<ModuleBoundaryEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleBoundaryReport {
    pub contract_id: String,
    pub module_count: usize,
    pub ownership_statement_count: usize,
    pub exclusion_statement_count: usize,
    pub dependency_edge_count: usize,
    pub downstream_edge_count: usize,
    pub caveat_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneCatalog {
    pub catalog_id: String,
    pub source_manifest_id: String,
    pub generated_on: String,
    pub zones: Vec<ZoneSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneCatalogReport {
    pub catalog_id: String,
    pub zone_count: usize,
    pub whole_hour_offset_count: usize,
    pub non_whole_hour_offset_count: usize,
    pub half_hour_offset_count: usize,
    pub quarter_hour_offset_count: usize,
    pub min_utc_offset_minutes: i32,
    pub max_utc_offset_minutes: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalExtent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

impl TemporalExtent {
    pub fn current() -> Self {
        Self {
            valid_from: None,
            valid_to: None,
        }
    }

    pub fn validate(&self) -> Result<(), TemporalModelError> {
        if let (Some(from), Some(to)) = (&self.valid_from, &self.valid_to) {
            if from >= to {
                return Err(TemporalModelError::InvalidTemporalExtent {
                    valid_from: from.clone(),
                    valid_to: to.clone(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnitLevel {
    Country,
    State,
    Province,
    County,
    Municipality,
    District,
    Imported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RepresentativePointMethod {
    GeometryCentroid,
    InternalPoint,
    PopulationCenter,
    SourceProvided,
    Imported,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepresentativePoint {
    pub latitude: f64,
    pub longitude: f64,
    pub method: RepresentativePointMethod,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Jurisdiction {
    pub jurisdiction_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_jurisdiction_id: Option<String>,
    pub source_id: String,
    pub temporal_extent: TemporalExtent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalBoundaryUnit {
    pub unit_id: String,
    pub jurisdiction_id: String,
    pub unit_level: UnitLevel,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_ref: Option<String>,
    pub representative_point: RepresentativePoint,
    pub temporal_extent: TemporalExtent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemporalEdgeKind {
    LandBoundary,
    WaterBoundary,
    Ferry,
    Bridge,
    PointTouch,
    AdministrativeException,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalUnitEdge {
    pub to: usize,
    pub kind: TemporalEdgeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryGraphVersion {
    pub graph_id: String,
    pub unit_universe_id: String,
    pub source_id: String,
    pub temporal_extent: TemporalExtent,
    pub adjacency: Vec<Vec<TemporalUnitEdge>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegimeAuthority {
    CurrentLaw,
    HistoricalLaw,
    ProposedScenario,
    AnalyticCounterfactual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetRule {
    pub rule_id: String,
    pub zone_id: String,
    pub standard_offset_minutes: i32,
    pub temporal_extent: TemporalExtent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_delta_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_rule_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observance_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeZoneAssignment {
    pub unit_id: String,
    pub zone_id: String,
    pub temporal_extent: TemporalExtent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneRegime {
    pub regime_id: String,
    pub authority: RegimeAuthority,
    pub jurisdiction_scope: String,
    pub source_id: String,
    pub temporal_extent: TemporalExtent,
    pub assignments: Vec<TimeZoneAssignment>,
    pub offset_rules: Vec<OffsetRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationContext {
    pub evaluation_period: TemporalExtent,
    pub boundary_graph_id: String,
    pub regime_id: String,
    pub representative_point_method: RepresentativePointMethod,
    pub weighting_source_id: String,
    pub source_vintage: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalDataset {
    pub dataset_id: String,
    pub source_manifest: SourceManifest,
    pub jurisdictions: Vec<Jurisdiction>,
    pub units: Vec<TemporalBoundaryUnit>,
    pub boundary_graphs: Vec<BoundaryGraphVersion>,
    pub regimes: Vec<TimeZoneRegime>,
    pub evaluation_contexts: Vec<EvaluationContext>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalDatasetReport {
    pub dataset_id: String,
    pub source_manifest_id: String,
    pub jurisdiction_count: usize,
    pub unit_count: usize,
    pub boundary_graph_count: usize,
    pub regime_count: usize,
    pub evaluation_context_count: usize,
    pub current_law_regime_count: usize,
    pub historical_law_regime_count: usize,
    pub proposed_scenario_regime_count: usize,
    pub analytic_counterfactual_regime_count: usize,
    pub offset_rule_count: usize,
    pub dst_rule_count: usize,
    pub non_whole_hour_rule_count: usize,
    pub caveat_count: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TemporalModelError {
    #[error("temporal extent valid_from {valid_from} must be earlier than valid_to {valid_to}")]
    InvalidTemporalExtent {
        valid_from: String,
        valid_to: String,
    },
    #[error("empty id for {kind}")]
    EmptyId { kind: &'static str },
    #[error("representative point latitude {latitude} is outside [-90, 90]")]
    InvalidLatitude { latitude: String },
    #[error("representative point longitude {longitude} is outside [-180, 180]")]
    InvalidLongitude { longitude: String },
    #[error(
        "boundary graph adjacency length {adjacency_len} does not match unit count {unit_count}"
    )]
    GraphUnitMismatch {
        adjacency_len: usize,
        unit_count: usize,
    },
    #[error("edge from {from} to {to} is outside unit range 0..{unit_count}")]
    EdgeOutOfBounds {
        from: usize,
        to: usize,
        unit_count: usize,
    },
    #[error("edge from {from} to {to} has invalid weight {weight}")]
    InvalidEdgeWeight {
        from: usize,
        to: usize,
        weight: String,
    },
    #[error("offset rule {rule_id} has unsupported UTC offset {standard_offset_minutes}")]
    InvalidUtcOffset {
        rule_id: String,
        standard_offset_minutes: i32,
    },
    #[error("assignment references unknown unit id {unit_id}")]
    UnknownAssignmentUnit { unit_id: String },
    #[error("assignment references unknown zone id {zone_id}")]
    UnknownAssignmentZone { zone_id: String },
    #[error("source manifest contains duplicate source id {source_id}")]
    DuplicateSourceId { source_id: String },
    #[error("source limitation matrix contains duplicate source id {source_id}")]
    DuplicateSourceLimitationSourceId { source_id: String },
    #[error("source gate policy contains duplicate source id {source_id}")]
    DuplicateSourceGateEntrySourceId { source_id: String },
    #[error("module boundary contract contains duplicate module id {module_id}")]
    DuplicateModuleBoundaryModuleId { module_id: String },
    #[error("source {source_id} has empty URL")]
    EmptySourceUrl { source_id: String },
    #[error("source limitation entry {source_id} has no assessments")]
    EmptySourceAssessments { source_id: String },
    #[error("module boundary entry {module_id} has no ownership statements")]
    EmptyModuleOwnership { module_id: String },
    #[error("{owner_kind} references unknown source id {source_id}")]
    UnknownSourceReference {
        owner_kind: &'static str,
        source_id: String,
    },
    #[error("dataset contains duplicate {kind} id {id}")]
    DuplicateDatasetId { kind: &'static str, id: String },
    #[error("evaluation context references unknown boundary graph {boundary_graph_id}")]
    UnknownEvaluationBoundaryGraph { boundary_graph_id: String },
    #[error("evaluation context references unknown regime {regime_id}")]
    UnknownEvaluationRegime { regime_id: String },
    #[error("{owner_kind} references unknown jurisdiction {jurisdiction_id}")]
    UnknownJurisdiction {
        owner_kind: &'static str,
        jurisdiction_id: String,
    },
}

impl SourceCitation {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source.source_id", &self.source_id)?;
        validate_non_empty("source.title", &self.title)?;
        validate_non_empty("source.retrieved_on", &self.retrieved_on)?;
        if self.url.is_empty() {
            return Err(TemporalModelError::EmptySourceUrl {
                source_id: self.source_id.clone(),
            });
        }
        Ok(())
    }
}

impl SourceManifest {
    pub fn source_ids(&self) -> BTreeSet<&str> {
        self.sources
            .iter()
            .map(|source| source.source_id.as_str())
            .collect()
    }

    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_manifest.manifest_id", &self.manifest_id)?;
        validate_non_empty("source_manifest.generated_on", &self.generated_on)?;
        let mut source_ids = BTreeSet::new();
        for source in &self.sources {
            source.validate()?;
            if !source_ids.insert(source.source_id.as_str()) {
                return Err(TemporalModelError::DuplicateSourceId {
                    source_id: source.source_id.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn report(&self) -> Result<SourceManifestReport, TemporalModelError> {
        self.validate()?;
        let mut report = SourceManifestReport {
            manifest_id: self.manifest_id.clone(),
            source_count: self.sources.len(),
            caveated_source_count: 0,
            legal_text_count: 0,
            geospatial_boundary_count: 0,
            time_rule_database_count: 0,
            population_count: 0,
            representative_point_count: 0,
            derived_manifest_count: 0,
            research_note_count: 0,
            imported_count: 0,
        };
        for source in &self.sources {
            if !source.caveats.is_empty() {
                report.caveated_source_count += 1;
            }
            match source.kind {
                SourceKind::LegalText => report.legal_text_count += 1,
                SourceKind::GeospatialBoundary => report.geospatial_boundary_count += 1,
                SourceKind::TimeRuleDatabase => report.time_rule_database_count += 1,
                SourceKind::Population => report.population_count += 1,
                SourceKind::RepresentativePoint => report.representative_point_count += 1,
                SourceKind::DerivedManifest => report.derived_manifest_count += 1,
                SourceKind::ResearchNote => report.research_note_count += 1,
                SourceKind::Imported => report.imported_count += 1,
            }
        }
        Ok(report)
    }
}

impl SourceGateEntry {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_gate_entry.source_id", &self.source_id)?;
        validate_non_empty("source_gate_entry.rights_posture", &self.rights_posture)?;
        validate_non_empty(
            "source_gate_entry.expected_artifact",
            &self.expected_artifact,
        )
    }
}

impl SourceGatePolicy {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_gate_policy.policy_id", &self.policy_id)?;
        validate_non_empty(
            "source_gate_policy.source_manifest_id",
            &self.source_manifest_id,
        )?;
        validate_non_empty("source_gate_policy.generated_on", &self.generated_on)?;
        let mut source_ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !source_ids.insert(entry.source_id.as_str()) {
                return Err(TemporalModelError::DuplicateSourceGateEntrySourceId {
                    source_id: entry.source_id.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn report(
        &self,
        manifest: &SourceManifest,
    ) -> Result<SourceGateReport, TemporalModelError> {
        self.validate()?;
        manifest.validate()?;
        let manifest_source_ids = manifest.source_ids();
        let policy_source_ids = self
            .entries
            .iter()
            .map(|entry| entry.source_id.as_str())
            .collect::<BTreeSet<_>>();
        let missing_source_ids = manifest_source_ids
            .difference(&policy_source_ids)
            .map(|source_id| (*source_id).to_string())
            .collect::<Vec<_>>();
        let extra_entry_source_ids = policy_source_ids
            .difference(&manifest_source_ids)
            .map(|source_id| (*source_id).to_string())
            .collect::<Vec<_>>();
        let mut report = SourceGateReport {
            policy_id: self.policy_id.clone(),
            source_manifest_id: self.source_manifest_id.clone(),
            source_count: manifest.sources.len(),
            policy_entry_count: self.entries.len(),
            covered_source_count: manifest_source_ids.intersection(&policy_source_ids).count(),
            missing_source_ids,
            extra_entry_source_ids,
            manual_reference_count: 0,
            fletch_candidate_count: 0,
            local_derived_count: 0,
            synthetic_fixture_count: 0,
            imported_count: 0,
            reference_only_count: 0,
            ignored_local_cache_count: 0,
            derived_metadata_only_count: 0,
            committed_fixture_count: 0,
            hash_required_count: 0,
            gate_note_count: 0,
            source_gate_ready: false,
        };

        for entry in &self.entries {
            match entry.acquisition_mode {
                SourceAcquisitionMode::ManualReference => report.manual_reference_count += 1,
                SourceAcquisitionMode::FletchCandidate => report.fletch_candidate_count += 1,
                SourceAcquisitionMode::LocalDerived => report.local_derived_count += 1,
                SourceAcquisitionMode::SyntheticFixture => report.synthetic_fixture_count += 1,
                SourceAcquisitionMode::Imported => report.imported_count += 1,
            }
            match entry.cache_policy {
                SourceCachePolicy::ReferenceOnly => report.reference_only_count += 1,
                SourceCachePolicy::IgnoredLocalCache => report.ignored_local_cache_count += 1,
                SourceCachePolicy::DerivedMetadataOnly => report.derived_metadata_only_count += 1,
                SourceCachePolicy::CommittedFixture => report.committed_fixture_count += 1,
            }
            if entry.hash_required {
                report.hash_required_count += 1;
            }
            report.gate_note_count += entry.gate_notes.len();
        }

        report.source_gate_ready = self.source_manifest_id == manifest.manifest_id
            && report.covered_source_count == report.source_count
            && report.policy_entry_count == report.source_count
            && report.extra_entry_source_ids.is_empty()
            && report.hash_required_count > 0;
        Ok(report)
    }
}

impl CountyTimeZoneAssignment {
    pub fn validate(&self, manifest: &SourceManifest) -> Result<(), TemporalModelError> {
        validate_non_empty("county_time_zone_assignment.unit_id", &self.unit_id)?;
        validate_non_empty("county_time_zone_assignment.zone_id", &self.zone_id)?;
        validate_non_empty(
            "county_time_zone_assignment.legal_source_id",
            &self.legal_source_id,
        )?;
        validate_non_empty(
            "county_time_zone_assignment.legal_clause",
            &self.legal_clause,
        )?;
        let source_ids = manifest.source_ids();
        if !source_ids.contains(self.legal_source_id.as_str()) {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_time_zone_assignment.legal_source_id",
                source_id: self.legal_source_id.clone(),
            });
        }
        if let Some(geometry_source_id) = &self.geometry_source_id {
            validate_non_empty(
                "county_time_zone_assignment.geometry_source_id",
                geometry_source_id,
            )?;
            if !source_ids.contains(geometry_source_id.as_str()) {
                return Err(TemporalModelError::UnknownSourceReference {
                    owner_kind: "county_time_zone_assignment.geometry_source_id",
                    source_id: geometry_source_id.clone(),
                });
            }
        }
        Ok(())
    }
}

impl CountyTimeZoneAssignmentSet {
    pub fn report(
        &self,
        manifest: &SourceManifest,
    ) -> Result<CountyTimeZoneAssignmentReport, TemporalModelError> {
        manifest.validate()?;
        validate_non_empty(
            "county_time_zone_assignment_set.assignment_id",
            &self.assignment_id,
        )?;
        validate_non_empty(
            "county_time_zone_assignment_set.source_manifest_id",
            &self.source_manifest_id,
        )?;
        validate_non_empty(
            "county_time_zone_assignment_set.generated_on",
            &self.generated_on,
        )?;
        validate_non_empty(
            "county_time_zone_assignment_set.scenario_id",
            &self.scenario_id,
        )?;
        if self.source_manifest_id != manifest.manifest_id {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_time_zone_assignment_set.source_manifest_id",
                source_id: self.source_manifest_id.clone(),
            });
        }

        let mut report = CountyTimeZoneAssignmentReport {
            assignment_id: self.assignment_id.clone(),
            source_manifest_id: self.source_manifest_id.clone(),
            assignment_count: self.assignments.len(),
            legal_source_ref_count: 0,
            legal_clause_count: 0,
            geometry_source_ref_count: 0,
            placeholder_count: 0,
            reconciled_count: 0,
            split_county_count: 0,
            uncertain_count: 0,
            caveated_assignment_count: 0,
            caveat_count: 0,
            assignment_evidence_ready: false,
        };

        for assignment in &self.assignments {
            assignment.validate(manifest)?;
            report.legal_source_ref_count += 1;
            report.legal_clause_count += 1;
            if assignment.geometry_source_id.is_some() {
                report.geometry_source_ref_count += 1;
            }
            match assignment.status {
                CountyAssignmentStatus::Placeholder => report.placeholder_count += 1,
                CountyAssignmentStatus::Reconciled => report.reconciled_count += 1,
                CountyAssignmentStatus::SplitCounty => report.split_county_count += 1,
                CountyAssignmentStatus::Uncertain => report.uncertain_count += 1,
            }
            if !assignment.caveats.is_empty() {
                report.caveated_assignment_count += 1;
                report.caveat_count += assignment.caveats.len();
            }
        }

        report.assignment_evidence_ready = report.assignment_count > 0
            && report.legal_source_ref_count == report.assignment_count
            && report.legal_clause_count == report.assignment_count
            && report.geometry_source_ref_count == report.assignment_count
            && report.placeholder_count == 0
            && report.uncertain_count == 0;
        Ok(report)
    }
}

impl CountyRepresentativePointRecord {
    pub fn validate(&self, manifest: &SourceManifest) -> Result<(), TemporalModelError> {
        validate_non_empty("county_representative_point.unit_id", &self.unit_id)?;
        self.point.validate()?;
        if !manifest
            .source_ids()
            .contains(self.point.source_id.as_str())
        {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_representative_point.source_id",
                source_id: self.point.source_id.clone(),
            });
        }
        if !self.solar_offset_minutes.is_finite() {
            return Err(TemporalModelError::InvalidLongitude {
                longitude: self.solar_offset_minutes.to_string(),
            });
        }
        Ok(())
    }
}

impl CountyRepresentativePointSet {
    pub fn report(
        &self,
        manifest: &SourceManifest,
    ) -> Result<CountyRepresentativePointReport, TemporalModelError> {
        manifest.validate()?;
        validate_non_empty(
            "county_representative_point_set.point_set_id",
            &self.point_set_id,
        )?;
        validate_non_empty(
            "county_representative_point_set.source_manifest_id",
            &self.source_manifest_id,
        )?;
        validate_non_empty(
            "county_representative_point_set.generated_on",
            &self.generated_on,
        )?;
        if self.source_manifest_id != manifest.manifest_id {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_representative_point_set.source_manifest_id",
                source_id: self.source_manifest_id.clone(),
            });
        }

        let mut report = CountyRepresentativePointReport {
            point_set_id: self.point_set_id.clone(),
            source_manifest_id: self.source_manifest_id.clone(),
            record_count: self.records.len(),
            internal_point_count: 0,
            population_center_count: 0,
            geometry_centroid_count: 0,
            source_provided_count: 0,
            imported_count: 0,
            caveated_record_count: 0,
            caveat_count: 0,
            max_solar_offset_delta_minutes: 0.0,
            exploratory_point_method: false,
            strong_claim_point_method_ready: false,
        };

        for record in &self.records {
            record.validate(manifest)?;
            match record.point.method {
                RepresentativePointMethod::InternalPoint => report.internal_point_count += 1,
                RepresentativePointMethod::PopulationCenter => report.population_center_count += 1,
                RepresentativePointMethod::GeometryCentroid => report.geometry_centroid_count += 1,
                RepresentativePointMethod::SourceProvided => report.source_provided_count += 1,
                RepresentativePointMethod::Imported => report.imported_count += 1,
            }
            if !record.caveats.is_empty() {
                report.caveated_record_count += 1;
                report.caveat_count += record.caveats.len();
            }
            let expected = record.point.solar_offset_minutes();
            report.max_solar_offset_delta_minutes = report
                .max_solar_offset_delta_minutes
                .max((record.solar_offset_minutes - expected).abs());
        }

        report.exploratory_point_method =
            report.internal_point_count > 0 || report.geometry_centroid_count > 0;
        report.strong_claim_point_method_ready = report.record_count > 0
            && report.population_center_count == report.record_count
            && report.max_solar_offset_delta_minutes < 1e-9;
        Ok(report)
    }
}

impl CountyGeometryReconciliation {
    pub fn validate(&self, manifest: &SourceManifest) -> Result<(), TemporalModelError> {
        validate_non_empty("county_geometry_reconciliation.unit_id", &self.unit_id)?;
        validate_non_empty(
            "county_geometry_reconciliation.assignment_zone_id",
            &self.assignment_zone_id,
        )?;
        validate_non_empty(
            "county_geometry_reconciliation.legal_source_id",
            &self.legal_source_id,
        )?;
        validate_non_empty(
            "county_geometry_reconciliation.geometry_source_id",
            &self.geometry_source_id,
        )?;
        validate_non_empty(
            "county_geometry_reconciliation.evidence_note",
            &self.evidence_note,
        )?;
        let source_ids = manifest.source_ids();
        if !source_ids.contains(self.legal_source_id.as_str()) {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_geometry_reconciliation.legal_source_id",
                source_id: self.legal_source_id.clone(),
            });
        }
        if !source_ids.contains(self.geometry_source_id.as_str()) {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_geometry_reconciliation.geometry_source_id",
                source_id: self.geometry_source_id.clone(),
            });
        }
        Ok(())
    }
}

impl CountyGeometryReconciliationSet {
    pub fn report(
        &self,
        manifest: &SourceManifest,
    ) -> Result<CountyGeometryReconciliationReport, TemporalModelError> {
        manifest.validate()?;
        validate_non_empty(
            "county_geometry_reconciliation_set.reconciliation_id",
            &self.reconciliation_id,
        )?;
        validate_non_empty(
            "county_geometry_reconciliation_set.source_manifest_id",
            &self.source_manifest_id,
        )?;
        validate_non_empty(
            "county_geometry_reconciliation_set.generated_on",
            &self.generated_on,
        )?;
        if self.source_manifest_id != manifest.manifest_id {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "county_geometry_reconciliation_set.source_manifest_id",
                source_id: self.source_manifest_id.clone(),
            });
        }

        let mut report = CountyGeometryReconciliationReport {
            reconciliation_id: self.reconciliation_id.clone(),
            source_manifest_id: self.source_manifest_id.clone(),
            row_count: self.rows.len(),
            pending_count: 0,
            representative_point_matched_count: 0,
            reconciled_count: 0,
            split_county_count: 0,
            mismatch_count: 0,
            caveated_row_count: 0,
            caveat_count: 0,
            geometry_reconciliation_ready: false,
        };
        for row in &self.rows {
            row.validate(manifest)?;
            match row.status {
                GeometryReconciliationStatus::Pending => report.pending_count += 1,
                GeometryReconciliationStatus::RepresentativePointMatched => {
                    report.representative_point_matched_count += 1
                }
                GeometryReconciliationStatus::Reconciled => report.reconciled_count += 1,
                GeometryReconciliationStatus::SplitCounty => report.split_county_count += 1,
                GeometryReconciliationStatus::Mismatch => report.mismatch_count += 1,
            }
            if !row.caveats.is_empty() {
                report.caveated_row_count += 1;
                report.caveat_count += row.caveats.len();
            }
        }
        report.geometry_reconciliation_ready =
            report.row_count > 0 && report.reconciled_count == report.row_count;
        Ok(report)
    }
}

impl SourceClaimAssessment {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_claim_assessment.notes", &self.notes)
    }
}

impl SourceLimitationEntry {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_limitation_entry.source_id", &self.source_id)?;
        validate_non_empty("source_limitation_entry.title", &self.title)?;
        if self.assessments.is_empty() {
            return Err(TemporalModelError::EmptySourceAssessments {
                source_id: self.source_id.clone(),
            });
        }
        for assessment in &self.assessments {
            assessment.validate()?;
        }
        Ok(())
    }
}

impl SourceLimitationMatrix {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("source_limitation_matrix.matrix_id", &self.matrix_id)?;
        validate_non_empty("source_limitation_matrix.generated_on", &self.generated_on)?;
        let mut source_ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !source_ids.insert(entry.source_id.as_str()) {
                return Err(TemporalModelError::DuplicateSourceLimitationSourceId {
                    source_id: entry.source_id.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn report(&self) -> Result<SourceLimitationMatrixReport, TemporalModelError> {
        self.validate()?;
        let mut report = SourceLimitationMatrixReport {
            matrix_id: self.matrix_id.clone(),
            entry_count: self.entries.len(),
            assessment_count: 0,
            supports_count: 0,
            partial_count: 0,
            not_supported_count: 0,
            unknown_count: 0,
            caveated_entry_count: 0,
        };
        for entry in &self.entries {
            if !entry.caveats.is_empty() {
                report.caveated_entry_count += 1;
            }
            for assessment in &entry.assessments {
                report.assessment_count += 1;
                match assessment.support {
                    SourceSupportLevel::Supports => report.supports_count += 1,
                    SourceSupportLevel::Partial => report.partial_count += 1,
                    SourceSupportLevel::NotSupported => report.not_supported_count += 1,
                    SourceSupportLevel::Unknown => report.unknown_count += 1,
                }
            }
        }
        Ok(report)
    }
}

impl ModuleBoundaryEntry {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("module_boundary_entry.module_id", &self.module_id)?;
        if self.owns.is_empty() {
            return Err(TemporalModelError::EmptyModuleOwnership {
                module_id: self.module_id.clone(),
            });
        }
        for statement in &self.owns {
            validate_non_empty("module_boundary_entry.owns", statement)?;
        }
        for statement in &self.must_not_own {
            validate_non_empty("module_boundary_entry.must_not_own", statement)?;
        }
        for dependency in &self.upstream_dependencies {
            validate_non_empty("module_boundary_entry.upstream_dependencies", dependency)?;
        }
        for consumer in &self.downstream_consumers {
            validate_non_empty("module_boundary_entry.downstream_consumers", consumer)?;
        }
        Ok(())
    }
}

impl ModuleBoundaryContract {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("module_boundary_contract.contract_id", &self.contract_id)?;
        validate_non_empty("module_boundary_contract.generated_on", &self.generated_on)?;
        let mut module_ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !module_ids.insert(entry.module_id.as_str()) {
                return Err(TemporalModelError::DuplicateModuleBoundaryModuleId {
                    module_id: entry.module_id.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn report(&self) -> Result<ModuleBoundaryReport, TemporalModelError> {
        self.validate()?;
        Ok(ModuleBoundaryReport {
            contract_id: self.contract_id.clone(),
            module_count: self.entries.len(),
            ownership_statement_count: self.entries.iter().map(|entry| entry.owns.len()).sum(),
            exclusion_statement_count: self
                .entries
                .iter()
                .map(|entry| entry.must_not_own.len())
                .sum(),
            dependency_edge_count: self
                .entries
                .iter()
                .map(|entry| entry.upstream_dependencies.len())
                .sum(),
            downstream_edge_count: self
                .entries
                .iter()
                .map(|entry| entry.downstream_consumers.len())
                .sum(),
            caveat_count: self.caveats.len(),
        })
    }
}

impl ZoneCatalog {
    pub fn validate(&self) -> Result<(), ZonePlanError> {
        validate_non_empty_plan("zone_catalog.catalog_id", &self.catalog_id)?;
        validate_non_empty_plan("zone_catalog.source_manifest_id", &self.source_manifest_id)?;
        validate_non_empty_plan("zone_catalog.generated_on", &self.generated_on)?;
        validate_zones(&self.zones)
    }

    pub fn report(&self) -> Result<ZoneCatalogReport, ZonePlanError> {
        self.validate()?;
        let mut whole_hour_offset_count = 0;
        let mut non_whole_hour_offset_count = 0;
        let mut half_hour_offset_count = 0;
        let mut quarter_hour_offset_count = 0;
        let mut min_utc_offset_minutes = i32::MAX;
        let mut max_utc_offset_minutes = i32::MIN;
        for zone in &self.zones {
            let offset = zone.utc_offset_minutes;
            min_utc_offset_minutes = min_utc_offset_minutes.min(offset);
            max_utc_offset_minutes = max_utc_offset_minutes.max(offset);
            if offset % 60 == 0 {
                whole_hour_offset_count += 1;
            } else {
                non_whole_hour_offset_count += 1;
            }
            if offset % 60 == 30 || offset % 60 == -30 {
                half_hour_offset_count += 1;
            }
            if offset % 60 == 15 || offset % 60 == 45 || offset % 60 == -15 || offset % 60 == -45 {
                quarter_hour_offset_count += 1;
            }
        }

        Ok(ZoneCatalogReport {
            catalog_id: self.catalog_id.clone(),
            zone_count: self.zones.len(),
            whole_hour_offset_count,
            non_whole_hour_offset_count,
            half_hour_offset_count,
            quarter_hour_offset_count,
            min_utc_offset_minutes,
            max_utc_offset_minutes,
        })
    }
}

pub fn validate_source_references(
    manifest: &SourceManifest,
    references: &[(&'static str, &str)],
) -> Result<(), TemporalModelError> {
    manifest.validate()?;
    let source_ids = manifest.source_ids();
    for (owner_kind, source_id) in references {
        if !source_ids.contains(source_id) {
            return Err(TemporalModelError::UnknownSourceReference {
                owner_kind,
                source_id: (*source_id).to_string(),
            });
        }
    }
    Ok(())
}

impl RepresentativePoint {
    pub fn solar_offset_minutes(&self) -> f64 {
        self.longitude * 4.0
    }

    pub fn validate(&self) -> Result<(), TemporalModelError> {
        if !self.latitude.is_finite() || self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(TemporalModelError::InvalidLatitude {
                latitude: self.latitude.to_string(),
            });
        }
        if !self.longitude.is_finite() || self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(TemporalModelError::InvalidLongitude {
                longitude: self.longitude.to_string(),
            });
        }
        validate_non_empty("representative_point.source_id", &self.source_id)
    }
}

impl TemporalBoundaryUnit {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("boundary_unit.unit_id", &self.unit_id)?;
        validate_non_empty("boundary_unit.jurisdiction_id", &self.jurisdiction_id)?;
        validate_non_empty("boundary_unit.name", &self.name)?;
        self.representative_point.validate()?;
        self.temporal_extent.validate()
    }
}

impl Jurisdiction {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("jurisdiction.jurisdiction_id", &self.jurisdiction_id)?;
        validate_non_empty("jurisdiction.name", &self.name)?;
        validate_non_empty("jurisdiction.source_id", &self.source_id)?;
        self.temporal_extent.validate()
    }
}

impl BoundaryGraphVersion {
    pub fn validate(&self, unit_count: usize) -> Result<(), TemporalModelError> {
        validate_non_empty("boundary_graph.graph_id", &self.graph_id)?;
        validate_non_empty("boundary_graph.unit_universe_id", &self.unit_universe_id)?;
        validate_non_empty("boundary_graph.source_id", &self.source_id)?;
        self.temporal_extent.validate()?;
        if self.adjacency.len() != unit_count {
            return Err(TemporalModelError::GraphUnitMismatch {
                adjacency_len: self.adjacency.len(),
                unit_count,
            });
        }
        for (from, edges) in self.adjacency.iter().enumerate() {
            for edge in edges {
                if edge.to >= unit_count {
                    return Err(TemporalModelError::EdgeOutOfBounds {
                        from,
                        to: edge.to,
                        unit_count,
                    });
                }
                if let Some(weight) = edge.weight {
                    if !weight.is_finite() || weight < 0.0 {
                        return Err(TemporalModelError::InvalidEdgeWeight {
                            from,
                            to: edge.to,
                            weight: weight.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

impl OffsetRule {
    pub fn effective_offset_minutes(&self, daylight_saving_active: bool) -> i32 {
        self.standard_offset_minutes
            + if daylight_saving_active {
                self.dst_delta_minutes.unwrap_or(0)
            } else {
                0
            }
    }

    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("offset_rule.rule_id", &self.rule_id)?;
        validate_non_empty("offset_rule.zone_id", &self.zone_id)?;
        if !(-14 * 60..=14 * 60).contains(&self.standard_offset_minutes) {
            return Err(TemporalModelError::InvalidUtcOffset {
                rule_id: self.rule_id.clone(),
                standard_offset_minutes: self.standard_offset_minutes,
            });
        }
        self.temporal_extent.validate()
    }
}

impl TimeZoneRegime {
    pub fn validate(&self, units: &[TemporalBoundaryUnit]) -> Result<(), TemporalModelError> {
        validate_non_empty("regime.regime_id", &self.regime_id)?;
        validate_non_empty("regime.jurisdiction_scope", &self.jurisdiction_scope)?;
        validate_non_empty("regime.source_id", &self.source_id)?;
        self.temporal_extent.validate()?;

        let unit_ids = units
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let mut zone_ids = BTreeSet::new();
        for rule in &self.offset_rules {
            rule.validate()?;
            zone_ids.insert(rule.zone_id.as_str());
        }

        for assignment in &self.assignments {
            assignment.temporal_extent.validate()?;
            if !unit_ids.contains(assignment.unit_id.as_str()) {
                return Err(TemporalModelError::UnknownAssignmentUnit {
                    unit_id: assignment.unit_id.clone(),
                });
            }
            if !zone_ids.contains(assignment.zone_id.as_str()) {
                return Err(TemporalModelError::UnknownAssignmentZone {
                    zone_id: assignment.zone_id.clone(),
                });
            }
        }
        Ok(())
    }
}

impl EvaluationContext {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        self.evaluation_period.validate()?;
        validate_non_empty(
            "evaluation_context.boundary_graph_id",
            &self.boundary_graph_id,
        )?;
        validate_non_empty("evaluation_context.regime_id", &self.regime_id)?;
        validate_non_empty(
            "evaluation_context.weighting_source_id",
            &self.weighting_source_id,
        )?;
        validate_non_empty("evaluation_context.source_vintage", &self.source_vintage)
    }
}

impl TemporalDataset {
    pub fn validate(&self) -> Result<(), TemporalModelError> {
        validate_non_empty("temporal_dataset.dataset_id", &self.dataset_id)?;
        self.source_manifest.validate()?;

        validate_unique_ids(
            "jurisdiction",
            self.jurisdictions
                .iter()
                .map(|jurisdiction| jurisdiction.jurisdiction_id.as_str()),
        )?;
        validate_unique_ids("unit", self.units.iter().map(|unit| unit.unit_id.as_str()))?;
        validate_unique_ids(
            "boundary_graph",
            self.boundary_graphs
                .iter()
                .map(|graph| graph.graph_id.as_str()),
        )?;
        validate_unique_ids(
            "regime",
            self.regimes.iter().map(|regime| regime.regime_id.as_str()),
        )?;
        let jurisdiction_ids = self
            .jurisdictions
            .iter()
            .map(|jurisdiction| jurisdiction.jurisdiction_id.as_str())
            .collect::<BTreeSet<_>>();

        let mut source_refs = Vec::new();
        for jurisdiction in &self.jurisdictions {
            jurisdiction.validate()?;
            if let Some(parent_id) = &jurisdiction.parent_jurisdiction_id {
                if !jurisdiction_ids.contains(parent_id.as_str()) {
                    return Err(TemporalModelError::UnknownJurisdiction {
                        owner_kind: "jurisdiction.parent_jurisdiction_id",
                        jurisdiction_id: parent_id.clone(),
                    });
                }
            }
            source_refs.push(("jurisdiction", jurisdiction.source_id.as_str()));
        }
        for unit in &self.units {
            unit.validate()?;
            if !jurisdiction_ids.contains(unit.jurisdiction_id.as_str()) {
                return Err(TemporalModelError::UnknownJurisdiction {
                    owner_kind: "boundary_unit.jurisdiction_id",
                    jurisdiction_id: unit.jurisdiction_id.clone(),
                });
            }
            source_refs.push((
                "representative_point",
                unit.representative_point.source_id.as_str(),
            ));
        }
        for graph in &self.boundary_graphs {
            graph.validate(self.units.len())?;
            source_refs.push(("boundary_graph", graph.source_id.as_str()));
        }
        for regime in &self.regimes {
            regime.validate(&self.units)?;
            source_refs.push(("regime", regime.source_id.as_str()));
        }
        for context in &self.evaluation_contexts {
            context.validate()?;
            source_refs.push(("evaluation_context", context.weighting_source_id.as_str()));
        }
        validate_source_references(&self.source_manifest, &source_refs)?;

        let graph_ids = self
            .boundary_graphs
            .iter()
            .map(|graph| graph.graph_id.as_str())
            .collect::<BTreeSet<_>>();
        let regime_ids = self
            .regimes
            .iter()
            .map(|regime| regime.regime_id.as_str())
            .collect::<BTreeSet<_>>();
        for context in &self.evaluation_contexts {
            if !graph_ids.contains(context.boundary_graph_id.as_str()) {
                return Err(TemporalModelError::UnknownEvaluationBoundaryGraph {
                    boundary_graph_id: context.boundary_graph_id.clone(),
                });
            }
            if !regime_ids.contains(context.regime_id.as_str()) {
                return Err(TemporalModelError::UnknownEvaluationRegime {
                    regime_id: context.regime_id.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn report(&self) -> Result<TemporalDatasetReport, TemporalModelError> {
        self.validate()?;
        let mut report = TemporalDatasetReport {
            dataset_id: self.dataset_id.clone(),
            source_manifest_id: self.source_manifest.manifest_id.clone(),
            jurisdiction_count: self.jurisdictions.len(),
            unit_count: self.units.len(),
            boundary_graph_count: self.boundary_graphs.len(),
            regime_count: self.regimes.len(),
            evaluation_context_count: self.evaluation_contexts.len(),
            current_law_regime_count: 0,
            historical_law_regime_count: 0,
            proposed_scenario_regime_count: 0,
            analytic_counterfactual_regime_count: 0,
            offset_rule_count: 0,
            dst_rule_count: 0,
            non_whole_hour_rule_count: 0,
            caveat_count: self.caveats.len(),
        };
        for regime in &self.regimes {
            match regime.authority {
                RegimeAuthority::CurrentLaw => report.current_law_regime_count += 1,
                RegimeAuthority::HistoricalLaw => report.historical_law_regime_count += 1,
                RegimeAuthority::ProposedScenario => report.proposed_scenario_regime_count += 1,
                RegimeAuthority::AnalyticCounterfactual => {
                    report.analytic_counterfactual_regime_count += 1
                }
            }
            report.offset_rule_count += regime.offset_rules.len();
            report.dst_rule_count += regime
                .offset_rules
                .iter()
                .filter(|rule| rule.dst_delta_minutes.unwrap_or(0) != 0)
                .count();
            report.non_whole_hour_rule_count += regime
                .offset_rules
                .iter()
                .filter(|rule| rule.standard_offset_minutes % 60 != 0)
                .count();
        }
        Ok(report)
    }
}

fn validate_unique_ids<'a>(
    kind: &'static str,
    ids: impl IntoIterator<Item = &'a str>,
) -> Result<(), TemporalModelError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        validate_non_empty(kind, id)?;
        if !seen.insert(id) {
            return Err(TemporalModelError::DuplicateDatasetId {
                kind,
                id: id.to_string(),
            });
        }
    }
    Ok(())
}

fn validate_non_empty(kind: &'static str, value: &str) -> Result<(), TemporalModelError> {
    if value.is_empty() {
        Err(TemporalModelError::EmptyId { kind })
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapPoint {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
pub enum MapGeometry {
    Point([f64; 2]),
    Polygon(Vec<Vec<[f64; 2]>>),
    MultiPolygon(Vec<Vec<Vec<[f64; 2]>>>),
}

impl MapGeometry {
    pub fn geometry_type(&self) -> &'static str {
        match self {
            Self::Point(_) => "Point",
            Self::Polygon(_) => "Polygon",
            Self::MultiPolygon(_) => "MultiPolygon",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryUnitSourceRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_point_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_assignment_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_geometry_source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryUnit {
    pub id: String,
    pub name: String,
    pub solar_offset_minutes: f64,
    pub population: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_point: Option<MapPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_geometry: Option<MapGeometry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_refs: Option<BoundaryUnitSourceRefs>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneSpec {
    pub id: String,
    pub utc_offset_minutes: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZonePlan {
    pub name: String,
    pub zones: Vec<ZoneSpec>,
    pub assignment: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZoneScenarioKind {
    CurrentLaw,
    HistoricalLaw,
    ProposedScenario,
    AnalyticCounterfactual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneScenario {
    pub scenario_id: String,
    pub kind: ZoneScenarioKind,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_source_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZonePlanInput {
    pub input_id: String,
    pub source_manifest_id: String,
    pub scenario: ZoneScenario,
    pub units: Vec<BoundaryUnit>,
    pub adjacency: Vec<Vec<usize>>,
    pub plan: ZonePlan,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_assignment: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeometryJoinOptions {
    pub unit_id_property: String,
    pub require_all_units: bool,
}

impl Default for GeometryJoinOptions {
    fn default() -> Self {
        Self {
            unit_id_property: "unit_id".to_string(),
            require_all_units: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeometryJoinReport {
    pub matched_unit_count: usize,
    pub unmatched_unit_ids: Vec<String>,
    pub unused_feature_unit_ids: Vec<String>,
    pub unit_statuses: Vec<GeometryJoinUnitStatus>,
    pub input: ZonePlanInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeometryJoinUnitStatus {
    pub unit_id: String,
    pub matched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct GeoJsonFeatureCollection {
    #[serde(rename = "type")]
    kind: String,
    features: Vec<GeoJsonFeature>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct GeoJsonFeature {
    #[serde(default)]
    id: Option<Value>,
    #[serde(default)]
    properties: BTreeMap<String, Value>,
    geometry: Option<MapGeometry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZonePlanReport {
    pub plan_name: String,
    pub unit_count: usize,
    pub zone_count: usize,
    pub boundary_edges: usize,
    pub all_zones_connected: bool,
    pub weighted_mean_absolute_error_minutes: f64,
    pub max_absolute_error_minutes: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved_unit_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved_population: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneUnitScore {
    pub unit_id: String,
    pub unit_name: String,
    pub zone_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_refs: Option<BoundaryUnitSourceRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved_from_reference: Option<bool>,
    pub population: u64,
    pub solar_offset_minutes: f64,
    pub zone_utc_offset_minutes: i32,
    pub absolute_error_minutes: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneSummary {
    pub zone_id: String,
    pub unit_count: usize,
    pub population: u64,
    pub moved_unit_count: usize,
    pub moved_population: u64,
    pub weighted_mean_absolute_error_minutes: f64,
    pub max_absolute_error_minutes: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZonePlanEvaluation {
    pub input_id: String,
    pub scenario: ZoneScenario,
    pub source_manifest_id: String,
    pub source_manifest_generated_on: String,
    pub plan_report: ZonePlanReport,
    pub zone_summaries: Vec<ZoneSummary>,
    pub unit_scores: Vec<ZoneUnitScore>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_caveats: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateComparisonRow {
    pub candidate_id: String,
    pub label: String,
    pub kind: ZoneScenarioKind,
    pub plan_report: ZonePlanReport,
    pub weighted_error_delta_minutes: f64,
    pub max_error_delta_minutes: f64,
    pub moved_unit_count: usize,
    pub moved_population: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateComparisonReport {
    pub input_id: String,
    pub source_manifest_id: String,
    pub baseline: ZonePlanReport,
    pub candidates: Vec<CandidateComparisonRow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caveats: Vec<String>,
    pub recommendation_gate_closed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetFitUnitScore {
    pub unit_id: String,
    pub unit_name: String,
    pub population: u64,
    pub solar_offset_minutes: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_point: Option<MapPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_geometry: Option<MapGeometry>,
    pub current_zone_id: String,
    pub current_standard_offset_minutes: i32,
    pub current_standard_error_minutes: f64,
    pub current_dst_offset_minutes: i32,
    pub current_dst_error_minutes: f64,
    pub best_whole_hour_offset_minutes: i32,
    pub best_whole_hour_error_minutes: f64,
    pub best_half_hour_offset_minutes: i32,
    pub best_half_hour_error_minutes: f64,
    pub best_quarter_hour_offset_minutes: i32,
    pub best_quarter_hour_error_minutes: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetFitReport {
    pub input_id: String,
    pub unit_count: usize,
    pub total_population: u64,
    pub current_weighted_mean_standard_error_minutes: f64,
    pub current_weighted_mean_dst_error_minutes: f64,
    pub best_whole_hour_weighted_mean_error_minutes: f64,
    pub best_half_hour_weighted_mean_error_minutes: f64,
    pub best_quarter_hour_weighted_mean_error_minutes: f64,
    pub units_improved_by_whole_hour_count: usize,
    pub units_improved_by_half_hour_count: usize,
    pub units_improved_by_quarter_hour_count: usize,
    pub unit_scores: Vec<OffsetFitUnitScore>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OffsetCandidateGrid {
    WholeHour,
    HalfHour,
    QuarterHour,
}

impl OffsetCandidateGrid {
    pub fn step_minutes(self) -> i32 {
        match self {
            Self::WholeHour => 60,
            Self::HalfHour => 30,
            Self::QuarterHour => 15,
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::WholeHour => "whole-hour",
            Self::HalfHour => "half-hour",
            Self::QuarterHour => "quarter-hour",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::WholeHour => "whole-hour",
            Self::HalfHour => "half-hour",
            Self::QuarterHour => "quarter-hour",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OffsetMapView {
    CurrentStandard,
    CurrentDst,
    BestWholeHour,
    BestHalfHour,
    BestQuarterHour,
}

impl OffsetMapView {
    pub fn slug(self) -> &'static str {
        match self {
            Self::CurrentStandard => "current-standard",
            Self::CurrentDst => "current-dst",
            Self::BestWholeHour => "best-whole-hour",
            Self::BestHalfHour => "best-half-hour",
            Self::BestQuarterHour => "best-quarter-hour",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::CurrentStandard => "Current standard-time offset error",
            Self::CurrentDst => "Current DST-period clock error",
            Self::BestWholeHour => "Best whole-hour offset error",
            Self::BestHalfHour => "Best half-hour offset error",
            Self::BestQuarterHour => "Best quarter-hour offset error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffsetMapRenderOptions {
    pub width: u32,
    pub height: u32,
}

impl Default for OffsetMapRenderOptions {
    fn default() -> Self {
        Self {
            width: 960,
            height: 560,
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ZonePlanError {
    #[error("unit count {unit_count} does not match plan assignment count {assignment_count}")]
    UnitAssignmentMismatch {
        unit_count: usize,
        assignment_count: usize,
    },
    #[error("adjacency count {adjacency_count} does not match unit count {unit_count}")]
    AdjacencyUnitMismatch {
        adjacency_count: usize,
        unit_count: usize,
    },
    #[error("plan must define at least one zone")]
    EmptyZones,
    #[error("zone {zone_id} has unsupported UTC offset {utc_offset_minutes}")]
    InvalidZoneUtcOffset {
        zone_id: String,
        utc_offset_minutes: i32,
    },
    #[error("unit {unit_index} is assigned to missing zone index {zone_index}")]
    UnknownZone {
        unit_index: usize,
        zone_index: usize,
    },
    #[error("reference assignment count {reference_assignment_count} does not match unit count {unit_count}")]
    ReferenceAssignmentMismatch {
        reference_assignment_count: usize,
        unit_count: usize,
    },
    #[error("unit {unit_index} has reference assignment to missing zone index {zone_index}")]
    UnknownReferenceZone {
        unit_index: usize,
        zone_index: usize,
    },
    #[error("edge from {from} to {to} is outside unit range 0..{unit_count}")]
    EdgeOutOfBounds {
        from: usize,
        to: usize,
        unit_count: usize,
    },
    #[error("total population must be greater than zero")]
    EmptyPopulation,
    #[error("unit {unit_id} map latitude {latitude} is outside [-90, 90]")]
    InvalidMapLatitude { unit_id: String, latitude: String },
    #[error("unit {unit_id} map longitude {longitude} is outside [-180, 180]")]
    InvalidMapLongitude { unit_id: String, longitude: String },
    #[error("RPLAN context has no graph")]
    MissingRplanGraph,
    #[error("RPLAN context has no populations")]
    MissingRplanPopulations,
    #[error(
        "solar offset count {solar_offset_count} does not match RPLAN unit count {unit_count}"
    )]
    SolarOffsetMismatch {
        solar_offset_count: usize,
        unit_count: usize,
    },
    #[error("RPLAN population at unit {unit_index} must be non-negative")]
    NegativeRplanPopulation { unit_index: usize },
    #[error("RPLAN context validation failed: {0}")]
    RplanContext(String),
    #[error("graph boundary metric failed: {0}")]
    BoundaryMetric(String),
    #[error("graph connectivity metric failed: {0}")]
    Connectivity(String),
    #[error("empty id for {kind}")]
    EmptyId { kind: &'static str },
    #[error("source manifest validation failed: {0}")]
    SourceManifest(String),
    #[error(
        "plan input source_manifest_id {input_source_manifest_id} does not match manifest id {manifest_id}"
    )]
    SourceManifestMismatch {
        input_source_manifest_id: String,
        manifest_id: String,
    },
    #[error("scenario {scenario_id} is missing authority_source_id for current-law or historical-law evaluation")]
    ScenarioAuthorityRequired { scenario_id: String },
    #[error("scenario {scenario_id} references unknown authority source {source_id}")]
    UnknownScenarioAuthoritySource {
        scenario_id: String,
        source_id: String,
    },
    #[error("unit {unit_id} {field} references unknown source {source_id}")]
    UnknownUnitSourceReference {
        unit_id: String,
        field: &'static str,
        source_id: String,
    },
    #[error("zone catalog validation failed: {0}")]
    ZoneCatalog(String),
    #[error(
        "zone catalog source_manifest_id {catalog_source_manifest_id} does not match source manifest id {manifest_id}"
    )]
    ZoneCatalogSourceManifestMismatch {
        catalog_source_manifest_id: String,
        manifest_id: String,
    },
    #[error("plan zone {zone_id} is missing from zone catalog {catalog_id}")]
    PlanZoneMissingFromCatalog { zone_id: String, catalog_id: String },
    #[error(
        "plan zone {zone_id} offset {plan_utc_offset_minutes} does not match catalog offset {catalog_utc_offset_minutes}"
    )]
    PlanZoneCatalogOffsetMismatch {
        zone_id: String,
        plan_utc_offset_minutes: i32,
        catalog_utc_offset_minutes: i32,
    },
}

#[derive(Debug, Error, PartialEq)]
pub enum GeometryJoinError {
    #[error("failed to parse GeoJSON: {0}")]
    Parse(String),
    #[error("GeoJSON root type must be FeatureCollection, found {kind}")]
    InvalidRootType { kind: String },
    #[error("GeoJSON feature for unit {unit_id} has no geometry")]
    MissingGeometry { unit_id: String },
    #[error("GeoJSON has duplicate feature for unit {unit_id}")]
    DuplicateFeatureUnitId { unit_id: String },
    #[error("plan input validation failed after geometry join: {0}")]
    InvalidPlan(#[from] ZonePlanError),
    #[error("GeoJSON did not provide geometries for required units: {unit_ids:?}")]
    MissingRequiredUnits { unit_ids: Vec<String> },
}

pub fn evaluate_zone_plan(
    units: &[BoundaryUnit],
    adjacency: &[Vec<usize>],
    plan: &ZonePlan,
) -> Result<ZonePlanReport, ZonePlanError> {
    validate_inputs(units, adjacency, plan)?;

    let boundary_edges = undirected_edge_cut(adjacency, &plan.assignment)
        .map_err(|err: EdgeCutError| ZonePlanError::BoundaryMetric(err.to_string()))?;
    let all_zones_connected =
        labels_in_use(&plan.assignment)
            .into_iter()
            .try_fold(true, |connected, zone_index| {
                let zone_connected =
                    assignment_label_connected(adjacency, &plan.assignment, zone_index)
                        .map_err(|err| ZonePlanError::Connectivity(err.to_string()))?;
                Ok::<bool, ZonePlanError>(connected && zone_connected)
            })?;

    let total_population: u64 = units.iter().map(|unit| unit.population).sum();
    if total_population == 0 {
        return Err(ZonePlanError::EmptyPopulation);
    }

    let mut weighted_error = 0.0;
    let mut max_error = 0.0_f64;
    for (unit_index, unit) in units.iter().enumerate() {
        let zone = &plan.zones[plan.assignment[unit_index]];
        let error = (unit.solar_offset_minutes - zone.utc_offset_minutes as f64).abs();
        weighted_error += error * unit.population as f64;
        max_error = max_error.max(error);
    }

    Ok(ZonePlanReport {
        plan_name: plan.name.clone(),
        unit_count: units.len(),
        zone_count: plan.zones.len(),
        boundary_edges,
        all_zones_connected,
        weighted_mean_absolute_error_minutes: weighted_error / total_population as f64,
        max_absolute_error_minutes: max_error,
        moved_unit_count: None,
        moved_population: None,
    })
}

pub fn evaluate_zone_plan_input(input: &ZonePlanInput) -> Result<ZonePlanReport, ZonePlanError> {
    validate_non_empty_plan("zone_plan_input.input_id", &input.input_id)?;
    validate_non_empty_plan(
        "zone_plan_input.source_manifest_id",
        &input.source_manifest_id,
    )?;
    validate_scenario_shape(&input.scenario)?;
    evaluate_zone_plan_with_reference(
        &input.units,
        &input.adjacency,
        &input.plan,
        &input.reference_assignment,
    )
}

pub fn attach_geojson_geometries(
    input: &ZonePlanInput,
    geojson: &str,
    options: &GeometryJoinOptions,
) -> Result<GeometryJoinReport, GeometryJoinError> {
    let collection: GeoJsonFeatureCollection =
        serde_json::from_str(geojson).map_err(|err| GeometryJoinError::Parse(err.to_string()))?;
    if collection.kind != "FeatureCollection" {
        return Err(GeometryJoinError::InvalidRootType {
            kind: collection.kind,
        });
    }

    let mut geometries_by_unit_id = BTreeMap::new();
    for feature in collection.features {
        if let Some(unit_id) = geojson_feature_unit_id(&feature, &options.unit_id_property) {
            let geometry = feature
                .geometry
                .ok_or_else(|| GeometryJoinError::MissingGeometry {
                    unit_id: unit_id.clone(),
                })?;
            if geometries_by_unit_id
                .insert(unit_id.clone(), geometry)
                .is_some()
            {
                return Err(GeometryJoinError::DuplicateFeatureUnitId { unit_id });
            }
        }
    }

    let mut joined = input.clone();
    let unit_ids = joined
        .units
        .iter()
        .map(|unit| unit.id.clone())
        .collect::<BTreeSet<_>>();
    let mut matched_unit_count = 0;
    let mut unmatched_unit_ids = Vec::new();
    let mut unit_statuses = Vec::new();
    for unit in &mut joined.units {
        if let Some(geometry) = geometries_by_unit_id.remove(&unit.id) {
            let geometry_type = geometry.geometry_type().to_string();
            unit.map_geometry = Some(geometry);
            matched_unit_count += 1;
            unit_statuses.push(GeometryJoinUnitStatus {
                unit_id: unit.id.clone(),
                matched: true,
                geometry_type: Some(geometry_type),
            });
        } else {
            unmatched_unit_ids.push(unit.id.clone());
            unit_statuses.push(GeometryJoinUnitStatus {
                unit_id: unit.id.clone(),
                matched: false,
                geometry_type: None,
            });
        }
    }

    if options.require_all_units && !unmatched_unit_ids.is_empty() {
        return Err(GeometryJoinError::MissingRequiredUnits {
            unit_ids: unmatched_unit_ids,
        });
    }

    evaluate_zone_plan_input(&joined)?;
    let unused_feature_unit_ids = geometries_by_unit_id
        .into_keys()
        .filter(|unit_id| !unit_ids.contains(unit_id))
        .collect();
    Ok(GeometryJoinReport {
        matched_unit_count,
        unmatched_unit_ids,
        unused_feature_unit_ids,
        unit_statuses,
        input: joined,
    })
}

fn geojson_feature_unit_id(feature: &GeoJsonFeature, unit_id_property: &str) -> Option<String> {
    feature
        .properties
        .get(unit_id_property)
        .and_then(geojson_id_value)
        .or_else(|| feature.id.as_ref().and_then(geojson_id_value))
}

fn geojson_id_value(value: &Value) -> Option<String> {
    match value {
        Value::String(value) if !value.is_empty() => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

pub fn evaluate_zone_plan_input_with_manifest(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<ZonePlanReport, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    evaluate_zone_plan_input(input)
}

pub fn zone_plan_source_ref_report(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<ZonePlanSourceRefReport, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    evaluate_zone_plan_input(input)?;

    let mut report = ZonePlanSourceRefReport {
        input_id: input.input_id.clone(),
        source_manifest_id: manifest.manifest_id.clone(),
        unit_count: input.units.len(),
        units_with_source_refs: 0,
        units_with_complete_source_refs: 0,
        units_missing_source_refs: 0,
        boundary_source_ref_count: 0,
        missing_boundary_source_ref_count: 0,
        representative_point_source_ref_count: 0,
        missing_representative_point_source_ref_count: 0,
        population_source_ref_count: 0,
        missing_population_source_ref_count: 0,
        time_zone_assignment_source_ref_count: 0,
        missing_time_zone_assignment_source_ref_count: 0,
        time_zone_geometry_source_ref_count: 0,
        missing_time_zone_geometry_source_ref_count: 0,
        units_with_source_caveats: 0,
        units_missing_source_caveats: 0,
        unit_source_caveat_count: 0,
        publishable_source_ref_coverage: false,
    };

    for unit in &input.units {
        let Some(source_refs) = &unit.source_refs else {
            report.units_missing_source_refs += 1;
            report.missing_boundary_source_ref_count += 1;
            report.missing_representative_point_source_ref_count += 1;
            report.missing_population_source_ref_count += 1;
            report.missing_time_zone_assignment_source_ref_count += 1;
            report.missing_time_zone_geometry_source_ref_count += 1;
            report.units_missing_source_caveats += 1;
            continue;
        };
        report.units_with_source_refs += 1;
        let mut has_complete_refs = true;
        if source_refs.boundary_source_id.is_some() {
            report.boundary_source_ref_count += 1;
        } else {
            report.missing_boundary_source_ref_count += 1;
            has_complete_refs = false;
        }
        if source_refs.representative_point_source_id.is_some() {
            report.representative_point_source_ref_count += 1;
        } else {
            report.missing_representative_point_source_ref_count += 1;
            has_complete_refs = false;
        }
        if source_refs.population_source_id.is_some() {
            report.population_source_ref_count += 1;
        } else {
            report.missing_population_source_ref_count += 1;
            has_complete_refs = false;
        }
        if source_refs.time_zone_assignment_source_id.is_some() {
            report.time_zone_assignment_source_ref_count += 1;
        } else {
            report.missing_time_zone_assignment_source_ref_count += 1;
            has_complete_refs = false;
        }
        if source_refs.time_zone_geometry_source_id.is_some() {
            report.time_zone_geometry_source_ref_count += 1;
        } else {
            report.missing_time_zone_geometry_source_ref_count += 1;
            has_complete_refs = false;
        }
        if has_complete_refs {
            report.units_with_complete_source_refs += 1;
        }
        if !source_refs.caveats.is_empty() {
            report.units_with_source_caveats += 1;
            report.unit_source_caveat_count += source_refs.caveats.len();
        } else {
            report.units_missing_source_caveats += 1;
        }
    }

    report.publishable_source_ref_coverage = report.units_with_complete_source_refs
        == report.unit_count
        && report.units_with_source_caveats == report.unit_count;

    Ok(report)
}

pub fn evaluate_zone_plan_input_with_manifest_and_catalog(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
    catalog: &ZoneCatalog,
) -> Result<ZonePlanReport, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    validate_catalog_against_manifest_and_plan(catalog, manifest, &input.plan)?;
    evaluate_zone_plan_input(input)
}

pub fn evaluate_zone_plan_evaluation(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<ZonePlanEvaluation, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    let plan_report = evaluate_zone_plan_input(input)?;
    let unit_scores = score_units(&input.units, &input.plan, &input.reference_assignment);
    let zone_summaries = summarize_zones(&unit_scores);
    let source_caveats = manifest
        .sources
        .iter()
        .flat_map(|source| {
            source
                .caveats
                .iter()
                .map(|caveat| format!("{}: {}", source.source_id, caveat))
        })
        .collect();

    Ok(ZonePlanEvaluation {
        input_id: input.input_id.clone(),
        scenario: input.scenario.clone(),
        source_manifest_id: input.source_manifest_id.clone(),
        source_manifest_generated_on: manifest.generated_on.clone(),
        plan_report,
        zone_summaries,
        unit_scores,
        input_caveats: input.caveats.clone(),
        source_caveats,
    })
}

pub fn evaluate_zone_plan_evaluation_with_catalog(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
    catalog: &ZoneCatalog,
) -> Result<ZonePlanEvaluation, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    validate_catalog_against_manifest_and_plan(catalog, manifest, &input.plan)?;
    evaluate_zone_plan_evaluation(input, manifest)
}

pub fn evaluate_offset_fit(
    input: &ZonePlanInput,
    dst_delta_minutes: i32,
) -> Result<OffsetFitReport, ZonePlanError> {
    evaluate_zone_plan_input(input)?;
    let total_population = input.units.iter().map(|unit| unit.population).sum::<u64>();
    let mut unit_scores = Vec::new();
    let mut current_standard_weighted_error = 0.0;
    let mut current_dst_weighted_error = 0.0;
    let mut whole_hour_weighted_error = 0.0;
    let mut half_hour_weighted_error = 0.0;
    let mut quarter_hour_weighted_error = 0.0;
    let mut units_improved_by_whole_hour_count = 0;
    let mut units_improved_by_half_hour_count = 0;
    let mut units_improved_by_quarter_hour_count = 0;

    for (unit_index, unit) in input.units.iter().enumerate() {
        let current_zone = &input.plan.zones[input.plan.assignment[unit_index]];
        let current_standard_error =
            offset_error_minutes(unit.solar_offset_minutes, current_zone.utc_offset_minutes);
        let current_dst_offset = current_zone.utc_offset_minutes + dst_delta_minutes;
        let current_dst_error = offset_error_minutes(unit.solar_offset_minutes, current_dst_offset);
        let (best_whole_hour_offset, best_whole_hour_error) =
            best_candidate_offset(unit.solar_offset_minutes, 60);
        let (best_half_hour_offset, best_half_hour_error) =
            best_candidate_offset(unit.solar_offset_minutes, 30);
        let (best_quarter_hour_offset, best_quarter_hour_error) =
            best_candidate_offset(unit.solar_offset_minutes, 15);
        let population = unit.population as f64;

        current_standard_weighted_error += current_standard_error * population;
        current_dst_weighted_error += current_dst_error * population;
        whole_hour_weighted_error += best_whole_hour_error * population;
        half_hour_weighted_error += best_half_hour_error * population;
        quarter_hour_weighted_error += best_quarter_hour_error * population;
        if best_whole_hour_error < current_standard_error {
            units_improved_by_whole_hour_count += 1;
        }
        if best_half_hour_error < current_standard_error {
            units_improved_by_half_hour_count += 1;
        }
        if best_quarter_hour_error < current_standard_error {
            units_improved_by_quarter_hour_count += 1;
        }

        unit_scores.push(OffsetFitUnitScore {
            unit_id: unit.id.clone(),
            unit_name: unit.name.clone(),
            population: unit.population,
            solar_offset_minutes: unit.solar_offset_minutes,
            map_point: unit.map_point.clone(),
            map_geometry: unit.map_geometry.clone(),
            current_zone_id: current_zone.id.clone(),
            current_standard_offset_minutes: current_zone.utc_offset_minutes,
            current_standard_error_minutes: current_standard_error,
            current_dst_offset_minutes: current_dst_offset,
            current_dst_error_minutes: current_dst_error,
            best_whole_hour_offset_minutes: best_whole_hour_offset,
            best_whole_hour_error_minutes: best_whole_hour_error,
            best_half_hour_offset_minutes: best_half_hour_offset,
            best_half_hour_error_minutes: best_half_hour_error,
            best_quarter_hour_offset_minutes: best_quarter_hour_offset,
            best_quarter_hour_error_minutes: best_quarter_hour_error,
        });
    }

    let total_population_f64 = total_population as f64;
    Ok(OffsetFitReport {
        input_id: input.input_id.clone(),
        unit_count: input.units.len(),
        total_population,
        current_weighted_mean_standard_error_minutes: current_standard_weighted_error
            / total_population_f64,
        current_weighted_mean_dst_error_minutes: current_dst_weighted_error / total_population_f64,
        best_whole_hour_weighted_mean_error_minutes: whole_hour_weighted_error
            / total_population_f64,
        best_half_hour_weighted_mean_error_minutes: half_hour_weighted_error / total_population_f64,
        best_quarter_hour_weighted_mean_error_minutes: quarter_hour_weighted_error
            / total_population_f64,
        units_improved_by_whole_hour_count,
        units_improved_by_half_hour_count,
        units_improved_by_quarter_hour_count,
        unit_scores,
    })
}

pub fn build_offset_candidate_plan(
    input: &ZonePlanInput,
    grid: OffsetCandidateGrid,
) -> Result<ZonePlanInput, ZonePlanError> {
    validate_non_empty_plan("zone_plan_input.input_id", &input.input_id)?;
    validate_non_empty_plan(
        "zone_plan_input.source_manifest_id",
        &input.source_manifest_id,
    )?;
    validate_scenario_shape(&input.scenario)?;
    validate_inputs(&input.units, &input.adjacency, &input.plan)?;

    let mut zone_index_by_offset = BTreeMap::new();
    let mut assignment = Vec::with_capacity(input.units.len());
    for unit in &input.units {
        let (offset, _) = best_candidate_offset(unit.solar_offset_minutes, grid.step_minutes());
        let next_index = zone_index_by_offset.len();
        let zone_index = *zone_index_by_offset.entry(offset).or_insert(next_index);
        assignment.push(zone_index);
    }

    let zones = zone_index_by_offset
        .into_keys()
        .map(|offset| ZoneSpec {
            id: offset_zone_id(offset),
            utc_offset_minutes: offset,
        })
        .collect();
    let mut candidate = input.clone();
    candidate.input_id = format!("{}-{}", input.input_id, grid.slug());
    candidate.scenario = ZoneScenario {
        scenario_id: format!("{}-{}", input.scenario.scenario_id, grid.slug()),
        kind: ZoneScenarioKind::AnalyticCounterfactual,
        label: format!("{} candidate offset grid", grid.label()),
        authority_source_id: None,
    };
    candidate.plan = ZonePlan {
        name: format!("{} {} candidate", input.plan.name, grid.label()),
        zones,
        assignment,
    };
    candidate.reference_assignment = Vec::new();
    candidate.caveats.push(format!(
        "Generated analytic counterfactual from {} using nearest {} UTC offsets.",
        input.input_id,
        grid.label()
    ));
    evaluate_zone_plan_input(&candidate)?;
    Ok(candidate)
}

pub fn compare_offset_candidate_plans(
    input: &ZonePlanInput,
    grids: &[OffsetCandidateGrid],
) -> Result<CandidateComparisonReport, ZonePlanError> {
    let baseline = evaluate_zone_plan_input(input)?;
    let mut candidates = Vec::new();
    for &grid in grids {
        let candidate = build_offset_candidate_plan(input, grid)?;
        let report = evaluate_zone_plan_input(&candidate)?;
        let (moved_unit_count, moved_population) = moved_from_baseline_zone_ids(input, &candidate);
        candidates.push(CandidateComparisonRow {
            candidate_id: candidate.input_id,
            label: candidate.scenario.label,
            kind: candidate.scenario.kind,
            weighted_error_delta_minutes: report.weighted_mean_absolute_error_minutes
                - baseline.weighted_mean_absolute_error_minutes,
            max_error_delta_minutes: report.max_absolute_error_minutes
                - baseline.max_absolute_error_minutes,
            moved_unit_count,
            moved_population,
            caveats: candidate.caveats,
            plan_report: report,
        });
    }

    Ok(CandidateComparisonReport {
        input_id: input.input_id.clone(),
        source_manifest_id: input.source_manifest_id.clone(),
        baseline,
        candidates,
        caveats: vec![
            "Analytic counterfactual comparison only; not a recommendation.".to_string(),
            "Seed scope is four counties and is not a national baseline.".to_string(),
            "Representative points are Census internal points and remain exploratory.".to_string(),
        ],
        recommendation_gate_closed: true,
    })
}

fn moved_from_baseline_zone_ids(
    baseline: &ZonePlanInput,
    candidate: &ZonePlanInput,
) -> (usize, u64) {
    let mut moved_unit_count = 0;
    let mut moved_population = 0;
    for (unit_index, unit) in baseline.units.iter().enumerate() {
        let baseline_zone_id = &baseline.plan.zones[baseline.plan.assignment[unit_index]].id;
        let candidate_zone_id = &candidate.plan.zones[candidate.plan.assignment[unit_index]].id;
        if baseline_zone_id != candidate_zone_id {
            moved_unit_count += 1;
            moved_population += unit.population;
        }
    }
    (moved_unit_count, moved_population)
}

pub fn render_offset_fit_svg(
    report: &OffsetFitReport,
    view: OffsetMapView,
    options: &OffsetMapRenderOptions,
) -> String {
    let width = options.width.max(640);
    let height = options.height.max(360);
    let left = 72.0;
    let right = 44.0;
    let top = 92.0;
    let bottom = 92.0;
    let plot_width = width as f64 - left - right;
    let plot_height = height as f64 - top - bottom;
    let points = offset_map_points(report);
    let has_real_placement = report
        .unit_scores
        .iter()
        .any(|score| score.map_geometry.is_some() || score.map_point.is_some());
    let bounds_points = offset_map_bounds_points(report, &points);
    let min_x = bounds_points
        .iter()
        .map(|point| point.x)
        .fold(f64::INFINITY, f64::min)
        .floor()
        - if has_real_placement { 2.0 } else { 30.0 };
    let max_x = bounds_points
        .iter()
        .map(|point| point.x)
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil()
        + if has_real_placement { 2.0 } else { 30.0 };
    let min_y = bounds_points
        .iter()
        .map(|point| point.y)
        .fold(f64::INFINITY, f64::min)
        .floor()
        - 2.0;
    let max_y = bounds_points
        .iter()
        .map(|point| point.y)
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil()
        + 2.0;
    let max_error = report
        .unit_scores
        .iter()
        .map(|score| offset_map_error(score, view))
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let max_population = report
        .unit_scores
        .iter()
        .map(|score| score.population)
        .max()
        .unwrap_or(1) as f64;
    let has_geometry = report
        .unit_scores
        .iter()
        .any(|score| score.map_geometry.is_some());

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" role=\"img\" aria-labelledby=\"title desc\">\n"
    ));
    svg.push_str(&format!(
        "<title>{}</title>\n<desc>Offset-fit map for {} units. Color is absolute clock error.</desc>\n",
        escape_xml(view.title()),
        report.unit_count
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#f8fafc\"/>\n");
    svg.push_str(&format!(
        "<text x=\"32\" y=\"42\" font-family=\"system-ui, sans-serif\" font-size=\"24\" font-weight=\"700\" fill=\"#111827\">{}</text>\n",
        escape_xml(view.title())
    ));
    let geometry_note = if has_real_placement {
        "using plan geometry or map_point coordinates where available"
    } else {
        "schematic until geometry-backed boundaries are available"
    };
    svg.push_str(&format!(
        "<text x=\"32\" y=\"68\" font-family=\"system-ui, sans-serif\" font-size=\"13\" fill=\"#475569\">input: {} | {}</text>\n",
        escape_xml(&report.input_id),
        geometry_note
    ));
    svg.push_str(&format!(
        "<line x1=\"{left}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>\n",
        height as f64 - bottom,
        width as f64 - right,
        height as f64 - bottom
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"system-ui, sans-serif\" font-size=\"12\" fill=\"#475569\">{}</text>\n",
        left,
        height as f64 - 28.0,
        if has_real_placement { "longitude" } else { "solar offset minutes from UTC" }
    ));

    for marker in offset_axis_markers(min_x, max_x, has_real_placement) {
        let x = scale(marker as f64, min_x, max_x, left, left + plot_width);
        svg.push_str(&format!(
            "<line x1=\"{x:.2}\" y1=\"{top}\" x2=\"{x:.2}\" y2=\"{}\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>\n",
            height as f64 - bottom
        ));
        svg.push_str(&format!(
            "<text x=\"{x:.2}\" y=\"{}\" text-anchor=\"middle\" font-family=\"system-ui, sans-serif\" font-size=\"11\" fill=\"#64748b\">{}</text>\n",
            height as f64 - bottom + 20.0,
            if has_real_placement { marker.to_string() } else { format_offset(marker) }
        ));
    }

    for (index, score) in report.unit_scores.iter().enumerate() {
        let point = &points[index];
        let x = scale(point.x, min_x, max_x, left, left + plot_width);
        let y = scale(point.y, min_y, max_y, top + plot_height, top);
        let radius = 10.0 + 14.0 * ((score.population as f64 / max_population).sqrt());
        let error = offset_map_error(score, view);
        let offset = offset_map_offset(score, view);
        let color = error_color(error, max_error);
        if let Some(geometry) = &score.map_geometry {
            let path = map_geometry_to_svg_path(
                geometry,
                min_x,
                max_x,
                left,
                left + plot_width,
                min_y,
                max_y,
                top + plot_height,
                top,
            );
            svg.push_str(&format!(
                "<path d=\"{}\" fill=\"{color}\" fill-opacity=\"0.86\" stroke=\"#0f172a\" stroke-width=\"1.2\" fill-rule=\"evenodd\">\n<title>{}: offset {}, error {:.1} min</title>\n</path>\n",
                path,
                escape_xml(&score.unit_name),
                format_offset(offset),
                error
            ));
        } else {
            let fill_opacity = if has_geometry { "0.70" } else { "1.0" };
            svg.push_str(&format!(
                "<circle cx=\"{x:.2}\" cy=\"{y:.2}\" r=\"{radius:.2}\" fill=\"{color}\" fill-opacity=\"{fill_opacity}\" stroke=\"#0f172a\" stroke-width=\"1.2\">\n<title>{}: offset {}, error {:.1} min</title>\n</circle>\n",
                escape_xml(&score.unit_name),
                format_offset(offset),
                error
            ));
        }
        svg.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"system-ui, sans-serif\" font-size=\"12\" fill=\"#111827\">{}</text>\n",
            if score.map_geometry.is_some() { x + 6.0 } else { x + radius + 6.0 },
            y + 4.0,
            escape_xml(&score.unit_id)
        ));
    }

    let legend_x = width as f64 - 260.0;
    let legend_y = 96.0;
    svg.push_str(&format!(
        "<g font-family=\"system-ui, sans-serif\" font-size=\"12\" fill=\"#334155\">\n<text x=\"{legend_x}\" y=\"{legend_y}\" font-weight=\"700\">Error color</text>\n"
    ));
    for (index, (label, color)) in [
        ("low", "#2563eb"),
        ("medium", "#f59e0b"),
        ("high", "#dc2626"),
    ]
    .iter()
    .enumerate()
    {
        let y = legend_y + 22.0 + index as f64 * 22.0;
        svg.push_str(&format!(
            "<rect x=\"{legend_x}\" y=\"{}\" width=\"16\" height=\"16\" rx=\"3\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\">{}</text>\n",
            y - 12.0,
            legend_x + 24.0,
            y + 1.0,
            label
        ));
    }
    svg.push_str("</g>\n</svg>\n");
    svg
}

pub fn render_offset_fit_geojson(report: &OffsetFitReport) -> String {
    let mut geojson = String::new();
    geojson
        .push_str("{\"type\":\"FeatureCollection\",\"name\":\"zones_offset_fit\",\"features\":[\n");
    for (index, score) in report.unit_scores.iter().enumerate() {
        if index > 0 {
            geojson.push_str(",\n");
        }
        let (geometry, geometry_note) = if let Some(geometry) = &score.map_geometry {
            (
                map_geometry_to_geojson(geometry),
                "geometry from plan input",
            )
        } else if let Some(point) = &score.map_point {
            (
                point_geometry_to_geojson(point.longitude, point.latitude),
                "representative point from plan input",
            )
        } else {
            (
                point_geometry_to_geojson(
                    score.solar_offset_minutes / 4.0,
                    schematic_latitude(index, report.unit_scores.len()),
                ),
                "schematic point; longitude inferred from solar offset",
            )
        };
        geojson.push_str(&format!(
            "{{\"type\":\"Feature\",\"id\":\"{}\",\"geometry\":{},\"properties\":{{\"unit_id\":\"{}\",\"unit_name\":\"{}\",\"population\":{},\"solar_offset_minutes\":{},\"current_zone_id\":\"{}\",\"current_standard_offset_minutes\":{},\"current_standard_error_minutes\":{},\"current_dst_offset_minutes\":{},\"current_dst_error_minutes\":{},\"best_whole_hour_offset_minutes\":{},\"best_whole_hour_error_minutes\":{},\"best_half_hour_offset_minutes\":{},\"best_half_hour_error_minutes\":{},\"best_quarter_hour_offset_minutes\":{},\"best_quarter_hour_error_minutes\":{},\"geometry_note\":\"{}\"}}}}",
            escape_json(&score.unit_id),
            geometry,
            escape_json(&score.unit_id),
            escape_json(&score.unit_name),
            score.population,
            score.solar_offset_minutes,
            escape_json(&score.current_zone_id),
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
            escape_json(geometry_note),
        ));
    }
    geojson.push_str("\n]}\n");
    geojson
}

fn schematic_latitude(index: usize, count: usize) -> f64 {
    if count <= 1 {
        return 0.0;
    }
    let top = 48.0;
    let bottom = 24.0;
    top - (index as f64 * (top - bottom) / (count - 1) as f64)
}

#[derive(Clone, Copy)]
struct OffsetRenderPoint {
    x: f64,
    y: f64,
}

fn offset_map_points(report: &OffsetFitReport) -> Vec<OffsetRenderPoint> {
    report
        .unit_scores
        .iter()
        .enumerate()
        .map(|(index, score)| {
            if let Some(geometry) = &score.map_geometry {
                let (x, y) = map_geometry_centroid(geometry).unwrap_or((
                    score.solar_offset_minutes / 4.0,
                    schematic_latitude(index, report.unit_scores.len()),
                ));
                OffsetRenderPoint { x, y }
            } else if let Some(point) = &score.map_point {
                OffsetRenderPoint {
                    x: point.longitude,
                    y: point.latitude,
                }
            } else {
                OffsetRenderPoint {
                    x: score.solar_offset_minutes,
                    y: schematic_latitude(index, report.unit_scores.len()),
                }
            }
        })
        .collect()
}

fn offset_map_bounds_points(
    report: &OffsetFitReport,
    render_points: &[OffsetRenderPoint],
) -> Vec<OffsetRenderPoint> {
    let mut points = Vec::new();
    for (index, score) in report.unit_scores.iter().enumerate() {
        if let Some(geometry) = &score.map_geometry {
            points.extend(map_geometry_points(geometry).into_iter().map(|point| {
                OffsetRenderPoint {
                    x: point[0],
                    y: point[1],
                }
            }));
        } else if let Some(point) = render_points.get(index) {
            points.push(*point);
        }
    }
    if points.is_empty() {
        render_points.to_vec()
    } else {
        points
    }
}

fn point_geometry_to_geojson(longitude: f64, latitude: f64) -> String {
    format!("{{\"type\":\"Point\",\"coordinates\":[{longitude:.6},{latitude:.6}]}}")
}

fn map_geometry_to_geojson(geometry: &MapGeometry) -> String {
    match geometry {
        MapGeometry::Point(point) => point_geometry_to_geojson(point[0], point[1]),
        MapGeometry::Polygon(rings) => {
            format!(
                "{{\"type\":\"Polygon\",\"coordinates\":{}}}",
                coordinate_rings_to_json(rings)
            )
        }
        MapGeometry::MultiPolygon(polygons) => {
            let polygon_json = polygons
                .iter()
                .map(|polygon| coordinate_rings_to_json(polygon))
                .collect::<Vec<_>>()
                .join(",");
            format!("{{\"type\":\"MultiPolygon\",\"coordinates\":[{polygon_json}]}}")
        }
    }
}

fn coordinate_rings_to_json(rings: &[Vec<[f64; 2]>]) -> String {
    let ring_json = rings
        .iter()
        .map(|ring| {
            let points = ring
                .iter()
                .map(|point| format!("[{:.6},{:.6}]", point[0], point[1]))
                .collect::<Vec<_>>()
                .join(",");
            format!("[{points}]")
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{ring_json}]")
}

fn map_geometry_to_svg_path(
    geometry: &MapGeometry,
    min_x: f64,
    max_x: f64,
    out_min_x: f64,
    out_max_x: f64,
    min_y: f64,
    max_y: f64,
    out_min_y: f64,
    out_max_y: f64,
) -> String {
    match geometry {
        MapGeometry::Point(point) => {
            let x = scale(point[0], min_x, max_x, out_min_x, out_max_x);
            let y = scale(point[1], min_y, max_y, out_min_y, out_max_y);
            format!("M {x:.2} {y:.2}")
        }
        MapGeometry::Polygon(rings) => coordinate_rings_to_svg_path(
            rings, min_x, max_x, out_min_x, out_max_x, min_y, max_y, out_min_y, out_max_y,
        ),
        MapGeometry::MultiPolygon(polygons) => polygons
            .iter()
            .map(|rings| {
                coordinate_rings_to_svg_path(
                    rings, min_x, max_x, out_min_x, out_max_x, min_y, max_y, out_min_y, out_max_y,
                )
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn coordinate_rings_to_svg_path(
    rings: &[Vec<[f64; 2]>],
    min_x: f64,
    max_x: f64,
    out_min_x: f64,
    out_max_x: f64,
    min_y: f64,
    max_y: f64,
    out_min_y: f64,
    out_max_y: f64,
) -> String {
    rings
        .iter()
        .filter(|ring| !ring.is_empty())
        .map(|ring| {
            let mut commands = String::new();
            for (index, point) in ring.iter().enumerate() {
                let x = scale(point[0], min_x, max_x, out_min_x, out_max_x);
                let y = scale(point[1], min_y, max_y, out_min_y, out_max_y);
                if index == 0 {
                    commands.push_str(&format!("M {x:.2} {y:.2}"));
                } else {
                    commands.push_str(&format!(" L {x:.2} {y:.2}"));
                }
            }
            commands.push_str(" Z");
            commands
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn map_geometry_centroid(geometry: &MapGeometry) -> Option<(f64, f64)> {
    let mut longitude_sum = 0.0;
    let mut latitude_sum = 0.0;
    let mut count = 0.0;
    for point in map_geometry_points(geometry) {
        longitude_sum += point[0];
        latitude_sum += point[1];
        count += 1.0;
    }
    if count == 0.0 {
        None
    } else {
        Some((longitude_sum / count, latitude_sum / count))
    }
}

fn map_geometry_points(geometry: &MapGeometry) -> Vec<[f64; 2]> {
    match geometry {
        MapGeometry::Point(point) => vec![*point],
        MapGeometry::Polygon(rings) => rings.iter().flatten().copied().collect(),
        MapGeometry::MultiPolygon(polygons) => polygons
            .iter()
            .flat_map(|polygon| polygon.iter().flatten().copied())
            .collect(),
    }
}

fn offset_map_offset(score: &OffsetFitUnitScore, view: OffsetMapView) -> i32 {
    match view {
        OffsetMapView::CurrentStandard => score.current_standard_offset_minutes,
        OffsetMapView::CurrentDst => score.current_dst_offset_minutes,
        OffsetMapView::BestWholeHour => score.best_whole_hour_offset_minutes,
        OffsetMapView::BestHalfHour => score.best_half_hour_offset_minutes,
        OffsetMapView::BestQuarterHour => score.best_quarter_hour_offset_minutes,
    }
}

fn offset_map_error(score: &OffsetFitUnitScore, view: OffsetMapView) -> f64 {
    match view {
        OffsetMapView::CurrentStandard => score.current_standard_error_minutes,
        OffsetMapView::CurrentDst => score.current_dst_error_minutes,
        OffsetMapView::BestWholeHour => score.best_whole_hour_error_minutes,
        OffsetMapView::BestHalfHour => score.best_half_hour_error_minutes,
        OffsetMapView::BestQuarterHour => score.best_quarter_hour_error_minutes,
    }
}

fn offset_axis_markers(min_value: f64, max_value: f64, longitude_axis: bool) -> Vec<i32> {
    let step = if longitude_axis { 10 } else { 60 };
    let start = ((min_value / step as f64).ceil() as i32) * step;
    let end = ((max_value / step as f64).floor() as i32) * step;
    (start..=end).step_by(step as usize).collect()
}

fn scale(value: f64, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64 {
    if (input_max - input_min).abs() < f64::EPSILON {
        return (output_min + output_max) / 2.0;
    }
    output_min + ((value - input_min) / (input_max - input_min)) * (output_max - output_min)
}

fn error_color(error: f64, max_error: f64) -> &'static str {
    let ratio = error / max_error;
    if ratio >= 0.67 {
        "#dc2626"
    } else if ratio >= 0.34 {
        "#f59e0b"
    } else {
        "#2563eb"
    }
}

fn format_offset(offset_minutes: i32) -> String {
    let sign = if offset_minutes < 0 { "-" } else { "+" };
    let absolute = offset_minutes.abs();
    format!("UTC{sign}{:02}:{:02}", absolute / 60, absolute % 60)
}

fn offset_zone_id(offset_minutes: i32) -> String {
    let sign = if offset_minutes < 0 { "minus" } else { "plus" };
    let absolute = offset_minutes.abs();
    format!("utc-{sign}-{:02}-{:02}", absolute / 60, absolute % 60)
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn best_candidate_offset(solar_offset_minutes: f64, step_minutes: i32) -> (i32, f64) {
    (-14 * 60..=14 * 60)
        .step_by(step_minutes as usize)
        .map(|offset| (offset, offset_error_minutes(solar_offset_minutes, offset)))
        .min_by(|(left_offset, left_error), (right_offset, right_error)| {
            left_error
                .partial_cmp(right_error)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left_offset.abs().cmp(&right_offset.abs()))
        })
        .expect("candidate offset range is non-empty")
}

fn offset_error_minutes(solar_offset_minutes: f64, utc_offset_minutes: i32) -> f64 {
    (solar_offset_minutes - utc_offset_minutes as f64).abs()
}

fn summarize_zones(unit_scores: &[ZoneUnitScore]) -> Vec<ZoneSummary> {
    #[derive(Default)]
    struct Accumulator {
        unit_count: usize,
        population: u64,
        moved_unit_count: usize,
        moved_population: u64,
        weighted_error: f64,
        max_error: f64,
    }

    let mut by_zone = BTreeMap::<String, Accumulator>::new();
    for score in unit_scores {
        let entry = by_zone.entry(score.zone_id.clone()).or_default();
        entry.unit_count += 1;
        entry.population += score.population;
        if score.moved_from_reference.unwrap_or(false) {
            entry.moved_unit_count += 1;
            entry.moved_population += score.population;
        }
        entry.weighted_error += score.absolute_error_minutes * score.population as f64;
        entry.max_error = entry.max_error.max(score.absolute_error_minutes);
    }

    by_zone
        .into_iter()
        .map(|(zone_id, entry)| ZoneSummary {
            zone_id,
            unit_count: entry.unit_count,
            population: entry.population,
            moved_unit_count: entry.moved_unit_count,
            moved_population: entry.moved_population,
            weighted_mean_absolute_error_minutes: if entry.population == 0 {
                0.0
            } else {
                entry.weighted_error / entry.population as f64
            },
            max_absolute_error_minutes: entry.max_error,
        })
        .collect()
}

fn validate_input_manifest_pair(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<(), ZonePlanError> {
    manifest
        .validate()
        .map_err(|err| ZonePlanError::SourceManifest(err.to_string()))?;
    if input.source_manifest_id != manifest.manifest_id {
        return Err(ZonePlanError::SourceManifestMismatch {
            input_source_manifest_id: input.source_manifest_id.clone(),
            manifest_id: manifest.manifest_id.clone(),
        });
    }
    validate_scenario_against_manifest(&input.scenario, manifest)?;
    validate_unit_source_refs(input, manifest)
}

fn validate_scenario_shape(scenario: &ZoneScenario) -> Result<(), ZonePlanError> {
    validate_non_empty_plan("zone_scenario.scenario_id", &scenario.scenario_id)?;
    validate_non_empty_plan("zone_scenario.label", &scenario.label)?;
    if matches!(
        scenario.kind,
        ZoneScenarioKind::CurrentLaw | ZoneScenarioKind::HistoricalLaw
    ) && scenario.authority_source_id.is_none()
    {
        return Err(ZonePlanError::ScenarioAuthorityRequired {
            scenario_id: scenario.scenario_id.clone(),
        });
    }
    Ok(())
}

fn validate_scenario_against_manifest(
    scenario: &ZoneScenario,
    manifest: &SourceManifest,
) -> Result<(), ZonePlanError> {
    validate_scenario_shape(scenario)?;
    if let Some(source_id) = &scenario.authority_source_id {
        if !manifest.source_ids().contains(source_id.as_str()) {
            return Err(ZonePlanError::UnknownScenarioAuthoritySource {
                scenario_id: scenario.scenario_id.clone(),
                source_id: source_id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_unit_source_refs(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<(), ZonePlanError> {
    let source_ids = manifest.source_ids();
    for unit in &input.units {
        let Some(source_refs) = &unit.source_refs else {
            continue;
        };
        validate_optional_unit_source_ref(
            &unit.id,
            "boundary_source_id",
            source_refs.boundary_source_id.as_deref(),
            &source_ids,
        )?;
        validate_optional_unit_source_ref(
            &unit.id,
            "representative_point_source_id",
            source_refs.representative_point_source_id.as_deref(),
            &source_ids,
        )?;
        validate_optional_unit_source_ref(
            &unit.id,
            "population_source_id",
            source_refs.population_source_id.as_deref(),
            &source_ids,
        )?;
        validate_optional_unit_source_ref(
            &unit.id,
            "time_zone_assignment_source_id",
            source_refs.time_zone_assignment_source_id.as_deref(),
            &source_ids,
        )?;
        validate_optional_unit_source_ref(
            &unit.id,
            "time_zone_geometry_source_id",
            source_refs.time_zone_geometry_source_id.as_deref(),
            &source_ids,
        )?;
    }
    Ok(())
}

fn validate_optional_unit_source_ref(
    unit_id: &str,
    field: &'static str,
    source_id: Option<&str>,
    source_ids: &BTreeSet<&str>,
) -> Result<(), ZonePlanError> {
    let Some(source_id) = source_id else {
        return Ok(());
    };
    validate_non_empty_plan(field, source_id)?;
    if !source_ids.contains(source_id) {
        return Err(ZonePlanError::UnknownUnitSourceReference {
            unit_id: unit_id.to_string(),
            field,
            source_id: source_id.to_string(),
        });
    }
    Ok(())
}

fn validate_catalog_against_manifest_and_plan(
    catalog: &ZoneCatalog,
    manifest: &SourceManifest,
    plan: &ZonePlan,
) -> Result<(), ZonePlanError> {
    catalog
        .validate()
        .map_err(|err| ZonePlanError::ZoneCatalog(err.to_string()))?;
    if catalog.source_manifest_id != manifest.manifest_id {
        return Err(ZonePlanError::ZoneCatalogSourceManifestMismatch {
            catalog_source_manifest_id: catalog.source_manifest_id.clone(),
            manifest_id: manifest.manifest_id.clone(),
        });
    }
    let catalog_zones = catalog
        .zones
        .iter()
        .map(|zone| (zone.id.as_str(), zone.utc_offset_minutes))
        .collect::<BTreeMap<_, _>>();
    for zone in &plan.zones {
        let Some(catalog_offset) = catalog_zones.get(zone.id.as_str()) else {
            return Err(ZonePlanError::PlanZoneMissingFromCatalog {
                zone_id: zone.id.clone(),
                catalog_id: catalog.catalog_id.clone(),
            });
        };
        if *catalog_offset != zone.utc_offset_minutes {
            return Err(ZonePlanError::PlanZoneCatalogOffsetMismatch {
                zone_id: zone.id.clone(),
                plan_utc_offset_minutes: zone.utc_offset_minutes,
                catalog_utc_offset_minutes: *catalog_offset,
            });
        }
    }
    Ok(())
}

fn score_units(
    units: &[BoundaryUnit],
    plan: &ZonePlan,
    reference_assignment: &[usize],
) -> Vec<ZoneUnitScore> {
    units
        .iter()
        .enumerate()
        .map(|(unit_index, unit)| {
            let zone = &plan.zones[plan.assignment[unit_index]];
            let reference_zone_id = reference_assignment
                .get(unit_index)
                .map(|&zone_index| plan.zones[zone_index].id.clone());
            let moved_from_reference = reference_assignment
                .get(unit_index)
                .map(|&reference_zone_index| reference_zone_index != plan.assignment[unit_index]);
            ZoneUnitScore {
                unit_id: unit.id.clone(),
                unit_name: unit.name.clone(),
                zone_id: zone.id.clone(),
                source_refs: unit.source_refs.clone(),
                reference_zone_id,
                moved_from_reference,
                population: unit.population,
                solar_offset_minutes: unit.solar_offset_minutes,
                zone_utc_offset_minutes: zone.utc_offset_minutes,
                absolute_error_minutes: (unit.solar_offset_minutes
                    - zone.utc_offset_minutes as f64)
                    .abs(),
            }
        })
        .collect()
}

pub fn evaluate_rplan_zone_context(
    context: &RplanContext,
    solar_offset_minutes: &[f64],
    plan: &ZonePlan,
) -> Result<ZonePlanReport, ZonePlanError> {
    context
        .validate()
        .map_err(|err| ZonePlanError::RplanContext(err.to_string()))?;
    let graph = context
        .graph
        .as_ref()
        .ok_or(ZonePlanError::MissingRplanGraph)?;
    let populations = context
        .populations
        .as_ref()
        .ok_or(ZonePlanError::MissingRplanPopulations)?;
    if solar_offset_minutes.len() != context.units.unit_ids.len() {
        return Err(ZonePlanError::SolarOffsetMismatch {
            solar_offset_count: solar_offset_minutes.len(),
            unit_count: context.units.unit_ids.len(),
        });
    }

    let mut units = Vec::with_capacity(context.units.unit_ids.len());
    for (unit_index, unit_id) in context.units.unit_ids.iter().enumerate() {
        let population = populations[unit_index];
        if population < 0 {
            return Err(ZonePlanError::NegativeRplanPopulation { unit_index });
        }
        units.push(BoundaryUnit {
            id: unit_id.clone(),
            name: unit_id.clone(),
            solar_offset_minutes: solar_offset_minutes[unit_index],
            population: population as u64,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        });
    }
    let adjacency = graph
        .adjacency
        .iter()
        .map(|edges| edges.iter().map(|edge| edge.to as usize).collect())
        .collect::<Vec<Vec<usize>>>();

    evaluate_zone_plan(&units, &adjacency, plan)
}

pub fn rplan_context_intake_report(
    context: &RplanContext,
) -> Result<RplanContextIntakeReport, ZonePlanError> {
    context
        .validate()
        .map_err(|err| ZonePlanError::RplanContext(err.to_string()))?;
    let computed_context_hash = context
        .compute_context_hash()
        .map_err(|err| ZonePlanError::RplanContext(err.to_string()))?;
    let graph_edge_count = context
        .graph
        .as_ref()
        .map(|graph| graph.adjacency.iter().map(Vec::len).sum::<usize>() / 2)
        .unwrap_or(0);
    let population_count = context
        .populations
        .as_ref()
        .map(Vec::len)
        .unwrap_or_default();
    let context_hash_matches = context.context_hash == computed_context_hash;
    let rplan_context_ready = context_hash_matches
        && context.graph.is_some()
        && context.populations.is_some()
        && population_count == context.units.unit_ids.len()
        && !context.source_hashes.entries.is_empty();

    Ok(RplanContextIntakeReport {
        context_hash: context.context_hash.clone(),
        computed_context_hash,
        context_hash_matches,
        unit_kind: format!("{:?}", context.units.unit_kind),
        canonical_order: format!("{:?}", context.units.canonical_order),
        unit_count: context.units.unit_ids.len(),
        has_graph: context.graph.is_some(),
        graph_edge_count,
        has_populations: context.populations.is_some(),
        population_count,
        has_geometry_context: context.geometry.is_some(),
        source_hash_count: context.source_hashes.entries.len(),
        rplan_context_ready,
    })
}

pub fn seed_us_county_smoke_rplan_context() -> RplanContext {
    let mut units = PlanUnitIndex {
        unit_kind: UnitKind::County,
        state: None,
        year: Some(2024),
        canonical_order: CanonicalOrder::SortedGeoid,
        unit_ids: vec![
            "01001".to_string(),
            "01003".to_string(),
            "12001".to_string(),
            "12003".to_string(),
        ],
        unit_universe_hash: String::new(),
        source_id: Some("census-tiger-counties-2024".to_string()),
    };
    units.unit_universe_hash = units.compute_unit_universe_hash().unwrap();

    let mut source_hashes = SourceHashes::default();
    source_hashes.entries.insert(
        "census-tiger-counties-2024".to_string(),
        "sha256:source-gate-required-before-national-ingest".to_string(),
    );
    source_hashes.entries.insert(
        "census-county-population-estimates-2024".to_string(),
        "sha256:source-gate-required-before-national-ingest".to_string(),
    );

    let mut context = RplanContext {
        rctx_version: RCTX_VERSION.to_string(),
        context_hash: String::new(),
        units,
        graph: Some(UnitGraph {
            edge_semantics: EdgeSemantics::Undirected,
            adjacency: vec![
                vec![UnitEdge {
                    to: 1,
                    kind: EdgeKind::Boundary,
                    weight: Some(1.0),
                }],
                vec![
                    UnitEdge {
                        to: 0,
                        kind: EdgeKind::Boundary,
                        weight: Some(1.0),
                    },
                    UnitEdge {
                        to: 2,
                        kind: EdgeKind::Boundary,
                        weight: Some(1.0),
                    },
                ],
                vec![
                    UnitEdge {
                        to: 1,
                        kind: EdgeKind::Boundary,
                        weight: Some(1.0),
                    },
                    UnitEdge {
                        to: 3,
                        kind: EdgeKind::Boundary,
                        weight: Some(1.0),
                    },
                ],
                vec![UnitEdge {
                    to: 2,
                    kind: EdgeKind::Boundary,
                    weight: Some(1.0),
                }],
            ],
        }),
        populations: Some(vec![1, 1, 1, 1]),
        subdivisions: None,
        demographics: None,
        geometry: Some(GeometryContext {
            source_id: Some("census-tiger-counties-2024".to_string()),
            crs: Some("EPSG:4269".to_string()),
            unit_geometry_hashes: None,
        }),
        source_hashes,
    };
    context.context_hash = context.compute_context_hash().unwrap();
    context
}

pub fn seed_us_county_seed_rplan_context() -> RplanContext {
    let mut context = seed_us_county_smoke_rplan_context();
    context.graph = Some(UnitGraph {
        edge_semantics: EdgeSemantics::Undirected,
        adjacency: vec![vec![], vec![], vec![], vec![]],
    });
    context.populations = Some(vec![61_464, 261_608, 291_782, 29_325]);
    context.source_hashes.entries.insert(
        "census-tiger-counties-2024".to_string(),
        "sha256:04e668d3502757c837c13444730547cd967f28a2c49aeffb873d1792ab2cb97b".to_string(),
    );
    context.source_hashes.entries.insert(
        "census-county-population-estimates-2024".to_string(),
        "sha256:abcc8720d669e793bbfdcd440eeec37a78db3b452adbe4ccd1eadf7c72b522b9".to_string(),
    );
    context.context_hash = context.compute_context_hash().unwrap();
    context
}

pub fn seed_us_county_smoke_time_zone_assignments() -> CountyTimeZoneAssignmentSet {
    CountyTimeZoneAssignmentSet {
        assignment_id: "zones-us-county-smoke-current-law-assignments".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-26".to_string(),
        scenario_id: "us-county-smoke-current-law-shape".to_string(),
        assignments: vec![
            CountyTimeZoneAssignment {
                unit_id: "01001".to_string(),
                zone_id: "utc-minus-06-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "smoke-placeholder-49-cfr-71".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Placeholder,
                caveats: vec![
                    "Smoke assignment only; clause-level county evidence is not reconciled."
                        .to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "01003".to_string(),
                zone_id: "utc-minus-06-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "smoke-placeholder-49-cfr-71".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Placeholder,
                caveats: vec![
                    "Smoke assignment only; clause-level county evidence is not reconciled."
                        .to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "12001".to_string(),
                zone_id: "utc-minus-05-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "smoke-placeholder-49-cfr-71".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Placeholder,
                caveats: vec![
                    "Smoke assignment only; clause-level county evidence is not reconciled."
                        .to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "12003".to_string(),
                zone_id: "utc-minus-05-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "smoke-placeholder-49-cfr-71".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Placeholder,
                caveats: vec![
                    "Smoke assignment only; clause-level county evidence is not reconciled."
                        .to_string(),
                ],
            },
        ],
    }
}

pub fn seed_us_county_seed_time_zone_assignments() -> CountyTimeZoneAssignmentSet {
    CountyTimeZoneAssignmentSet {
        assignment_id: "zones-us-county-seed-current-law-assignments".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-27".to_string(),
        scenario_id: "us-county-seed-current-law-shape".to_string(),
        assignments: vec![
            CountyTimeZoneAssignment {
                unit_id: "01001".to_string(),
                zone_id: "utc-minus-06-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "49 CFR 71.5(e); 49 CFR 71.6(a)".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Reconciled,
                caveats: vec![
                    "Seed interpretation: Alabama is west of the eastern/central boundary described along the Alabama-Georgia line; county-level DOT geometry reconciliation remains required before publication.".to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "01003".to_string(),
                zone_id: "utc-minus-06-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "49 CFR 71.5(e); 49 CFR 71.6(a)".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Reconciled,
                caveats: vec![
                    "Seed interpretation: Alabama is west of the eastern/central boundary described along the Alabama-Georgia line; county-level DOT geometry reconciliation remains required before publication.".to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "12001".to_string(),
                zone_id: "utc-minus-05-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "49 CFR 71.4; 49 CFR 71.5(f)".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Reconciled,
                caveats: vec![
                    "Seed interpretation: county is east of the Florida boundary line described from the Apalachicola/Jackson River and Gulf County line; county-level DOT geometry reconciliation remains required before publication.".to_string(),
                ],
            },
            CountyTimeZoneAssignment {
                unit_id: "12003".to_string(),
                zone_id: "utc-minus-05-00".to_string(),
                legal_source_id: "dot-49-cfr-71".to_string(),
                legal_clause: "49 CFR 71.4; 49 CFR 71.5(f)".to_string(),
                geometry_source_id: Some("dot-time-zone-map-layer".to_string()),
                status: CountyAssignmentStatus::Reconciled,
                caveats: vec![
                    "Seed interpretation: county is east of the Florida boundary line described from the Apalachicola/Jackson River and Gulf County line; county-level DOT geometry reconciliation remains required before publication.".to_string(),
                ],
            },
        ],
    }
}

pub fn seed_us_county_seed_geometry_reconciliation() -> CountyGeometryReconciliationSet {
    let rows = seed_us_county_seed_time_zone_assignments()
        .assignments
        .into_iter()
        .map(|assignment| {
            let assignment_zone_id = assignment.zone_id;
            let (coverage_note, caveats) = match assignment.unit_id.as_str() {
                "01003" => (
                    "TIGER 2024 county polygon intersects the expected BTS/NTAD Central polygon with coverage ratio 0.9999933435; residual outside area is a small coastal/source-boundary sliver.",
                    vec![
                        "Baldwin County has a tiny residual outside the BTS/NTAD Central polygon at source precision; review before publication."
                            .to_string(),
                    ],
                ),
                _ => (
                    "TIGER 2024 county polygon is fully covered by the expected BTS/NTAD time-zone polygon at seed precision.",
                    Vec::new(),
                ),
            };
            CountyGeometryReconciliation {
                unit_id: assignment.unit_id,
                assignment_zone_id: assignment_zone_id.clone(),
                legal_source_id: assignment.legal_source_id,
                geometry_source_id: assignment
                    .geometry_source_id
                    .unwrap_or_else(|| "dot-time-zone-map-layer".to_string()),
                status: GeometryReconciliationStatus::Reconciled,
                evidence_note: format!(
                    "{coverage_note} Expected assignment zone: {assignment_zone_id}; geometry source: BTS/NTAD Time Zones ArcGIS FeatureServer layer 0."
                ),
                caveats,
            }
        })
        .collect();

    CountyGeometryReconciliationSet {
        reconciliation_id: "zones-us-county-seed-dot-geometry-reconciliation".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-27".to_string(),
        rows,
    }
}

pub fn seed_us_county_smoke_representative_points() -> CountyRepresentativePointSet {
    let records = vec![
        ("01001", 32.5, -86.65),
        ("01003", 30.7, -87.7),
        ("12001", 29.7, -82.4),
        ("12003", 30.3, -82.3),
    ]
    .into_iter()
    .map(|(unit_id, latitude, longitude)| {
        let point = RepresentativePoint {
            latitude,
            longitude,
            method: RepresentativePointMethod::InternalPoint,
            source_id: "census-county-gazetteer-2024".to_string(),
        };
        CountyRepresentativePointRecord {
            unit_id: unit_id.to_string(),
            solar_offset_minutes: point.solar_offset_minutes(),
            point,
            caveats: vec![
                "Census internal point is exploratory and not population-weighted.".to_string(),
            ],
        }
    })
    .collect();

    CountyRepresentativePointSet {
        point_set_id: "zones-us-county-smoke-representative-points".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-26".to_string(),
        records,
    }
}

pub fn seed_us_county_seed_representative_points() -> CountyRepresentativePointSet {
    let records = vec![
        ("01001", 32.532237, -86.64644),
        ("01003", 30.659218, -87.746067),
        ("12001", 29.67574, -82.357221),
        ("12003", 30.324442, -82.302284),
    ]
    .into_iter()
    .map(|(unit_id, latitude, longitude)| {
        let point = RepresentativePoint {
            latitude,
            longitude,
            method: RepresentativePointMethod::InternalPoint,
            source_id: "census-county-gazetteer-2024".to_string(),
        };
        CountyRepresentativePointRecord {
            unit_id: unit_id.to_string(),
            solar_offset_minutes: point.solar_offset_minutes(),
            point,
            caveats: vec![
                "Source-derived Census Gazetteer internal point; still exploratory and not population-weighted.".to_string(),
            ],
        }
    })
    .collect();

    CountyRepresentativePointSet {
        point_set_id: "zones-us-county-seed-representative-points".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-27".to_string(),
        records,
    }
}

pub fn build_county_baseline_plan_input(
    input_id: &str,
    scenario: ZoneScenario,
    context: &RplanContext,
    assignments: &CountyTimeZoneAssignmentSet,
    representative_points: &CountyRepresentativePointSet,
    catalog: &ZoneCatalog,
    caveats: Vec<String>,
) -> Result<ZonePlanInput, ZonePlanError> {
    context
        .validate()
        .map_err(|err| ZonePlanError::RplanContext(err.to_string()))?;
    catalog
        .validate()
        .map_err(|err| ZonePlanError::ZoneCatalog(err.to_string()))?;
    validate_non_empty_plan("zone_plan_input.input_id", input_id)?;
    validate_scenario_shape(&scenario)?;
    if assignments.source_manifest_id != representative_points.source_manifest_id
        || assignments.source_manifest_id != catalog.source_manifest_id
    {
        return Err(ZonePlanError::SourceManifestMismatch {
            input_source_manifest_id: assignments.source_manifest_id.clone(),
            manifest_id: catalog.source_manifest_id.clone(),
        });
    }
    let graph = context
        .graph
        .as_ref()
        .ok_or(ZonePlanError::MissingRplanGraph)?;
    let populations = context
        .populations
        .as_ref()
        .ok_or(ZonePlanError::MissingRplanPopulations)?;

    let assignments_by_unit = assignments
        .assignments
        .iter()
        .map(|assignment| (assignment.unit_id.as_str(), assignment))
        .collect::<BTreeMap<_, _>>();
    let points_by_unit = representative_points
        .records
        .iter()
        .map(|record| (record.unit_id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let catalog_offsets = catalog
        .zones
        .iter()
        .map(|zone| (zone.id.as_str(), zone.utc_offset_minutes))
        .collect::<BTreeMap<_, _>>();

    let mut zone_offsets = BTreeMap::new();
    for assignment in &assignments.assignments {
        let Some(offset) = catalog_offsets.get(assignment.zone_id.as_str()) else {
            return Err(ZonePlanError::PlanZoneMissingFromCatalog {
                zone_id: assignment.zone_id.clone(),
                catalog_id: catalog.catalog_id.clone(),
            });
        };
        zone_offsets.insert(assignment.zone_id.clone(), *offset);
    }
    let zones = zone_offsets
        .iter()
        .map(|(zone_id, utc_offset_minutes)| ZoneSpec {
            id: zone_id.clone(),
            utc_offset_minutes: *utc_offset_minutes,
        })
        .collect::<Vec<_>>();
    let zone_index_by_id = zones
        .iter()
        .enumerate()
        .map(|(index, zone)| (zone.id.as_str(), index))
        .collect::<BTreeMap<_, _>>();

    let mut units = Vec::with_capacity(context.units.unit_ids.len());
    let mut assignment_indices = Vec::with_capacity(context.units.unit_ids.len());
    for (unit_index, unit_id) in context.units.unit_ids.iter().enumerate() {
        let population = populations[unit_index];
        if population < 0 {
            return Err(ZonePlanError::NegativeRplanPopulation { unit_index });
        }
        let assignment = assignments_by_unit.get(unit_id.as_str()).ok_or_else(|| {
            ZonePlanError::RplanContext(format!(
                "missing county time-zone assignment for unit {unit_id}"
            ))
        })?;
        let point = points_by_unit.get(unit_id.as_str()).ok_or_else(|| {
            ZonePlanError::RplanContext(format!("missing representative point for unit {unit_id}"))
        })?;
        let zone_index = *zone_index_by_id
            .get(assignment.zone_id.as_str())
            .ok_or_else(|| ZonePlanError::PlanZoneMissingFromCatalog {
                zone_id: assignment.zone_id.clone(),
                catalog_id: catalog.catalog_id.clone(),
            })?;
        assignment_indices.push(zone_index);

        let mut unit_caveats = point.caveats.clone();
        unit_caveats.extend(assignment.caveats.iter().cloned());
        units.push(BoundaryUnit {
            id: unit_id.clone(),
            name: format!("GEOID {unit_id} baseline smoke unit"),
            solar_offset_minutes: point.solar_offset_minutes,
            population: population as u64,
            map_point: Some(MapPoint {
                latitude: point.point.latitude,
                longitude: point.point.longitude,
                source_id: Some(point.point.source_id.clone()),
            }),
            map_geometry: None,
            source_refs: Some(BoundaryUnitSourceRefs {
                boundary_source_id: context.units.source_id.clone(),
                representative_point_source_id: Some(point.point.source_id.clone()),
                population_source_id: Some("census-county-population-estimates-2024".to_string()),
                time_zone_assignment_source_id: Some(assignment.legal_source_id.clone()),
                time_zone_geometry_source_id: assignment.geometry_source_id.clone(),
                caveats: unit_caveats,
            }),
        });
    }

    let adjacency = graph
        .adjacency
        .iter()
        .map(|edges| edges.iter().map(|edge| edge.to as usize).collect())
        .collect::<Vec<_>>();
    let plan = ZonePlan {
        name: format!("{input_id}-plan"),
        zones,
        assignment: assignment_indices.clone(),
    };

    let input = ZonePlanInput {
        input_id: input_id.to_string(),
        source_manifest_id: assignments.source_manifest_id.clone(),
        scenario,
        units,
        adjacency,
        plan,
        reference_assignment: assignment_indices,
        caveats,
    };
    evaluate_zone_plan_input(&input)?;
    Ok(input)
}

pub fn seed_us_county_baseline_smoke_plan_input() -> ZonePlanInput {
    build_county_baseline_plan_input(
        "zones-us-county-baseline-smoke-plan-input",
        ZoneScenario {
            scenario_id: "us-county-smoke-current-law-shape".to_string(),
            kind: ZoneScenarioKind::CurrentLaw,
            label: "US county smoke current-law baseline input".to_string(),
            authority_source_id: Some("dot-49-cfr-71".to_string()),
        },
        &seed_us_county_smoke_rplan_context(),
        &seed_us_county_smoke_time_zone_assignments(),
        &seed_us_county_smoke_representative_points(),
        &seed_zone_catalog(),
        vec![
            "Baseline smoke input assembled from RPLAN context, assignment, and representative-point smoke fixtures.".to_string(),
            "Current-law assignment evidence remains placeholder and is not publication-ready.".to_string(),
            "Representative points are Census internal points and remain exploratory.".to_string(),
        ],
    )
    .unwrap()
}

pub fn seed_us_county_baseline_seed_plan_input() -> ZonePlanInput {
    build_county_baseline_plan_input(
        "zones-us-county-baseline-seed-plan-input",
        ZoneScenario {
            scenario_id: "us-county-seed-current-law-shape".to_string(),
            kind: ZoneScenarioKind::CurrentLaw,
            label: "US county seed current-law baseline input".to_string(),
            authority_source_id: Some("dot-49-cfr-71".to_string()),
        },
        &seed_us_county_seed_rplan_context(),
        &seed_us_county_seed_time_zone_assignments(),
        &seed_us_county_seed_representative_points(),
        &seed_zone_catalog(),
        vec![
            "Baseline seed input uses source-derived Census Gazetteer points and 2024 county population estimates for four county-shaped rows.".to_string(),
            "Current-law assignment evidence cites 49 CFR clauses for the four seed counties but still needs county-level DOT geometry reconciliation before publication.".to_string(),
            "RPLAN adjacency is TIGER-derived for the four seed counties; no boundary adjacencies exist among this selected set.".to_string(),
            "Representative points are Census internal points and remain exploratory.".to_string(),
        ],
    )
    .unwrap()
}

fn validate_inputs(
    units: &[BoundaryUnit],
    adjacency: &[Vec<usize>],
    plan: &ZonePlan,
) -> Result<(), ZonePlanError> {
    if plan.zones.is_empty() {
        return Err(ZonePlanError::EmptyZones);
    }
    validate_zones(&plan.zones)?;
    if units.len() != plan.assignment.len() {
        return Err(ZonePlanError::UnitAssignmentMismatch {
            unit_count: units.len(),
            assignment_count: plan.assignment.len(),
        });
    }
    if adjacency.len() != units.len() {
        return Err(ZonePlanError::AdjacencyUnitMismatch {
            adjacency_count: adjacency.len(),
            unit_count: units.len(),
        });
    }
    for unit in units {
        if let Some(point) = &unit.map_point {
            if !point.latitude.is_finite() || point.latitude < -90.0 || point.latitude > 90.0 {
                return Err(ZonePlanError::InvalidMapLatitude {
                    unit_id: unit.id.clone(),
                    latitude: point.latitude.to_string(),
                });
            }
            if !point.longitude.is_finite() || point.longitude < -180.0 || point.longitude > 180.0 {
                return Err(ZonePlanError::InvalidMapLongitude {
                    unit_id: unit.id.clone(),
                    longitude: point.longitude.to_string(),
                });
            }
        }
        if let Some(geometry) = &unit.map_geometry {
            for point in map_geometry_points(geometry) {
                let longitude = point[0];
                let latitude = point[1];
                if !latitude.is_finite() || latitude < -90.0 || latitude > 90.0 {
                    return Err(ZonePlanError::InvalidMapLatitude {
                        unit_id: unit.id.clone(),
                        latitude: latitude.to_string(),
                    });
                }
                if !longitude.is_finite() || longitude < -180.0 || longitude > 180.0 {
                    return Err(ZonePlanError::InvalidMapLongitude {
                        unit_id: unit.id.clone(),
                        longitude: longitude.to_string(),
                    });
                }
            }
        }
    }
    for (unit_index, &zone_index) in plan.assignment.iter().enumerate() {
        if zone_index >= plan.zones.len() {
            return Err(ZonePlanError::UnknownZone {
                unit_index,
                zone_index,
            });
        }
    }
    for (from, edges) in adjacency.iter().enumerate() {
        for &to in edges {
            if to >= units.len() {
                return Err(ZonePlanError::EdgeOutOfBounds {
                    from,
                    to,
                    unit_count: units.len(),
                });
            }
        }
    }
    Ok(())
}

fn evaluate_zone_plan_with_reference(
    units: &[BoundaryUnit],
    adjacency: &[Vec<usize>],
    plan: &ZonePlan,
    reference_assignment: &[usize],
) -> Result<ZonePlanReport, ZonePlanError> {
    let mut report = evaluate_zone_plan(units, adjacency, plan)?;
    if reference_assignment.is_empty() {
        return Ok(report);
    }
    validate_reference_assignment(units, plan, reference_assignment)?;
    let mut moved_unit_count = 0;
    let mut moved_population = 0;
    for (unit_index, unit) in units.iter().enumerate() {
        if plan.assignment[unit_index] != reference_assignment[unit_index] {
            moved_unit_count += 1;
            moved_population += unit.population;
        }
    }
    report.moved_unit_count = Some(moved_unit_count);
    report.moved_population = Some(moved_population);
    Ok(report)
}

fn validate_reference_assignment(
    units: &[BoundaryUnit],
    plan: &ZonePlan,
    reference_assignment: &[usize],
) -> Result<(), ZonePlanError> {
    if reference_assignment.len() != units.len() {
        return Err(ZonePlanError::ReferenceAssignmentMismatch {
            reference_assignment_count: reference_assignment.len(),
            unit_count: units.len(),
        });
    }
    for (unit_index, &zone_index) in reference_assignment.iter().enumerate() {
        if zone_index >= plan.zones.len() {
            return Err(ZonePlanError::UnknownReferenceZone {
                unit_index,
                zone_index,
            });
        }
    }
    Ok(())
}

fn labels_in_use(assignment: &[usize]) -> BTreeSet<usize> {
    assignment.iter().copied().collect()
}

fn validate_non_empty_plan(kind: &'static str, value: &str) -> Result<(), ZonePlanError> {
    if value.is_empty() {
        Err(ZonePlanError::EmptyId { kind })
    } else {
        Ok(())
    }
}

fn validate_zones(zones: &[ZoneSpec]) -> Result<(), ZonePlanError> {
    if zones.is_empty() {
        return Err(ZonePlanError::EmptyZones);
    }
    for zone in zones {
        if !(-14 * 60..=14 * 60).contains(&zone.utc_offset_minutes) {
            return Err(ZonePlanError::InvalidZoneUtcOffset {
                zone_id: zone.id.clone(),
                utc_offset_minutes: zone.utc_offset_minutes,
            });
        }
    }
    Ok(())
}

pub fn seed_fixture() -> (Vec<BoundaryUnit>, Vec<Vec<usize>>, ZonePlan) {
    let units = vec![
        BoundaryUnit {
            id: "west-a".to_string(),
            name: "West A County".to_string(),
            solar_offset_minutes: -360.0,
            population: 100,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        },
        BoundaryUnit {
            id: "west-b".to_string(),
            name: "West B County".to_string(),
            solar_offset_minutes: -345.0,
            population: 80,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        },
        BoundaryUnit {
            id: "east-a".to_string(),
            name: "East A County".to_string(),
            solar_offset_minutes: -300.0,
            population: 90,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        },
        BoundaryUnit {
            id: "east-b".to_string(),
            name: "East B County".to_string(),
            solar_offset_minutes: -285.0,
            population: 70,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        },
    ];
    let adjacency = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    let plan = ZonePlan {
        name: "seed-two-zone-plan".to_string(),
        zones: vec![
            ZoneSpec {
                id: "utc-minus-06-00".to_string(),
                utc_offset_minutes: -360,
            },
            ZoneSpec {
                id: "utc-minus-05-00".to_string(),
                utc_offset_minutes: -300,
            },
        ],
        assignment: vec![0, 0, 1, 1],
    };
    (units, adjacency, plan)
}

pub fn seed_plan_input() -> ZonePlanInput {
    let (units, adjacency, plan) = seed_fixture();
    ZonePlanInput {
        input_id: "zones-seed-plan-input".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        scenario: ZoneScenario {
            scenario_id: "seed-current-law-baseline".to_string(),
            kind: ZoneScenarioKind::CurrentLaw,
            label: "Seed current-law baseline".to_string(),
            authority_source_id: Some("dot-49-cfr-71".to_string()),
        },
        units,
        adjacency,
        plan,
        reference_assignment: vec![0, 0, 1, 1],
        caveats: vec!["Synthetic fixture for evaluator contract tests only.".to_string()],
    }
}

pub fn seed_plan_input_with_map_points() -> ZonePlanInput {
    let mut input = seed_plan_input();
    input.input_id = "zones-seed-plan-input-map-points".to_string();
    input.units[0].map_point = Some(MapPoint {
        latitude: 41.8781,
        longitude: -87.6298,
        source_id: Some("synthetic-map-points".to_string()),
    });
    input.units[0].map_geometry = Some(seed_square_geometry(-88.1, 41.4, -87.1, 42.2));
    input.units[1].map_point = Some(MapPoint {
        latitude: 39.7684,
        longitude: -86.1581,
        source_id: Some("synthetic-map-points".to_string()),
    });
    input.units[1].map_geometry = Some(seed_square_geometry(-86.7, 39.3, -85.7, 40.2));
    input.units[2].map_point = Some(MapPoint {
        latitude: 40.7128,
        longitude: -74.0060,
        source_id: Some("synthetic-map-points".to_string()),
    });
    input.units[2].map_geometry = Some(seed_square_geometry(-74.5, 40.3, -73.5, 41.1));
    input.units[3].map_point = Some(MapPoint {
        latitude: 39.9526,
        longitude: -75.1652,
        source_id: Some("synthetic-map-points".to_string()),
    });
    input.units[3].map_geometry = Some(seed_square_geometry(-75.7, 39.5, -74.7, 40.4));
    input.caveats.push(
        "Synthetic map points validate coordinate-aware rendering only; not a county dataset."
            .to_string(),
    );
    input
}

fn seed_square_geometry(west: f64, south: f64, east: f64, north: f64) -> MapGeometry {
    MapGeometry::Polygon(vec![vec![
        [west, south],
        [east, south],
        [east, north],
        [west, north],
        [west, south],
    ]])
}

pub fn seed_source_manifest() -> SourceManifest {
    SourceManifest {
        manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-26".to_string(),
        sources: vec![
            SourceCitation {
                source_id: "census-tiger-counties-2024".to_string(),
                title: "2024 TIGER/Line Counties".to_string(),
                kind: SourceKind::GeospatialBoundary,
                url: "https://www.census.gov/cgi-bin/geo/shapefiles/index.php?layergroup=Counties&year=2024".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: Some("2024".to_string()),
                content_hash: None,
                caveats: vec!["Boundary source only; not evidence of legal time-zone assignment.".to_string()],
            },
            SourceCitation {
                source_id: "census-county-gazetteer-2024".to_string(),
                title: "2024 County Gazetteer Files".to_string(),
                kind: SourceKind::RepresentativePoint,
                url: "https://www.census.gov/geographies/reference-files/2024/geo/gazetter-file.html".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: Some("2024".to_string()),
                content_hash: None,
                caveats: vec!["Internal points are exploratory and not population-weighted.".to_string()],
            },
            SourceCitation {
                source_id: "census-county-population-estimates-2024".to_string(),
                title: "County Population Totals and Components of Change: 2020-2024".to_string(),
                kind: SourceKind::Population,
                url: "https://www.census.gov/programs-surveys/popest/data/tables.html".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: Some("2024".to_string()),
                content_hash: None,
                caveats: vec![
                    "Population source selected for county weighting; smoke fixtures may use tiny placeholder weights until source-derived rows are generated.".to_string(),
                ],
            },
            SourceCitation {
                source_id: "dot-49-cfr-71".to_string(),
                title: "49 CFR Part 71".to_string(),
                kind: SourceKind::LegalText,
                url: "https://www.ecfr.gov/current/title-49/subtitle-A/part-71".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: None,
                content_hash: None,
                caveats: vec!["Legal text must be converted to geospatial assignments before county scoring.".to_string()],
            },
            SourceCitation {
                source_id: "dot-time-zone-map-layer".to_string(),
                title: "DOT Time Zones Geospatial Map".to_string(),
                kind: SourceKind::GeospatialBoundary,
                url: "https://www.transportation.gov/regulations/time-act".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: None,
                content_hash: None,
                caveats: vec![
                    "Map layer must be reconciled with 49 CFR Part 71 and county boundaries before county-level assignments are treated as evidence.".to_string(),
                ],
            },
            SourceCitation {
                source_id: "dot-time-zone-procedure".to_string(),
                title: "Procedure for Moving an Area from One Time Zone to Another".to_string(),
                kind: SourceKind::LegalText,
                url: "https://www.transportation.gov/regulations/procedure-moving-area-one-time-zone-another".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: None,
                content_hash: None,
                caveats: vec!["Procedure guidance, not a scoring dataset.".to_string()],
            },
            SourceCitation {
                source_id: "iana-tzdb-theory".to_string(),
                title: "Theory and pragmatics of the tz code and data".to_string(),
                kind: SourceKind::TimeRuleDatabase,
                url: "https://ftp.iana.org/tz/theory.html".to_string(),
                retrieved_on: "2026-05-26".to_string(),
                vintage: None,
                content_hash: None,
                caveats: vec!["IANA tzdb does not record complete legal boundaries.".to_string()],
            },
        ],
    }
}

pub fn seed_source_gate_policy() -> SourceGatePolicy {
    SourceGatePolicy {
        policy_id: "zones-us-foundation-source-gate-v0".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-26".to_string(),
        entries: vec![
            SourceGateEntry {
                source_id: "census-tiger-counties-2024".to_string(),
                acquisition_mode: SourceAcquisitionMode::FletchCandidate,
                cache_policy: SourceCachePolicy::IgnoredLocalCache,
                rights_posture:
                    "US Census public data; raw national GIS cache must stay out of git."
                        .to_string(),
                expected_artifact: "RPLAN county context with GEOID unit order, adjacency, and source hash references."
                    .to_string(),
                hash_required: true,
                gate_notes: vec![
                    "Boundary source only; time-zone assignments must cite DOT/49 CFR evidence separately."
                        .to_string(),
                ],
            },
            SourceGateEntry {
                source_id: "census-county-gazetteer-2024".to_string(),
                acquisition_mode: SourceAcquisitionMode::FletchCandidate,
                cache_policy: SourceCachePolicy::IgnoredLocalCache,
                rights_posture:
                    "US Census public data; derived representative-point rows may be regenerated."
                        .to_string(),
                expected_artifact:
                    "County representative-point table with GEOID, longitude, latitude, and source hash."
                        .to_string(),
                hash_required: true,
                gate_notes: vec![
                    "Internal points are exploratory and must be labeled before publication."
                        .to_string(),
                ],
            },
            SourceGateEntry {
                source_id: "census-county-population-estimates-2024".to_string(),
                acquisition_mode: SourceAcquisitionMode::FletchCandidate,
                cache_policy: SourceCachePolicy::IgnoredLocalCache,
                rights_posture:
                    "US Census public data; source-derived county weights must record vintage."
                        .to_string(),
                expected_artifact:
                    "County population table keyed by GEOID with source hash and vintage."
                        .to_string(),
                hash_required: true,
                gate_notes: vec![
                    "Population weights must not remain placeholders for the baseline scorecard."
                        .to_string(),
                ],
            },
            SourceGateEntry {
                source_id: "dot-49-cfr-71".to_string(),
                acquisition_mode: SourceAcquisitionMode::ManualReference,
                cache_policy: SourceCachePolicy::ReferenceOnly,
                rights_posture: "Federal regulation reference; cite retrieved eCFR text and clause paths."
                    .to_string(),
                expected_artifact:
                    "Machine-readable current-law assignment evidence table citing 49 CFR Part 71 clauses."
                        .to_string(),
                hash_required: false,
                gate_notes: vec![
                    "County-level assignments must preserve clause evidence and uncertainty."
                        .to_string(),
                ],
            },
            SourceGateEntry {
                source_id: "dot-time-zone-map-layer".to_string(),
                acquisition_mode: SourceAcquisitionMode::FletchCandidate,
                cache_policy: SourceCachePolicy::IgnoredLocalCache,
                rights_posture:
                    "DOT public map reference; verify metadata, vintage, and distribution terms before broad cache."
                        .to_string(),
                expected_artifact:
                    "Reconciliation table comparing DOT geometry to county units and 49 CFR clauses."
                        .to_string(),
                hash_required: true,
                gate_notes: vec![
                    "Endpoint metadata is recorded at data/source-endpoints/dot-time-zones-arcgis.json."
                        .to_string(),
                    "Map geometry cannot override 49 CFR text without a documented reconciliation note."
                        .to_string(),
                ],
            },
            SourceGateEntry {
                source_id: "dot-time-zone-procedure".to_string(),
                acquisition_mode: SourceAcquisitionMode::ManualReference,
                cache_policy: SourceCachePolicy::ReferenceOnly,
                rights_posture: "DOT public guidance; cite as process context, not score data."
                    .to_string(),
                expected_artifact:
                    "Research note for boundary-change procedure and convenience-of-commerce context."
                        .to_string(),
                hash_required: false,
                gate_notes: vec!["Procedure guidance must not be treated as a scoring dataset.".to_string()],
            },
            SourceGateEntry {
                source_id: "iana-tzdb-theory".to_string(),
                acquisition_mode: SourceAcquisitionMode::ManualReference,
                cache_policy: SourceCachePolicy::ReferenceOnly,
                rights_posture:
                    "IANA tzdb documentation reference; use for limitations and offset-rule context."
                        .to_string(),
                expected_artifact:
                    "Research note explaining why IANA tzdb is not legal-boundary evidence."
                        .to_string(),
                hash_required: false,
                gate_notes: vec![
                    "Do not use as complete legal-boundary geometry for the US county baseline."
                        .to_string(),
                ],
            },
        ],
    }
}

pub fn seed_source_limitation_matrix() -> SourceLimitationMatrix {
    SourceLimitationMatrix {
        matrix_id: "zones-source-limitation-matrix-v0".to_string(),
        generated_on: "2026-05-26".to_string(),
        entries: vec![
            SourceLimitationEntry {
                source_id: "iana-tzdb".to_string(),
                source_kind: SourceKind::TimeRuleDatabase,
                title: "IANA Time Zone Database".to_string(),
                assessments: vec![
                    SourceClaimAssessment {
                        claim: SourceClaim::OffsetRuleHistory,
                        support: SourceSupportLevel::Supports,
                        notes: "Records post-1970 civil-time transitions for representative tzdb zones."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::LegalBoundaryGeometry,
                        support: SourceSupportLevel::NotSupported,
                        notes: "Does not provide authoritative legal polygons or administrative boundary assignments."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::HistoricalReconstruction,
                        support: SourceSupportLevel::Partial,
                        notes: "Contains some pre-1970 material, but its own theory file warns that complete past-time handling everywhere is out of scope."
                            .to_string(),
                    },
                ],
                caveats: vec![
                    "Use for offset-rule history, not as complete legal-boundary evidence."
                        .to_string(),
                ],
            },
            SourceLimitationEntry {
                source_id: "unicode-cldr".to_string(),
                source_kind: SourceKind::TimeRuleDatabase,
                title: "Unicode CLDR time-zone metadata".to_string(),
                assessments: vec![
                    SourceClaimAssessment {
                        claim: SourceClaim::DisplayMetadata,
                        support: SourceSupportLevel::Supports,
                        notes: "Provides localized time-zone names, aliases, and interoperability metadata."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::LegalBoundaryGeometry,
                        support: SourceSupportLevel::NotSupported,
                        notes: "Not a legal or geospatial boundary authority.".to_string(),
                    },
                ],
                caveats: vec![
                    "Useful for labels and mappings, not for scoring legal zone boundaries."
                        .to_string(),
                ],
            },
            SourceLimitationEntry {
                source_id: "national-legal-sources".to_string(),
                source_kind: SourceKind::LegalText,
                title: "National statutes, regulations, and official maps".to_string(),
                assessments: vec![
                    SourceClaimAssessment {
                        claim: SourceClaim::CurrentLegalOffset,
                        support: SourceSupportLevel::Supports,
                        notes: "Best authority for current legal claims when current and accessible."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::LegalBoundaryGeometry,
                        support: SourceSupportLevel::Partial,
                        notes: "May define legal boundaries textually or by map, but availability and machine readability vary."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::OffsetRuleHistory,
                        support: SourceSupportLevel::Partial,
                        notes: "Historical law can be authoritative, but coverage, language, and amendment history vary by jurisdiction."
                            .to_string(),
                    },
                ],
                caveats: vec![
                    "Requires country-specific audit for licensing, language, currency, and historical completeness."
                        .to_string(),
                ],
            },
            SourceLimitationEntry {
                source_id: "official-administrative-boundaries".to_string(),
                source_kind: SourceKind::GeospatialBoundary,
                title: "Official administrative boundary geometry".to_string(),
                assessments: vec![
                    SourceClaimAssessment {
                        claim: SourceClaim::AdministrativeBoundaryGeometry,
                        support: SourceSupportLevel::Supports,
                        notes: "Primary input for state, county, province, municipality, or district graph construction."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::LegalBoundaryGeometry,
                        support: SourceSupportLevel::Partial,
                        notes: "Supports legal time-zone scoring only when time law aligns to these units or assignments are independently derived."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::CurrentLegalOffset,
                        support: SourceSupportLevel::NotSupported,
                        notes: "Boundary geometry alone does not identify the legal UTC offset.".to_string(),
                    },
                ],
                caveats: vec![
                    "Must be paired with legal time-zone assignments and source vintages."
                        .to_string(),
                ],
            },
            SourceLimitationEntry {
                source_id: "population-and-representative-points".to_string(),
                source_kind: SourceKind::Population,
                title: "Population weights and representative points".to_string(),
                assessments: vec![
                    SourceClaimAssessment {
                        claim: SourceClaim::PopulationWeights,
                        support: SourceSupportLevel::Supports,
                        notes: "Needed for population-weighted error and disruption metrics."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::RepresentativePoint,
                        support: SourceSupportLevel::Supports,
                        notes: "Needed to compute longitude-derived solar offset when geometry is not directly integrated."
                            .to_string(),
                    },
                    SourceClaimAssessment {
                        claim: SourceClaim::LegalBoundaryGeometry,
                        support: SourceSupportLevel::NotSupported,
                        notes: "Representative points and population tables do not establish legal time-zone boundaries."
                            .to_string(),
                    },
                ],
                caveats: vec![
                    "Centroid, internal-point, and population-center choices can change measured solar error."
                        .to_string(),
                ],
            },
        ],
    }
}

pub fn seed_module_boundary_contract() -> ModuleBoundaryContract {
    ModuleBoundaryContract {
        contract_id: "zones-module-boundaries-v0".to_string(),
        generated_on: "2026-05-26".to_string(),
        entries: vec![
            ModuleBoundaryEntry {
                module_id: "rline".to_string(),
                owns: vec![
                    "Reusable graph kernels and connectivity metrics.".to_string(),
                    "Generic statistical or optimization primitives with no civic-time semantics."
                        .to_string(),
                ],
                must_not_own: vec![
                    "Time-zone legal authority or solar-time policy assumptions.".to_string(),
                    "ZONES source caveats, scenario labels, or civil-time scoring definitions."
                        .to_string(),
                ],
                upstream_dependencies: vec![],
                downstream_consumers: vec!["rplan".to_string(), "zones".to_string()],
            },
            ModuleBoundaryEntry {
                module_id: "rplan".to_string(),
                owns: vec![
                    "Portable legal-boundary unit graph and context contracts.".to_string(),
                    "Unit ids, adjacency, populations, source hashes, and assignment carriers."
                        .to_string(),
                ],
                must_not_own: vec![
                    "Time-zone reform recommendations.".to_string(),
                    "Solar-noon error scoring or DST scenario semantics.".to_string(),
                ],
                upstream_dependencies: vec!["rline".to_string()],
                downstream_consumers: vec!["zones".to_string()],
            },
            ModuleBoundaryEntry {
                module_id: "zones".to_string(),
                owns: vec![
                    "Civil-time regimes, offset rules, DST deltas, and scenario labels."
                        .to_string(),
                    "Solar-time scoring, source limitation matrices, and time-zone research artifacts."
                        .to_string(),
                    "Mappings from RPLAN contexts into time-zone plan evaluations.".to_string(),
                ],
                must_not_own: vec![
                    "Generic graph-kernel implementations that belong in RLINE.".to_string(),
                    "Reusable non-time-zone redistricting context semantics that belong in RPLAN."
                        .to_string(),
                ],
                upstream_dependencies: vec!["rline".to_string(), "rplan".to_string()],
                downstream_consumers: vec![],
            },
            ModuleBoundaryEntry {
                module_id: "bisect".to_string(),
                owns: vec![
                    "Election/redistricting application precedent and Census/GEOID implementation examples."
                        .to_string(),
                ],
                must_not_own: vec![
                    "New ZONES time-zone policy logic.".to_string(),
                    "Shared boundary contracts that should be promoted into RPLAN.".to_string(),
                ],
                upstream_dependencies: vec!["rline".to_string(), "rplan".to_string()],
                downstream_consumers: vec!["zones-reference-only".to_string()],
            },
        ],
        caveats: vec![
            "This contract is architectural guidance; it does not prevent future extraction of reusable ZONES components into shared crates."
                .to_string(),
        ],
    }
}

pub fn seed_zone_catalog() -> ZoneCatalog {
    ZoneCatalog {
        catalog_id: "zones-seed-offset-catalog".to_string(),
        source_manifest_id: "zones-us-foundation-sources".to_string(),
        generated_on: "2026-05-26".to_string(),
        zones: vec![
            ZoneSpec {
                id: "utc-minus-08-00".to_string(),
                utc_offset_minutes: -480,
            },
            ZoneSpec {
                id: "utc-minus-06-00".to_string(),
                utc_offset_minutes: -360,
            },
            ZoneSpec {
                id: "utc-minus-05-00".to_string(),
                utc_offset_minutes: -300,
            },
            ZoneSpec {
                id: "utc-plus-05-30".to_string(),
                utc_offset_minutes: 330,
            },
            ZoneSpec {
                id: "utc-plus-05-45".to_string(),
                utc_offset_minutes: 345,
            },
        ],
        caveats: vec![
            "Seed offset catalog for validation only; not a complete legal time-zone catalog."
                .to_string(),
        ],
    }
}

pub fn seed_temporal_dataset() -> TemporalDataset {
    TemporalDataset {
        dataset_id: "zones-temporal-non-us-pilot".to_string(),
        source_manifest: SourceManifest {
            manifest_id: "zones-temporal-pilot-sources".to_string(),
            generated_on: "2026-05-26".to_string(),
            sources: vec![
                SourceCitation {
                    source_id: "pilot-boundaries".to_string(),
                    title: "Synthetic civic boundary pilot".to_string(),
                    kind: SourceKind::GeospatialBoundary,
                    url: "https://github.com/giodl73-repo/ZONES".to_string(),
                    retrieved_on: "2026-05-26".to_string(),
                    vintage: Some("fixture".to_string()),
                    content_hash: None,
                    caveats: vec![
                        "Synthetic boundary graph used only to validate the temporal model contract."
                            .to_string(),
                    ],
                },
                SourceCitation {
                    source_id: "pilot-time-law".to_string(),
                    title: "Synthetic non-US time-law pilot".to_string(),
                    kind: SourceKind::LegalText,
                    url: "https://github.com/giodl73-repo/ZONES".to_string(),
                    retrieved_on: "2026-05-26".to_string(),
                    vintage: Some("fixture".to_string()),
                    content_hash: None,
                    caveats: vec![
                        "Synthetic authority source; proves shape, not legal truth.".to_string(),
                    ],
                },
                SourceCitation {
                    source_id: "pilot-population".to_string(),
                    title: "Synthetic population weights".to_string(),
                    kind: SourceKind::Population,
                    url: "https://github.com/giodl73-repo/ZONES".to_string(),
                    retrieved_on: "2026-05-26".to_string(),
                    vintage: Some("fixture".to_string()),
                    content_hash: None,
                    caveats: vec![
                        "Synthetic weights for evaluation-context validation only.".to_string(),
                    ],
                },
            ],
        },
        jurisdictions: vec![
            Jurisdiction {
                jurisdiction_id: "NP".to_string(),
                name: "Nepal".to_string(),
                parent_jurisdiction_id: None,
                source_id: "pilot-boundaries".to_string(),
                temporal_extent: TemporalExtent::current(),
            },
            Jurisdiction {
                jurisdiction_id: "AU".to_string(),
                name: "Australia".to_string(),
                parent_jurisdiction_id: None,
                source_id: "pilot-boundaries".to_string(),
                temporal_extent: TemporalExtent::current(),
            },
        ],
        units: vec![
            TemporalBoundaryUnit {
                unit_id: "NP-KTM".to_string(),
                jurisdiction_id: "NP".to_string(),
                unit_level: UnitLevel::District,
                name: "Kathmandu pilot district".to_string(),
                geometry_ref: Some("pilot-boundaries#NP-KTM".to_string()),
                representative_point: RepresentativePoint {
                    latitude: 27.7172,
                    longitude: 85.3240,
                    method: RepresentativePointMethod::SourceProvided,
                    source_id: "pilot-boundaries".to_string(),
                },
                temporal_extent: TemporalExtent::current(),
            },
            TemporalBoundaryUnit {
                unit_id: "AU-ADL".to_string(),
                jurisdiction_id: "AU".to_string(),
                unit_level: UnitLevel::Municipality,
                name: "Adelaide pilot municipality".to_string(),
                geometry_ref: Some("pilot-boundaries#AU-ADL".to_string()),
                representative_point: RepresentativePoint {
                    latitude: -34.9285,
                    longitude: 138.6007,
                    method: RepresentativePointMethod::SourceProvided,
                    source_id: "pilot-boundaries".to_string(),
                },
                temporal_extent: TemporalExtent::current(),
            },
        ],
        boundary_graphs: vec![BoundaryGraphVersion {
            graph_id: "pilot-global-temporal-graph".to_string(),
            unit_universe_id: "pilot-global-temporal-units".to_string(),
            source_id: "pilot-boundaries".to_string(),
            temporal_extent: TemporalExtent::current(),
            adjacency: vec![vec![], vec![]],
        }],
        regimes: vec![
            TimeZoneRegime {
                regime_id: "nepal-current-law".to_string(),
                authority: RegimeAuthority::CurrentLaw,
                jurisdiction_scope: "NP".to_string(),
                source_id: "pilot-time-law".to_string(),
                temporal_extent: TemporalExtent::current(),
                assignments: vec![TimeZoneAssignment {
                    unit_id: "NP-KTM".to_string(),
                    zone_id: "NPT".to_string(),
                    temporal_extent: TemporalExtent::current(),
                }],
                offset_rules: vec![OffsetRule {
                    rule_id: "npt-current".to_string(),
                    zone_id: "NPT".to_string(),
                    standard_offset_minutes: 345,
                    temporal_extent: TemporalExtent::current(),
                    dst_delta_minutes: None,
                    transition_rule_ref: None,
                    observance_notes: vec!["Quarter-hour offset pilot.".to_string()],
                }],
            },
            TimeZoneRegime {
                regime_id: "south-australia-current-law".to_string(),
                authority: RegimeAuthority::CurrentLaw,
                jurisdiction_scope: "AU-SA".to_string(),
                source_id: "pilot-time-law".to_string(),
                temporal_extent: TemporalExtent::current(),
                assignments: vec![TimeZoneAssignment {
                    unit_id: "AU-ADL".to_string(),
                    zone_id: "ACST".to_string(),
                    temporal_extent: TemporalExtent::current(),
                }],
                offset_rules: vec![OffsetRule {
                    rule_id: "acst-current".to_string(),
                    zone_id: "ACST".to_string(),
                    standard_offset_minutes: 570,
                    temporal_extent: TemporalExtent::current(),
                    dst_delta_minutes: Some(60),
                    transition_rule_ref: Some("southern-hemisphere-seasonal-dst".to_string()),
                    observance_notes: vec![
                        "Half-hour standard offset with a DST delta pilot.".to_string(),
                    ],
                }],
            },
        ],
        evaluation_contexts: vec![
            EvaluationContext {
                evaluation_period: TemporalExtent::current(),
                boundary_graph_id: "pilot-global-temporal-graph".to_string(),
                regime_id: "nepal-current-law".to_string(),
                representative_point_method: RepresentativePointMethod::SourceProvided,
                weighting_source_id: "pilot-population".to_string(),
                source_vintage: "fixture".to_string(),
            },
            EvaluationContext {
                evaluation_period: TemporalExtent::current(),
                boundary_graph_id: "pilot-global-temporal-graph".to_string(),
                regime_id: "south-australia-current-law".to_string(),
                representative_point_method: RepresentativePointMethod::SourceProvided,
                weighting_source_id: "pilot-population".to_string(),
                source_vintage: "fixture".to_string(),
            },
        ],
        caveats: vec![
            "Synthetic non-US fixture validates the data contract only; it is not a legal dataset."
                .to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rplan_core::{
        CanonicalOrder, EdgeKind, EdgeSemantics, PlanUnitIndex, RplanContext, SourceHashes,
        UnitEdge, UnitGraph, UnitKind, RCTX_VERSION,
    };

    #[test]
    fn seed_plan_scores_boundary_connectivity_and_error() {
        let (units, adjacency, plan) = seed_fixture();

        let report = evaluate_zone_plan(&units, &adjacency, &plan).unwrap();

        assert_eq!(report.plan_name, "seed-two-zone-plan");
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.zone_count, 2);
        assert_eq!(report.boundary_edges, 2);
        assert!(report.all_zones_connected);
        assert!((report.weighted_mean_absolute_error_minutes - 6.617647058823529).abs() < 1e-9);
        assert_eq!(report.max_absolute_error_minutes, 15.0);
        assert_eq!(report.moved_unit_count, None);
        assert_eq!(report.moved_population, None);
    }

    #[test]
    fn seed_plan_input_scores_through_file_contract() {
        let report = evaluate_zone_plan_input(&seed_plan_input()).unwrap();

        assert_eq!(report.plan_name, "seed-two-zone-plan");
        assert_eq!(report.boundary_edges, 2);
        assert!(report.all_zones_connected);
        assert_eq!(report.moved_unit_count, Some(0));
        assert_eq!(report.moved_population, Some(0));
    }

    #[test]
    fn plan_input_reports_moves_from_reference_assignment() {
        let mut input = seed_plan_input();
        input.plan.assignment = vec![0, 1, 1, 1];

        let report = evaluate_zone_plan_input(&input).unwrap();

        assert_eq!(report.moved_unit_count, Some(1));
        assert_eq!(report.moved_population, Some(80));
    }

    #[test]
    fn reference_assignment_length_is_validated() {
        let mut input = seed_plan_input();
        input.reference_assignment = vec![0, 1];

        assert_eq!(
            evaluate_zone_plan_input(&input),
            Err(ZonePlanError::ReferenceAssignmentMismatch {
                reference_assignment_count: 2,
                unit_count: 4,
            })
        );
    }

    #[test]
    fn seed_plan_input_scores_with_matching_source_manifest() {
        let report =
            evaluate_zone_plan_input_with_manifest(&seed_plan_input(), &seed_source_manifest())
                .unwrap();

        assert_eq!(report.plan_name, "seed-two-zone-plan");
        assert_eq!(report.unit_count, 4);
    }

    #[test]
    fn seed_plan_input_scores_with_matching_catalog() {
        let report = evaluate_zone_plan_input_with_manifest_and_catalog(
            &seed_plan_input(),
            &seed_source_manifest(),
            &seed_zone_catalog(),
        )
        .unwrap();

        assert_eq!(report.plan_name, "seed-two-zone-plan");
        assert_eq!(report.unit_count, 4);
    }

    #[test]
    fn plan_input_rejects_missing_catalog_zone() {
        let mut catalog = seed_zone_catalog();
        catalog.zones.retain(|zone| zone.id != "utc-minus-06-00");

        assert_eq!(
            evaluate_zone_plan_input_with_manifest_and_catalog(
                &seed_plan_input(),
                &seed_source_manifest(),
                &catalog,
            ),
            Err(ZonePlanError::PlanZoneMissingFromCatalog {
                zone_id: "utc-minus-06-00".to_string(),
                catalog_id: "zones-seed-offset-catalog".to_string(),
            })
        );
    }

    #[test]
    fn plan_input_rejects_catalog_offset_mismatch() {
        let mut catalog = seed_zone_catalog();
        catalog.zones[1].utc_offset_minutes = -300;

        assert_eq!(
            evaluate_zone_plan_input_with_manifest_and_catalog(
                &seed_plan_input(),
                &seed_source_manifest(),
                &catalog,
            ),
            Err(ZonePlanError::PlanZoneCatalogOffsetMismatch {
                zone_id: "utc-minus-06-00".to_string(),
                plan_utc_offset_minutes: -360,
                catalog_utc_offset_minutes: -300,
            })
        );
    }

    #[test]
    fn seed_plan_evaluation_carries_unit_scores_and_caveats() {
        let evaluation = evaluate_zone_plan_evaluation_with_catalog(
            &seed_plan_input(),
            &seed_source_manifest(),
            &seed_zone_catalog(),
        )
        .unwrap();

        assert_eq!(evaluation.input_id, "zones-seed-plan-input");
        assert_eq!(evaluation.scenario.scenario_id, "seed-current-law-baseline");
        assert_eq!(evaluation.scenario.kind, ZoneScenarioKind::CurrentLaw);
        assert_eq!(evaluation.source_manifest_id, "zones-us-foundation-sources");
        assert_eq!(evaluation.zone_summaries.len(), 2);
        assert_eq!(evaluation.zone_summaries[0].zone_id, "utc-minus-05-00");
        assert_eq!(evaluation.zone_summaries[0].population, 160);
        assert_eq!(evaluation.zone_summaries[0].moved_population, 0);
        assert!(
            (evaluation.zone_summaries[0].weighted_mean_absolute_error_minutes - 6.5625).abs()
                < 1e-9
        );
        assert_eq!(
            evaluation.zone_summaries[0].max_absolute_error_minutes,
            15.0
        );
        assert_eq!(evaluation.unit_scores.len(), 4);
        assert_eq!(evaluation.unit_scores[1].unit_id, "west-b");
        assert_eq!(evaluation.unit_scores[1].zone_id, "utc-minus-06-00");
        assert_eq!(
            evaluation.unit_scores[1].reference_zone_id,
            Some("utc-minus-06-00".to_string())
        );
        assert_eq!(evaluation.unit_scores[1].moved_from_reference, Some(false));
        assert_eq!(evaluation.unit_scores[1].absolute_error_minutes, 15.0);
        assert_eq!(evaluation.input_caveats.len(), 1);
        assert_eq!(evaluation.source_caveats.len(), 7);
    }

    #[test]
    fn plan_input_rejects_mismatched_source_manifest() {
        let mut manifest = seed_source_manifest();
        manifest.manifest_id = "different-manifest".to_string();

        assert_eq!(
            evaluate_zone_plan_input_with_manifest(&seed_plan_input(), &manifest),
            Err(ZonePlanError::SourceManifestMismatch {
                input_source_manifest_id: "zones-us-foundation-sources".to_string(),
                manifest_id: "different-manifest".to_string(),
            })
        );
    }

    #[test]
    fn current_law_scenario_requires_authority_source() {
        let mut input = seed_plan_input();
        input.scenario.authority_source_id = None;

        assert_eq!(
            evaluate_zone_plan_input(&input),
            Err(ZonePlanError::ScenarioAuthorityRequired {
                scenario_id: "seed-current-law-baseline".to_string(),
            })
        );
    }

    #[test]
    fn scenario_authority_source_must_exist_in_manifest() {
        let mut input = seed_plan_input();
        input.scenario.authority_source_id = Some("missing-source".to_string());

        assert_eq!(
            evaluate_zone_plan_input_with_manifest(&input, &seed_source_manifest()),
            Err(ZonePlanError::UnknownScenarioAuthoritySource {
                scenario_id: "seed-current-law-baseline".to_string(),
                source_id: "missing-source".to_string(),
            })
        );
    }

    #[test]
    fn unit_source_refs_must_exist_in_manifest() {
        let mut input = seed_plan_input();
        input.units[0].source_refs = Some(BoundaryUnitSourceRefs {
            boundary_source_id: Some("missing-source".to_string()),
            representative_point_source_id: None,
            population_source_id: None,
            time_zone_assignment_source_id: None,
            time_zone_geometry_source_id: None,
            caveats: vec![],
        });

        assert_eq!(
            evaluate_zone_plan_input_with_manifest(&input, &seed_source_manifest()),
            Err(ZonePlanError::UnknownUnitSourceReference {
                unit_id: "west-a".to_string(),
                field: "boundary_source_id",
                source_id: "missing-source".to_string(),
            })
        );
    }

    #[test]
    fn counterfactual_scenario_does_not_require_authority_source() {
        let mut input = seed_plan_input();
        input.scenario.kind = ZoneScenarioKind::AnalyticCounterfactual;
        input.scenario.scenario_id = "seed-counterfactual".to_string();
        input.scenario.authority_source_id = None;

        let report = evaluate_zone_plan_input(&input).unwrap();

        assert_eq!(report.unit_count, 4);
    }

    #[test]
    fn offset_fit_compares_current_dst_and_candidate_offsets() {
        let report = evaluate_offset_fit(&seed_plan_input(), 60).unwrap();

        assert_eq!(report.input_id, "zones-seed-plan-input");
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.total_population, 340);
        assert!(
            (report.current_weighted_mean_standard_error_minutes - 6.617647058823529).abs() < 1e-9
        );
        assert!((report.current_weighted_mean_dst_error_minutes - 53.38235294117647).abs() < 1e-9);
        assert_eq!(report.units_improved_by_whole_hour_count, 0);
        assert_eq!(report.units_improved_by_half_hour_count, 0);
        assert_eq!(report.units_improved_by_quarter_hour_count, 2);
        assert_eq!(report.unit_scores[1].unit_id, "west-b");
        assert_eq!(report.unit_scores[1].current_standard_error_minutes, 15.0);
        assert_eq!(report.unit_scores[1].current_dst_error_minutes, 45.0);
        assert_eq!(report.unit_scores[1].best_whole_hour_offset_minutes, -360);
        assert_eq!(report.unit_scores[1].best_half_hour_offset_minutes, -330);
        assert_eq!(report.unit_scores[1].best_quarter_hour_offset_minutes, -345);
        assert_eq!(report.unit_scores[1].best_quarter_hour_error_minutes, 0.0);
    }

    #[test]
    fn offset_fit_can_score_no_dst_scenarios() {
        let report = evaluate_offset_fit(&seed_plan_input(), 0).unwrap();

        assert!(
            (report.current_weighted_mean_dst_error_minutes
                - report.current_weighted_mean_standard_error_minutes)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn offset_fit_svg_renders_each_option_view() {
        let report = evaluate_offset_fit(&seed_plan_input(), 60).unwrap();

        for view in [
            OffsetMapView::CurrentStandard,
            OffsetMapView::CurrentDst,
            OffsetMapView::BestWholeHour,
            OffsetMapView::BestHalfHour,
            OffsetMapView::BestQuarterHour,
        ] {
            let svg = render_offset_fit_svg(&report, view, &OffsetMapRenderOptions::default());

            assert!(svg.contains("<svg"));
            assert!(svg.contains(view.title()));
            assert!(svg.contains("west-b"));
            assert!(svg.contains("UTC-05:45") || view != OffsetMapView::BestQuarterHour);
        }
    }

    #[test]
    fn offset_fit_svg_uses_geometry_when_available() {
        let report = evaluate_offset_fit(&seed_plan_input_with_map_points(), 60).unwrap();
        let svg = render_offset_fit_svg(
            &report,
            OffsetMapView::CurrentStandard,
            &OffsetMapRenderOptions::default(),
        );

        assert!(svg.contains("using plan geometry or map_point coordinates"));
        assert!(svg.contains("longitude"));
        assert!(svg.contains("west-a"));
        assert!(svg.contains("<path"));
        assert!(svg.contains("fill-rule=\"evenodd\""));
    }

    #[test]
    fn offset_fit_geojson_renders_point_features() {
        let report = evaluate_offset_fit(&seed_plan_input(), 60).unwrap();
        let geojson = render_offset_fit_geojson(&report);

        assert!(geojson.contains("\"type\":\"FeatureCollection\""));
        assert!(geojson.contains("\"id\":\"west-b\""));
        assert!(geojson.contains("\"coordinates\":[-86.250000"));
        assert!(geojson.contains("\"best_quarter_hour_offset_minutes\":-345"));
        assert!(geojson.contains("schematic point"));
    }

    #[test]
    fn offset_fit_geojson_uses_geometry_when_available() {
        let report = evaluate_offset_fit(&seed_plan_input_with_map_points(), 60).unwrap();
        let geojson = render_offset_fit_geojson(&report);

        assert!(geojson.contains("\"type\":\"Polygon\""));
        assert!(geojson.contains("geometry from plan input"));
        assert!(geojson.contains("[-88.100000,41.400000]"));
    }

    #[test]
    fn offset_candidate_plan_builds_whole_hour_grid() {
        let candidate =
            build_offset_candidate_plan(&seed_plan_input(), OffsetCandidateGrid::WholeHour)
                .unwrap();

        assert_eq!(
            candidate.scenario.kind,
            ZoneScenarioKind::AnalyticCounterfactual
        );
        assert_eq!(candidate.scenario.authority_source_id, None);
        assert_eq!(candidate.plan.assignment, vec![0, 0, 1, 1]);
        assert_eq!(
            candidate
                .plan
                .zones
                .iter()
                .map(|zone| zone.utc_offset_minutes)
                .collect::<Vec<_>>(),
            vec![-360, -300]
        );
        assert!(candidate.reference_assignment.is_empty());
        assert!(evaluate_zone_plan_input(&candidate).is_ok());
    }

    #[test]
    fn offset_candidate_plan_builds_quarter_hour_grid() {
        let candidate =
            build_offset_candidate_plan(&seed_plan_input(), OffsetCandidateGrid::QuarterHour)
                .unwrap();

        assert_eq!(candidate.plan.assignment, vec![0, 1, 2, 3]);
        assert_eq!(
            candidate
                .plan
                .zones
                .iter()
                .map(|zone| (zone.id.as_str(), zone.utc_offset_minutes))
                .collect::<Vec<_>>(),
            vec![
                ("utc-minus-06-00", -360),
                ("utc-minus-05-45", -345),
                ("utc-minus-05-00", -300),
                ("utc-minus-04-45", -285),
            ]
        );
        assert!(candidate
            .caveats
            .iter()
            .any(|caveat| caveat.contains("nearest quarter-hour UTC offsets")));
    }

    #[test]
    fn offset_candidate_comparison_reports_tradeoffs() {
        let report = compare_offset_candidate_plans(
            &seed_us_county_baseline_seed_plan_input(),
            &[
                OffsetCandidateGrid::WholeHour,
                OffsetCandidateGrid::HalfHour,
                OffsetCandidateGrid::QuarterHour,
            ],
        )
        .unwrap();

        assert_eq!(report.candidates.len(), 3);
        assert!(report.recommendation_gate_closed);
        assert_eq!(
            report.candidates[0].kind,
            ZoneScenarioKind::AnalyticCounterfactual
        );
        assert_eq!(report.candidates[0].moved_population, 0);
        assert!(report.candidates[1].moved_population > 0);
        assert!(report.candidates[2].weighted_error_delta_minutes < 0.0);
    }

    #[test]
    fn attach_geojson_geometries_matches_plan_units() {
        let input = seed_plan_input();
        let geojson = include_str!("../../../data/boundaries/seed-boundaries.geojson");
        let report =
            attach_geojson_geometries(&input, geojson, &GeometryJoinOptions::default()).unwrap();

        assert_eq!(report.matched_unit_count, 4);
        assert!(report.unmatched_unit_ids.is_empty());
        assert!(report.unused_feature_unit_ids.is_empty());
        assert_eq!(report.unit_statuses.len(), 4);
        assert_eq!(
            report.unit_statuses[0],
            GeometryJoinUnitStatus {
                unit_id: "west-a".to_string(),
                matched: true,
                geometry_type: Some("Polygon".to_string()),
            }
        );
        assert!(matches!(
            report.input.units[0].map_geometry,
            Some(MapGeometry::Polygon(_))
        ));
    }

    #[test]
    fn attach_geojson_geometries_reports_missing_required_units() {
        let input = seed_plan_input();
        let geojson = r#"{
            "type": "FeatureCollection",
            "features": [{
                "type": "Feature",
                "properties": { "unit_id": "west-a" },
                "geometry": { "type": "Point", "coordinates": [-87.6, 41.8] }
            }]
        }"#;
        let err = attach_geojson_geometries(
            &input,
            geojson,
            &GeometryJoinOptions {
                unit_id_property: "unit_id".to_string(),
                require_all_units: true,
            },
        )
        .unwrap_err();

        assert_eq!(
            err,
            GeometryJoinError::MissingRequiredUnits {
                unit_ids: vec![
                    "west-b".to_string(),
                    "east-a".to_string(),
                    "east-b".to_string()
                ],
            }
        );
    }

    #[test]
    fn plan_input_rejects_invalid_map_point() {
        let mut input = seed_plan_input();
        input.units[0].map_point = Some(MapPoint {
            latitude: 91.0,
            longitude: -90.0,
            source_id: None,
        });

        assert_eq!(
            evaluate_zone_plan_input(&input),
            Err(ZonePlanError::InvalidMapLatitude {
                unit_id: "west-a".to_string(),
                latitude: "91".to_string(),
            })
        );
    }

    #[test]
    fn committed_plan_input_matches_seed_input() {
        let input: ZonePlanInput =
            serde_json::from_str(include_str!("../../../data/plan-inputs/seed-plan.json")).unwrap();

        assert_eq!(input, seed_plan_input());
        assert!(evaluate_zone_plan_input_with_manifest(&input, &seed_source_manifest()).is_ok());
    }

    #[test]
    fn committed_map_point_plan_input_matches_seed_input() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/seed-plan-map-points.json"
        ))
        .unwrap();

        assert_eq!(input, seed_plan_input_with_map_points());
        assert!(evaluate_zone_plan_input_with_manifest(&input, &seed_source_manifest()).is_ok());
    }

    #[test]
    fn disconnected_zone_is_reported() {
        let (units, adjacency, mut plan) = seed_fixture();
        plan.assignment = vec![0, 1, 1, 0];

        let report = evaluate_zone_plan(&units, &adjacency, &plan).unwrap();

        assert!(!report.all_zones_connected);
    }

    #[test]
    fn missing_zone_assignment_is_rejected() {
        let (units, adjacency, mut plan) = seed_fixture();
        plan.assignment[0] = 7;

        assert_eq!(
            evaluate_zone_plan(&units, &adjacency, &plan),
            Err(ZonePlanError::UnknownZone {
                unit_index: 0,
                zone_index: 7
            })
        );
    }

    #[test]
    fn half_hour_offsets_are_supported() {
        let units = vec![BoundaryUnit {
            id: "half-hour-a".to_string(),
            name: "Half Hour Pilot".to_string(),
            solar_offset_minutes: 330.0,
            population: 100,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        }];
        let adjacency = vec![vec![]];
        let plan = ZonePlan {
            name: "half-hour-zone-plan".to_string(),
            zones: vec![ZoneSpec {
                id: "utc-plus-05-30".to_string(),
                utc_offset_minutes: 330,
            }],
            assignment: vec![0],
        };

        let report = evaluate_zone_plan(&units, &adjacency, &plan).unwrap();

        assert_eq!(report.weighted_mean_absolute_error_minutes, 0.0);
        assert_eq!(report.max_absolute_error_minutes, 0.0);
    }

    #[test]
    fn quarter_hour_offsets_are_supported() {
        let units = vec![BoundaryUnit {
            id: "quarter-hour-a".to_string(),
            name: "Quarter Hour Pilot".to_string(),
            solar_offset_minutes: 345.0,
            population: 100,
            map_point: None,
            map_geometry: None,
            source_refs: None,
        }];
        let adjacency = vec![vec![]];
        let plan = ZonePlan {
            name: "quarter-hour-zone-plan".to_string(),
            zones: vec![ZoneSpec {
                id: "utc-plus-05-45".to_string(),
                utc_offset_minutes: 345,
            }],
            assignment: vec![0],
        };

        let report = evaluate_zone_plan(&units, &adjacency, &plan).unwrap();

        assert_eq!(report.weighted_mean_absolute_error_minutes, 0.0);
        assert_eq!(report.max_absolute_error_minutes, 0.0);
    }

    #[test]
    fn implausible_zone_offsets_are_rejected() {
        let (units, adjacency, mut plan) = seed_fixture();
        plan.zones[0].utc_offset_minutes = 15 * 60;

        assert_eq!(
            evaluate_zone_plan(&units, &adjacency, &plan),
            Err(ZonePlanError::InvalidZoneUtcOffset {
                zone_id: "utc-minus-06-00".to_string(),
                utc_offset_minutes: 900,
            })
        );
    }

    #[test]
    fn out_of_bounds_adjacency_is_rejected() {
        let (units, mut adjacency, plan) = seed_fixture();
        adjacency[0].push(99);

        assert_eq!(
            evaluate_zone_plan(&units, &adjacency, &plan),
            Err(ZonePlanError::EdgeOutOfBounds {
                from: 0,
                to: 99,
                unit_count: 4,
            })
        );
    }

    #[test]
    fn rplan_context_supplies_legal_boundary_graph() {
        let context = seed_rplan_context();
        let plan = ZonePlan {
            name: "rplan-county-zone-plan".to_string(),
            zones: vec![
                ZoneSpec {
                    id: "utc-minus-06-00".to_string(),
                    utc_offset_minutes: -360,
                },
                ZoneSpec {
                    id: "utc-minus-05-00".to_string(),
                    utc_offset_minutes: -300,
                },
            ],
            assignment: vec![0, 0, 1, 1],
        };
        let solar_offsets = vec![-360.0, -345.0, -300.0, -285.0];

        let report = evaluate_rplan_zone_context(&context, &solar_offsets, &plan).unwrap();

        assert_eq!(report.boundary_edges, 2);
        assert!(report.all_zones_connected);
        assert_eq!(report.unit_count, 4);
    }

    #[test]
    fn committed_county_smoke_rplan_context_matches_seed_context() {
        let context: RplanContext = serde_json::from_str(include_str!(
            "../../../data/rplan-contexts/us-county-smoke-rplan-context.json"
        ))
        .unwrap();

        assert_eq!(context, seed_us_county_smoke_rplan_context());
        let report = rplan_context_intake_report(&context).unwrap();
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.graph_edge_count, 3);
        assert!(report.context_hash_matches);
        assert!(report.rplan_context_ready);
    }

    #[test]
    fn committed_county_seed_rplan_context_matches_seed_context() {
        let context: RplanContext = serde_json::from_str(include_str!(
            "../../../data/rplan-contexts/us-county-seed-rplan-context.json"
        ))
        .unwrap();

        assert_eq!(context, seed_us_county_seed_rplan_context());
        let report = rplan_context_intake_report(&context).unwrap();
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.graph_edge_count, 0);
        assert_eq!(report.population_count, 4);
        assert_eq!(report.source_hash_count, 2);
        assert!(report.context_hash_matches);
        assert!(report.rplan_context_ready);
    }

    #[test]
    fn committed_county_smoke_assignments_match_seed_assignments() {
        let assignments: CountyTimeZoneAssignmentSet = serde_json::from_str(include_str!(
            "../../../data/legal-assignments/us-county-smoke-current-law.json"
        ))
        .unwrap();

        assert_eq!(assignments, seed_us_county_smoke_time_zone_assignments());
        let report = assignments.report(&seed_source_manifest()).unwrap();
        assert_eq!(report.assignment_count, 4);
        assert_eq!(report.legal_source_ref_count, 4);
        assert_eq!(report.geometry_source_ref_count, 4);
        assert_eq!(report.placeholder_count, 4);
        assert!(!report.assignment_evidence_ready);
    }

    #[test]
    fn committed_county_seed_assignments_match_seed_assignments() {
        let assignments: CountyTimeZoneAssignmentSet = serde_json::from_str(include_str!(
            "../../../data/legal-assignments/us-county-seed-current-law.json"
        ))
        .unwrap();

        assert_eq!(assignments, seed_us_county_seed_time_zone_assignments());
        let report = assignments.report(&seed_source_manifest()).unwrap();
        assert_eq!(report.assignment_count, 4);
        assert_eq!(report.legal_source_ref_count, 4);
        assert_eq!(report.geometry_source_ref_count, 4);
        assert_eq!(report.placeholder_count, 0);
        assert_eq!(report.reconciled_count, 4);
        assert!(report.assignment_evidence_ready);
    }

    #[test]
    fn committed_county_seed_geometry_reconciliation_matches_seed_reconciliation() {
        let reconciliation: CountyGeometryReconciliationSet = serde_json::from_str(include_str!(
            "../../../data/geometry-reconciliation/us-county-seed-dot-reconciliation.json"
        ))
        .unwrap();

        assert_eq!(
            reconciliation,
            seed_us_county_seed_geometry_reconciliation()
        );
        let report = reconciliation.report(&seed_source_manifest()).unwrap();
        assert_eq!(report.row_count, 4);
        assert_eq!(report.pending_count, 0);
        assert_eq!(report.representative_point_matched_count, 0);
        assert_eq!(report.reconciled_count, 4);
        assert_eq!(report.caveated_row_count, 1);
        assert!(report.geometry_reconciliation_ready);
    }

    #[test]
    fn committed_county_smoke_representative_points_match_seed_points() {
        let points: CountyRepresentativePointSet = serde_json::from_str(include_str!(
            "../../../data/representative-points/us-county-smoke-gazetteer.json"
        ))
        .unwrap();

        assert_eq!(points, seed_us_county_smoke_representative_points());
        let report = points.report(&seed_source_manifest()).unwrap();
        assert_eq!(report.record_count, 4);
        assert_eq!(report.internal_point_count, 4);
        assert_eq!(report.caveated_record_count, 4);
        assert_eq!(report.max_solar_offset_delta_minutes, 0.0);
        assert!(report.exploratory_point_method);
        assert!(!report.strong_claim_point_method_ready);
    }

    #[test]
    fn committed_county_seed_representative_points_match_seed_points() {
        let points: CountyRepresentativePointSet = serde_json::from_str(include_str!(
            "../../../data/representative-points/us-county-seed-gazetteer.json"
        ))
        .unwrap();

        assert_eq!(points, seed_us_county_seed_representative_points());
        let report = points.report(&seed_source_manifest()).unwrap();
        assert_eq!(report.record_count, 4);
        assert_eq!(report.internal_point_count, 4);
        assert_eq!(report.caveated_record_count, 4);
        assert_eq!(report.max_solar_offset_delta_minutes, 0.0);
        assert!(report.exploratory_point_method);
        assert!(!report.strong_claim_point_method_ready);
    }

    #[test]
    fn temporal_global_model_validates_non_us_regime_over_time() {
        let unit = TemporalBoundaryUnit {
            unit_id: "CA-BC-001".to_string(),
            jurisdiction_id: "CA-BC".to_string(),
            unit_level: UnitLevel::Province,
            name: "British Columbia pilot unit".to_string(),
            geometry_ref: Some("sha256:geometry-placeholder".to_string()),
            representative_point: RepresentativePoint {
                latitude: 49.25,
                longitude: -123.1,
                method: RepresentativePointMethod::InternalPoint,
                source_id: "pilot-source".to_string(),
            },
            temporal_extent: TemporalExtent {
                valid_from: Some("2020-01-01".to_string()),
                valid_to: None,
            },
        };
        let graph = BoundaryGraphVersion {
            graph_id: "ca-bc-pilot-2020".to_string(),
            unit_universe_id: "ca-bc-pilot-units".to_string(),
            source_id: "pilot-source".to_string(),
            temporal_extent: TemporalExtent {
                valid_from: Some("2020-01-01".to_string()),
                valid_to: None,
            },
            adjacency: vec![vec![]],
        };
        let regime = TimeZoneRegime {
            regime_id: "ca-bc-current-law-pilot".to_string(),
            authority: RegimeAuthority::CurrentLaw,
            jurisdiction_scope: "CA-BC".to_string(),
            source_id: "pilot-time-source".to_string(),
            temporal_extent: TemporalExtent {
                valid_from: Some("2020-01-01".to_string()),
                valid_to: None,
            },
            assignments: vec![TimeZoneAssignment {
                unit_id: "CA-BC-001".to_string(),
                zone_id: "america-vancouver".to_string(),
                temporal_extent: TemporalExtent::current(),
            }],
            offset_rules: vec![OffsetRule {
                rule_id: "america-vancouver-standard-dst".to_string(),
                zone_id: "america-vancouver".to_string(),
                standard_offset_minutes: -480,
                temporal_extent: TemporalExtent::current(),
                dst_delta_minutes: Some(60),
                transition_rule_ref: Some("iana:America/Vancouver".to_string()),
                observance_notes: vec!["pilot rule; verify against source before use".to_string()],
            }],
        };

        assert!(unit.validate().is_ok());
        assert!(graph.validate(1).is_ok());
        assert!(regime.validate(&[unit]).is_ok());
        assert_eq!(regime.offset_rules[0].effective_offset_minutes(false), -480);
        assert_eq!(regime.offset_rules[0].effective_offset_minutes(true), -420);
    }

    #[test]
    fn temporal_extent_rejects_reversed_dates() {
        let extent = TemporalExtent {
            valid_from: Some("2026-01-01".to_string()),
            valid_to: Some("2025-01-01".to_string()),
        };

        assert_eq!(
            extent.validate(),
            Err(TemporalModelError::InvalidTemporalExtent {
                valid_from: "2026-01-01".to_string(),
                valid_to: "2025-01-01".to_string(),
            })
        );
    }

    #[test]
    fn regime_rejects_unknown_zone_assignment() {
        let unit = TemporalBoundaryUnit {
            unit_id: "FR-001".to_string(),
            jurisdiction_id: "FR".to_string(),
            unit_level: UnitLevel::Imported,
            name: "France pilot unit".to_string(),
            geometry_ref: None,
            representative_point: RepresentativePoint {
                latitude: 46.0,
                longitude: 2.0,
                method: RepresentativePointMethod::SourceProvided,
                source_id: "pilot-source".to_string(),
            },
            temporal_extent: TemporalExtent::current(),
        };
        let regime = TimeZoneRegime {
            regime_id: "fr-bad".to_string(),
            authority: RegimeAuthority::AnalyticCounterfactual,
            jurisdiction_scope: "FR".to_string(),
            source_id: "pilot-source".to_string(),
            temporal_extent: TemporalExtent::current(),
            assignments: vec![TimeZoneAssignment {
                unit_id: "FR-001".to_string(),
                zone_id: "missing-zone".to_string(),
                temporal_extent: TemporalExtent::current(),
            }],
            offset_rules: vec![],
        };

        assert_eq!(
            regime.validate(&[unit]),
            Err(TemporalModelError::UnknownAssignmentZone {
                zone_id: "missing-zone".to_string(),
            })
        );
    }

    #[test]
    fn seed_temporal_dataset_reports_non_us_offsets_and_dst() {
        let report = seed_temporal_dataset().report().unwrap();

        assert_eq!(report.dataset_id, "zones-temporal-non-us-pilot");
        assert_eq!(report.unit_count, 2);
        assert_eq!(report.current_law_regime_count, 2);
        assert_eq!(report.offset_rule_count, 2);
        assert_eq!(report.dst_rule_count, 1);
        assert_eq!(report.non_whole_hour_rule_count, 2);
    }

    #[test]
    fn committed_temporal_dataset_matches_seed_dataset() {
        let dataset: TemporalDataset = serde_json::from_str(include_str!(
            "../../../data/temporal-fixtures/non-us-pilot.json"
        ))
        .unwrap();

        assert_eq!(dataset, seed_temporal_dataset());
        assert!(dataset.report().is_ok());
    }

    #[test]
    fn temporal_dataset_rejects_unknown_evaluation_regime() {
        let mut dataset = seed_temporal_dataset();
        dataset.evaluation_contexts[0].regime_id = "missing-regime".to_string();

        assert_eq!(
            dataset.validate(),
            Err(TemporalModelError::UnknownEvaluationRegime {
                regime_id: "missing-regime".to_string(),
            })
        );
    }

    #[test]
    fn source_manifest_validates_referenced_sources() {
        let manifest = SourceManifest {
            manifest_id: "zones-us-baseline-sources".to_string(),
            generated_on: "2026-05-26".to_string(),
            sources: vec![
                SourceCitation {
                    source_id: "census-tiger-counties-2024".to_string(),
                    title: "2024 TIGER/Line Counties".to_string(),
                    kind: SourceKind::GeospatialBoundary,
                    url: "https://www.census.gov/cgi-bin/geo/shapefiles/index.php?layergroup=Counties&year=2024".to_string(),
                    retrieved_on: "2026-05-26".to_string(),
                    vintage: Some("2024".to_string()),
                    content_hash: None,
                    caveats: vec!["not a legal time-zone source".to_string()],
                },
                SourceCitation {
                    source_id: "dot-49-cfr-71".to_string(),
                    title: "49 CFR Part 71".to_string(),
                    kind: SourceKind::LegalText,
                    url: "https://www.ecfr.gov/current/title-49/subtitle-A/part-71".to_string(),
                    retrieved_on: "2026-05-26".to_string(),
                    vintage: None,
                    content_hash: None,
                    caveats: vec![],
                },
            ],
        };

        assert!(validate_source_references(
            &manifest,
            &[
                ("boundary_graph", "census-tiger-counties-2024"),
                ("time_zone_regime", "dot-49-cfr-71")
            ],
        )
        .is_ok());
    }

    #[test]
    fn seed_source_manifest_reports_source_mix() {
        let report = seed_source_manifest().report().unwrap();

        assert_eq!(report.manifest_id, "zones-us-foundation-sources");
        assert_eq!(report.source_count, 7);
        assert_eq!(report.caveated_source_count, 7);
        assert_eq!(report.legal_text_count, 2);
        assert_eq!(report.geospatial_boundary_count, 2);
        assert_eq!(report.time_rule_database_count, 1);
        assert_eq!(report.population_count, 1);
        assert_eq!(report.representative_point_count, 1);
    }

    #[test]
    fn seed_source_gate_policy_reports_ready_gate() {
        let report = seed_source_gate_policy()
            .report(&seed_source_manifest())
            .unwrap();

        assert_eq!(report.policy_id, "zones-us-foundation-source-gate-v0");
        assert_eq!(report.source_count, 7);
        assert_eq!(report.policy_entry_count, 7);
        assert_eq!(report.covered_source_count, 7);
        assert_eq!(report.missing_source_ids, Vec::<String>::new());
        assert_eq!(report.extra_entry_source_ids, Vec::<String>::new());
        assert_eq!(report.fletch_candidate_count, 4);
        assert_eq!(report.manual_reference_count, 3);
        assert_eq!(report.ignored_local_cache_count, 4);
        assert_eq!(report.reference_only_count, 3);
        assert_eq!(report.hash_required_count, 4);
        assert_eq!(report.gate_note_count, 8);
        assert!(report.source_gate_ready);
    }

    #[test]
    fn seed_source_limitation_matrix_reports_claim_support() {
        let report = seed_source_limitation_matrix().report().unwrap();

        assert_eq!(report.matrix_id, "zones-source-limitation-matrix-v0");
        assert_eq!(report.entry_count, 5);
        assert_eq!(report.assessment_count, 14);
        assert_eq!(report.supports_count, 6);
        assert_eq!(report.partial_count, 4);
        assert_eq!(report.not_supported_count, 4);
        assert_eq!(report.caveated_entry_count, 5);
    }

    #[test]
    fn committed_source_limitation_matrix_matches_seed_matrix() {
        let matrix: SourceLimitationMatrix = serde_json::from_str(include_str!(
            "../../../data/source-limitation-matrix/global-source-claims.json"
        ))
        .unwrap();

        assert_eq!(matrix, seed_source_limitation_matrix());
        assert!(matrix.report().is_ok());
    }

    #[test]
    fn source_limitation_matrix_rejects_duplicate_sources() {
        let mut matrix = seed_source_limitation_matrix();
        matrix.entries[1].source_id = "iana-tzdb".to_string();

        assert_eq!(
            matrix.validate(),
            Err(TemporalModelError::DuplicateSourceLimitationSourceId {
                source_id: "iana-tzdb".to_string(),
            })
        );
    }

    #[test]
    fn seed_module_boundary_contract_reports_ownership() {
        let report = seed_module_boundary_contract().report().unwrap();

        assert_eq!(report.contract_id, "zones-module-boundaries-v0");
        assert_eq!(report.module_count, 4);
        assert_eq!(report.ownership_statement_count, 8);
        assert_eq!(report.exclusion_statement_count, 8);
        assert_eq!(report.dependency_edge_count, 5);
        assert_eq!(report.downstream_edge_count, 4);
        assert_eq!(report.caveat_count, 1);
    }

    #[test]
    fn committed_module_boundary_contract_matches_seed_contract() {
        let contract: ModuleBoundaryContract = serde_json::from_str(include_str!(
            "../../../data/module-boundaries/zones-rplan-rline.json"
        ))
        .unwrap();

        assert_eq!(contract, seed_module_boundary_contract());
        assert!(contract.report().is_ok());
    }

    #[test]
    fn module_boundary_contract_rejects_empty_ownership() {
        let mut contract = seed_module_boundary_contract();
        contract.entries[0].owns.clear();

        assert_eq!(
            contract.validate(),
            Err(TemporalModelError::EmptyModuleOwnership {
                module_id: "rline".to_string(),
            })
        );
    }

    #[test]
    fn seed_zone_catalog_reports_non_whole_hour_offsets() {
        let report = seed_zone_catalog().report().unwrap();

        assert_eq!(report.catalog_id, "zones-seed-offset-catalog");
        assert_eq!(report.zone_count, 5);
        assert_eq!(report.whole_hour_offset_count, 3);
        assert_eq!(report.non_whole_hour_offset_count, 2);
        assert_eq!(report.half_hour_offset_count, 1);
        assert_eq!(report.quarter_hour_offset_count, 1);
        assert_eq!(report.min_utc_offset_minutes, -480);
        assert_eq!(report.max_utc_offset_minutes, 345);
    }

    #[test]
    fn committed_zone_catalog_matches_seed_catalog() {
        let catalog: ZoneCatalog = serde_json::from_str(include_str!(
            "../../../data/zone-catalogs/seed-offsets.json"
        ))
        .unwrap();

        assert_eq!(catalog, seed_zone_catalog());
        assert!(catalog.report().is_ok());
    }

    #[test]
    fn committed_source_manifest_matches_seed_manifest() {
        let manifest: SourceManifest = serde_json::from_str(include_str!(
            "../../../data/source-manifests/us-foundation.json"
        ))
        .unwrap();

        assert_eq!(manifest, seed_source_manifest());
        assert!(manifest.report().is_ok());
    }

    #[test]
    fn committed_source_gate_policy_matches_seed_policy() {
        let policy: SourceGatePolicy = serde_json::from_str(include_str!(
            "../../../data/source-gates/us-foundation-source-gate.json"
        ))
        .unwrap();

        assert_eq!(policy, seed_source_gate_policy());
        assert!(
            policy
                .report(&seed_source_manifest())
                .unwrap()
                .source_gate_ready
        );
    }

    #[test]
    fn committed_us_county_smoke_plan_scores_with_manifest_and_catalog() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/us-county-smoke.json"
        ))
        .unwrap();

        let report = evaluate_zone_plan_input_with_manifest_and_catalog(
            &input,
            &seed_source_manifest(),
            &seed_zone_catalog(),
        )
        .unwrap();

        assert_eq!(input.input_id, "zones-us-county-smoke-plan-input");
        let source_refs = input.units[0].source_refs.as_ref().unwrap();
        assert_eq!(
            source_refs.time_zone_assignment_source_id.as_deref(),
            Some("dot-49-cfr-71")
        );
        assert_eq!(
            source_refs.time_zone_geometry_source_id.as_deref(),
            Some("dot-time-zone-map-layer")
        );
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.zone_count, 2);
        assert!(report.all_zones_connected);
    }

    #[test]
    fn committed_county_baseline_smoke_plan_matches_seed_input() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/us-county-baseline-smoke.json"
        ))
        .unwrap();

        assert_eq!(input, seed_us_county_baseline_smoke_plan_input());
        let report = evaluate_zone_plan_input_with_manifest_and_catalog(
            &input,
            &seed_source_manifest(),
            &seed_zone_catalog(),
        )
        .unwrap();
        let source_ref_report =
            zone_plan_source_ref_report(&input, &seed_source_manifest()).unwrap();

        assert_eq!(report.unit_count, 4);
        assert_eq!(report.zone_count, 2);
        assert!(report.all_zones_connected);
        assert!(source_ref_report.publishable_source_ref_coverage);
    }

    #[test]
    fn committed_county_baseline_seed_plan_matches_seed_input() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/us-county-baseline-seed.json"
        ))
        .unwrap();

        assert_eq!(input, seed_us_county_baseline_seed_plan_input());
        let report = evaluate_zone_plan_input_with_manifest_and_catalog(
            &input,
            &seed_source_manifest(),
            &seed_zone_catalog(),
        )
        .unwrap();
        let source_ref_report =
            zone_plan_source_ref_report(&input, &seed_source_manifest()).unwrap();

        assert_eq!(report.unit_count, 4);
        assert_eq!(report.zone_count, 2);
        assert!(!report.all_zones_connected);
        assert!(report.weighted_mean_absolute_error_minutes > 0.0);
        assert!(source_ref_report.publishable_source_ref_coverage);
    }

    #[test]
    fn committed_county_seed_boundaries_match_seed_plan_units() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/us-county-baseline-seed.json"
        ))
        .unwrap();
        let geojson = include_str!("../../../data/boundaries/us-county-seed-boundaries.geojson");
        let report =
            attach_geojson_geometries(&input, geojson, &GeometryJoinOptions::default()).unwrap();
        let offset_fit = evaluate_offset_fit(&report.input, 60).unwrap();
        let svg = render_offset_fit_svg(
            &offset_fit,
            OffsetMapView::CurrentStandard,
            &OffsetMapRenderOptions::default(),
        );

        assert_eq!(report.matched_unit_count, 4);
        assert!(report.unmatched_unit_ids.is_empty());
        assert!(report.unused_feature_unit_ids.is_empty());
        assert!(report
            .input
            .units
            .iter()
            .all(|unit| matches!(unit.map_geometry, Some(MapGeometry::Polygon(_)))));
        assert!(svg.contains("<path"));
        assert!(svg.contains("fill-rule=\"evenodd\""));
    }

    #[test]
    fn county_smoke_source_ref_report_counts_intake_coverage() {
        let input: ZonePlanInput = serde_json::from_str(include_str!(
            "../../../data/plan-inputs/us-county-smoke.json"
        ))
        .unwrap();

        let report = zone_plan_source_ref_report(&input, &seed_source_manifest()).unwrap();

        assert_eq!(report.input_id, "zones-us-county-smoke-plan-input");
        assert_eq!(report.unit_count, 4);
        assert_eq!(report.units_with_source_refs, 4);
        assert_eq!(report.units_with_complete_source_refs, 4);
        assert_eq!(report.units_missing_source_refs, 0);
        assert_eq!(report.boundary_source_ref_count, 4);
        assert_eq!(report.missing_boundary_source_ref_count, 0);
        assert_eq!(report.representative_point_source_ref_count, 4);
        assert_eq!(report.missing_representative_point_source_ref_count, 0);
        assert_eq!(report.population_source_ref_count, 4);
        assert_eq!(report.missing_population_source_ref_count, 0);
        assert_eq!(report.time_zone_assignment_source_ref_count, 4);
        assert_eq!(report.missing_time_zone_assignment_source_ref_count, 0);
        assert_eq!(report.time_zone_geometry_source_ref_count, 4);
        assert_eq!(report.missing_time_zone_geometry_source_ref_count, 0);
        assert_eq!(report.units_with_source_caveats, 4);
        assert_eq!(report.units_missing_source_caveats, 0);
        assert_eq!(report.unit_source_caveat_count, 8);
        assert!(report.publishable_source_ref_coverage);
    }

    #[test]
    fn source_manifest_rejects_unknown_reference() {
        let manifest = SourceManifest {
            manifest_id: "zones-empty".to_string(),
            generated_on: "2026-05-26".to_string(),
            sources: vec![],
        };

        assert_eq!(
            validate_source_references(&manifest, &[("boundary_graph", "missing-source")]),
            Err(TemporalModelError::UnknownSourceReference {
                owner_kind: "boundary_graph",
                source_id: "missing-source".to_string(),
            })
        );
    }

    fn seed_rplan_context() -> RplanContext {
        let mut units = PlanUnitIndex {
            unit_kind: UnitKind::County,
            state: Some("ZZ".to_string()),
            year: Some(2020),
            canonical_order: CanonicalOrder::SortedGeoid,
            unit_ids: vec![
                "99001".to_string(),
                "99003".to_string(),
                "99005".to_string(),
                "99007".to_string(),
            ],
            unit_universe_hash: String::new(),
            source_id: Some("zones-seed-rplan-context".to_string()),
        };
        units.unit_universe_hash = units.compute_unit_universe_hash().unwrap();

        RplanContext {
            rctx_version: RCTX_VERSION.to_string(),
            context_hash: "seed".to_string(),
            units,
            graph: Some(UnitGraph {
                edge_semantics: EdgeSemantics::Undirected,
                adjacency: vec![
                    vec![
                        UnitEdge {
                            to: 1,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                        UnitEdge {
                            to: 2,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                    ],
                    vec![
                        UnitEdge {
                            to: 0,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                        UnitEdge {
                            to: 3,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                    ],
                    vec![
                        UnitEdge {
                            to: 0,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                        UnitEdge {
                            to: 3,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                    ],
                    vec![
                        UnitEdge {
                            to: 1,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                        UnitEdge {
                            to: 2,
                            kind: EdgeKind::Boundary,
                            weight: Some(1.0),
                        },
                    ],
                ],
            }),
            populations: Some(vec![100, 80, 90, 70]),
            subdivisions: None,
            demographics: None,
            geometry: None,
            source_hashes: SourceHashes::default(),
        }
    }
}
