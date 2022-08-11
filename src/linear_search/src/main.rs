fn main() {
    let arr = [3, 4, 8, 100, 11];
    assert_eq!(2, linear_search(&arr, &8));
    assert_eq!(-1, linear_search(&arr, &7));

    let arr1 = ["ask", "me", "what", "is"];
    assert_eq!(3, linear_search(&arr1, &"is"));
    assert_eq!(-1, linear_search(&arr1, &"test"));

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
    assert_eq!(1, linear_search(&arr2, &Test {
        test1: String::from("test1"),
        test2: 3,
        test3: 1.0,
    }));
    assert_eq!(-1, linear_search(&arr2, &Test {
        test1: String::from("test1"),
        test2: 3,
        test3: 1.001,
    }));
}

struct Test {
    test1: String,
    test2: i32,
    test3: f64,
}

impl Eq for Test{

}

impl PartialEq for Test {
    fn eq(&self, other: &Self) -> bool {
        self.test1 == other.test1 && self.test2 == other.test2 && f64::abs(self.test3 - other.test3) < 1e-5
    }
}

fn linear_search<T: Eq>(elements: &[T], element: &T) -> isize {
    for (index, e) in elements.iter().enumerate() {
        if e == element {
            return index as isize;
        }
    }
    -1
}
