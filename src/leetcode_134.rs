pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    assert_eq!(gas.len(), cost.len());
    let (mut index, len) = (0, gas.len());
    while index < len {
        let (mut count, mut remain) = (0, 0);
        while count < len {
            let start = (index + count) % gas.len();
            remain += gas[start] - cost[start];
            if remain < 0 {
                break;
            }
            count += 1;
        }
        if count == len {
            return index as i32;
        } else {
            index = index + count + 1;
        }
    }
    -1
}

// pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//
//     fn cc(remain: i32, start: usize, gas: &Vec<i32>, cost: &Vec<i32>) -> i32 {
//         let mut index = start;
//         while index < gas.len() && cost[index] > gas[index]  {
//             index += 1;
//         }
//         if index >= gas.len() {
//             return -1;
//         }
//         let mut remain = remain;
//         for count in 0..gas.len() {
//             remain += gas[index] - cost[index];
//             if remain < 0 {
//                 return cc(0, (start + count + 1) % gas.len(), gas, cost);
//             } else {
//                 index = (index + 1) % gas.len();
//             }
//         }
//         index as i32
//     }
//     assert_eq!(gas.len(), cost.len());
//     cc(0, 0, &gas, &cost)
// }

#[cfg(test)]
mod test {
    use crate::leetcode_134::can_complete_circuit;

    #[test]
    fn test() {
        let gas = Vec::from([1, 2, 3, 4, 5]);
        let cost = Vec::from([3, 4, 5, 1, 2]);
        println!("{:?}", can_complete_circuit(gas, cost));
    }
}
