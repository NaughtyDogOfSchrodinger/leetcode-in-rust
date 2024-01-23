
pub fn is_happy(n: i32) -> bool {

    fn next(n: i32) -> i32 {
        let mut sum = 0;
        let mut num = n;
        while num != 0 {
            let a = num % 10;
            sum += a * a;
            num /= 10;
        }
        sum
    }

    let (mut fast, mut slow) = (n, n);
    fast = next(fast);
    loop {
        fast = next(fast);
        if fast == 1 {
            return true;
        }
        fast = next(fast);
        slow = next(slow);
        if fast == slow {
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_202::is_happy;

    #[test]
    fn test() {
        println!("{:?}", is_happy(2));
        println!("{:?}", is_happy(19));
    }
}