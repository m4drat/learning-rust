fn main() {
    /// Dereferencing a Raw Pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    /// Calling an Unsafe Function or Method
    unsafe fn dangerous() {
        let heap_str = Box::new("Hello from unsafe Rust!".to_string());
        let heap_ptr = Box::into_raw(heap_str);

        println!("Raw pointer: {:p}", heap_ptr);

        // when you want managed memory again:
        let box_again = Box::from_raw(heap_ptr);
        drop(box_again);
    }
    unsafe { dangerous() };

    /// Creating a Safe Abstraction over Unsafe Code
    use std::slice;
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    dbg!(a);
    dbg!(b);

    /// Using extern Functions to Call External Code
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    /// Calling Rust Functions from Other Languages
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    /// Accessing or Modifying a Mutable Static Variable
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    /// Implementing an Unsafe Trait
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    /// 1. Accessing Fields of a Union
    union MyUnion { f1: u32, f2: f32 }

    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };

    /// 2. Accessing Fields of a Union
    #[repr(u32)]
    enum Tag { I, F }

    #[repr(C)]
    union U {
        i: i32,
        f: f32,
    }

    #[repr(C)]
    struct Value {
        tag: Tag,
        u: U,
    }

    fn is_zero(v: Value) -> bool {
        unsafe {
            match v {
                Value { tag: Tag::I, u: U { i: 0 } } => true,
                Value { tag: Tag::F, u: U { f: num } } if num == 0.0 => true,
                _ => false,
            }
        }
    }
}
