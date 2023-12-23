use std::num::Wrapping;

pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let digit_num = num.ilog10() + 1;
    (0..digit_num)
        .map(|n| (num / 10_u32.pow(n) % 10).pow(digit_num))
        .try_fold(0, |acc, val| val.checked_add(acc))
        .is_some_and(|sum| sum == num)
}
