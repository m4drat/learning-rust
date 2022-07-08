use core::fmt::Debug;

// ** Generic data types ** //
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ** Traits ** //
pub trait Summary {
    fn summarize(&self) -> String;
    fn default_impl(&self) -> String {
        "Default method implementation".to_string()
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ** Traits as parameters ** //
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ** Trait Bound Syntax ** //
// Actually it's just a longer form of:
// pub fn notify(item: &impl Summary) { ... }
pub fn notify_long<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// ** Specifying Multiple Trait Bounds with the + Syntax ** //
pub fn notify_multiple_trait_bounds<T: Summary + std::fmt::Display>(item: &T) {}

// ** Clearer Trait Bounds with where Clauses ** //
// Instead of:
fn some_function_bulky<T: std::fmt::Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
// Use:
fn some_function_concise<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + Debug,
{
    0
}

// ** Returning Types that Implement Traits ** //
// You can only use impl Trait if you’re returning a single type.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// ** Using Trait Bounds to Conditionally Implement Methods ** //
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{}", float.distance_from_origin());
}
