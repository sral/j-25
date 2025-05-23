use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    #[allow(dead_code)]
    pub fn dot(self, other: Vec2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[allow(dead_code)]
    pub fn scale(self, factor: f32) -> Vec2d {
        Vec2d {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    
    #[allow(dead_code)]
    pub fn normalize(&self) -> Vec2d {
        let m = 1.0 / self.magnitude();
        self.scale(m)
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let expected = Vec2d { x: 2.0, y: 4.0 };

        let first = Vec2d { x: 1.0, y: 1.0 };
        let second = Vec2d { x: 1.0, y: 3.0 };

        assert_eq!(expected, first + second);
    }

    #[test]
    fn test_sub() {
        let expected = Vec2d { x: 0.0, y: -2.0 };

        let first = Vec2d { x: 1.0, y: 1.0 };
        let second = Vec2d { x: 1.0, y: 3.0 };

        assert_eq!(expected, first - second);
    }

    #[test]
    fn test_dot() {
        let first = Vec2d { x: 2.0, y: 3.0 };
        let second = Vec2d { x: 4.0, y: 5.0 };

        assert_eq!(first.dot(second), 2.0 * 4.0 + 3.0 * 5.0);
    }

    #[test]
    fn test_magnitude() {
        let a: f32 = 3.0;
        let b: f32 = 2.0;
        let vec = Vec2d { x: a, y: b };

        assert_eq!((a * a + b * b).sqrt(), vec.magnitude());
    }

    #[test]
    fn test_scale() {
        let a: f32 = 3.0;
        let b: f32 = 2.0;
        let c: f32 = 2.0;
        let vec = Vec2d { x: a, y: b };

        assert_eq!(Vec2d { x: a * c, y: b * c }, vec.scale(c));
    }

    #[test]
    fn test_normalize() {
        let epsilon = 0.00001;
        let vectors = vec![
            Vec2d { x: 0.5, y: 0.5 },
            Vec2d { x: 1.0, y: 3.2 },
            Vec2d { x: -2.2, y: 1.0 },
            Vec2d { x: -1.5, y: -1.5 },
            Vec2d { x: 5.0, y: 5.0 },
            Vec2d { x: 1.0, y: 1.0 },
        ];

        for v in &vectors {
            let m = v.normalize().magnitude();
            let rel_err = (1.0 - m) / m;
            assert!(epsilon > rel_err);
        }
    }
}
