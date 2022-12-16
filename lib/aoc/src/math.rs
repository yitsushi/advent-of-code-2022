use num::PrimInt;

pub fn abs<T: PrimInt>(value: T) -> T {
    if value < T::zero() { T::zero() - value } else { value }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + std::fmt::Debug> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - T::one(), self.y),
            Self::new(self.x + T::one(), self.y),
            Self::new(self.x, self.y - T::one()),
            Self::new(self.x, self.y + T::one()),
        ]
    }

    pub fn manhattan_to(&self, other: &Self) -> T {
        abs(self.x - other.x) + abs(self.y - other.y)
    }

    pub fn closest(&self, others: &[Vec2D<T>]) -> Vec2D<T> {
        let mut others = others.iter()
            .map(|v| {
                (self.manhattan_to(v), v.clone())
            })
            .collect::<Vec<(T, Self)>>();
        others.sort_by(|(d1, _), (d2, _)| d1.cmp(d2));
        others.remove(0).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors() {
        let input: Vec2D<i64> = Vec2D::new(4, 8);
        let expected: Vec<Vec2D<i64>> = vec![
            Vec2D::new(3, 8),
            Vec2D::new(5, 8),
            Vec2D::new(4, 7),
            Vec2D::new(4, 9),
        ];

        assert_eq!(input.neighbors(), expected)
    }

    #[test]
    fn manhattan() {
        let cases: Vec<(Vec2D<i64>, Vec2D<i64>, i64)> = vec![
            (Vec2D::new(1, 1), Vec2D::new(2, 2), 2),
            (Vec2D::new(1, 1), Vec2D::new(1, 5), 4),
            (Vec2D::new(1, 5), Vec2D::new(1, 1), 4),
            (Vec2D::new(1, 1), Vec2D::new(5, 1), 4),
            (Vec2D::new(5, 1), Vec2D::new(1, 1), 4),
            (Vec2D::new(1, 1), Vec2D::new(5, 5), 8),
            (Vec2D::new(5, 5), Vec2D::new(1, 1), 8),
            (Vec2D::new(-1, -1), Vec2D::new(1, 1), 4),
            (Vec2D::new(10, 10), Vec2D::new(5, 19), 14),
            (Vec2D::new(10, 10), Vec2D::new(3, 4), 13),
        ];

        for case in cases {
            assert_eq!(case.0.manhattan_to(&case.1), case.2);
        }
    }

    #[test]
    fn closest() {
        let base: Vec2D<i64> = Vec2D::new(10, 10);
        let input: Vec<Vec2D<i64>> = vec![
            Vec2D::new(1, 4),
            Vec2D::new(2, 1),
            Vec2D::new(-3, 6),
            Vec2D::new(3, 4),
            Vec2D::new(20, 15),
            Vec2D::new(5, 19),
            Vec2D::new(5, 19),
        ];
        let expected: Vec2D<i64> = Vec2D::new(3, 4);

        assert_eq!(base.closest(&input), expected);
    }
}
