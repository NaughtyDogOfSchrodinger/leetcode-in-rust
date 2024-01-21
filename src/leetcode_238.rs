pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut r = vec![1; nums.len()];
    for i in (1..nums.len()) {
        r[i] = r[i - 1] * nums[i - 1];
    }
    let mut tmp = 1;
    for i in (0.. nums.len()).rev() {
        r[i] *= tmp;
        tmp *= nums[i];
    }
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_238::product_except_self;

    #[test]
    fn test() {
        let mut a = Vec::from([1, 2, 3, 4]);
        println!("{:?}", product_except_self(a));
    }
}