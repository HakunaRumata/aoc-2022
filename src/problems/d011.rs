use std::borrow::BorrowMut;

pub struct Day {
    p1: u128,
    p2: u128,
    input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 11;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input: String::from("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1")
        }
    }

    fn do_p1(&mut self) {
        let mut monkees: Vec<Monke> = Vec::new();
        let mut all_items = Vec::new();
        for monke in self.input.split("\n\n") {
            monkees.push(Monke::new(monke, all_items.borrow_mut()));
        }
        for _i in 0..20 {
            for (j, monke) in &mut monkees.iter_mut().enumerate() {
                monke.process_item(all_items.borrow_mut(), j, true);
            }
        }
        monkees.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());
        monkees.reverse();
        self.p1 = monkees[0].inspections * monkees[1].inspections;
    }

    fn do_p2(&mut self) {
        let mut monkees: Vec<Monke> = Vec::new();
        let mut all_items = Vec::new();
        for monke in self.input.split("\n\n") {
            monkees.push(Monke::new(monke, all_items.borrow_mut()));
        }
        // TODO: deal with overflow :)
        for _i in 0..20 {
            for (j, monke) in &mut monkees.iter_mut().enumerate() {
                monke.process_item(all_items.borrow_mut(), j, false);
            }
        }
        // for item in all_items {
        //     println!("{:?}", item);
        // }
        monkees.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());
        monkees.reverse();
        self.p2 = monkees[0].inspections * monkees[1].inspections;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}

struct Monke {
    inspections: u128,
    option_a: usize,
    option_b: usize,
    operation: Box<dyn Fn(&u128) -> u128>,
    test_num: u128,
}

impl Monke {
    fn new(monke: &str, all_items: &mut Vec<Vec<u128>>) -> Self
    {
        let values: Vec<&str> = monke.lines().collect();
        let items = values[1][18..].split(", ").map(|i| i.parse::<u128>().unwrap()).collect();
        all_items.push(items);
        let operation_input = &values[2][19..];
        let operation: Box<dyn Fn(&u128) -> u128> = match operation_input {
            _square if operation_input ==  "old * old" => {
                // println!("1");
                Box::new(|&num| num * num)
            },
            _double if operation_input == "old + old" => {
                // println!("2");
                Box::new(|&num| num + num)
            } ,
            add if operation_input.contains("old +") => {
                // println!("3");
                let op_number = add[6..].parse::<u128>().unwrap();
                Box::new(move |&num| num + op_number)
            },
            multiply if operation_input.contains("old *") => {
                // println!("4");
                let op_number = multiply[6..].parse::<u128>().unwrap();
                Box::new(move |&num| num * op_number)
            }
            _ => panic!()
        };
        Self { 
            inspections: 0, 
            operation,
            option_a: values[4][29..].parse::<usize>().unwrap(),
            option_b: values[5][30..].parse::<usize>().unwrap(),
            test_num: values[3][21..].parse::<u128>().unwrap(),
        }
    }

    fn process_item(&mut self, all_items: &mut Vec<Vec<u128>>, current: usize, p1: bool) {
        for _i in 0..all_items[current].len() {
            let mut item = all_items[current].pop().unwrap();
            item =  (self.operation)(&item);
            if p1 {
                item = item / 3;
            }
            self.inspections += 1;
            if item % self.test_num == 0 {
                all_items[self.option_a].push(item)
            }
            else {
                all_items[self.option_b].push(item)
            }
        }
    }
}