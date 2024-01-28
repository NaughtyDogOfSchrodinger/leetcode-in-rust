pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    fn longest(set: &mut HashSet<i32>, max: i32) -> i32 {
        if max as usize > set.len() {
            return max;
        }

        if let Some(&num) = set.iter().next() {
            let (mut l, mut r) = (-1, 1);
            assert!(set.remove(&num));
            while set.remove(&(num + l)) {
                l += -1;
            }
            while set.remove(&(num + r)) {
                r += 1;
            }
            let sum = r - l - 1;

            longest(set, if sum > max { sum } else { max })
        } else {
            return max;
        }
    }

    match nums.len() {
        0 => 0,
        1 => 1,
        _ => longest(&mut nums.iter().map(|&i| i).collect::<HashSet<i32>>(), 1),
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_128::longest_consecutive;

    #[test]
    fn test() {
        println!("{:?}", longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
        println!(
            "{:?}",
            longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
        println!(
            "{:?}",
            longest_consecutive(vec![
                -7, -1, 3, -9, -4, 7, -3, 2, 4, 9, 4, -9, 8, -7, 5, -1, -7
            ])
        );
        println!(
            "{:?}",
            longest_consecutive(vec![7, 2, -2, 9, 6, 9, -7, 2, 1, -3, -1, -7, -5])
        );
    }
}
