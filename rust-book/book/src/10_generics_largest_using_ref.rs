use num_traits::Float;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    &largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
    point_name: String
}

impl<T: Float> Point<T> {
    fn distance_from_origin(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.point_name == other.point_name
    }

    fn ne(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.point_name == other.point_name
    }
}

impl<T: PartialEq + Float> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p1_distance = self.distance_from_origin();
        let p2_distance = other.distance_from_origin();

        if p1_distance < p2_distance {
            return Some(core::cmp::Ordering::Less);
        } else if p1_distance > p2_distance {
            return Some(core::cmp::Ordering::Greater);
        } else {
            return Some(core::cmp::Ordering::Equal);
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

    let points_list = vec![
        Point{x: 9., y: 9., point_name: "Third".to_string()},
        Point{x: 12., y: 5., point_name: "Fourth".to_string()},
        Point{x: 1., y: 1., point_name: "First".to_string()},
        Point{x: 2., y: 2., point_name: "Second".to_string()}
    ];
    let result = largest(&points_list);
    println!("The largest point is {:#?}", result);
}