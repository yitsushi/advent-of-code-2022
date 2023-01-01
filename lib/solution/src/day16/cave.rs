use std::collections::{HashMap, BinaryHeap, HashSet, VecDeque};
use itertools::Itertools;

#[derive(Debug, Default, Clone)]
pub struct Valve {
    id: usize,
    name: String,
    bit: i64,
    rate: i64,
    connections: Vec<String>,
}

// TODO: move to lib/aoc
// references: day15
fn parse_i64(s: &str) -> i64 {
    s.trim_matches(|f| f != '-' && !char::is_numeric(f))
        .parse::<i64>()
        .unwrap()
}

impl std::str::FromStr for Valve {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.splitn(10, ' ').into_iter().collect::<Vec<&str>>();
        let name = parts.get(1)
            .ok_or(format!("unable to parse line: {}", line))?
            .to_string();
        let rate = parse_i64(parts[4]);
        let connections = parts[9].split(", ")
            .map(|f| f.to_string()).collect::<Vec<String>>();

        Ok(Valve{ name, rate, connections, bit: 0, id: 0 })
    }
}

impl Valve {
    pub fn set_bit(&mut self, value: usize) {
        self.bit = i64::pow(2, value as u32);
    }

    pub fn mask(&self) -> i64 { self.bit }
}

#[derive(PartialEq, Eq)]
pub struct Node {
    cost: i64,
    head: String,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Cave {
    valves: HashMap<String, Valve>,
    max_mask: i64,
    min_distances: HashMap<(String, String), i64>,
    flowing: Vec<Valve>,
}

impl Cave {
    pub fn add_valve(&mut self, valve: Valve) {
        self.max_mask |= valve.mask();
        let mut valve = valve;
        valve.id = self.valves.len();
        self.valves.insert(valve.name.clone(), valve);
    }

    fn min_cost(&self, from: String, to: String) -> i64 {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            head: from.clone(),
        });
        visited.insert(from);

        while let Some(Node { cost, head }) = pq.pop() {
            if head == to {
                return cost
            }

            for next in self.valves[&head].connections.iter() {
                let next = next.to_string();
                if visited.insert(next.clone()) {
                    pq.push(Node { cost: cost + 1, head: next });
                }
            }
        }

        i64::MAX
    }

    fn released(&self, mask: i64) -> i64 {
        self.flowing.iter()
            .filter_map(|valve| if mask & valve.mask() > 0 { Some(valve.rate) } else { None })
            .sum()
    }

    pub fn prepare(&mut self) {
        self.flowing = self.valves.iter()
            .filter_map(|(_, valve)| if valve.rate > 0 { Some(valve) } else { None } )
            .cloned()
            .collect();

        self.max_mask = self.flowing.iter().fold(0, |c, valve| c | valve.mask());

        self.min_distances = self.flowing.iter()
            .map(|valve| valve.name.clone())
            .tuple_combinations()
            .fold(HashMap::new(), |mut carry, (from, to)| {
                carry.entry(("AA".to_string(), from.clone()))
                    .or_insert_with(|| self.min_cost("AA".to_string(), from.clone()));
                carry.entry(("AA".to_string(), to.clone()))
                    .or_insert_with(|| self.min_cost("AA".to_string(), to.clone()));

                let dist = self.min_cost(from.clone(), to.clone());

                carry.insert((from.clone(), to.clone()), dist);
                carry.insert((to, from), dist);

                carry
            })
    }

    pub fn walk(&self, timeout: i32) -> HashMap::<i64, i64> {
        let mut max = HashMap::<i64, i64>::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(State {
            head: "AA".to_string(),
            remaining_time: timeout as i64,
            released_pressure: 0,
            valves: 0,
        });

        visited.insert((0, 0, 0));

        while let Some(state) = queue.pop_front() {
            let current_max = state.released_pressure + self.released(state.valves) * state.remaining_time;
            max.entry(state.valves)
                .and_modify(|value| *value = current_max.max(*value))
                .or_insert(current_max);

            if state.valves == self.max_mask || state.remaining_time <= 0 {
                continue
            }

            let closed = self.flowing.iter()
                .filter(|valve| state.valves & valve.mask() == 0);

            for to in closed {
                let cost = self.min_distances[&(state.head.clone(), to.name.clone())] + 1;
                let remaining_time = state.remaining_time - cost;
                if remaining_time <= 0 {
                    continue
                }

                let released_pressure = state.released_pressure + self.released(state.valves) * cost;
                let valves = state.valves | to.mask();
                if visited.insert((valves, remaining_time, released_pressure)) {
                    queue.push_back(State {
                        head: to.name.clone(),
                        remaining_time,
                        released_pressure,
                        valves,
                    })
                }
            }
        }

        max
    }
}

#[derive(Debug)]
struct State {
    head: String,
    remaining_time: i64,
    released_pressure: i64,
    valves: i64,
}
