// ** Moving Captured Values Out of the Closure and the Fn Traits ** //
// FnOnce - applies to closures that can be called at least once. All closures implement this trait, because all closures can be called. If a closure moves captured values out of its body, then that closure only implements FnOnce and not any of the other Fn traits, because it can only be called once.
// FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
// Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. Closures that don’t capture anything from their environment implement Fn.
enum MyOption<T> {
    Some(T),
    None
}

impl<T> MyOption<T> {
    pub fn _unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            MyOption::Some(x) => x,
            MyOption::None => f(),
        }
    }
}

// ** The Iterator Trait and the next Method ** //
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let _vector: Vec<Vec<i32>> = vec![vec![1337, 50]; 24];

    // ** Closures default syntax ** //
    let _add_one = |x: u32| -> u32 { x + 1 };

    // ** Iterators ** //
    let v1 = vec![1, 2, 3];
    // Iterators are lazy!
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // ** Methods that Produce Other Iterators ** //
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
