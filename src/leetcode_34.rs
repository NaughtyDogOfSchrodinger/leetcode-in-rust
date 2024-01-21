pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r = [-1i32;2];
    match nums.len() {
        0 => Vec::from(r),
        1 => if nums[0] == target {
            Vec::from([0i32;2])
        } else {
            Vec::from(r)
        },
        len => {
            let mut left = 0;
            let mut right = len - 1;
            while left < right && nums[left] < target {
                left += 1;
            }
            while left < right && nums[right] > target {
                right -= 1;
            }
            if left <= right  {
                if nums[left] == target {
                    r[0] = left as i32;
                }
                if nums[right] == target {
                    r[1] = right as i32;
                }
            }
            Vec::from(r)
        }
    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_34::search_range;

    #[test]
    fn test() {
        // println!("{:?}", search_range(Vec::from([5, 7, 7, 8, 8, 10]), 8));
        // println!("{:?}", search_range(Vec::from([5, 7, 7, 8, 8, 10]), 6));
        // println!("{:?}", search_range(vec![], 0));
        println!("{:?}", search_range(Vec::from([2, 2]), 1));
    }
}