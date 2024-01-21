use std::cmp::max;
use std::collections::{HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        0
    } else {
        let mut begin: i32 =-1;
        let mut end: i32 = 0;
        let mut max_len = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            end = i as i32;
            if map.contains_key(&c) {
                begin = max(begin, *map.get(&c).unwrap());
            }
            map.insert(c, end);
            max_len = max(max_len, end - begin);
        }

        max_len
    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_003::length_of_longest_substring;

    #[test]
    fn test() {
        println!("{}",length_of_longest_substring(String::from("abcabcbb")));
        println!("{}",length_of_longest_substring(String::from("bbbbb")));
        println!("{}",length_of_longest_substring(String::from("pwwkew")));
        println!("{}",length_of_longest_substring(String::from(" ")));
        println!("{}",length_of_longest_substring(String::from("dvdf")));
        println!("{}",length_of_longest_substring(String::from("cdd")));
        println!("{}",length_of_longest_substring(String::from("abba")));
    }
}