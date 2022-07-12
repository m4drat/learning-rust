// ** Smart pointers ** //
// Default smart pointers:
// Box<<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

struct MyBox<T>(T)
where
    T: std::fmt::Display;

impl<T> MyBox<T>
where
    T: std::fmt::Display,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: std::fmt::Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

    // DerefMut
}

impl<T> Drop for MyBox<T>
where
    T: std::fmt::Display,
{
    fn drop(&mut self) {
        println!("Dropping MyBox<T> with data `{}`!", self.0);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // ** Using Box<T> to Point to Data on the Heap ** //
    let heap_allocated = Box::new(5);
    println!("b = {}", heap_allocated);

    let list = ListBox::Cons(
        1,
        Box::new(ListBox::Cons(
            2,
            Box::new(ListBox::Cons(3, Box::new(ListBox::Nil))),
        )),
    );
    println!("{:?}", list);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // without deref coercion we would have to write: hello(&(*m)[..]);

    // Explicit destructor call
    drop(m);

    // ** Rc<T>, the Reference Counted Smart Pointer ** //
    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    println!("Total count of references to a = {}", Rc::strong_count(&a));
    let _b = ListRc::Cons(3, Rc::clone(&a));
    println!("Total count of references to a = {}", Rc::strong_count(&a));
    let _c = ListRc::Cons(4, Rc::clone(&a));
    println!("Total count of references to a = {}", Rc::strong_count(&a));
}

