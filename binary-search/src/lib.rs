pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Less => left = mid + 1,
        }
    }

    None
}
