pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        false
    } else {
        let mut map = [0u16;26];
        for sc in s.chars() {
            let index = sc as u8 - 'a' as u8;
            map[index as usize] += 1;
        }
        for tc in t.chars() {
            let index = tc as u8 - 'a' as u8;
            if map[index as usize] == 0 {
                return false;
            } else {
                map[index as usize] -= 1;
            }
        }
        true
    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_242::is_anagram;

    #[test]
    fn test() {
        println!("{:?}", is_anagram("anagram".to_string(), "nagaram".to_string()));
        println!("{:?}", is_anagram("rat".to_string(), "car".to_string()));
    }
}