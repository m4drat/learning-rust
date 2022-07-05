pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c);
    };

    result
}

/// Reverses given string
/// 
/// Reverses given string using .rev() and .collect()
/// 
/// * `input` - input string to reverse
pub fn reverse_alternative(input: &str) -> String {
    input.chars().rev().collect()
}
