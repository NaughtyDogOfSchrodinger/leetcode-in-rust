//字符          数值
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
pub fn int_to_roman(num: i32) -> String {
    let mut x = num;
    let mut r = String::new();
    let map = [
        (1000, 'M'),
        (500, 'D'),
        (100, 'C'),
        (50, 'L'),
        (10, 'X'),
        (5, 'V'),
        (1, 'I'),
    ];
    for index in [1, 3, 5] {
        let (pre_num, pre_letter) = map[index - 1];
        let (cur_num, cur_letter) = map[index];
        let (next_num, next_letter) = map[index + 1];
        (0..x / pre_num).for_each(|_| r.push(pre_letter));
        x %= pre_num;
        match (x / cur_num, x / next_num) {
            (1, 9) => {
                r.push(next_letter);
                r.push(pre_letter);
                x -= 9 * next_num;
            }
            (0, 4) => {
                r.push(next_letter);
                r.push(cur_letter);
                x -= 4 * next_num;
            }
            (cur, next) => {
                if cur != 0 {
                    (0..cur).into_iter().for_each(|_| r.push(cur_letter));
                    x %= cur_num;
                    if x / next_num != 0 {
                        (0..x / next_num)
                            .into_iter()
                            .for_each(|_| r.push(next_letter));
                        x %= next_num;
                    }
                } else {
                    if next != 0 {
                        (0..next).into_iter().for_each(|_| r.push(next_letter));
                        x %= next_num;
                    }
                }
            }
        };
    }
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_12::int_to_roman;

    #[test]
    fn test() {
        println!("{}", int_to_roman(3));
        println!("{}", int_to_roman(4));
        println!("{}", int_to_roman(9));
        println!("{}", int_to_roman(58));
        println!("{}", int_to_roman(1994));
    }
}
