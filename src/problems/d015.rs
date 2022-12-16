pub struct Day {
    p1: i64,
    p2: i64,
    _input: String,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 15;

    fn new(_input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            _input,
        }
    }

    fn do_p1(&mut self) {
        // for line in self.input.lines() {
        //     if let Some((s, b)) = line[10..].split_once(": closest beacon is at ") {
        //         println!("{s}");
        //         println!("{b}");
        //     }
        // }
    }

    fn do_p2(&mut self) {}

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}

// struct Sensor{
//     location: Point,
//     beacon: Point,
//     covered_areas: Vec<u32>,
// }

// impl Sensor {
//     fn new(location: Point, beacon: Point) -> Self {
//         Self { location, beacon, covered_areas: Vec::new() }
//     }
// }

// struct Point {
//     x: u32,
//     y: u32,
// }

// impl Point {
//     fn new(x: u32, y: u32) -> Self {
//         Self { x, y, }
//     }
// }