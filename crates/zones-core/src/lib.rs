use rgraph_core::{assignment_label_connected, undirected_edge_cut, EdgeCutError};
use rplan_core::RplanContext;
use serde::{Deserialize, Serialize};
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
    #[error("source {source_id} has empty URL")]
    EmptySourceUrl { source_id: String },
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
pub struct BoundaryUnit {
    pub id: String,
    pub name: String,
    pub solar_offset_minutes: f64,
    pub population: u64,
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

pub fn evaluate_zone_plan_input_with_manifest(
    input: &ZonePlanInput,
    manifest: &SourceManifest,
) -> Result<ZonePlanReport, ZonePlanError> {
    validate_input_manifest_pair(input, manifest)?;
    evaluate_zone_plan_input(input)
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
    Ok(())
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
        });
    }
    let adjacency = graph
        .adjacency
        .iter()
        .map(|edges| edges.iter().map(|edge| edge.to as usize).collect())
        .collect::<Vec<Vec<usize>>>();

    evaluate_zone_plan(&units, &adjacency, plan)
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
        },
        BoundaryUnit {
            id: "west-b".to_string(),
            name: "West B County".to_string(),
            solar_offset_minutes: -345.0,
            population: 80,
        },
        BoundaryUnit {
            id: "east-a".to_string(),
            name: "East A County".to_string(),
            solar_offset_minutes: -300.0,
            population: 90,
        },
        BoundaryUnit {
            id: "east-b".to_string(),
            name: "East B County".to_string(),
            solar_offset_minutes: -285.0,
            population: 70,
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
        assert_eq!(evaluation.source_caveats.len(), 5);
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
    fn counterfactual_scenario_does_not_require_authority_source() {
        let mut input = seed_plan_input();
        input.scenario.kind = ZoneScenarioKind::AnalyticCounterfactual;
        input.scenario.scenario_id = "seed-counterfactual".to_string();
        input.scenario.authority_source_id = None;

        let report = evaluate_zone_plan_input(&input).unwrap();

        assert_eq!(report.unit_count, 4);
    }

    #[test]
    fn committed_plan_input_matches_seed_input() {
        let input: ZonePlanInput =
            serde_json::from_str(include_str!("../../../data/plan-inputs/seed-plan.json")).unwrap();

        assert_eq!(input, seed_plan_input());
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
        assert_eq!(report.source_count, 5);
        assert_eq!(report.caveated_source_count, 5);
        assert_eq!(report.legal_text_count, 2);
        assert_eq!(report.geospatial_boundary_count, 1);
        assert_eq!(report.time_rule_database_count, 1);
        assert_eq!(report.representative_point_count, 1);
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
