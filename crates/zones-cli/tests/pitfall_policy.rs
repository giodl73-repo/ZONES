const README: &str = include_str!("../../../README.md");
const ROLES: &str = include_str!("../../../.roles/ROLE.md");
const RESEARCH: &str = include_str!("../../../research/RESEARCH.md");
const TRACKING: &str = include_str!("../../../research/TRACKING.md");
const FAIRNESS: &str = include_str!("../../../docs/research/fairness-principles.md");
const FEDERAL_AUTHORITY: &str = include_str!("../../../docs/research/federal-time-authority.md");
const PULSE_04: &str =
    include_str!("../../../context/waves/2026-05-25-foundation-contract/pulses/pulse-04.md");
const SEED_PLAN: &str = include_str!("../../../data/plan-inputs/us-county-baseline-seed.json");
const PITFALLS: &str = include_str!("../../../.pitfall/zones-pitfalls.md");
const AUTHORITY_BOUNDARIES: &str = include_str!("../../../docs/authority-boundaries.v1.json");

fn assert_contains(haystack: &str, needle: &str) {
    let normalized_haystack = haystack.replace("\r\n", "\n");
    let normalized_needle = needle.replace("\r\n", "\n");
    assert!(
        normalized_haystack.contains(&normalized_needle),
        "expected text to contain: {needle}"
    );
}

fn load_boundaries() -> serde_json::Value {
    let boundaries: serde_json::Value =
        serde_json::from_str(AUTHORITY_BOUNDARIES).expect("authority boundaries parse");
    assert_eq!(boundaries["schema"], "zones.authority-boundaries.v1");
    assert_eq!(boundaries["repo"], "ZONES");
    boundaries
}

fn pitfall<'a>(boundaries: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    let pitfall = boundaries["pitfalls"]
        .as_array()
        .expect("pitfalls is an array")
        .iter()
        .find(|pitfall| pitfall["id"] == id)
        .unwrap_or_else(|| panic!("{id} boundary is recorded"));
    assert_eq!(pitfall["status"], "mitigated");
    pitfall
}

fn assert_array_contains(pitfall: &serde_json::Value, field: &str, expected: &[&str]) {
    let actual = pitfall[field]
        .as_array()
        .unwrap_or_else(|| panic!("{field} must be an array"));
    for value in expected {
        assert!(
            actual.iter().any(|actual| actual == value),
            "{field} must contain {value}"
        );
    }
}

#[test]
fn candidate_score_does_not_become_time_zone_recommendation() {
    assert_contains(PITFALLS, "ZONES-PF-01");
    assert_contains(PITFALLS, "Candidate Score Becomes Time-Zone Recommendation");
    assert_contains(PITFALLS, "civil-time-policy reviewer");

    assert_contains(ROLES, "Recommendation gate");
    assert_contains(ROLES, "no candidate may be described as preferred, best");
    assert_contains(README, "recommendation_gate_closed");
    assert_contains(README, "not a versioned crate, schema, or dataset contract");
    assert_contains(PULSE_04, "recommendation");

    let boundaries = load_boundaries();
    let recommendation = pitfall(&boundaries, "ZONES-PF-01");
    assert_contains(
        recommendation["boundary"]
            .as_str()
            .expect("boundary is text"),
        "recommendation gate is closed",
    );
    assert_array_contains(
        recommendation,
        "not_authorized",
        &[
            "time_zone_recommendation",
            "preferred_plan_claim",
            "best_plan_claim",
            "ready_to_adopt_claim",
            "public_policy_advice",
            "legal_authority_claim",
            "operational_scheduling_advice",
        ],
    );
    assert_array_contains(
        recommendation,
        "required_before_upgrade",
        &[
            "civil_time_policy_review",
            "source_review",
            "optimization_review",
            "public_map_review",
            "legal_authority_review",
            "recommendation_gate_opened",
        ],
    );
}

#[test]
fn source_derived_seed_does_not_become_national_baseline() {
    assert_contains(PITFALLS, "ZONES-PF-02");
    assert_contains(PITFALLS, "Source-Derived Seed Becomes National Baseline");
    assert_contains(PITFALLS, "boundary-data steward");

    assert_contains(README, "four-county fixture");
    assert_contains(README, "remaining a smoke\nfixture");
    assert_contains(
        TRACKING,
        "legal-boundary assignment and population-weighted county scores",
    );
    assert_contains(SEED_PLAN, "us-county-baseline-seed");
    assert_contains(SEED_PLAN, "source_refs");

    let boundaries = load_boundaries();
    let baseline = pitfall(&boundaries, "ZONES-PF-02");
    assert_contains(
        baseline["boundary"].as_str().expect("boundary is text"),
        "not a national current-law baseline",
    );
    assert_array_contains(
        baseline,
        "not_authorized",
        &[
            "national_current_law_baseline",
            "legal_assignment_scorecard",
            "complete_county_context_coverage",
            "complete_source_ready_dataset",
            "population_weighted_national_scorecard",
            "publication_ready_map",
        ],
    );
    assert_array_contains(
        baseline,
        "required_before_upgrade",
        &[
            "national_county_context",
            "complete_legal_assignments",
            "population_weighted_scores",
            "source_gate_pass",
            "geometry_reconciliation_pass",
            "boundary_data_steward_review",
        ],
    );
}

#[test]
fn solar_error_remains_one_metric_not_whole_policy_objective() {
    assert_contains(PITFALLS, "ZONES-PF-03");
    assert_contains(PITFALLS, "Solar Error Becomes Whole Policy Objective");
    assert_contains(PITFALLS, "solar-time methodologist");

    assert_contains(
        README,
        "without pretending the metric is the whole policy decision",
    );
    assert_contains(RESEARCH, "convenience of commerce, not solar\n  accuracy");
    assert_contains(FAIRNESS, "Solar fit is a measurement, not a mandate");
    assert_contains(
        FAIRNESS,
        "Minimize disruption unless evidence supports change",
    );
    assert_contains(FEDERAL_AUTHORITY, "49 CFR part 71");
    assert_contains(
        TRACKING,
        "DOT-convenience proxy schema before candidate recommendations",
    );

    let boundaries = load_boundaries();
    let solar = pitfall(&boundaries, "ZONES-PF-03");
    assert_contains(
        solar["boundary"].as_str().expect("boundary is text"),
        "one measurement axis",
    );
    assert_array_contains(
        solar,
        "not_authorized",
        &[
            "solar_error_only_policy_objective",
            "fairness_claim_from_solar_fit_only",
            "better_plan_claim_without_tradeoffs",
            "dot_convenience_claim",
            "implementation_ready_claim",
            "public_preference_claim",
        ],
    );
    assert_array_contains(
        solar,
        "must_remain_visible",
        &[
            "convenience_of_commerce",
            "disruption",
            "public_preference",
            "legal_process",
            "implementation_cost",
            "source_completeness",
            "review_status",
        ],
    );
    assert_array_contains(
        solar,
        "required_before_upgrade",
        &[
            "dot_convenience_proxy_schema",
            "tradeoff_weights",
            "implementation_cost_evidence",
            "public_preference_evidence",
            "civil_time_policy_review",
            "public_map_review",
        ],
    );
}
