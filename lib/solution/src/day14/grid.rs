use std::collections::HashMap;
use aoc::grid_helper;
use aoc::grid_helper::min_max_coordinates;
use aoc::math::Vec2D;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum Cell {
    #[default] Empty,
    Wall,
    Sand,
    Source,
}

#[allow(dead_code)]
impl Cell {
    pub fn is_empty(&self) -> bool { *self == Self::Empty }
    pub fn is_source(&self) -> bool { *self == Self::Source }
    pub fn is_sand(&self) -> bool { *self == Self::Sand }
    pub fn is_wall(&self) -> bool { *self == Self::Wall }
}

#[derive(Debug)]
pub struct Grid {
    cells: HashMap<Vec2D<i64>, Cell>,
    pub source: Vec2D<i64>,
    pub min: Vec2D<i64>,
    pub max: Vec2D<i64>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cells: Default::default(),
            source: Vec2D::default(),
            min: Vec2D::new(i64::MAX, i64::MAX),
            max: Vec2D::new(i64::MIN, i64::MIN),
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.cells.keys().cloned().collect::<Vec<Vec2D<i64>>>();
        let (min_coordinate, max_coordinate) = grid_helper::min_max_coordinates(list);

        let lines = grid_helper::display_i64(min_coordinate, max_coordinate, |pos| {
            match self.cells.get(&pos) {
                Some(Cell::Wall) => '#',
                Some(Cell::Sand) => 'o',
                Some(Cell::Source) => '+',
                _ => '.',
            }
        });

        write!(f, "{}", lines.join("\n"))
    }
}

impl Grid {
    fn update_min_max(&mut self, positions: Vec<Vec2D<i64>>) {
        let (min, max) = grid_helper::min_max_coordinates(positions);
        self.min = grid_helper::min_coordinates(self.min.clone(), min);
        self.max = grid_helper::max_coordinates(self.max.clone(), max);
    }

    pub fn add_source(&mut self, pos: Vec2D<i64>) {
        self.cells.insert(pos.clone(), Cell::Source);
        self.update_min_max(vec![pos.clone()]);
        self.source = pos;
    }

    pub fn add_sand(&mut self, pos: Vec2D<i64>) {
        self.cells.insert(pos, Cell::Sand);
    }

    pub fn add_wall(&mut self, from: Vec2D<i64>, to: Vec2D<i64>) {
        let (min, max) = min_max_coordinates(vec![from, to]);
        let mut counter = 0;
        (min.x..=max.x).for_each(|x| {
            (min.y..=max.y).for_each(|y| {
                self.cells.insert(Vec2D::new(x, y), Cell::Wall);
                counter += 1;
            })
        });

        self.update_min_max(vec![min, max]);
    }

    pub fn contains(&self, pos: &Vec2D<i64>) -> bool {
        (self.min.x..=self.max.x).contains(&pos.x)
            && (self.min.y..=self.max.y).contains(&pos.y)
    }

    pub fn get(&self, pos: Vec2D<i64>) -> Cell {
        self.cells.get(&pos).unwrap_or(&Cell::Empty).clone()
    }

