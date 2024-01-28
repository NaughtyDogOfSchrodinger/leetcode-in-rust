pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    match nums.len() {
        0 => vec![],
        1 => vec![format!("{}", nums[0])],
        len => {
            let mut index = 0;
            let mut r = vec![];
            let mut s;
            while index < len {
                s = format!("{}", nums[index]);
                index += 1;
                while index < len && nums[index] == nums[index - 1] {
                    index += 1;
                }
                let start = index;
                while index < len && nums[index] == nums[index - 1] + 1 {
                    index += 1;
                }
                if index > start {
                    s.push_str(format!("->{}", nums[index - 1]).as_str());
                }
                r.push(s);
            }
            r
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_228::summary_ranges;

    #[test]
    fn test() {
        println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7]));
        println!("{:?}", summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]));
    }
}
