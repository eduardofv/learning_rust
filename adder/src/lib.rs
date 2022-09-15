#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(value: usize) -> usize {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_can_hold() {
        let larger = Rect{ width: 8, height: 8 };
        let smaller = Rect{ width: 5, height: 5 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn rect_can_not_hold() {
        let larger = Rect{ width: 8, height: 8 };
        let smaller = Rect{ width: 5, height: 5 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

/*
    #[test]
    fn another() {
        panic!("Error");
    }
*/
}
