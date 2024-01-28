pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn has_inter(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[1] >= b[0] && b[1] >= a[0]
    }

    fn union(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        vec![
            if a[0] < b[0] { a[0] } else { b[0] },
            if a[1] > b[1] { a[1] } else { b[1] },
        ]
    }
    match intervals.len() {
        1 => intervals.clone(),
        _ => {
            let mut r = vec![];
            for interval in intervals {
                if r.is_empty() {
                    r.push(interval);
                } else {
                    let (left, right) = (interval[0], interval[1]);
                    assert!(left <= right);
                    if left > r[r.len() - 1][1] {
                        r.push(interval);
                    } else if right < r[0][0] {
                        r.insert(0, interval);
                    } else {
                        let mut flag = false;
                        let (mut i, mut j) = (0, r.len() - 1);
                        while i <= j && !has_inter(&r[i], &interval) {
                            if i + 1 < r.len() && right < r[i + 1][0] && left > r[i][1] {
                                r.insert(i + 1, interval.clone());
                                flag = true;
                                break;
                            }
                            i += 1;
                        }
                        if i < r.len() {
                            while i <= j && !has_inter(&r[j], &interval) {
                                j -= 1;
                            }
                        }

                        if !flag {
                            if i == j {
                                r[i] = union(&r[i], &interval);
                            } else if i < j {
                                r[i] = union(&union(&interval, &r[i]), &r[j]);
                                let mut new_r = vec![];
                                new_r.extend_from_slice(&r[0..=i]);
                                if j < r.len() {
                                    new_r.extend_from_slice(&r[j + 1..]);
                                }
                                r = new_r;
                            }
                        }
                    }
                }
            }
            r
        }
    }
}

pub fn merge_1(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort();
    let mut ans = vec![];
    let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
    intervals.iter().skip(1).for_each(|x| {
        if x[0] > end {
            ans.push(vec![start, end]);
            start = x[0];
        }
        end = end.max(x[1]);
    });
    ans.push(vec![start, end]);
    ans
}

#[cfg(test)]
mod test {
    use crate::leetcode_56::merge;

    #[test]
    fn test() {
        // println!("{:?}", merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
        // println!("{:?}", merge(vec![vec![1, 4], vec![4, 5]]));
        // println!("{:?}", merge(vec![vec![1, 4], vec![0, 1]]));
        // println!("{:?}", merge(vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![11,20],vec![1,10]]));
        println!(
            "{:?}",
            merge(vec![
                vec![-4, -3],
                vec![4, 5],
                vec![2, 4],
                vec![4, 6],
                vec![3, 4],
                vec![0, 0],
                vec![8, 9],
                vec![1, 1],
                vec![3, 5],
                vec![2, 2]
            ])
        );
    }
}
