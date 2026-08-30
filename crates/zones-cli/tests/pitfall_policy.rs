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

fn assert_contains(haystack: &str, needle: &str) {
    let normalized_haystack = haystack.replace("\r\n", "\n");
    let normalized_needle = needle.replace("\r\n", "\n");
    assert!(
        normalized_haystack.contains(&normalized_needle),
        "expected text to contain: {needle}"
    );
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
}
