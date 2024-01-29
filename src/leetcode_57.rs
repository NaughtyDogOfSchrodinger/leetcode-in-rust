pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    fn has_inter(a: &[i32], b: &[i32]) -> bool {
        a[1] >= b[0] && b[1] >= a[0]
    }

    fn union(a: &[i32], b: &[i32]) -> Vec<i32> {
        vec![
            if a[0] < b[0] { a[0] } else { b[0] },
            if a[1] > b[1] { a[1] } else { b[1] },
        ]
    }

    match intervals.len() {
        0 => vec![new_interval],
        len => {
            let mut intervals = intervals;
            let (mut start, mut end) = (0, len - 1);
            if new_interval[1] < intervals[start][0] {
                intervals.insert(0, new_interval);
                return intervals;
            }
            if new_interval[0] > intervals[end][1] {
                intervals.push(new_interval);
                return intervals;
            }
            while start <= end && !has_inter(&new_interval, &intervals[start]) {
                if new_interval[1] < intervals[start][0] {
                    intervals.insert(start, new_interval);
                    return intervals;
                }
                start += 1;
            }
            while start <= end && !has_inter(&new_interval, &intervals[end]) {
                end -= 1;
            }
            if start == end {
                intervals[start] = union(&new_interval, &intervals[start]);
            } else {
                intervals[start] = union(&union(&new_interval, &intervals[end]), &intervals[start]);
                let mut r = vec![];
                r.extend_from_slice(&intervals[0..=start]);
                if end < len {
                    r.extend_from_slice(&intervals[end + 1..]);
                }
                return r;
            }
            intervals
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_57::insert;

    #[test]
    fn test() {
        println!(
            "{:?}",
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
        println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
        println!("{:?}", insert(vec![vec![1, 5]], vec![2, 3]));
        println!("{:?}", insert(vec![vec![1, 5]], vec![6, 8]));
        println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
        println!("{:?}", insert(vec![vec![3, 5], vec![12, 15]], vec![6, 6]));
    }
}
