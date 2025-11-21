/// Returns content of the text file.
pub fn load_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|e| panic!("failed to load file: {} with reason: {}", file_name, e))
}

/// Retrieves lines from provided text starting from line with `start_index`
/// until the line with `end_index` inclusive.
pub fn select_lines(content: impl AsRef<str>, start_index: usize, end_index: usize) -> Vec<String> {
    let mut lines = vec![];
    for (i, line) in content.as_ref().lines().enumerate() {
        if (start_index..=end_index).contains(&i) {
            lines.push(line.to_string());
        }
    }
    lines
}

/// Prints lines to standard output.
pub fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("{}", line)
    }
}
