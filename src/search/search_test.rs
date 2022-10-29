use std::time::Instant;

const TEST_COUNT: usize = 3;
const ELEMENT_COUNT: usize = 100000;

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
        let mut array = [0; ELEMENT_COUNT];
        for i in 0..TEST_COUNT {
            array[i] = i;
        }
        let now = Instant::now();
        for _ in 0..TEST_COUNT {
            Self::search(&array, &(ELEMENT_COUNT - 1));
        }
        println!("{} do search elapsed: {} s",
                 std::any::type_name::<Self>(), now.elapsed().as_millis() as f64 / 1000.0 / TEST_COUNT as f64);
    }
}