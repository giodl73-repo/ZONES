//! Recorded metadata checks, without score or legal/publication promotion.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use zones_core::{
    CountyGeometryReconciliationSet, CountyRepresentativePointSet, CountyTimeZoneAssignmentSet,
    SourceGatePolicy, SourceManifest, ZonePlanInput,
};
const MANIFEST: &str = include_str!("../../../data/source-manifests/us-foundation.json");
const POLICY: &str = include_str!("../../../data/source-gates/us-foundation-source-gate.json");
const PLAN: &str = include_str!("../../../data/plan-inputs/us-county-baseline-seed.json");
const ASSIGNMENTS: &str =
    include_str!("../../../data/legal-assignments/us-county-seed-current-law.json");
const GEOMETRY: &str =
    include_str!("../../../data/geometry-reconciliation/us-county-seed-dot-reconciliation.json");
const POINTS: &str =
    include_str!("../../../data/representative-points/us-county-seed-gazetteer.json");
const CONTEXT: &str =
    include_str!("../../../data/rplan-contexts/us-county-seed-rplan-context.json");
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Lens {
    Manifest,
    Policy,
    References,
    Assignments,
    Geometry,
    Points,
    Context,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    lens: Lens,
}
fn parse<T: serde::de::DeserializeOwned>(text: &str) -> Result<T, String> {
    serde_json::from_str(text).map_err(|e| e.to_string())
}
fn canonical<T: Serialize, E: std::fmt::Display>(report: Result<T, E>) -> Result<String, String> {
    serde_json::to_string_pretty(&report.map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > 65536 {
        return Err("Use at most 64 KiB of evidence-query JSON".into());
    }
    let query: Query = parse(input)?;
    let manifest: SourceManifest = parse(MANIFEST)?;
    let policy: SourceGatePolicy = parse(POLICY)?;
    let plan: ZonePlanInput = parse(PLAN)?;
    let assignments: CountyTimeZoneAssignmentSet = parse(ASSIGNMENTS)?;
    let geometry: CountyGeometryReconciliationSet = parse(GEOMETRY)?;
    let points: CountyRepresentativePointSet = parse(POINTS)?;
    let context = parse(CONTEXT)?;
    let reports: BTreeMap<&str, String> = BTreeMap::from([
        ("manifest", canonical(manifest.report())?),
        ("policy", canonical(policy.report(&manifest))?),
        (
            "references",
            canonical(zones_core::zone_plan_source_ref_report(&plan, &manifest))?,
        ),
        ("assignments", canonical(assignments.report(&manifest))?),
        ("geometry", canonical(geometry.report(&manifest))?),
        ("points", canonical(points.report(&manifest))?),
        (
            "context",
            canonical(zones_core::rplan_context_intake_report(&context))?,
        ),
    ]);
    let (key,label,meaning,inputs)=match query.lens {
 Lens::Manifest=>("manifest","Recorded source inventory","Seven source citations with caveats; listing is not acquisition or primary-byte verification.",vec!["manifest"]),
 Lens::Policy=>("policy","Source-policy coverage","source_gate_ready checks policy/manifest coverage and declared hash requirements, not downloaded or verified publisher bytes.",vec!["manifest","policy"]),
 Lens::References=>("references","County source-reference coverage","publishable_source_ref_coverage means complete reference fields and caveats, not publication approval or current legal validity.",vec!["manifest","seed-plan"]),
 Lens::Assignments=>("assignments","Recorded assignment-evidence checks","assignment_evidence_ready validates recorded clause references and evidence states, not present-day legal time or scheduling authority.",vec!["manifest","assignments"]),
 Lens::Geometry=>("geometry","Recorded geometry reconciliation","geometry_reconciliation_ready checks recorded reconciliation rows; no fresh GIS overlay or legal boundary determination is performed.",vec!["manifest","geometry"]),
 Lens::Points=>("points","Representative-point method","Four internal points are exploratory; zero selected-versus-internal delta is not population-center accuracy or zero uncertainty.",vec!["manifest","points"]),
 Lens::Context=>("context","County context consistency","A matching context hash and four units with zero edges are structural context, not verified primary bytes or a connected national baseline.",vec!["context"]),
 };
    let report: Value = parse(&reports[key])?;
    let p: Value = parse(&reports["points"])?;
    let c: Value = parse(&reports["context"])?;
    let policy_report: Value = parse(&reports["policy"])?;
    let refs: Value = parse(&reports["references"])?;
    let a: Value = parse(&reports["assignments"])?;
    let g: Value = parse(&reports["geometry"])?;
    serde_json::to_string(&json!({"schema":"zones.source-evidence-inspector.v1","lens":key,"label":label,"meaning":meaning,
 "source_period":format!("Recorded source manifest {}; frozen May 2026 metadata/derived fixtures, not a current-law query",manifest.generated_on),
 "report":report,"native_report_canonical":reports[key],"input_keys":inputs,
 "fixed_context":{"policy_coverage_ready":policy_report["source_gate_ready"],"reference_fields_complete":refs["publishable_source_ref_coverage"],"recorded_assignment_check_ready":a["assignment_evidence_ready"],"recorded_geometry_check_ready":g["geometry_reconciliation_ready"],"context_hash_matches":c["context_hash_matches"],"county_units":c["unit_count"],"graph_edges":c["graph_edge_count"],"internal_points":p["internal_point_count"],"population_centers":p["population_center_count"],"strong_claim_point_method_ready":p["strong_claim_point_method_ready"]},
 "custody":{"input_bytes":"canonical committed LF JSON; see source-custody.json","raw_primary_bytes_in_bundle":false,"primary_hashes_reverified":false,"current_law_revalidated":false,"fresh_geometry_overlay":false},
 "authority":{"legal_assignment":false,"scheduling":false,"enactment":false,"candidate_recommendation":false,"county_scorecard_publication":false,"national_baseline":false,"primary_source_verification":false},
 "unobserved":{"population_center_accuracy":null,"current_law_verified":null,"regional_program_outcomes":null},
 "holds":["No county scorecard or candidate map is published by this inspector.","The four source-derived county records are distinct from the four invented clock-demo units.","Pulse 04 publication/recommendation holds remain; structural true flags never form a publication verdict."]
 })).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_seven_lenses_replay_native_reports_with_separate_holds() {
        for lens in [
            "manifest",
            "policy",
            "references",
            "assignments",
            "geometry",
            "points",
            "context",
        ] {
            let d: Value =
                serde_json::from_str(&evaluate(&json!({"lens":lens}).to_string()).unwrap())
                    .unwrap();
            assert_eq!(
                serde_json::from_str::<Value>(d["native_report_canonical"].as_str().unwrap())
                    .unwrap(),
                d["report"]
            );
            assert_eq!(d["fixed_context"]["policy_coverage_ready"], true);
            assert_eq!(d["fixed_context"]["strong_claim_point_method_ready"], false);
            assert_eq!(d["fixed_context"]["graph_edges"], 0);
            assert_eq!(d["fixed_context"]["county_units"], 4);
            assert!(d["authority"]
                .as_object()
                .unwrap()
                .values()
                .all(|v| v == false));
            assert!(d["unobserved"]
                .as_object()
                .unwrap()
                .values()
                .all(Value::is_null));
        }
    }
    #[test]
    fn rejects_unknown_lenses_fields_and_oversized_inputs() {
        for s in [
            r#"{"lens":"unknown"}"#,
            r#"{"lens":"points","county":"01001"}"#,
            r#"{"lens":"policy","date":"2026-10-29"}"#,
            r#"{"lens":"context","candidate":true}"#,
            "{}",
        ] {
            assert!(evaluate(s).is_err());
        }
        assert!(evaluate(&"x".repeat(65537)).is_err());
    }
    #[test]
    fn internal_point_delta_does_not_establish_accuracy() {
        let d: Value = serde_json::from_str(&evaluate(r#"{"lens":"points"}"#).unwrap()).unwrap();
        assert_eq!(d["report"]["internal_point_count"], 4);
        assert_eq!(d["report"]["population_center_count"], 0);
        assert_eq!(d["report"]["strong_claim_point_method_ready"], false);
        assert_eq!(d["custody"]["primary_hashes_reverified"], false);
        assert!(d["unobserved"]["population_center_accuracy"].is_null());
    }
}
