pub fn candy(ratings: Vec<i32>) -> i32 {
    assert!(ratings.len() > 0);
    let len = ratings.len();
    let mut r = vec![1;len];
    for i in 1..len {
        if ratings[i] > ratings[i - 1] {
            r[i] = r[i - 1] + 1;
        }
    }
    for i in (0..len - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            r[i] = if r[i + 1] + 1 > r[i] {
                r[i + 1] + 1
            } else {
                r[i]
            };
        }
    }
    r.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::leetcode_135::candy;

    #[test]
    fn test() {
        let gas = Vec::from([1,0,2]);
        println!("{:?}", candy(gas));
    }
}