fn main() {
    for (assignment, west, east, daylight) in [
        ([0, 0, 1, 1], -360, -300, 0),
        ([0, 1, 1, 0], -360, -300, 0),
        ([0, 0, 1, 1], -360, -300, 60),
        ([0, 0, 1, 1], 0, 60, 0),
    ] {
        let canonical =
            serde_json::to_string(&zones_web::compare(&assignment, west, east, daylight).unwrap())
                .unwrap();
        println!(
            "{}",
            serde_json::json!({"assignment":assignment,"west":west,"east":east,"daylight":daylight,"canonical":canonical})
        );
    }
}
