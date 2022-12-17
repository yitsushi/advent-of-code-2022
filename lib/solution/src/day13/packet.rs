use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Item {
    Value(i64),
    SubList(String),
    Nothing,
}

#[derive(Debug, Clone)]
pub struct Packet {
    line: String,
    pub values: Vec<Item>,
}

impl Packet {
    pub fn from_string(line: String) -> Packet {
        let mut packet = Packet {
            line: line.clone(),
            values: vec![],
        };

        let mut section = line[1..line.len() - 1].to_string();

        if section.is_empty() {
            return packet
        }

        loop {
            if section.starts_with('[') {
                // sub list
                let mut sub: Vec<u8> = vec![b'['];
                let mut rest: Vec<u8> = vec![];
                let mut open_counter = 0;
                for ch in section[1..].bytes() {
                    if open_counter < 0 {
                        rest.push(ch);
                        continue
                    }

                    if ch == b'[' {
                        open_counter += 1;
                    } else if ch == b']' {
                        open_counter -= 1;
                    }

                    sub.push(ch);
                }

                let sub: String = String::from_utf8(sub).unwrap();
                let rest: String = String::from_utf8(rest).unwrap();

                packet.values.push(Item::SubList(sub));

                let rest = rest.trim_start_matches(',');
                if rest.is_empty() {
                    break
                }

                section = rest.to_string();
            } else {
                // value
                let parts = section.splitn(2, ',').map(|f| f.to_string()).collect::<Vec<String>>();
                match &parts[..] {
                    [x, rest] => {
                        packet.values.push(Item::Value(x.parse::<i64>().unwrap()));
                        section = rest.clone();
                    }
                    [x] => {
                        packet.values.push(Item::Value(x.parse::<i64>().unwrap()));
                        break
                    }
                    _ => {
                        println!("[value] nope: {:?}", parts);
                        unreachable!()
                    },
                };
            }
        }

        packet
    }
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.line == other.line {
            return Ordering::Equal
        }

        let mut fst = self.values.to_vec();
        let mut snd = other.values.to_vec();

        match fst.len().cmp(&snd.len()) {
            Ordering::Less => {
                let missing = snd.len() - fst.len();
                for _ in 0..missing {
                    fst.push(Item::Nothing);
                }
            },
            Ordering::Greater => {
                let missing = fst.len() - snd.len();
                for _ in 0..missing {
                    snd.push(Item::Nothing);
                }
            },
            Ordering::Equal => {}
        }

        for pair in fst.iter().zip(snd.iter()) {
            // println!("fst = {:?}; snd = {:?}", pair.0, pair.1);
            match pair {
                (Item::SubList(_), Item::Nothing) | (Item::Value(_), Item::Nothing) => return Ordering::Greater,
                (Item::Nothing, Item::Nothing) => return Ordering::Equal,
                (Item::Nothing, _) => return Ordering::Less,
                (Item::Value(x), Item::Value(y)) =>
                    match x.cmp(y) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                ,
                (Item::SubList(x), Item::SubList(y)) => {
                    let res = Packet::from_string(x.clone()).cmp(&Packet::from_string(y.clone()));
                    if res != Ordering::Equal {
                        return res
                    }
                },
                (Item::Value(x), Item::SubList(y)) => {
                    let x = Packet::from_string(format!("[{}]", x));
                    let res = x.cmp(&Packet::from_string(y.clone()));
                    if res != Ordering::Equal {
                        return res
                    }
                },
                (Item::SubList(x), Item::Value(y)) => {
                    let y = Packet::from_string(format!("[{}]", y));
                    let res = Packet::from_string(x.clone()).cmp(&y);
                    if res != Ordering::Equal {
                        return res
                    }
                },
            }
        }

        Ordering::Equal
    }
}