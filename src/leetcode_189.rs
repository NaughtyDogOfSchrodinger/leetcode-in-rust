pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut start = 0;
    let mut next = start;

    let mut count = 0;
    let mut last_read = nums[next];
    while count < len  {
        if count != 0 && next == start {
            start += 1;
            next += 1;
            last_read = nums[next];
        }
        let last_write = nums[(next + k as usize) % len];
        nums[(next + k as usize) % len] = last_read;
        last_read = last_write;
        next = (next + k as usize) % len;
        count += 1;
    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_189::rotate;

    #[test]
    fn test() {
        let mut a = Vec::from([-1,-100,3,99]);
        rotate(&mut a, 2);
        println!("{:?}", a);
    }
}