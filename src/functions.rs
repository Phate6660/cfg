pub fn ensure_directory(dir: &std::path::Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
}

pub fn format_and_trim(part1: &str, part2: &str) -> String {
    let output = format!("{}/{}", part1, part2);
    output.trim().to_string()
}
