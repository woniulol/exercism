pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] > key {
            if mid == 0 {
                return None;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    if left == right {
        if array[left] == key {
            return Some(left);
        }
    }

    None
}
