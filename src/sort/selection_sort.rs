use crate::sort::sort_test::{Sort};

struct SelectionSort {
    _private: (),
}

impl Sort for SelectionSort {
    fn sort<T: Ord>(elements: &mut [T]) {
        let count = elements.len();
        for i in 0..count {
            let mut mini_index = i;
            for j in i..count {
                if &elements[j] < &elements[mini_index] {
                    mini_index = j;
                }
            }
            elements.swap(i, mini_index);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::sort::selection_sort::SelectionSort;
    use crate::sort::sort_test::SortTest;

    impl SortTest for SelectionSort {}

    #[test]
    fn test_search() {
        SelectionSort::test_sort();
    }
}
