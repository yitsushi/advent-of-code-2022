use std::collections::{HashMap, BinaryHeap};
use aoc::grid_helper;

use aoc::math::Vec2D;
use super::state::State;

#[derive(Default, Debug)]
pub struct Grid {
    fields: HashMap<Vec2D<i64>, u8>,
}

impl Grid {
    pub fn add(&mut self, pos: Vec2D<i64>, value: u8) {
        self.fields.insert(pos, value);
    }

    pub fn walk(
        &mut self, init_value: u8,
        start: Vec2D<i64>,
        aim: impl Fn(Vec2D<i64>) -> Vec2D<i64>,
        win: impl Fn(Vec2D<i64>) -> bool,
        allowed: impl Fn(u8, u8) -> bool,
    ) -> Option<i64> {
        let mut open_set = BinaryHeap::<State>::new();
        let mut came_from = HashMap::<Vec2D<i64>, Vec2D<i64>>::new();
        let mut score = HashMap::<Vec2D<i64>, i64>::new();

        open_set.push(State::new(start.clone(), start.clone(), 0, init_value));
        score.insert(start, 0);


        while let Some(state) = open_set.pop() {
            if self.fields.get(&state.coordinate).is_none() {
                println!(" >>> met a weird field: {:?}", state.coordinate);
                continue
            }

            if win(state.coordinate.clone()) {
                return Some(state.estimated_cost)
            }

            let current_score = score.get(&state.coordinate).unwrap();
            let next_score = current_score + 1;

            for target in state.coordinate.neighbors() {
                // out of bound
                let next_value = if let Some(v) = self.fields.get(&target) { *v } else { continue };
                if !allowed(state.value, next_value) { continue }

                // not a better path
                if let Some(known_score) = score.get(&target) {
                    if *known_score <= next_score { continue }
                }

                let aim_for = aim(target.clone());
                score.insert(target.clone(), next_score);
                came_from.insert(target.clone(), state.coordinate.clone());
                let new_state = State::new(
                    target.clone(),
                    aim_for,
                    next_score,
                    next_value,
                );
                open_set.push(new_state);
            }
        }

        None
    }

    pub fn lowest_points(&self) -> Vec<Vec2D<i64>> {
        self.fields.iter()
            .filter_map(|(pos, value)| if *value == 0 { Some(pos.clone()) } else { None })
            .collect()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.fields.keys().cloned().collect::<Vec<Vec2D<i64>>>();
        let (min_coordinate, max_coordinate) = grid_helper::min_max_coordinates(list);

        let lines = grid_helper::display_i64(min_coordinate, max_coordinate, |pos| {
            if let Some(v) = self.fields.get(&pos) {
                (v + 0x61) as char
            } else {
                '.'
            }
        });

        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut grid = Grid::default();
        grid.add(Vec2D::new(0, 0), 0); grid.add(Vec2D::new(1, 0), 0); grid.add(Vec2D::new(2, 0), 1);
        grid.add(Vec2D::new(0, 1), 7); grid.add(Vec2D::new(1, 1), 2); grid.add(Vec2D::new(2, 1), 4);
        grid.add(Vec2D::new(0, 2), 15); grid.add(Vec2D::new(1, 2), 8); grid.add(Vec2D::new(2, 2), 20);

        assert_eq!(format!("{}", grid), "aab\nhce\npiu".to_string());
    }
}
