pub struct Day {
    p1: i64,
    p2: i64,
    input: String
}

impl crate::Problem for Day{
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 1;

    fn new(input: String) -> Self {
        Self { p1: 0, p2: 0, input }
    }

    fn do_p1(&mut self) {
        let mut elves: Vec<i64> = self.input.split("\n\n")
        .map(|group| 
            group.lines().map(|line| line.parse::<i64>().unwrap())
            .sum()
            )
        .collect();
        elves.sort();
        
        self.p1 = elves.last().unwrap().to_owned();
    }

    fn do_p2(&mut self) {
        let mut elves: Vec<i64> = Vec::new();
        let mut it = self.input.lines().peekable();
        let mut current = 0;
        while let Some(line) = it.next()  {

            if let Ok(num) = line.parse::<i64>(){
                current += num;
            }
            else{
                elves.push(current);
                current = 0;
            }
            if it.peek().is_none() {
                elves.push(current);
            }
        }
        
        elves.sort();
        elves.reverse();

        self.p2 = elves[0] + elves[1] + elves[2]
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
