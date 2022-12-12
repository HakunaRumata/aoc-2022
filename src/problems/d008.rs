pub struct Day {
    p1: u64,
    p2: u64,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 8;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        let trees: Vec<Vec<u32>> = self
            .input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let row_len = trees[0].len();
        let last_row = trees.len();

        let mut vertical_trees: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_len {
            vertical_trees.push(Vec::new());
            for j in 0..last_row {
                vertical_trees[i].push(trees[j][i]);
            }
        }

        for i in 1..last_row - 1 {
            for j in 1..row_len - 1 {
                if !vertical_trees[j][0..i].iter().any(|t| t >= &trees[i][j])
                    || !vertical_trees[j][i + 1..last_row]
                        .iter()
                        .any(|t| t >= &trees[i][j])
                    || !trees[i][0..j].iter().any(|t| t >= &trees[i][j])
                    || !trees[i][j + 1..row_len].iter().any(|t| t >= &trees[i][j])
                {
                    self.p1 += 1;
                }
            }
            self.p1 += 2;
        }

        self.p1 += 2 * row_len as u64
    }

    fn do_p2(&mut self) {
        let trees: Vec<Vec<u32>> = self
            .input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let row_len = trees[0].len();
        let last_row = trees.len();

        let mut vertical_trees: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_len {
            vertical_trees.push(Vec::new());
            for j in 0..last_row {
                vertical_trees[i].push(trees[j][i]);
            }
        }

        for i in 1..last_row - 1 {
            for j in 1..row_len - 1 {
                let tree = trees[i][j];
                let (mut left, mut right, mut top, mut bottom) = (0_u32, 0_u32, 0_u32, 0_u32);
                for t in vertical_trees[j][0..i].iter().rev() {
                    top += 1;
                    if t >= &tree {
                        break;
                    }
                }
                for t in vertical_trees[j][i + 1..row_len].iter() {
                    bottom += 1;
                    if t >= &tree {
                        break;
                    }
                }
                for t in trees[i][0..j].iter().rev() {
                    left += 1;
                    if t >= &tree {
                        break;
                    }
                }
                for t in trees[i][j + 1..row_len].iter() {
                    right += 1;
                    if t >= &tree {
                        break;
                    }
                }
                let score = left * right * top * bottom;
                if score as u64 > self.p2 {
                    self.p2 = score as u64
                }
            }
        }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
