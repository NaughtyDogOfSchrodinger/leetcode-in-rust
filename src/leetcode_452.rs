pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    fn has_intersection(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[1] >= b[0] && b[1] >= a[0]
    }

    fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        vec![a[0].max(b[0]), a[1].min(b[1])]
    }
    let mut points = points;
    points.sort();
    let mut r = 0;
    let mut index = 0;
    while index < points.len() {
        let mut tmp = points[index].clone();
        index += 1;
        while index < points.len() && has_intersection(&tmp, &points[index]) {
            tmp = intersection(&tmp, &points[index]);
            index += 1;
        }
        r += 1;
    }
    r
}

pub fn find_min_arrow_shots_1(points: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut points = points;
    points.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
    let mut end = i32::MAX;
    for point in points {
        if point[0] <= end {
            end = end.min(point[1]);
        } else {
            end = point[1];
            ans += 1;
        }
    }

    ans + 1
}

#[cfg(test)]
mod test {
    use crate::leetcode_452::find_min_arrow_shots;

    #[test]
    fn test() {
        println!(
            "{:?}",
            find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
        println!(
            "{:?}",
            find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        );
        println!(
            "{:?}",
            find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]])
        );
    }
}