    pub fn cycle(&mut self) -> bool {
        let mut sand = self.source.clone();

        if !self.get(sand.clone()).is_source() {
            return false
        }

        loop {
            if !self.contains(&sand) {
                return false
            }

            let below = self.get(Vec2D::new(sand.x, sand.y + 1));
            let left = self.get(Vec2D::new(sand.x - 1, sand.y + 1));
            let right = self.get(Vec2D::new(sand.x + 1, sand.y + 1));

            if !below.is_empty() && !left.is_empty() && !right.is_empty() {
                self.add_sand(sand);

                return true
            }

            sand.y += 1;

            if below.is_empty() {
                continue
            }

            if left.is_empty() {
                sand.x -= 1;
                continue;
            }

            if right.is_empty() {
                sand.x += 1;
                continue;
            }

            unreachable!();
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::math::Vec2D;

    #[test]
    fn add_wall() {
        let mut grid = super::Grid::default();
        grid.add_wall(Vec2D::new(-2, -2), Vec2D::new(-2, 2));
        grid.add_wall(Vec2D::new(-2, 2), Vec2D::new(3, 2));

        assert_eq!(grid.min, Vec2D::new(-2, -2));
        assert_eq!(grid.max, Vec2D::new(3, 2));
        assert!(grid.get(Vec2D::new(1, 2)).is_wall());
        assert!(grid.get(Vec2D::new(5, 5)).is_empty());
        assert!(grid.contains(&Vec2D::new(1, 1)));
        assert!(!grid.contains(&Vec2D::new(5, 5)));
    }

    #[test]
    fn add_sand() {
        let mut grid = super::Grid::default();
        grid.add_wall(Vec2D::new(-2, -2), Vec2D::new(-2, 2));
        grid.add_wall(Vec2D::new(-2, 2), Vec2D::new(3, 2));
        grid.add_sand(Vec2D::new(1, 1));
        grid.add_sand(Vec2D::new(-1, -1));

        assert_eq!(grid.min, Vec2D::new(-2, -2));
        assert_eq!(grid.max, Vec2D::new(3, 2));
        assert!(grid.get(Vec2D::new(3, 2)).is_wall());
        assert!(grid.get(Vec2D::new(5, 5)).is_empty());
        assert!(grid.get(Vec2D::new(1, 1)).is_sand());
        assert!(grid.contains(&Vec2D::new(1, 1)));
        assert!(!grid.contains(&Vec2D::new(5, 5)));
    }

    #[test]
    fn cycle() {
        let mut grid = super::Grid::default();
        grid.add_wall(Vec2D::new(2, 2), Vec2D::new(2, 5));
        grid.add_wall(Vec2D::new(2, 5), Vec2D::new(6, 5));
        grid.add_source(Vec2D::new(4, 0));

        assert_eq!(grid.min, Vec2D::new(2, 0));
        assert_eq!(grid.max, Vec2D::new(6, 5));
        assert!(grid.get(Vec2D::new(2, 3)).is_wall());
        assert!(grid.get(Vec2D::new(4, 2)).is_empty());
        assert!(grid.contains(&Vec2D::new(3, 1)));
        assert!(!grid.contains(&Vec2D::new(10, 10)));

        assert!(grid.cycle());
        assert!(grid.get(Vec2D::new(4, 4)).is_sand());
        assert!(grid.get(Vec2D::new(3, 4)).is_empty());
        assert!(grid.get(Vec2D::new(5, 4)).is_empty());
        assert!(grid.get(Vec2D::new(4, 3)).is_empty());

        assert!(grid.cycle());
        assert!(grid.get(Vec2D::new(4, 4)).is_sand());
        assert!(grid.get(Vec2D::new(3, 4)).is_sand());
        assert!(grid.get(Vec2D::new(5, 4)).is_empty());
        assert!(grid.get(Vec2D::new(4, 3)).is_empty());

        assert!(grid.cycle());
        assert!(grid.get(Vec2D::new(4, 4)).is_sand());
        assert!(grid.get(Vec2D::new(3, 4)).is_sand());
        assert!(grid.get(Vec2D::new(5, 4)).is_sand());
        assert!(grid.get(Vec2D::new(4, 3)).is_empty());

        assert!(grid.cycle());
        assert!(grid.get(Vec2D::new(4, 4)).is_sand());
        assert!(grid.get(Vec2D::new(3, 4)).is_sand());
        assert!(grid.get(Vec2D::new(5, 4)).is_sand());
        assert!(grid.get(Vec2D::new(4, 3)).is_sand());
    }

    #[test]
    fn display() {
        let mut grid = super::Grid::default();
        grid.add_wall(Vec2D::new(2, 2), Vec2D::new(2, 5));
        grid.add_wall(Vec2D::new(2, 5), Vec2D::new(6, 5));
        grid.add_source(Vec2D::new(4, 0));

        while grid.cycle() {}

        let expected = vec![
            "..+..",
            ".....",
            "#....",
            "#oo..",
            "#ooo.",
            "#####",
        ].join("\n").to_string();

        assert_eq!(format!("{}", grid), expected);
    }
}