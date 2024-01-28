pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        s
    } else {
        let mut result: Vec<String> = (0..num_rows).map(|_| String::new()).collect();
        let add = num_rows - 2;
        for (i, c) in s.chars().enumerate() {
            let sum = (num_rows + add) as usize;
            let last_index = (num_rows - 1) as usize;
            let mod_i = i % sum;
            if mod_i > last_index {
                let index: usize = last_index - (mod_i - last_index);
                result[index].push(c);
            } else {
                result[mod_i].push(c);
            }
        }
        result.iter().flat_map(|s| s.chars()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_006::convert;

    #[test]
    fn test() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            convert("PAYPALISHIRING".to_string(), 4)
        );
    }
}
