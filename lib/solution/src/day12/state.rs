use aoc::math::Vec2D;

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    pub coordinate: Vec2D<i64>,
    pub estimated_cost: i64,
    pub value: u8,
}

impl State {
    pub fn new(coordinate: Vec2D<i64>, goal: Vec2D<i64>, score: i64, value: u8) -> Self {
        let estimated_cost = coordinate.manhattan_to(&goal) + score;
        Self { coordinate, estimated_cost, value }
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.estimated_cost.cmp(&other.estimated_cost) {
            core::cmp::Ordering::Equal => {},
            ord => return ord.reverse(),
        }
        self.value.cmp(&other.value)
    }
}
