
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sort = nums.clone();
    sort.sort();
    let len = sort.len();
    let mut r = vec![];
    for i in 0..len - 2 {
        let cur = sort[i];
        if cur + sort[i + 1] + sort[i + 2] > 0 {
            return r;
        }
        if cur + sort[len - 2] + sort[len - 1] < 0 {
            continue;
        }
        if i != 0 && sort[i - 1] == cur {
            continue;
        }
        let (mut left, mut right) = (i + 1, len - 1);
        while left < right {
            let sum = sort[i] + sort[left] + sort[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                r.push(vec![cur, sort[left], sort[right]]);
                left += 1;
                while left < right && sort[left] == sort[left - 1] {
                    left += 1;
                }
                right -= 1;
                while left < right && sort[right] == sort[right + 1] {
                    right -= 1;
                }
            }
        }
    }

    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_15::three_sum;

    #[test]
    fn test() {
        println!("{:?}", three_sum(Vec::from([-2, 0, 1, 1, 2])));
        println!("{:?}", three_sum(Vec::from([0, 0, 0])));
        println!("{:?}", three_sum(Vec::from([0, 0, 0, 0])));
        println!("{:?}", three_sum(Vec::from([-1, 0, 1, 2, -1, -4])));
        println!("{:?}", three_sum(Vec::from([0, 0, 0])));
    }
}