pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    use std::collections::VecDeque;
    assert!(matrix[0].len() >= 1);
    assert_eq!(matrix.len(), matrix[0].len());
    let n = matrix.len();
    fn r(
        matrix: &mut Vec<Vec<i32>>,
        row_start: usize,
        row_end: usize,
        line_start: usize,
        line_end: usize,
        queue: &mut VecDeque<i32>,
    ) {
        if row_start == row_end && line_start == line_end {
            return;
        }
        for r in row_start + 1..=row_end {
            queue.push_back(matrix[line_start][r]);
            let ii = matrix[line_end - (r - row_start)][row_start];
            matrix[line_start][r] = ii;
        }
        for l in line_start + 1..=line_end {
            queue.push_back(matrix[l][line_end]);
            matrix[l][line_end] = queue.pop_front().unwrap();
        }
        for r in (row_start..=row_end - 1).rev() {
            queue.push_back(matrix[line_end][r]);
            matrix[line_end][r] = queue.pop_front().unwrap();
        }
        for l in (line_start..=line_end - 1).rev() {
            matrix[l][row_start] = queue.pop_front().unwrap();
        }
        if line_start + 1 == line_end || row_start + 1 == row_end {
            return;
        }
        r(
            matrix,
            row_start + 1,
            row_end - 1,
            line_start + 1,
            line_end - 1,
            queue,
        );
    }
    let mut queue = VecDeque::new();
    r(matrix, 0, n - 1, 0, n - 1, &mut queue);
}

#[cfg(test)]
mod test {
    use crate::leetcode_48::rotate;

    #[test]
    fn test() {
        let mut ve = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut ve);
        println!("{:?}", ve);
        let mut ve = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut ve);
        println!("{:?}", ve);
    }
}
