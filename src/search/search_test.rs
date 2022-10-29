use std::time::Instant;
use crate::helper;

struct Test {
    test1: String,
    test2: i32,
    test3: f64,
}

impl Eq for Test {}

impl PartialEq for Test {
    fn eq(&self, other: &Self) -> bool {
        self.test1 == other.test1 && self.test2 == other.test2 && (self.test3 - other.test3).abs() < 1e-5
    }
}

pub trait Search {
    fn search<T: Eq>(elements: &[T], element: &T) -> isize;
}

pub(crate) trait SearchTest: Search {
    /// Computational complexity
    fn o() -> String;

    fn test_search() {
        Self::test_search_int();
        Self::test_search_string();
        Self::test_search_struct();
        Self::test_search_benchmark();
    }

    fn test_search_int() {
        let arr = [3, 4, 8, 100, 11];
        assert_eq!(2, Self::search(&arr, &8));
        assert_eq!(-1, Self::search(&arr, &7));
    }

    fn test_search_string() {
        let arr1 = ["ask", "me", "what", "is"];
        assert_eq!(3, Self::search(&arr1, &"is"));
        assert_eq!(-1, Self::search(&arr1, &"test"));
    }

    fn test_search_struct() {
        let arr2 = [
            Test {
                test1: String::from("test"),
                test2: 2,
                test3: 0.0,
            },
            Test {
                test1: String::from("test1"),
                test2: 3,
                test3: 1.0,
            },
        ];
        assert_eq!(1, Self::search(&arr2, &Test {
            test1: String::from("test1"),
            test2: 3,
            test3: 1.0,
        }));
        assert_eq!(-1, Self::search(&arr2, &Test {
            test1: String::from("test1"),
            test2: 3,
            test3: 1.001,
        }));
    }

    fn test_search_benchmark() {
        // get type name
        let type_name = helper::split_and_last_item(std::any::type_name::<Self>());

        let array = helper::generate_ordered_int_array::<100000>();
        let now = Instant::now();
        Self::search(&array, &(100000 - 1));
        println!("{type_name}, {}, n = 100000, elapsed: {} s", Self::o(),
                 now.elapsed().as_millis() as f64 / 1000.0 / 3 as f64);

        let array = helper::generate_ordered_int_array::<200000>();
        let now = Instant::now();
        Self::search(&array, &(200000 - 1));
        println!("{type_name}, {}, n = 200000, elapsed: {} s", Self::o(),
                 now.elapsed().as_millis() as f64 / 1000.0 / 3 as f64);
    }
}