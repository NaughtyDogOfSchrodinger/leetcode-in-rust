pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() {
        0
    } else {
        let mut r = 0;
        let mut native = true;
        let c_array: Vec<char> = s.chars().collect();
        let mut index = 0;
        while index < c_array.len() && c_array[index] == ' ' {
            index += 1;
        }
        if index >= c_array.len() {
            0
        } else {
            match c_array[index] {
                '-' => {
                    native = false;
                    index += 1;
                }
                '+' => {
                    index += 1;
                }
                _ => {}
            }
            while index < c_array.len() && c_array[index] >= '0' && c_array[index] <= '9' {
                let n: i8 = if native {
                    c_array[index] as i8 - '0' as i8
                } else {
                    -1 * (c_array[index] as u8 - '0' as u8) as i8
                };
                if native && r > (i32::MAX - n as i32) / 10 {
                    return i32::MAX;
                }
                if !native && r < (i32::MIN - n as i32) / 10 {
                    return i32::MIN;
                }

                r = 10 * r + n as i32;
                index += 1;
            }
            r
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_008::my_atoi;

    #[test]
    fn test() {
        // println!("{}", '0' as u8);
        // println!("{}", '1' as u8);
        // println!("{}", my_atoi("4193 with words".to_string()));
        // println!("{}", my_atoi("42".to_string()));
        // println!("{}", my_atoi("   -42".to_string()));
        // println!("{}", my_atoi(" ".to_string()));
        println!("{}", my_atoi("2147483648".to_string()));
        println!("{}", my_atoi("2147483646".to_string()));
    }
}
