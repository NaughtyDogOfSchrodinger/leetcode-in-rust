pub fn next_permutation(nums: &mut Vec<i32>) {
    match nums.len() {
        0 | 1 => {},
        2 => nums.swap(0, 1),
        _ => {
            let last = nums.len() - 1;
            let mut end = last;
            while end >= 1 && nums[end] <= nums[end - 1]  {
                end -= 1;
            }
            if end == 0 {
                nums.sort_unstable();
            } else {
                nums[end..=last].sort_unstable();
                for i in (end..=last) {
                    if nums[i] > nums[end - 1] {
                        nums.swap(i, end - 1);
                        break;
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_31::next_permutation;

    #[test]
    fn test() {
        // let mut vec = Vec::from([1, 2, 3, 7, 8]);
        // let mut vec = Vec::from([8, 7, 3, 2, 1]);
        let mut vec = Vec::from([1, 3, 2]);
        next_permutation(&mut vec);
        println!("{:?}", vec);
    }
}