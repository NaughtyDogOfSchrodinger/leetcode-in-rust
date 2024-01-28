pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_arr = s.chars().collect::<Vec<char>>();
    let mut t_arr = t.chars();
    let mut index = 0;
    while let Some(t) = t_arr.next() {
        if index < s_arr.len() && s_arr[index] == t {
            index += 1;
        }
    }
    index == s_arr.len()
}

#[cfg(test)]
mod test {
    use crate::leetcode_392::is_subsequence;

    #[test]
    fn test() {
        println!("{:?}", is_subsequence("c".to_string(), "b".to_string()));
    }
}
