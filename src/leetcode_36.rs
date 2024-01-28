use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut map: HashSet<u16> = HashSet::new();
    assert_eq!(board.len(), 9);
    assert_eq!(board[0].len(), 9);
    for line in (0..9) {
        for row in (0..9) {
            let c = board[line][row];
            if c == '.' {
                continue;
            } else if c >= '1' && c <= '9' {
                let c_n = c as u16 - '0' as u16;
                let l_index = (line + 1) as u16 * 10 + c_n;
                let r_index = 100 + (row + 1) as u16 * 10 + c_n;
                let s_index = 200 + (line / 3 * 3 + row / 3 + 1) as u16 * 10 + c_n;
                if map.contains(&l_index) || map.contains(&r_index) || map.contains(&s_index) {
                    return false;
                }
                map.insert(l_index);
                map.insert(r_index);
                map.insert(s_index);
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::leetcode_36::is_valid_sudoku;

    #[test]
    fn test() {
        let (line, row) = (0, 1);
        println!("{}", line / 3 * 3 + row / 3);
        let (line, row) = (0, 5);
        println!("{}", line / 3 * 3 + row / 3);
        let (line, row) = (0, 7);
        println!("{}", line / 3 * 3 + row / 3);
        let (line, row) = (3, 1);
        println!("{}", line / 3 * 3 + row / 3);
        let (line, row) = (8, 8);
        println!("{}", line / 3 * 3 + row / 3);

        let board = [
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        let board_true = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        let mut input = vec![];
        for arr in board_true {
            let mut inner = vec![];
            for c in arr {
                inner.push(c.chars().next().unwrap());
            }
            input.push(inner);
        }
        println!("{}", is_valid_sudoku(input));
    }
}
