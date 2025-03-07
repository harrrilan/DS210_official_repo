use std::ops::Neg;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: Copy + Neg<Output = T>> Point<T> {
    fn clockwise(&self) -> Point<T> {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    fn counterclockwise(&self) -> Point<T> {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.0f32, y: 2.0f32 };
    let p1_rotated = p1.clockwise();
    println!(
        "f32 point ({}, {}) rotated clockwise: ({}, {})",
        p1.x, p1.y, p1_rotated.x, p1_rotated.y
    );

    let p2 = Point { x: 3i64, y: 4i64 };
    let p2_rotated = p2.counterclockwise();
    println!(
        "i64 point ({}, {}) rotated counterclockwise: ({}, {})",
        p2.x, p2.y, p2_rotated.x, p2_rotated.y
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clockwise_int() {
        let p = Point { x: 3, y: 4 };
        let rotated = p.clockwise();
        assert_eq!(rotated.x, 4);
        assert_eq!(rotated.y, -3);
    }

    #[test]
    fn test_counterclockwise_int() {
        let p = Point { x: 3, y: 4 };
        let rotated = p.counterclockwise();
        assert_eq!(rotated.x, -4);
        assert_eq!(rotated.y, 3);
    }

    #[test]
    fn test_clockwise_float() {
        let p = Point {
            x: 1.0f32,
            y: 0.0f32,
        };
        let rotated = p.clockwise();
        assert_eq!(rotated.x, 0.0f32);
        assert_eq!(rotated.y, -1.0f32);
    }

    #[test]
    fn test_counterclockwise_float() {
        let p = Point {
            x: 0.0f32,
            y: 1.0f32,
        };
        let rotated = p.counterclockwise();
        assert_eq!(rotated.x, -1.0f32);
        assert_eq!(rotated.y, 0.0f32);
    }
}