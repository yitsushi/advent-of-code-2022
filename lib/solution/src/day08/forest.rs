#[derive(Default, Debug)]
pub struct Forest {
    trees: Vec<u8>,
    pub rows: usize,
    pub columns: usize,
}

impl Forest {
    pub fn new(trees: Vec<u8>, rows: usize, columns: usize) -> Forest {
        Forest { trees, rows, columns }
    }

    pub fn all(&self) -> &Vec<u8> {
        &self.trees
    }

    pub fn row_of(&self, idx: usize) -> usize {
        idx / self.columns
    }

    pub fn col_of(&self, idx: usize) -> usize {
        idx % self.columns
    }

    pub fn scenic_score_at(&self, idx: usize) -> u32 {
        let current = if let Some(v) = self.at(idx) { v } else { return 0 };
        let row_index = self.row_of(idx);
        let col_index = self.col_of(idx);
        let row = self.row(row_index);
        let col = self.col(col_index);

        if row_index == 0 && col_index == 0 && row_index == self.rows && col_index == self.columns {
            return 0;
        }

        let mut u = col[..row_index].to_vec();
        let d = col[row_index+1..].to_vec();
        let mut l = row[..col_index].to_vec();
        let r = row[col_index+1..].to_vec();
        u.reverse();
        l.reverse();

        let mut up = 0;
        for (_, tree) in u {
            up += 1;
            if tree >= current { break }
        }
        if up == 0 { return 0; }

        let mut down = 0;
        for (_, tree) in d {
            down += 1;
            if tree >= current { break }
        }
        if down == 0 { return 0; }

        let mut left = 0;
        for (_, tree) in l {
            left += 1;
            if tree >= current { break }
        }
        if left == 0 { return 0; }

        let mut right = 0;
        for (_, tree) in r {
            right += 1;
            if tree >= current { break }
        }

        up * down * left * right
    }

    pub fn at(&self, index: usize) -> Option<u8> {
        if self.trees.len() <= index {
            return None
        }

        Some(self.trees[index])
    }

    pub fn row(&self, row: usize) -> Vec<(usize, u8)> {
        let offset = self.columns * row;
        self.trees[offset..(offset + self.columns)]
            .iter()
            .enumerate()
            .map(|(idx, ch)| (idx + offset, *ch))
            .collect::<Vec<(usize, u8)>>()
    }

    pub fn col(&self, col: usize) -> Vec<(usize, u8)> {
        self.trees.iter()
            .enumerate()
            .filter(|(idx, _)| idx % self.columns == col)
            .map(|(idx, ch)| (idx, *ch))
            .collect::<Vec<(usize, u8)>>()
    }

    pub fn all_rows(&self) -> Vec<Vec<(usize, u8)>> {
        let mut result: Vec<Vec<(usize, u8)>> = Vec::new();

        for idx in 0..self.columns {
            result.push(self.row(idx))
        };

        result
    }

    pub fn all_columns(&self) -> Vec<Vec<(usize, u8)>> {
        let mut result: Vec<Vec<(usize, u8)>> = Vec::new();

        for idx in 0..self.columns {
            result.push(self.col(idx))
        };

        result
    }
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        for i in 0..self.rows {
            if i != 0 {
                str.push('\n');
            }

            let row_str = self.row(i)
                .iter()
                .map(|(_, ch)| (ch + 0x30) as char)
                .collect::<String>();

            str.push_str(&row_str);
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let forest = Forest::new(vec![1,2,3], 1, 3);

        assert_eq!(forest.at(0), Some(1));
        assert_eq!(forest.at(1), Some(2));
        assert_eq!(forest.at(2), Some(3));
    }

    #[test]
    fn at_out_of_bound() {
        let forest = Forest::new(vec![1,2,3], 1, 3);

        assert_eq!(forest.at(3), None);
    }

    #[test]
    fn col_of() {
        let forest = Forest::new(vec![1,2,3,4,5,6,7,8,9], 3, 3);

        assert_eq!(forest.col_of(0), 0);
        assert_eq!(forest.col_of(3), 0);
        assert_eq!(forest.col_of(6), 0);

        assert_eq!(forest.col_of(1), 1);
        assert_eq!(forest.col_of(4), 1);
        assert_eq!(forest.col_of(7), 1);

        assert_eq!(forest.col_of(2), 2);
        assert_eq!(forest.col_of(5), 2);
        assert_eq!(forest.col_of(8), 2);
    }

    #[test]
    fn row_of() {
        let forest = Forest::new(vec![1,2,3,4,5,6,7,8,9], 3, 3);

        assert_eq!(forest.row_of(0), 0);
        assert_eq!(forest.row_of(1), 0);
        assert_eq!(forest.row_of(2), 0);

        assert_eq!(forest.row_of(3), 1);
        assert_eq!(forest.row_of(4), 1);
        assert_eq!(forest.row_of(5), 1);

        assert_eq!(forest.row_of(6), 2);
        assert_eq!(forest.row_of(7), 2);
        assert_eq!(forest.row_of(8), 2);
    }

    #[test]
    fn scenic_score_at() {
        let forest = Forest::new(vec![
                                 1,2,3,
                                 4,5,6,
                                 7,8,9,
                                 1,2,3], 4, 3);

        assert_eq!(forest.scenic_score_at(4), 1);
        assert_eq!(forest.scenic_score_at(5), 0);
        assert_eq!(forest.scenic_score_at(7), 2);
    }

    #[test]
    fn display() {
        let forest = Forest::new(vec![1,2,3,4,5,6,7,8,9], 3, 3);

        let result = format!("{}", forest);
        let expected = "123\n456\n789".to_string();

        assert_eq!(result, expected);
    }
}
