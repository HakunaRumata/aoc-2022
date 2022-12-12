use std::collections::{HashMap, HashSet};

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    priority: HashMap<char, i64>,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 3;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            priority: HashMap::from([
                ('a', 1),
                ('b', 2),
                ('c', 3),
                ('d', 4),
                ('e', 5),
                ('f', 6),
                ('g', 7),
                ('h', 8),
                ('i', 9),
                ('j', 10),
                ('k', 11),
                ('l', 12),
                ('m', 13),
                ('n', 14),
                ('o', 15),
                ('p', 16),
                ('q', 17),
                ('r', 18),
                ('s', 19),
                ('t', 20),
                ('u', 21),
                ('v', 22),
                ('w', 23),
                ('x', 24),
                ('y', 25),
                ('z', 26),
                ('A', 27),
                ('B', 28),
                ('C', 29),
                ('D', 30),
                ('E', 31),
                ('F', 32),
                ('G', 33),
                ('H', 34),
                ('I', 35),
                ('J', 36),
                ('K', 37),
                ('L', 38),
                ('M', 39),
                ('N', 40),
                ('O', 41),
                ('P', 42),
                ('Q', 43),
                ('R', 44),
                ('S', 45),
                ('T', 46),
                ('U', 47),
                ('V', 48),
                ('W', 49),
                ('X', 50),
                ('Y', 51),
                ('Z', 52),
            ]),
        }
    }

    // Result: 7875
    fn do_p1(&mut self) {
        for (first, second) in self.input.lines().map(|line| line.split_at(line.len() / 2)) {
            let chars = first
                .chars()
                .filter(|c| second.contains(*c))
                .collect::<HashSet<char>>();
            for c in chars {
                if let Some(points) = self.priority.get(&c) {
                    self.p1 += points;
                }
            }
        }
    }

    // Result: 2479
    fn do_p2(&mut self) {
        let mut lines = self.input.lines();

        while let (Some(first), Some(second), Some(third)) =
            (lines.next(), lines.next(), lines.next())
        {
            let chars = first
                .chars()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .collect::<HashSet<char>>();
            for c in chars {
                if let Some(points) = self.priority.get(&c) {
                    self.p2 += points;
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
