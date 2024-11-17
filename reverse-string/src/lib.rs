pub fn reverse(input: &str) -> String {
    let reverse_string:String = input.chars()
        .rev()
        .collect();
    reverse_string
}
