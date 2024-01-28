pub fn str_str(haystack: String, needle: String) -> i32 {
    let pattern = needle.chars().collect::<Vec<char>>();
    let pattern_len = pattern.len();
    let mut pattern_i = 0;
    let arr = haystack.chars().collect::<Vec<char>>();
    let len = arr.len();
    let mut index = 0;
    let mut start = -1;

    while index < len {
        while index < len && pattern_i < pattern_len && pattern[pattern_i] == arr[index] {
            if pattern_i == 0 {
                start = index as i32;
            }
            index += 1;
            pattern_i += 1;
        }
        if pattern_i == pattern_len {
            return start;
        } else {
            pattern_i = 0;
            start = -1;
            index += 1;
        }
    }
    start
}

#[cfg(test)]
mod test {
    use crate::leetcode_28::str_str;

    #[test]
    fn test() {
        println!(
            "{:?}",
            str_str("mississippi".to_string(), "issip".to_string())
        );
    }
}
