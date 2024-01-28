pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        return n as i32;
    }
    let mut slow = 2;
    let mut fast = 2;
    while fast < n {
        if (nums[slow - 2] != nums[fast]) {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

#[cfg(test)]
mod test {
    use crate::leetcode_80::remove_duplicates;

    #[test]
    fn test() {
        let mut arr = Vec::from([1, 1, 1, 2, 2, 3]);
        println!("{:?}", remove_duplicates(&mut arr));
    }
}
