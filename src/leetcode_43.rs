pub fn multiply(num1: String, num2: String) -> String {
    if num1.is_empty() || num2.is_empty() {
        "".to_string()
    } else {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut result = Vec::new();
        let mut move_i = 1;
        for c1 in num1.chars().rev() {
            let mut carry: u8 = 0;
            let mut num_s = vec![];

            for c2 in num2.chars().rev() {
                let a = c1 as u8 - '0' as u8;
                let b = c2 as u8 - '0' as u8;
                let multi = a * b + carry;
                num_s.push(multi % 10);
                carry = multi / 10;
            }
            if carry != 0 {
                num_s.push(carry);
            }
            if result.is_empty() {
                result.append(&mut num_s);
            } else {
                let mut index = move_i;
                let mut o_index = move_i;
                let mut n_index = 0;
                let mut carry = 0;
                while o_index < result.len() || n_index < num_s.len() {
                    let sum = match (o_index < result.len(), n_index < num_s.len()) {
                        (true, true) => result[o_index] + num_s[n_index] + carry,
                        (false, true) => num_s[n_index] + carry,
                        (true, false) => result[o_index] + carry,
                        _ => unreachable!(),
                    };
                    if index >= result.len() {
                        result.push(sum % 10);
                    } else {
                        result[index] = sum % 10;
                    }
                    carry = sum / 10;
                    o_index += 1;
                    n_index += 1;
                    index += 1;
                }
                if carry != 0 {
                    result.push(carry);
                }
                move_i += 1;
            }
        }
        result.into_iter().rev().map(|num| char::from(num + '0' as u8)).collect::<String>()

    }

}

#[cfg(test)]
mod test {
    use crate::leetcode_43::multiply;

    #[test]
    fn test() {
        println!("{:?}", multiply("999".to_string(), "999".to_string()));
    }
}