pub fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut r = 0;
    while num != 0 {
        if i32::MAX / 10 < r || i32::MIN / 10 > r {
            return 0;
        }
        r = 10 * r + num % 10;
        num /= 10;
    }
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_007::reverse;

    #[test]
    fn test() {
        println!("{}", reverse(120));
        println!("{}", reverse(123));
        println!("{}", reverse(-123));
        println!("{}", reverse(1534236469));
    }
}