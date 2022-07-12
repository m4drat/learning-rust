#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // ** Creating Type Synonyms with Type Aliases ** //
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // A type alias makes this code more manageable by reducing the repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| ())
    }

    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
    use std::fmt;

    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
    
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    // ** The Never Type that Never Returns ** //
    fn bar() -> ! {
        panic!();
    }

    loop {
        let guess: u32 = match "123".trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }

    // ** Dynamically Sized Types and the Sized Trait ** //
    /// A trait bound on ?Sized means “T may or may not be Sized” and this notation
    /// overrides the default that generic types must have a known size at compile time.
    /// The ?Trait syntax with this meaning is only available for Sized, not any other traits.
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }

}