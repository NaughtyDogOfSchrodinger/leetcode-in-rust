pub fn trap(height: Vec<i32>) -> i32 {
    let tmp = Vec::from_iter(height.into_iter().scan(0, |state, x| {
        *state = x.max(*state);
        Some((*state, x))
    }));
    println!("{:?}", tmp);
    tmp.into_iter()
        .rev()
        .scan(0, |state, x| {
            Some({
                *state = x.0.min(*state).max(x.1);
                *state - x.1
            })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::leetcode_42::trap;

    #[test]
    fn test() {
        // println!("{:?}", trap(vec![]));
        // println!("{:?}", trap(vec![1]));
        // println!("{:?}", trap(vec![1, 2]));
        // println!("{:?}", trap(vec![2, 2]));
        // println!("{:?}", trap(vec![2, 2, 3]));
        // println!("{:?}", trap(vec![2, 2, 3, 2]));
        // println!("{:?}", trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
        println!("{:?}", trap(vec![4, 2, 0, 3, 2, 5]));
        // println!("{:?}", trap(vec![5,4,1,2]));
        // println!("{:?}", trap(vec![6,4,2,0,3,2,0,3,1,4,5,3,2,7,5,3,0,1,2,1,3,4,6,8,1,3]));
    }
}
