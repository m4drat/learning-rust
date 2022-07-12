fn add(x: i32, y: i32) -> i32 {
    if x == 7 {
        0
    } else {
        x + y
    }
}

#[cfg(test)]
mod tests {
    // Allows to test private functions
    use super::*;

    #[test]
    fn test_1_1() {
        assert_eq!(add(1, 1), 2);
    }
    
    #[test]
    fn test_1_2() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_7_1() {
        assert_eq!(add(7, 1), 8);
    }
}

fn main() {
}
