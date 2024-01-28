pub fn is_palindrome(s: String) -> bool {
    let mut arr = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    while let (Some(left), Some(right)) = (arr.next(), arr.next_back()) {
        if left != right {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::leetcode_125::is_palindrome;

    #[test]
    fn test() {
        println!(
            "{:?}",
            is_palindrome("A man, a plan, a canal: Panama".to_string())
        );
    }
}
