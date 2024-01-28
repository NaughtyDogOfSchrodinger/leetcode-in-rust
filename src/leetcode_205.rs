pub fn is_isomorphic(s: String, t: String) -> bool {
    assert_eq!(s.len(), t.len());
    let mut s_map = [u16::MAX; 128];
    let mut t_map = [u16::MAX; 128];
    let (mut s_chars, mut t_chars) = (s.chars(), t.chars());
    let mut i = 0usize;
    while let (Some(sc), Some(tc)) = (s_chars.next(), t_chars.next()) {
        assert!(sc.is_ascii() && tc.is_ascii());
        let s_index = sc as u8 - 0;
        let t_index = tc as u8 - 0;
        let s_last_index = s_map[s_index as usize];
        let t_last_index = t_map[t_index as usize];
        match (u16::MAX == s_last_index, u16::MAX == t_last_index) {
            (true, true) => {
                s_map[s_index as usize] = i as u16;
                t_map[t_index as usize] = i as u16;
            }
            (false, false) => {
                if t_last_index != s_last_index
                    || &t[i..i + 1] != &t[t_last_index as usize..t_last_index as usize + 1]
                    || &s[i..i + 1] != &s[s_last_index as usize..s_last_index as usize + 1]
                {
                    return false;
                } else {
                    s_map[s_index as usize] = i as u16;
                    t_map[t_index as usize] = i as u16;
                }
            }
            _ => return false,
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::leetcode_205::is_isomorphic;

    #[test]
    fn test() {
        // println!("{:?}", is_isomorphic("egg".to_string(), "add".to_string()));
        // println!("{:?}", is_isomorphic("foo".to_string(), "bar".to_string()));
        // println!("{:?}", is_isomorphic("badc".to_string(), "baba".to_string()));
        println!(
            "{:?}",
            is_isomorphic("bbbaaaba".to_string(), "aaabbbba".to_string())
        );
    }
}
