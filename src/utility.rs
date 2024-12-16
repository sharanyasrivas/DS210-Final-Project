use std::io::BufRead;

/// Example utility function.
pub fn example_utility_function() {
    println!("This is an example utility function.");
}

/// Utility to split strings from a line.
pub fn split_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

/// Function to load a file line-by-line (useful for handling large files).
pub fn read_file_lines(file_path: &str) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    reader.lines().collect()
}



