pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    fn spiral(
        matrix: &Vec<Vec<i32>>,
        row_start: usize,
        row_end: usize,
        line_start: usize,
        line_end: usize,
        result: &mut Vec<i32>,
    ) {
        if row_start > row_end && line_start > line_end {
            return;
        }
        match (line_start == line_end, row_start == row_end) {
            (true, true) => {
                result.push(matrix[line_start][row_start]);
                return;
            }
            (true, false) => {
                for r in line_start..=row_end {
                    result.push(matrix[line_start][r]);
                }
                return;
            }
            (false, true) => {
                for l in line_start..=line_end {
                    result.push(matrix[l][row_start]);
                }
                return;
            }
            (false, false) => {
                for r in row_start..=row_end - 1 {
                    result.push(matrix[line_start][r]);
                }
                for l in line_start..=line_end - 1 {
                    result.push(matrix[l][row_end]);
                }
                for r in (row_start + 1..=row_end).rev() {
                    result.push(matrix[line_end][r]);
                }
                for l in (line_start + 1..=line_end).rev() {
                    result.push(matrix[l][line_start]);
                }
            }
        }
        if row_start + 1 == row_end || line_start + 1 == line_end {
            return;
        }
        spiral(
            matrix,
            row_start + 1,
            row_end - 1,
            line_start + 1,
            line_end - 1,
            result,
        );
    }

    let mut r = vec![];
    spiral(&matrix, 0, matrix[0].len() - 1, 0, matrix.len() - 1, &mut r);
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_54::spiral_order;

    #[test]
    fn test() {
        println!(
            "{:?}",
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        println!(
            "{:?}",
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ])
        );
        println!("{:?}", spiral_order(vec![vec![1]]));
        println!("{:?}", spiral_order(vec![vec![2, 3]]));
        println!("{:?}", spiral_order(vec![vec![6, 9, 7]]));
        println!(
            "{:?}",
            spiral_order(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]])
        );
    }
}
