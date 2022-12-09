pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 6;

    fn new(input: String) -> Self {
        Self { p1: 0, p2: 0, input }
    }

    fn do_p1(&mut self) {
        let mut current;
        for i in 0..self.input.len() - 3 {
            current = &self.input[i..i + 4];
            let (a, b, c, d) = (&current[0..1], &current[1..2], &current[2..3], &current[3..4],);
            if  a != b && a != c && a != d && b != c && b != d && c != d {
                self.p1 = (i + 4) as i64;
                break;
            }
        }
    }

    fn do_p2(&mut self) {
        let mut current;
        let mut selected;
        let mut remaining;
        let mut is_unique = false;
        for i in 0..self.input.len() - 14 {
            current = &self.input[i..i + 14];
            for j in 0..14 {
                selected = &current[j..j+1];
                remaining = &current[j+1..14];
                if remaining.contains(selected) {
                    break;
                }
                else if j == 13 {
                    is_unique = true;
                }
            }
            if is_unique {
                self.p2 = (i + 14) as i64;
                break;
            }
        }

        // Initial solution => about 10x slower
        // let mut current;
        // for i in 0..self.input.len() - 3 {
        //     current = &self.input[i..i + 14];
        //     let mut chars = HashSet::new();
        //     for j in 0..14 {
        //         chars.insert(&current[j..j+1]);
        //     }
        //     if chars.len() == 14 {
        //         self.p2 = (i + 14) as i64;
        //         break;
        //     }
        // }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
