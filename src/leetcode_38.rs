pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        "1".to_string()
    } else if n == 2 {
        "11".to_string()
    } else {
        let pre = count_and_say(n - 1);
        let arr: Vec<char> = pre.chars().collect();
        let len = arr.len();
        let mut r = String::new();
        let mut count: u8 = 1;

        let mut index = 1;

        while index < len {
            while index < len && arr[index - 1] == arr[index] {
                count += 1;
                index += 1;
            }
            if count > 1 {
                r.push(char::from(count + '0' as u8));
                r.push(arr[index - 1]);
                if index == len - 1 {
                    r.push('1');
                    r.push(arr[index]);
                }
                index += 1;
                count = 1;
            }
            while index < len && arr[index - 1] != arr[index] {
                r.push('1');
                r.push(arr[index - 1]);
                if index == len - 1 {
                    r.push('1');
                    r.push(arr[index]);
                }
                index += 1;
            }
        }
        r
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_38::count_and_say;

    #[test]
    fn test() {
        println!("{:?}", count_and_say(7));
    }
}
