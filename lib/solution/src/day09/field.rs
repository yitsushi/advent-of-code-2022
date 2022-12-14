use std::{ops::Sub, collections::HashSet};

use super::movement::Movement;

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self{ x, y }
    }
}

impl<T: Clone> Vec2D<T> {
    pub fn to_pair(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
}

impl std::ops::Sub<Vec2D<i32>> for Vec2D<i32> {
    type Output = Vec2D<i32>;

    fn sub(self, rhs: Vec2D<i32>) -> Self::Output {
        Vec2D{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    pub parts: Vec<Vec2D<i32>>,
    pub head: Vec2D<i32>,
    pub tail_history: HashSet<Vec2D<i32>>,
    pub head_history: HashSet<Vec2D<i32>>,
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_coordinate: Vec2D<i32> = Vec2D::default();
        let mut max_coordinate: Vec2D<i32> = Vec2D::default();

        for item in self.tail_history.iter() {
            if item.x < min_coordinate.x { min_coordinate.x = item.x }
            if item.y < min_coordinate.y { min_coordinate.y = item.y }
            if item.x > max_coordinate.x { max_coordinate.x = item.x }
            if item.y > max_coordinate.y { max_coordinate.y = item.y }
        }

        for item in self.head_history.iter() {
            if item.x < min_coordinate.x { min_coordinate.x = item.x }
            if item.y < min_coordinate.y { min_coordinate.y = item.y }
            if item.x > max_coordinate.x { max_coordinate.x = item.x }
            if item.y > max_coordinate.y { max_coordinate.y = item.y }
        }

        let cols = max_coordinate.x - min_coordinate.x + 1;
        let rows = max_coordinate.y - min_coordinate.y + 1;

        let map = self.tail_history.union(&self.head_history)
            .into_iter()
            .fold(vec!["."; (cols * rows).try_into().unwrap()], |m, c| {
                let x = c.x - min_coordinate.x;
                let y = c.y - min_coordinate.y;
                let idx: usize = match (y * cols + x).try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        panic!("usize error: {} * {} + {} = {} << c = {:?}; min = {:?}", y, cols, x, (y * cols + x), c, min_coordinate)
                    }
                };

                let mut m = m;

                m[idx] = if *c == self.head {
                    "H"
                } else if *c == self.tail() {
                    "T"
                } else if self.parts.contains(c) {
                    "+"
                } else {
                    "#"
                };

                m
            })
            .chunks(cols.try_into().unwrap())
            .map(|f| f.join(""))
            .fold(vec![], |output, line| {
                let mut output = output;
                output.reverse();
                output.push(line);
                output.reverse();
                output
            })
            .join("\n")
            ;

        write!(f, "{}", map)
    }
}

impl Rope {
    pub fn new(length: usize) -> Self {
        Self{
            parts: vec![Vec2D{x: 0, y: 0}; length],
            ..Self::default()
        }
    }

    pub fn tail(&self) -> Vec2D<i32> {
        self.parts.last().unwrap().clone()
    }

    pub fn map(self, m: Movement) -> Self {
        let head = match m {
            Movement::Up(v) => Vec2D::new(self.head.x, self.head.y + v),
            Movement::Down(v) => Vec2D::new(self.head.x, self.head.y - v),
            Movement::Left(v) => Vec2D::new(self.head.x - v, self.head.y),
            Movement::Right(v) => Vec2D::new(self.head.x + v, self.head.y),
        };

        let parts: Vec<Vec2D<i32>> = self.parts.iter().fold(vec![], |parts, part| {
            let head = if parts.is_empty() {
                head.clone()
            } else {
                parts.last().unwrap().clone()
            };
            let diff = head.sub(part.clone());
            let current = match diff.to_pair() {
                (x, y) if (-1..=1).contains(&x) && (-1..=1).contains(&y) => part.clone(),
                (0, y) if y > 1 => Vec2D::new(part.x, part.y + 1),
                (0, y) if y < -1 => Vec2D::new(part.x, part.y - 1),
                (x, 0) if x > 1 => Vec2D::new(part.x + 1, part.y),
                (x, 0) if x < -1 => Vec2D::new(part.x - 1, part.y),
                (x, y) => {
                    let (x, y): (i32, i32) = if x > 0 && y > 0 {
                        // NE
                        (1, 1)
                    } else if x < 0 && y < 0 {
                        // SW
                        (-1, -1)
                    } else if x > 0 && y < 0 {
                        // SE
                        (1, -1)
                    } else {
                        // NW
                        (-1, 1)
                    };

                    Vec2D::new(part.x +x, part.y + y)
                },
            };

            let mut parts = parts;
            parts.push(current);
            parts
        });

        // Head history.
        let mut head_history = self.head_history;

        if head_history.is_empty() {
            head_history.insert(head.clone());
        }

        head_history.insert(head.clone());

        // Tail history.
        let mut tail_history = self.tail_history;

        if tail_history.is_empty() {
            tail_history.insert(parts.last().unwrap().clone());
        }

        tail_history.insert(parts.last().unwrap().clone());

        Self {
            head,
            tail_history,
            head_history,
            parts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2d_sub() {
        let cases: Vec<(Vec2D<i32>, Vec2D<i32>, Vec2D<i32>)>= vec![
            (Vec2D::new(1,1), Vec2D::new(1,1), Vec2D::new(0,0)),
            (Vec2D::new(5,3), Vec2D::new(2,4), Vec2D::new(3,-1)),
            (Vec2D::new(-5,-3), Vec2D::new(-2,-4), Vec2D::new(-3,1)),
        ];

        for case in cases {
            assert_eq!(case.0 - case.1, case.2);
        }
    }

    #[test]
    fn rope_map() {
        let mut rope = Rope::new(1);
        let mut history = HashSet::new();

        let cases: Vec<(Movement, Vec2D<i32>, Vec2D<i32>)> = vec![
            (Movement::Up(1), Vec2D::new(0, 1), Vec2D::new(0, 0)),
            (Movement::Up(1), Vec2D::new(0, 2), Vec2D::new(0, 1)),
            (Movement::Right(1), Vec2D::new(1, 2), Vec2D::new(0, 1)),
            (Movement::Right(1), Vec2D::new(2, 2), Vec2D::new(1, 2)),
        ];

        for case in cases {
            history.insert(case.2.clone());
            rope = rope.map(case.0);
            assert_eq!(rope.head, case.1);
            assert_eq!(rope.tail(), case.2);
        }

        assert_eq!(rope.tail_history, history);
        assert_eq!(format!("{}", rope), "#TH\n#..\n#..".to_string());
    }

    #[test]
    fn rope_map_len_two() {
        let mut rope = Rope::new(2);
        let mut history = HashSet::new();

        let cases: Vec<(Movement, Vec2D<i32>, Vec2D<i32>)> = vec![
            (Movement::Up(1), Vec2D::new(0, 1), Vec2D::new(0, 0)),
            (Movement::Up(1), Vec2D::new(0, 2), Vec2D::new(0, 0)),
            (Movement::Right(1), Vec2D::new(1, 2), Vec2D::new(0, 0)),
            (Movement::Right(1), Vec2D::new(2, 2), Vec2D::new(1, 1)),
        ];

        for case in cases {
            history.insert(case.2.clone());
            rope = rope.map(case.0);
            assert_eq!(rope.head, case.1);
            assert_eq!(rope.tail(), case.2);
        }

        assert_eq!(rope.tail_history, history);
        assert_eq!(format!("{}", rope), "#+H\n#T.\n#..".to_string());
    }
}
