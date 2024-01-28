pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut map = Vec::with_capacity(64);
    let (line_count, line_remain) = (matrix.len() / 8, matrix.len() % 8);
    let mut base = line_count;
    for _ in 0..line_count {
        map.push(u8::MAX);
    }
    if line_remain != 0 {
        map.push((1 << line_remain) - 1);
        base += 1;
    }
    let (row_count, row_remain) = (matrix[0].len() / 8, matrix[0].len() % 8);
    for _ in 0..row_count {
        map.push(u8::MAX);
    }
    if row_remain != 0 {
        map.push((1 << row_remain) - 1);
    }

    for l in 0..matrix.len() {
        for r in 0..matrix[l].len() {
            if matrix[l][r] == 0 {
                if map[l / 8] >> l % 8 & 1 != 0 {
                    map[l / 8] ^= 1 << l % 8;
                }
                if map[base + r / 8] >> r % 8 & 1 != 0 {
                    map[base + r / 8] ^= 1 << r % 8;
                }
            }
        }
    }
    for l in 0..matrix.len() {
        for r in 0..matrix[l].len() {
            if map[l / 8] >> l % 8 & 1 == 0 {
                matrix[l][r] = 0
            }
            if map[base + r / 8] >> r % 8 & 1 == 0 {
                matrix[l][r] = 0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_73::set_zeroes;

    #[test]
    fn test() {
        // let a = 0;
        // println!("{:#b}", (1 << 3) - 1);
        // println!("{:#b}", 0b1101 >> 1 & 1);
        // println!("{:#b}", 5);
        // let mut input = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        // set_zeroes(&mut input);
        // println!("{:?}", input);
        let mut input = vec![
            vec![8, 3, 1, 4, 6, 4, 0, 3, 4],
            vec![9, 1, 3, 0, 1, 5, 7, 4, 1],
            vec![2, 2, 5, 2147483647, 5, 4, 4, 3, 8],
            vec![4, 9, 7, 0, 3, 6, 9, 5, 9],
            vec![4, 1, 8, 8, 4, 1, 5, 7, 6],
        ];
        set_zeroes(&mut input);
        println!("{:?}", input);
    }
}
