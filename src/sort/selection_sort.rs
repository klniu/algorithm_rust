use crate::sort::sort_test::{Sort};

struct SelectionSort {}

impl Sort for SelectionSort {
    fn sort<T: Ord>(elements: &mut [T]) {
        let count = elements.len();
        for i in 0..count {
            for j in i..count {
                if &elements[i] > &elements[j] {
                    elements.swap(i, j);
                }
            }
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
