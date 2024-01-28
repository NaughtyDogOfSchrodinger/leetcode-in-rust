pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    fn key(s: &str) -> [u8; 26] {
        let mut r = [0; 26];
        s.chars().for_each(|c| {
            let num = c as u8 - 'a' as u8;
            assert!(num >= 0 && num <= 25);
            r[num as usize] += 1;
        });
        r
    }
    let mut map = HashMap::new();
    for s in strs {
        map.entry(key(&s))
            .and_modify(|mut arr: &mut Vec<String>| arr.push(s.clone()))
            .or_insert(vec![s]);
    }
    map.into_values().collect::<Vec<Vec<String>>>()
}

pub fn group_anagrams1(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    strs.into_iter()
        .fold(HashMap::new(), |mut map, s| {
            map.entry(
                s.chars()
                    .map(|c| {
                        let num = c as u8 - 'a' as u8;
                        assert!(num >= 0 && num <= 25);
                        1 << (num << 2)
                    })
                    .sum::<u128>(),
            )
            .and_modify(|mut arr: &mut Vec<String>| arr.push(s.clone()))
            .or_insert(vec![s]);
            map
        })
        .into_values()
        .collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod test {
    use crate::leetcode_49::{group_anagrams, group_anagrams1};

    #[test]
    fn test() {
        println!(
            "{:?}",
            group_anagrams1(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ])
        );
    }
}
