use rand::Rng;

// create ordered int array with N items.
pub fn generate_ordered_int_array<const N: usize>() -> [i32; N] {
    let mut array = [0; N];
    for i in 0..N {
        array[i] = i as i32;
    }
    array
}

// create random int array with N items.
pub fn generate_random_int_array<const N: usize>() -> [i32; N] {
    let mut array = [0; N];
    let mut rng = rand::thread_rng();
    for e in array.iter_mut() {
        *e = rng.gen()
    }
    array
}

/// split with delimiter and return the last item
pub fn split_and_last_item(str: &str) -> String {
    str.rsplit_once(":").unwrap().1.to_owned()
}

