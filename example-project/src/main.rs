use crate::math::{add::add, sub::sub};
pub mod math;

struct ComplexRef<'a>
{
    first_ref: &'a i32,
    second_ref: &'a i32
}

fn main() {
    println!("1 + 2 = {}", add(1, 2));
    println!("1 - 2 = {}", sub(1, 2));

    let a: i32 = 37;
    let b: i32 = 13;
    
    let something: ComplexRef = ComplexRef {
        first_ref: &a,
        second_ref: &b
    };
    
    let result;
    {
        let something_new = something;
        result = func(&a, &something_new);
    }
    println!("{}", result);
}

fn func<'a, 'b>(a: &'a i32, cr: &'b ComplexRef<'a>) -> &'a i32 {
    if *a < *cr.second_ref {
        a
    } else {
        cr.second_ref
    }
}