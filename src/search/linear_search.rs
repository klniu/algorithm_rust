use super::search_test::Search;

struct LinearSearch {
    _private: ()
}

impl Search for LinearSearch {
    fn search<T: Eq>(elements: &[T], element: &T) -> isize {
        for (index, e) in elements.iter().enumerate() {
            if e == element {
                return index as isize;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::search::linear_search::LinearSearch;
    use crate::search::search_test::SearchTest;

    impl SearchTest for LinearSearch {}

    #[test]
    fn test_search() {
        LinearSearch::test_search();
    }
}