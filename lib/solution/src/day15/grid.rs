use std::collections::HashMap;

use aoc::{math::Vec2D, grid_helper};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    #[default] Empty,
    Probe(Vec2D<i64>),
    Beacon,
}

#[derive(Clone)]
pub struct Probe {
    pub position: Vec2D<i64>,
    pub radius: i64,
}

pub struct Grid {
    cells: HashMap<Vec2D<i64>, Cell>,
    pub probes: Vec<Probe>,
    pub min: Vec2D<i64>,
    pub max: Vec2D<i64>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: HashMap::default(),
            probes: Vec::default(),
            min: Vec2D::new(i64::MAX, i64::MAX),
            max: Vec2D::new(i64::MIN, i64::MIN),
        }
    }
}

impl Grid {
    pub fn add_probe(&mut self, probe: Vec2D<i64>, beacon: Vec2D<i64>) {
        let distance = probe.manhattan_to(&beacon);
        let (min, max) = grid_helper::min_max_coordinates(
            vec![ probe.clone(), beacon.clone() ],
        );

        self.min = if self.min.x == i64::MAX {
            min
        } else {
            grid_helper::min_coordinates(self.min.clone(), min)
        };
        self.max = if self.max.x == i64::MIN {
            max
        } else {
            grid_helper::max_coordinates(self.max.clone(), max)
        };

        self.cells.insert(probe.clone(), Cell::Probe(beacon.clone()));
        self.cells.insert(beacon, Cell::Beacon);
        self.probes.push(Probe { position: probe, radius: distance });
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.cells.keys().cloned().collect::<Vec<Vec2D<i64>>>();
        let (min_coordinate, max_coordinate) = grid_helper::min_max_coordinates(list);

        let lines = grid_helper::display_i64(min_coordinate, max_coordinate, |pos| {
            match self.cells.get(&pos) {
                Some(Cell::Beacon) => 'B',
                Some(Cell::Probe(_)) => 'S',
                _ => '.',
            }
        });

        write!(f, "{}", lines.join("\n"))
    }
}

impl Grid {
    pub fn get(&self, pos: Vec2D<i64>) -> Cell {
        self.cells.get(&pos).unwrap_or(&Cell::Empty).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut grid = super::Grid::default();
        grid.add_probe(Vec2D::new(0, 0), Vec2D::new(3, 3));
        grid.add_probe(Vec2D::new(4, 4), Vec2D::new(4, 3));

        let expected = vec![
            "S....",
            ".....",
            ".....",
            "...BB",
            "....S",
        ].join("\n");

        assert_eq!(format!("{}", grid), expected);
    }
}
