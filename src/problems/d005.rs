pub struct Day {
    p1: String,
    p2: String,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 5;

    fn new(input: String) -> Self {
        Self { p1: String::new(), p2: String::new(), input }
    }

    fn do_p1(&mut self) {
        let chunks: Vec<&str> = self.input.split("\n\n").collect();
        let (stack_input, orders) = (chunks[0], chunks[1]);
        let mut stacks = get_stacks(stack_input);
        
        for order in orders.lines() {
            let binding = order.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "");
            let order: Vec<&str> = binding
                        .split(" ")
                        .collect();
            let (quantity, source, target) = (order[0].parse::<usize>().unwrap(), order[1].parse::<usize>().unwrap(), order[2].parse::<usize>().unwrap());
            for _i in 0..quantity {
                let c = stacks[source -1].pop().unwrap();
                stacks[target -1].push(c);
            }
        }
        
        for stack in stacks {
            let c = stack.last().unwrap();
            self.p1.push(*c);
        }
    }

    fn do_p2(&mut self) {
        let chunks: Vec<&str> = self.input.split("\n\n").collect();
        let (stack_input, orders) = (chunks[0], chunks[1]);
        let mut stacks = get_stacks(stack_input);


        for order in orders.lines() {
            let binding = order.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "");
            let order: Vec<&str> = binding
                        .split(" ")
                        .collect();
            let (quantity, source, target) = (order[0].parse::<usize>().unwrap(), order[1].parse::<usize>().unwrap(), order[2].parse::<usize>().unwrap());
            let binding = stacks[source - 1].clone();
            let (first, second) = binding.split_at(stacks[source - 1].len() - quantity);
            stacks[source -1] = first.to_vec();
            stacks[target -1 ].append(&mut second.to_vec());
            
        }

        for stack in stacks {
            let c = stack.last().unwrap();
            self.p2.push(*c);
        }

    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }

    
}

fn get_stacks(stack_input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut rows: Vec<Vec<char>> = stack_input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
    rows.reverse();
    let row_count = rows.len();
    for (i, c) in rows[0].iter().enumerate() {
        if !c.is_whitespace(){
            let mut stack = Vec::new();
            for j in 1..row_count {
                let current = rows[j][i];
                if !current.is_whitespace() {
                    stack.push(current)
                }
            }
            stacks.push(stack);
        }
    }

    stacks
}