//回文字符串边界的情况讨论
// 那么我们观察对称点左面出现的这个小回文字符串，这个字符串有三种情况：
//
// 情况1
// 如果左侧小回文字符串的左边界在大回文字符串的左边界之内，那么右面对称出的小回文字符串也不会触碰到大回文字符串的右边界。
//
// 比如“dacaxacad”这个字符串，左侧的“aca”没有超过这个大回文字符串的左边界，那么右面对称出的“aca”也不会超过右边界。
//
// 也就是说，在这种情况下，右面这个小回文字符串的长度与对称的小回文字符串的长度相等，绝对不会超过这个大回文字符串。
//
// 情况2
// 如果左侧小回文字符串的左边界超过了大回文字符串的左边界，那这个右面对称出的小回文字符串会正好触碰到大回文字符串的右边界，但是不会超出。
//
// 比如观察这个字符串“dcbabcdxdcbabce”。左侧的小回文字符串的边界超出了用下划线标出的大回文字符串的左边界。对称过来的右侧的小回文字符串的边界会刚好卡在大回文字符串的右边界。这是由于大回文字符串右边界之外的下一个字母（此处是“e”）绝对不是左边界的那个字母“d”的对称，所以右边的小回文字符串延申到边界之后也无法继续延申下去了。
//
// 也就是说，在这种情况下，右面这个小回文字符串的右边界与大回文字符串的右边界相同，那么这个小回文字符串的长度也绝对不会超过这个大回文字符串。
//
// 情况3
// 如果左侧小回文字符串的左边界正好卡在大回文字符串的左边界上，那么右面对称出的小回文字符串有可能会继续延伸下去，超过大回文字符串的右边界。
//
// 比如观察这个字符串“abcdcbabcdxdcbabcdxdcb"，左边的小回文字符串的左边界正好卡在大回文字符串的左边界上，那么对称过来的大回文字符串是有可能继续延申下去的。比如在这个例子中，右面以“a”为对称点的小回文字符串一直能向右延申到整个字符串的结尾。
//
// 也就是说，在这种情况下，右面这个小回文字符串的右边界至少与大回文字符串的右边界相同，并且有可能会延申。也就是说这个小回文字符串的长度可能会超过这个大回文字符串。
//
// 总结
// 我们考虑了左边的小回文字符串的边界与大回文字符串边界的三种情况，只有情况3，也就是正好边界在同一位置的时候，右侧的小回文字符串的长度才有可能会超过大回文字符串，而其他两种情况的话，我们可以跳过不去计算。
//
// 所以Manacher算法在先找到一个长回文字符串之后，可以选择性的跳过很多字母，无需一一计算，与暴力循环相比极大了提升了算法的效率。

use std::cmp::Ordering;

pub fn longest_palindrome(s: String) -> String {
    let mut r = 0;
    let len = 2 * s.len() + 1;
    let mut expand_array = Vec::with_capacity(len);
    for (i, c) in s.chars().enumerate() {
        expand_array.push('$');
        expand_array.push(c);
        if i == s.len() - 1 {
            expand_array.push('$');
        }
    }
    let mut result_array = Vec::with_capacity(len);
    for _ in 0..len {
        result_array.push(0);
    }
    let mut index = 0;
    while index < len {
        while index >= (r + 1)
            && index + (r + 1) < len
            && expand_array[index - (r + 1)] == expand_array[index + (r + 1)]
        {
            r += 1;
        }
        let center_index = index;
        result_array[center_index] = r;
        index += 1;

        let left_border = center_index - r;
        let right_border = center_index + r;
        r = 0;
        while index <= right_border && center_index >= index - center_index {
            let left_center_index = center_index - (index - center_index);
            let left = left_center_index - result_array[left_center_index];
            match left.cmp(&left_border) {
                Ordering::Greater => result_array[index] = result_array[left_center_index],
                Ordering::Less => result_array[index] = right_border - index,
                Ordering::Equal => {
                    r = right_border - index;
                    break;
                }
            }
            index += 1;
        }
    }
    let mut max_r = 0;
    let mut center_index = 0;
    for (i, r) in result_array.iter().enumerate() {
        if *r > max_r {
            max_r = *r;
            center_index = i;
        }
    }
    let mut result = String::new();
    for i in (center_index - max_r..=center_index + max_r) {
        if expand_array[i] != '$' {
            result.push(expand_array[i]);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::leetcode_005::longest_palindrome;

    #[test]
    fn test() {
        println!("{}", longest_palindrome("babad".to_string()));
        println!("{}", longest_palindrome("cbbd".to_string()));
    }
}
