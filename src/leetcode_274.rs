pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut sub_count = citations.len();
    let mut res = [0; 1001];
    for c in citations {
        res[c as usize] += 1;
    }
    for i in 0..res.len() {
        if sub_count < i {
            break;
        }
        sub_count -= res[i];
        ans = i;
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use crate::leetcode_274::h_index;

    #[test]
    fn test() {
        let mut a = Vec::from([3, 0, 6, 1, 5]);
        println!("{}", h_index(a));
    }
}
