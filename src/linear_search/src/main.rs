fn main() {
    let arr = [3, 4, 8, 100, 11];
    assert_eq!(2, linear_search(&arr, &8));
    assert_eq!(-1, linear_search(&arr, &7));

    let arr1 = ["ask", "me", "what", "is"];
    assert_eq!(3, linear_search(&arr1, &"is"));
    assert_eq!(-1, linear_search(&arr1, &"test"));
}

#[derive(PartialEq)]
struct Test {
    test1: String,
    test2: str,
    test3: double
}

fn linear_search<T: Eq>(elements: &[T], element: &T) -> isize {
    for (index, e) in elements.iter().enumerate() {
        if e == element {
            return index as isize;
        }
    }
    -1
}
