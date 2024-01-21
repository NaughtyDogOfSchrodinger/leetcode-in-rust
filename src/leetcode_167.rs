pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut start, mut end) = (0, numbers.len() - 1);
    while start < end {
        while start < end && target - numbers[start] < numbers[end] {
            end -= 1;
        }
        if numbers[end] + numbers[start] == target {
            return vec![(start + 1) as i32, (end + 1) as i32];
        }
        while start < end && numbers[start] < target - numbers[end] {
            start += 1;
        }
        if numbers[end] + numbers[start] == target {
            return vec![(start + 1) as i32, (end + 1) as i32];
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod test {
    use crate::leetcode_167::two_sum;

    #[test]
    fn test() {
        println!("{:?}", two_sum(Vec::from([-1, 0]), -1));
    }
}