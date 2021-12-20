// copied over from day 7

fn merge_sort(array: &mut Vec<usize>) {
    let mut helper = vec![0; array.len()];
    merge_sort_helper(array, &mut helper, 0, array.len() - 1);
}

fn merge_sort_helper(array: &mut Vec<usize>, helper: &mut Vec<usize>, low: usize, high: usize) {
    if low < high {
        let middle = (low + high) / 2;
        merge_sort_helper(array, helper, low, middle);
        merge_sort_helper(array, helper, middle + 1, high);
        merge(array, helper, low, middle, high);
    }
}

fn merge(array: &mut Vec<usize>, helper: &mut Vec<usize>, low: usize, middle: usize, high: usize) {
    for i in low..high + 1 {
        helper[i] = array[i];
    }
    let mut helper_left = low;
    let mut helper_right = middle + 1;
    let mut current = low;

    while helper_left <= middle && helper_right <= high {
        if helper[helper_left] <= helper[helper_right] {
            array[current] = helper[helper_left];
            helper_left += 1;
        } else {
            array[current] = helper[helper_right];
            helper_right += 1;
        }
        current += 1;
    }

    if helper_left <= middle {
        let remaining = middle - helper_left;
        for i in 0..remaining + 1 {
            array[current + i] = helper[helper_left + i];
        }
    }
}

// array is a terrible varible name
pub fn median(array: &mut Vec<usize>) -> usize {
    merge_sort(array);
    if array.len() % 2 != 0 {
        array[array.len() / 2]
    } else {
        let v1 = array[array.len() / 2 - 1];
        let v2 = array[array.len() / 2];
        (v1 + v2) / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    #[test]
    fn merge_sort_works() {
        let mut array = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        merge_sort(&mut array);
        assert_eq!(array, [0, 1, 1, 2, 2, 2, 4, 7, 14, 16]);
    }

    #[test]
    fn merge_works_tiny() {
        let mut array = vec![1, 3, 2];
        merge(&mut array, &mut vec![0, 0, 0], 0, 1, 2);
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn find_median_works() {
        let mut array = vec![0, 9, 56, 2, 3, 2, 2, 6, 0];
        assert_eq!(median(&mut array), 2);
    }

    #[test]
    fn find_median_works_even() {
        let mut array = vec![0, 9, 56, 2, 5, 2, 2, 6];
        assert_eq!(median(&mut array), 3);
    }
}