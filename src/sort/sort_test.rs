use rand::Rng;
use std::time::Instant;

const TEST_COUNT: usize = 3;
const ELEMENT_COUNT: usize = 10000;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct TestStruct {
    name: String,
}

pub trait Sort {
    fn sort<T: Ord>(elements: &mut [T]);
}

pub(crate) trait SortTest: Sort {
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
        let mut array = [0; ELEMENT_COUNT];
        let mut rng = rand::thread_rng();
        for e in array.iter_mut() {
            *e = rng.gen()
        }
        let now = Instant::now();
        for _ in 0..TEST_COUNT {
            Self::sort(&mut array)
        }
        println!("{} sort elapsed: {} s", std::any::type_name::<Self>(), now.elapsed().as_millis() as f64 / 1000.0 / TEST_COUNT as f64);
    }
}