struct ComplexRef<'a>
{
    first_ref: &'a i32,
    second_ref: &'a i32
}

pub fn module_main() {
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