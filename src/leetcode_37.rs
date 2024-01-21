pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {


    fn dfs(map: &mut [u16], index: usize, array: &Vec<(usize, usize)>, board: &mut Vec<Vec<char>>, mut end: &mut bool)  {
        if index == array.len() {
            *end = true;
            return;
        }
        let (line, row) = array[index];
        let tuple = (line, 9 + row, 18 + line / 3 * 3 + row / 3);
        let (l_index, r_index, s_index) = tuple;
        let remain = map[l_index] | map[r_index] | map[s_index];
        let mut remain_zeros = 0b111111111 ^ remain;
        while !*end && remain_zeros != 0 {
            let trail_zero_count = remain_zeros.trailing_zeros();
            let bin_count = 1 << trail_zero_count;
            map[l_index] |= bin_count;
            map[r_index] |= bin_count;
            map[s_index] |= bin_count;
            board[line][row] = char::from(trail_zero_count as u8 + '1' as u8);
            dfs(map, index + 1, array, board, end);
            map[l_index] ^= bin_count;
            map[r_index] ^= bin_count;
            map[s_index] ^= bin_count;
            remain_zeros -= bin_count;
        }
    }
    let mut map = [0u16;27];
    let mut s = vec![];
    for line in (0..9) {
        for row in (0..9) {
            let c = board[line][row];
            if c == '.' {
                s.push((line, row));
                continue;
            } else if c >= '1' && c <='9'{
                let c_n = c as u16 - '0' as u16;
                let bin_count = 1 << (c_n - 1);
                let l_index = line;
                let r_index = 9 + row;
                let s_index = 18 + (line / 3 * 3 + row / 3);
                map[l_index] |= bin_count;
                map[r_index] |= bin_count;
                map[s_index] |= bin_count;
            }

        }
    }

    let mut end = false;
    dfs(&mut map, 0, &s, board, &mut end);
}






mod test {
    use crate::leetcode_37::solve_sudoku;

    #[test]
    fn test() {
        let a = 0 + 1 << 8;
        let b = 0 + 1 << 7;
        let mut a = 0b1101101;
        // println!("{:#b}", 0b101101101);
        println!("{:#b}", 255);
        println!("{:#b}", 0b111111111 ^ 255);
        println!("{:#b}", 511);
        // println!("{:#b}", 0b11000);
        // println!("{:#b}", 1 << 0);
        // println!("{}", 0b11000_u8.trailing_ones());
        let mut count = 0;
        // while a != 0 {
        //     if a % 2 == 0 {
        //         println!("count: {}", count);
        //     }
        //     count += 1;
        //     a /= 2;
        // }
        // println!("{:#b}", 500);
        // println!("{:#b}", 500 >> 2 & 1);
        let board_true = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]];
        let mut input = vec![];
        for arr in board_true  {
            let mut inner = vec![];
            for c in arr {
                inner.push(c.chars().next().unwrap());
            }
            input.push(inner);
        }
        solve_sudoku(&mut input);
        println!("{:?}", input);
    }
}