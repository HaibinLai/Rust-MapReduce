

pub fn map(data: &str) -> Vec<(String, usize)> {
    data.split_whitespace()
        .map(|word| (word.to_string(), 1))
        .collect()
}