pub struct Day {
    p1: i64,
    p2: i64,
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
    If false: throw to monkey 1"),
        }
    }

    fn do_p1(&mut self) {
        let mut monkees: Vec<Monke> = Vec::new();
        for monke in self.input.split("\n\n") {
            monkees.push(Monke::new(monke));
        }
    }

    fn do_p2(&mut self) {}

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}

struct Monke {
    inspections: u32,
    items: Vec<u32>,
    option_a: usize,
    option_b: usize,
    operation: Box<dyn Fn(&u32) -> u32>,
    test_num: u32,
}

impl Monke {
    fn new(monke: &str) -> Self
    {
        let values: Vec<&str> = monke.lines().collect();
        let items = values[1][18..].split(", ").map(|i| i.parse::<u32>().unwrap()).collect();
        let operation_input = &values[2][19..];
        let operation: Box<dyn Fn(&u32) -> u32> = match operation_input {
            _square if operation_input == "old * old" => Box::new(|&num| num * num),
            _double if operation_input == "old + old" => Box::new(|&num| num + num),
            add if operation_input.contains("old +") => Box::new(|&num| num + add[5..].parse::<u32>().unwrap()),
            multiply if operation_input == "old +" => Box::new(|&num| num + multiply[5..].parse::<u32>().unwrap()),
            _ => panic!()
        };

        Self { 
            inspections: 0, 
            items,
            operation,
            option_a: values[4][29..].parse::<usize>().unwrap(),
            option_b: values[5][30..].parse::<usize>().unwrap(),
            test_num: values[3][21..].parse::<u32>().unwrap(),
        }
    }


    fn process_item(&mut self, monkees: &mut Vec<Monke>) {
        for item in self.items.clone() {
            self.inspections += 1;
            if (self.operation)(&item) / self.test_num == 0 {
                monkees[self.option_a].items.push(item)
            }
            else {
                monkees[self.option_b].items.push(item)
            }
        }
    }
}