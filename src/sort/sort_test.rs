use std::time::Instant;
use crate::helper;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct TestStruct {
    name: String,
}

pub trait Sort {
    fn sort<T: Ord>(elements: &mut [T]);
}

pub(crate) trait SortTest: Sort {
    /// Computational complexity
    fn o() -> String;

    fn test_sort() {
        Self::test_sort_int();
        Self::test_sort_string();
        Self::test_sort_struct();
        Self::test_sort_benchmark();
    }

    fn test_sort_int() {
        let mut array = [3, 5, 7, 9, 0, 102, 11];
        Self::sort(&mut array);
        assert_eq!([0, 3, 5, 7, 9, 11, 102], array);
    }

    fn test_sort_string() {
        let mut array = ["mammal", "much", "crash", "draw", "mom", "cup", "sick", "piano",
            "job", "canyon", "rack", "doll", "tennis", "april", "flame"];
        Self::sort(&mut array);
        assert_eq!(["april", "canyon", "crash", "cup", "doll", "draw", "flame", "job", "mammal",
                       "mom", "much", "piano", "rack", "sick", "tennis"], array)
    }

    fn test_sort_struct() {
        let mut array = [
            TestStruct { name: "mammal".to_string() },
            TestStruct { name: "much".to_string() },
            TestStruct { name: "crash".to_string() },
            TestStruct { name: "draw".to_string() },
            TestStruct { name: "mom".to_string() },
            TestStruct { name: "cup".to_string() },
            TestStruct { name: "sick".to_string() },
            TestStruct { name: "piano".to_string() },
            TestStruct { name: "job".to_string() },
            TestStruct { name: "canyon".to_string() },
            TestStruct { name: "rack".to_string() },
            TestStruct { name: "doll".to_string() },
            TestStruct { name: "tennis".to_string() },
            TestStruct { name: "april".to_string() },
            TestStruct { name: "flame".to_string() },
        ];
        Self::sort(&mut array);
    }

    fn test_sort_benchmark() {
        // get type name
        let type_name = helper::split_and_last_item(std::any::type_name::<Self>());

        let now = Instant::now();
        Self::sort(&mut helper::generate_random_int_array::<10000>());
        println!("{type_name}, {}, n = 10000, elapsed: {} s", Self::o(), now.elapsed().as_millis() as f64 / 1000.0);

        let now = Instant::now();
        Self::sort(&mut helper::generate_random_int_array::<20000>());
        println!("{type_name}, {}, n = 20000, elapsed: {} s", Self::o(), now.elapsed().as_millis() as f64 / 1000.0);
    }
}