use std::{collections::HashSet};

pub struct Day {
    p1: i64,
    p2: i64,
    input: String
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 9;

    fn new(input: String) -> Self {
        Self { 
            p1: 0, 
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) {
        let mut positions: HashSet<(i16, i16)> = HashSet::new();
        let mut rope = Node::new();
        rope.next_node = Some(Box::new(Node::new()));

        for line in self.input.lines() {
            rope.follow_motion(line, &mut positions);
        }

        self.p1 = positions.len() as i64;
    }

    fn do_p2(&mut self) {
        let mut positions: HashSet<(i16, i16)> = HashSet::new();
        let mut rope = Node::new();
        let mut new_node;
        for _i in 0..9 {
            new_node = Node::new();
            new_node.next_node = Some(Box::new(rope));
            rope = new_node
        }

        for line in self.input.lines() {
            rope.follow_motion(line, &mut positions);
        }

        self.p2 = positions.len() as i64;
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}

struct Node{
    x1: i16,
    y1: i16,
    next_node: Option<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        let node = Node {
            x1:0, 
            y1:0, 
            next_node: None,
        };

        node
    }

    fn follow_motion(&mut self, motion: &str, positions: &mut HashSet<(i16, i16)>) {
        for _i in 0..motion[2..].parse::<u8>().unwrap() {
            self.move_head(&motion[0..1]);
            self.follow_tail(self.x1, self.y1, positions)
        }
    }

    fn move_head(&mut self, direction: &str){
        match direction {
            _movement if direction.eq("R") => self.x1 += 1,
            _movement if direction.eq("L") => self.x1 -= 1,
            _movement if direction.eq("U") => self.y1 += 1,
            _movement if direction.eq("D") => self.y1 -= 1,
            _ => ()
        }
    }

    fn follow_tail(&mut self, prev_x: i16, prev_y: i16, positions: &mut HashSet<(i16, i16)>){
        match self {
            // straight movements
            _ if prev_x > self.x1 + 1 && prev_y == self.y1 => self.x1 += 1,
            _ if prev_x < self.x1 - 1 && prev_y == self.y1 => self.x1 -= 1,
            _ if prev_y > self.y1 + 1 && prev_x == self.x1 => self.y1 += 1,
            _ if prev_y < self.y1 - 1 && prev_x == self.x1 => self.y1 -= 1,
            // x + 1, y + 1
            _ if prev_x > self.x1 + 1 && prev_y > self.y1
              || prev_y > self.y1 + 1 && prev_x > self.x1 => {
                self.x1 += 1;
                self.y1 += 1
            },
            // x + 1, y - 1
            _ if prev_x > self.x1 + 1 && prev_y < self.y1
              || prev_y < self.y1 - 1 && prev_x > self.x1 => {
                self.x1 += 1;
                self.y1 -= 1
            },
            // x - 1, y + 1
            _ if prev_x < self.x1 - 1 && prev_y > self.y1
              || prev_x < self.x1 && prev_y > self.y1 + 1 => {
                self.x1 -= 1;
                self.y1 += 1
            },
            // x - 1, y - 1
            _ if prev_x < self.x1 - 1 && prev_y < self.y1
              || prev_x < self.x1 && prev_y < self.y1 - 1 => {
                self.x1 -= 1;
                self.y1 -= 1
            },
            _ => (),
        }

        if self.next_node.is_some() {
            self.next_node.as_mut().unwrap().follow_tail(self.x1, self.y1, positions)
        }
        else {
            positions.insert((self.x1, self.y1));
        }
    }
}