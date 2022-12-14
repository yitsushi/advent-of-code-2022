use super::instruction::Instruction;

#[derive(Debug, Default, Clone)]
pub struct Memory {
    pub x: i32,
}

impl Memory {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
}

#[derive(Debug, Default)]
pub struct Program {
    instructions: Vec<Instruction>,
    memory: Memory,
    screen: Vec<char>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.screen
            .chunks(40)
            .map(|line| line.iter().collect::<String>())
            .fold(Vec::<String>::new(), |results, line| {
                let mut results = results;
                results.push(line);
                results
            })
            .join("\n")
            ;
        writeln!(f, "{}", lines)
    }
}

impl Program {
    pub fn new(memory: Memory, instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            memory,
            screen: vec!['.'; 6*40],
        }
    }

    pub fn draw(&mut self, cycles: i32) {
        let pos = cycles % 40;
        let mem_pos = self.memory.x % 40;

        if (mem_pos-1..=mem_pos+1).contains(&pos) {
            self.screen[cycles as usize] = '#';
        }
    }

    pub fn run(&mut self) -> (Vec<i32>, Memory) {
        let mut next_checkpoint = 20;
        let mut cycles = 0;
        let mut head = 1;
        let mut strength: Vec<i32> = vec![];
        let first = self.instructions.first().unwrap();
        let mut on_hold: (Instruction, i32) = (first.clone(), first.cycles());

       loop {
            if next_checkpoint == cycles {
                strength.push(next_checkpoint * self.memory.x);
                next_checkpoint += 40;
            }

            cycles += 1;

            let (ins, counter) = on_hold;
            if counter == 0 {
                if let Instruction::Addx(v) = ins {
                    self.memory.x += v;
                }
            }

            self.draw(cycles-1);

            if counter > 0 {
                on_hold = (ins, counter-1);
                continue
            }

            let next = match self.instructions.get(head) {
                None => break,
                Some(v) => v,
            };

            on_hold = (next.clone(), next.cycles()-1);
            head += 1;
        }

        (strength, self.memory.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let instructions = vec![
            Instruction::NoOp,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        let mut program = Program::new(Memory::new(1), instructions);
        let (_, mem) = program.run();

        assert_eq!(mem.x, -1);
    }
}
