pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = [0u32; 26];
    for char in magazine.chars() {
        let index = char as u8 - 'a' as u8;
        assert!(index >= 0 && index < 26);
        map[index as usize] += 1;
    }
    for char in ransom_note.chars() {
        let index = char as u8 - 'a' as u8;
        if map[index as usize] == 0 {
            return false;
        } else {
            map[index as usize] -= 1;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::leetcode_383::can_construct;

    #[test]
    fn test() {
        println!("{:?}", can_construct("aa".to_string(), "aab".to_string()));
        println!("{:?}", can_construct("a".to_string(), "b".to_string()));
        println!("{:?}", can_construct("aa".to_string(), "ab".to_string()));
    }
}
