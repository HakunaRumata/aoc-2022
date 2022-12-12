pub struct Day {
    input: String,
    solution: Solution,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 10;

    fn new(input: String) -> Self {
        Self {
            solution: Solution {
                p1: 0,
                step: 0,
                x: 1,
                pos: 0,
            },
            input,
        }
    }

    fn do_p1(&mut self) {
        for line in self.input.lines() {
            if let Some((_, amount)) = line.split_once(" ") {
                let num = amount.parse::<i64>().unwrap();
                self.solution.cycle(0);
                self.solution.cycle(num)
            } else {
                self.solution.cycle(0);
            }
        }
    }

    fn do_p2(&mut self) {
        for line in self.input.lines() {
            if let Some((_, amount)) = line.split_once(" ") {
                let num = amount.parse::<i64>().unwrap();
                self.solution.cycle(0);
                self.solution.cycle(num)
            } else {
                self.solution.cycle(0);
            }
        }
    }

    fn p1_result(&self) -> String {
        format!("{}", self.solution.p1)
    }

    fn p2_result(&self) -> String {
        format!("lol, lmao")
    }
}

struct Solution {
    p1: i64,
    step: i64,
    x: i64,
    pos: i64,
}

impl Solution {
    fn cycle(&mut self, amount: i64) {
        self.step += 1;
        self.pos += 1;
        if (self.step - 20) % 40 == 0 {
            self.p1 += self.step * self.x;
        }
        if self.pos >= self.x && self.pos < self.x + 3 {
            print!("#")
        } else {
            print!(".")
        }
        if self.pos >= 40 {
            println!();
            self.pos = 0;
        }

        self.x += amount;
    }
}
