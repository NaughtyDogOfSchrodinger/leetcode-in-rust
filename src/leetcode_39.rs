pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(
        sorted: &Vec<i32>,
        target: i32,
        sum: i32,
        index: usize,
        mut path: &mut Vec<i32>,
        mut result: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            result.push(path.clone());
        } else if sum > target {
            return;
        } else {
            for i in (index..sorted.len()) {
                path.push(sorted[i]);
                dfs(sorted, target, sum + sorted[i], i, path, result);
                path.pop();
            }
        }
    }
    let mut sorted: Vec<i32> = candidates
        .iter()
        .filter(|&item| *item <= target)
        .map(|item| *item)
        .collect();
    sorted.sort();
    let mut path = Vec::with_capacity(150);
    let mut result = Vec::new();
    dfs(&sorted, target, 0, 0, &mut path, &mut result);
    result
}

#[cfg(test)]
mod test {
    use crate::leetcode_39::combination_sum;

    #[test]
    fn test() {
        println!("{:?}", combination_sum(Vec::from([2, 3, 6, 7]), 7));
    }
}
