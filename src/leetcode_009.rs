pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else if x >= 0 && x <= 9 {
        true
    } else if x % 10 == 0 {
        false
    } else {
        let mut origin = x;
        let mut reverse = 0;

        while origin > 0 {
            reverse = 10 * reverse + origin % 10;
            origin /= 10;
        }
        x == reverse
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_009::is_palindrome;

    #[test]
    fn test() {
        println!("{}", is_palindrome(121));
        println!("{}", is_palindrome(-121));
        println!("{}", is_palindrome(10));
        println!("{}", is_palindrome(1));
    }
}
