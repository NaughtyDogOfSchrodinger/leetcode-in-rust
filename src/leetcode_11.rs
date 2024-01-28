pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    while left < right {
        let sub = (right - left) as i32;
        let area = if height[left] < height[right] {
            height[left] * sub
        } else {
            height[right] * sub
        };
        if area > max {
            max = area;
        }
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max
}

#[cfg(test)]
mod test {
    use crate::leetcode_11::max_area;

    #[test]
    fn test() {
        println!("{:?}", max_area(Vec::from([1, 8, 6, 2, 5, 4, 8, 3, 7])));
    }
}
