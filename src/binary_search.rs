pub fn binary_search(arr: &[i32], item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        let guess = arr[mid];

        if guess == item {
            return Some(mid);
        } else if guess > item {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_element_in_middle() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], 5), Some(2));
    }

    #[test]
    fn finds_first_element() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], 1), Some(0));
    }

    #[test]
    fn finds_last_element() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], 9), Some(4));
    }

    #[test]
    fn none_when_smaller_than_all() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], -1), None);
    }

    #[test]
    fn none_when_larger_than_all() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], 100), None);
    }

    #[test]
    fn none_for_gap_between_elements() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], 4), None);
    }

    #[test]
    fn handles_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, 1), None);
    }

    #[test]
    fn single_element_found() {
        assert_eq!(binary_search(&[42], 42), Some(0));
    }

    #[test]
    fn single_element_not_found() {
        assert_eq!(binary_search(&[42], 1), None);
    }
}