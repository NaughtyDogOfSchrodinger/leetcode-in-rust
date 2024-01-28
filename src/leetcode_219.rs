pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashSet;

    match nums.len() {
        1 => false,
        len => {
            if k == 0 {
                return false;
            }
            let mut map = HashSet::new();
            let mut count = 0;
            for (i, num) in nums.iter().enumerate() {
                if i + 1 < len && *num == nums[i + 1]
                    || ((i + k as usize) < len && *num == nums[i + k as usize])
                {
                    return true;
                }
                if count <= k {
                    if map.contains(num) {
                        return true;
                    }
                    map.insert(*num);
                    count += 1;
                } else {
                    map.remove(&nums[i - k as usize - 1]);
                    if map.contains(num) {
                        return true;
                    }
                    map.insert(*num);
                }
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
        println!("{:?}", contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        println!("{:?}", contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        println!("{:?}", contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
        println!("{:?}", contains_nearby_duplicate(vec![1, 2, 2, 3], 3));
        println!(
            "{:?}",
            contains_nearby_duplicate(
                vec![
                    313, 64, 612, 515, 412, 661, 244, 164, 492, 744, 233, 579, 62, 786, 342, 817,
                    838, 396, 230, 79, 570, 702, 124, 727, 586, 542, 919, 372, 187, 626, 869, 923,
                    103, 932, 368, 891, 411, 125, 724, 287, 575, 326, 88, 125, 746, 117, 363, 8,
                    881, 441, 542, 653, 211, 180, 620, 175, 747, 276, 832, 772, 165, 733, 574, 186,
                    778, 586, 601, 165, 422, 956, 357, 134, 857, 114, 450, 64, 494, 679, 495, 205,
                    859, 136, 477, 879, 940, 139, 903, 689, 954, 335, 859, 78, 20
                ],
                22
            )
        );
    }
}
