//! Synthetic civil-time workbench; never a legal assignment or recommendation.
use serde::Serialize;
use zones_core::{
    evaluate_offset_fit, evaluate_zone_plan_input, render_offset_fit_svg,
    seed_plan_input_with_map_points, OffsetFitReport, OffsetMapRenderOptions, OffsetMapView,
    ZonePlanReport, ZoneScenarioKind,
};

#[derive(Serialize)]
pub struct Comparison {
    pub report: ZonePlanReport,
    pub baseline: ZonePlanReport,
    pub fit: OffsetFitReport,
    pub svg: String,
    pub scenario: &'static str,
}
pub fn compare(
    assignments: &[usize],
    west: i32,
    east: i32,
    daylight: i32,
) -> Result<Comparison, String> {
    if assignments.len() != 4 || assignments.iter().any(|&v| v > 1) {
        return Err("Assign each of the four sample units to zone A or B".into());
    }
    if !(-720..=840).contains(&west)
        || !(-720..=840).contains(&east)
        || west % 15 != 0
        || east % 15 != 0
        || ![0, 60].contains(&daylight)
    {
        return Err("Use quarter-hour UTC offsets and a 0 or 60 minute daylight shift".into());
    }
    let mut input = seed_plan_input_with_map_points();
    // Its coordinates and geography are synthetic, including the baseline.
    input.scenario.kind = ZoneScenarioKind::AnalyticCounterfactual;
    input.scenario.authority_source_id = None;
    input.scenario.label = "Synthetic clock experiment".into();
    input.plan.zones[0].id = "zone-a".into();
    input.plan.zones[1].id = "zone-b".into();
    let baseline = evaluate_zone_plan_input(&input).map_err(|e| e.to_string())?;
    input.plan.assignment = assignments.to_vec();
    input.plan.zones[0].utc_offset_minutes = west;
    input.plan.zones[1].utc_offset_minutes = east;
    input.plan.name = "synthetic-browser-experiment".into();
    let report = evaluate_zone_plan_input(&input).map_err(|e| e.to_string())?;
    let fit = evaluate_offset_fit(&input, daylight).map_err(|e| e.to_string())?;
    let view = if daylight == 0 {
        OffsetMapView::CurrentStandard
    } else {
        OffsetMapView::CurrentDst
    };
    let svg = render_offset_fit_svg(&fit, view, &OffsetMapRenderOptions::default());
    Ok(Comparison {
        report,
        baseline,
        fit,
        svg,
        scenario: "synthetic analytic counterfactual",
    })
}
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn compare_json(
    assignments: &str,
    west: i32,
    east: i32,
    daylight: i32,
) -> Result<String, wasm_bindgen::JsValue> {
    let result = serde_json::from_str::<Vec<usize>>(assignments)
        .map_err(|e| e.to_string())
        .and_then(|a| compare(&a, west, east, daylight))
        .and_then(|v| serde_json::to_string(&v).map_err(|e| e.to_string()));
    result.map_err(|e| wasm_bindgen::JsValue::from_str(&e))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_disconnected_assignment_and_changed_solar_fit() {
        let base = compare(&[0, 0, 1, 1], -360, -300, 0).unwrap();
        assert!(base.report.all_zones_connected);
        assert_eq!(base.report.moved_unit_count, Some(0));
        let disconnected = compare(&[0, 1, 1, 0], -360, -300, 0).unwrap();
        assert!(!disconnected.report.all_zones_connected);
        assert_eq!(disconnected.report.moved_unit_count, Some(2));
        let daylight = compare(&[0, 0, 1, 1], -360, -300, 60).unwrap();
        assert!(
            daylight.fit.current_weighted_mean_dst_error_minutes
                > base.fit.current_weighted_mean_dst_error_minutes
        );
        assert!(base.svg.contains("<svg"));
        let changed = compare(&[0, 0, 1, 1], 0, 60, 0).unwrap();
        assert_eq!(changed.fit.unit_scores[0].current_zone_id, "zone-a");
        assert_eq!(
            changed.fit.unit_scores[0].current_standard_offset_minutes,
            0
        );
    }
    #[test]
    fn rejects_unbounded_or_malformed_controls() {
        assert!(compare(&[0], -360, -300, 0).is_err());
        assert!(compare(&[0, 0, 1, 2], -360, -300, 0).is_err());
        assert!(compare(&[0, 0, 1, 1], -359, -300, 0).is_err());
        assert!(compare(&[0, 0, 1, 1], -360, -300, 30).is_err());
    }
}
