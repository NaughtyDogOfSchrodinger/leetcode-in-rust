pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    match nums.len() as i32 {
        1 => false,
        len => {
            let (mut slow, mut fast) = (0, 1i32);
            while fast < len {
                while fast < len && fast <= k + slow {
                    if nums[slow as usize] == nums[fast as usize] {
                        return true;
                    }
                    fast += 1;
                }
                slow += 1;
                fast = slow + 1;
            }
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_219::contains_nearby_duplicate;

    #[test]
    fn test() {
        println!("{:?}", contains_nearby_duplicate(vec![1,2,3,1], 3));
        println!("{:?}", contains_nearby_duplicate(vec![1,0,1,1], 1));
        println!("{:?}", contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
    }
}