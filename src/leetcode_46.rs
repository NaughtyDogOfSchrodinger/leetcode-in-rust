pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

    fn dfs(path: &mut Vec<i32>, index: usize, result: &mut Vec<Vec<i32>>) {
        if index == path.len(){
            result.push(path.clone());
            return;
        }
        for i in (index..path.len()) {
            println!("before dfs: {:?}, i: {}, index:{}", path, i, index);
            path.swap(i, index);
            println!("before dfs: {:?}, i: {}, index:{}", path, i, index);

            dfs(path, index + 1, result);
            println!("after dfs: {:?}, i: {}, index:{}", path, i, index);
            path.swap(i, index);
            println!("after dfs: {:?}, i: {}, index:{}", path, i, index);
        }
    }
    match nums.len() {
        0 | 1 => Vec::from([nums]),
        _ => {
            let mut result = vec![];
            let mut path = nums.clone();
            dfs(&mut path, 0, &mut result);
            result
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_46::permute;

    #[test]
    fn test() {
        println!("{:?}", permute(Vec::from([1, 4, 5, 7])));
    }
}