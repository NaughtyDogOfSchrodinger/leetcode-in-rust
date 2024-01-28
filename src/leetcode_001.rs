pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if let Some(j) = map.get(&(target - nums[i])) {
            return vec![i as i32, *j];
        }
        map.insert(nums[i], i as i32);
    }
    vec![]
}

#[cfg(test)]
mod test {
    use crate::leetcode_001::two_sum;

    #[test]
    fn test() {
        println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
        println!("{:?}", two_sum(vec![3, 2, 4], 6));
        println!("{:?}", two_sum(vec![3, 3], 6));
    }
}
