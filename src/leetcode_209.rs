pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => if nums[0] >= target { 1 } else { 0 },
        len => {
            let mut left = 0;
            let mut min = i32::MAX;

            let mut sum = 0;
            for right in 0..len {
                let cur = nums[right];
                if cur >= target {
                    return 1;
                }
                sum += cur;
                if sum >= target {
                    while sum >= target {
                        sum -= nums[left];
                        left += 1;
                    }
                    if right + 2 > left {
                        let v = (right + 2 - left) as i32;
                        if v < min {
                            min = v;
                        }
                    }
                }
            }
            if min == i32::MAX {
                0
            } else { min }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_209::min_sub_array_len;

    #[test]
    fn test() {
        println!("{:?}", min_sub_array_len(7, Vec::from([2, 3, 1, 2, 4, 3])));
        println!("{:?}", min_sub_array_len(4, Vec::from([1, 4, 4])));
        println!("{:?}", min_sub_array_len(11, Vec::from([1, 1, 1])));
        println!("{:?}", min_sub_array_len(11, Vec::from([1, 2, 3, 4, 5])));
        println!("{:?}", min_sub_array_len(7, Vec::from([5])));
        println!("{:?}", min_sub_array_len(7, Vec::from([1, 1, 1, 1, 7])));
    }
}