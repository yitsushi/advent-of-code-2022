use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Crate(char);

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for ch in s.chars() {
            if ch.is_uppercase() {
                return Ok(Self(ch))
            }
        }

        Err(format!("unable to parse crate: {}", s))
    }
}

impl std::string::ToString for Crate {
    fn to_string(&self) -> String {
        format!("[{}]", self.0)
    }
}

impl Crate {
    pub fn name(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Default)]
pub struct Stack(Vec<Crate>);

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", result)
    }
}

impl Stack {
    pub fn push(&mut self, item: Crate) {
        self.0.push(item)
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }

    pub fn pop(&mut self) -> Option<Crate> {
        self.0.pop()
    }

    pub fn top(&self) -> Option<&Crate> {
        self.0.last()
    }
}

#[cfg(test)]
mod tests {
    use super::{Crate, Stack};

    #[test]
    fn crate_to_string() {
        assert_eq!(Crate('A').to_string(), "[A]".to_string())
    }

    #[test]
    fn crate_name() {
        assert_eq!(Crate('A').name(), "A".to_string())
    }

    #[test]
    fn stack_display() {
        let mut stack = Stack::default();

        assert_eq!(format!("{}", stack), "".to_string());

        stack.push(Crate('A'));
        stack.push(Crate('B'));

        assert_eq!(format!("{}", stack), "[A] [B]".to_string())
    }

    #[test]
    fn stack_top() {
        let mut stack = Stack::default();

        assert!(stack.top().is_none());

        stack.push(Crate('A'));
        stack.push(Crate('B'));
        stack.push(Crate('C'));

        assert!(stack.top().is_some());
        assert_eq!(stack.top(), Some(&Crate('C')));
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::default();

        assert!(stack.pop().is_none());

        stack.push(Crate('A'));
        stack.push(Crate('B'));
        stack.push(Crate('C'));

        assert_eq!(stack.pop(), Some(Crate('C')));
    }
}
