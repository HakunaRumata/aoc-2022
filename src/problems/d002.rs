pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 2;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        for rps in self.input.lines() {
            match rps {
                "A X" => {
                    self.p1 += 4;
                    self.p2 += 3
                }
                "A Y" => {
                    self.p1 += 8;
                    self.p2 += 4
                }
                "A Z" => {
                    self.p1 += 3;
                    self.p2 += 8
                }
                "B X" => {
                    self.p1 += 1;
                    self.p2 += 1
                }
                "B Y" => {
                    self.p1 += 5;
                    self.p2 += 5
                }
                "B Z" => {
                    self.p1 += 9;
                    self.p2 += 9
                }
                "C X" => {
                    self.p1 += 7;
                    self.p2 += 2
                }
                "C Y" => {
                    self.p1 += 2;
                    self.p2 += 6
                }
                "C Z" => {
                    self.p1 += 6;
                    self.p2 += 7
                }
                _ => (),
            }
        }
    }

    // No reason to solve it in two steps
    fn do_p2(&mut self) {}

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
