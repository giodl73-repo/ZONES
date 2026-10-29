fn main() {
    let result = std::env::args()
        .nth(1)
        .ok_or_else(|| "Provide an evidence query file".to_string())
        .and_then(|p| std::fs::read_to_string(p).map_err(|e| e.to_string()))
        .and_then(|s| zones_web::evidence::evaluate(&s));
    match result {
        Ok(s) => print!("{s}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
