#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larger_rectangle_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 8,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_smaller_rectangle_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_returns_area_of_rectangle() {
        let rectangle = Rectangle {
            width: 10,
            height: 8,
        };

        assert_eq!(rectangle.area(), 80);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Baggy");

        assert!(
            result.contains("Baggy"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
