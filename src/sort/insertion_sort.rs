use crate::sort::sort_test::{Sort};

struct InsertionSort {
    _private: (),
}

impl Sort for InsertionSort {
    fn sort<T: Ord>(elements: &mut [T]) {
        if elements.is_empty() {
            return;
        }
        for i in 0..elements.len() {
            let mut j = i;
            while j > 0 && &elements[j] < &elements[j - 1] {
                elements.swap(j - 1, j);
                j = j - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::insertion_sort::InsertionSort;
    use crate::sort::sort_test::SortTest;

    impl SortTest for InsertionSort {
        fn o() -> String {
            "O(n^2)".to_string()
        }
    }

    #[test]
    fn test_sort() {
        InsertionSort::test_sort();
    }
}
