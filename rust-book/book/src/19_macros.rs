#![allow(unused_variables)]
#![allow(dead_code)]

// ** Declarative Macros with macro_rules! for General Metaprogramming ** //
#[macro_export]
macro_rules! vector {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vector!(123, 12);
}