// pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
//     let mut map = HashMap::new();
//     let word_len = words[0].len();
//     let total_count = words.len();
//     let s_len = s.len();
//     if s_len < word_len {
//         return vec![];
//     }
//     for word in words {
//         map.entry(word).and_modify(|counter| *counter += 1).or_insert(1);
//     }
//     let mut index = 0;
//     let mut r = vec![];
//     while index <= s_len - word_len {
//         let start = index;
//         let mut tmp = vec![];
//         while index <= s_len - word_len && map.contains_key(&s[index..index + word_len]) {
//             let mut flag = false;
//             let word = String::from(&s[index..index + word_len]);
//             map.entry(word.clone()).and_modify(|count| if *count > 0 {
//                 tmp.push(word);
//                 index += word_len;
//                 *count -= 1;
//             } else { flag = true; });
//             if flag {
//                 flag = false;
//                 break;
//             }
//         }
//         if index - start == total_count * word_len {
//             r.push(start as i32);
//         }
//         while let Some(w) = tmp.pop() {
//             map.entry(w).and_modify(|counter| *counter += 1).or_insert(1);
//         }
//         index = start + 1;
//     }
//     r
// }

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    fn tt(map: &mut HashMap<String, i32>, key: &str, count: i32) {
        *map.entry(String::from(key)).or_insert(0) += count;
        if *map.get(key).unwrap() == 0 {
            map.remove(key);
        }
    }
    let mut r = vec![];
    let mut map = HashMap::new();
    let (word_count, word_len) = (words.len(), words[0].len());
    let target_len = word_count * word_len;
    let s_len = s.len();
    for index in 0..word_len {
        if index + target_len > s_len {
            break;
        }
        for i in (index..index + target_len).step_by(word_len) {
            tt(&mut map, &s[i..i + word_len], 1);
        }
        for word in words.iter() {
            tt(&mut map, word, -1);
        }
        if map.is_empty() {
            r.push(index as i32);
        }
        for i in (index + word_len..s.len() - word_count * word_len + 1).step_by(word_len) {
            tt(&mut map, &s[i - word_len..i], -1); // 移除左边
            tt(
                &mut map,
                &s[i + (word_count - 1) * word_len..i + word_count * word_len],
                1,
            ); // 添加右边
            if map.is_empty() {
                r.push(i as i32)
            }
        }

        map.clear();
    }
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_30::find_substring;

    #[test]
    fn test() {
        println!(
            "{:?}",
            find_substring(
                "barfoothefoobarman".to_string(),
                Vec::from(["foo".to_string(), "bar".to_string()])
            )
        );
        println!(
            "{:?}",
            find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                Vec::from([
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ])
            )
        );
        println!(
            "{:?}",
            find_substring(
                "barfoofoobarthefoobarman".to_string(),
                Vec::from(["bar".to_string(), "foo".to_string(), "the".to_string()])
            )
        );
        println!(
            "{:?}",
            find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                Vec::from([
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ])
            )
        );
        println!(
            "{:?}",
            find_substring(
                "barfoothefoobarman".to_string(),
                Vec::from(["foo".to_string(), "bar".to_string()])
            )
        );
        println!(
            "{:?}",
            find_substring("a".to_string(), Vec::from(["a".to_string()]))
        );
    }
}
