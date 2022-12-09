use regex::Regex;

pub struct Day {
    p1: i64,
    p2: i64,
    input: String,
    re: Regex,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 4;
    fn new(input: String) -> Self {
        Self { 
            p1: 0, 
            p2: 0, 
            input,
            re: Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap(),
        }
    }

    fn do_p1(&mut self) {
        for line in self.input.lines() {
            if let Some(capture) = self.re.captures(line){
                let (s1, e1, s2, e2) = (&capture[1].parse::<u8>().unwrap(), &capture[2].parse::<u8>().unwrap(), &capture[3].parse::<u8>().unwrap(), &capture[4].parse::<u8>().unwrap());
                if (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1) {
                    self.p1 += 1;
                }
            }
        }
    }

    fn do_p2(&mut self) {
        for line in self.input.lines() {
            if let Some(capture) = self.re.captures(line){
                let (s1, e1, s2, e2) = (&capture[1].parse::<u8>().unwrap(), &capture[2].parse::<u8>().unwrap(), &capture[3].parse::<u8>().unwrap(), &capture[4].parse::<u8>().unwrap());
                if (s1 < s2 && e1 >= s2) || s1 > s2 && e1 <= e2 || s1 <= e2 && e1 >= e2 || s2 <= e1 && e2 >= e1 {
                    self.p2 += 1;
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