use std::str::Lines;

pub struct Day {
    input: String,
    file_system: FileSystem,
}

impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 7;

    fn new(input: String) -> Self {
        Self {
            input,
            file_system: FileSystem {
                p1: 0,
                p2: 0,
                required: 0,
            },
        }
    }

    fn do_p1(&mut self) {
        let total = self.file_system.get_dir_size(&mut self.input.lines());
        self.file_system.required = 30000000 - (70000000 - total);
    }

    fn do_p2(&mut self) {
        self.file_system.get_dir_to_delete(&mut self.input.lines());
    }

    fn p1_result(&self) -> String {
        format!("{}", self.file_system.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.file_system.p2)
    }
}

struct FileSystem {
    p1: u64,
    p2: u64,
    required: u64,
}

impl FileSystem {
    fn get_dir_size(&mut self, lines: &mut Lines) -> u64 {
        let mut count = 0_u64;
        let mut broke = false;
        while let Some(line) = lines.next() {
            match line {
                _back if line.contains("..") => {
                    if count <= 100000 {
                        self.p1 += count;
                        broke = true;
                        break;
                    }
                }
                _next_dir if line.contains("$ cd") => count += self.get_dir_size(lines),
                _ls if line.contains("$ ls") => (),
                _dir if line.contains("dir") => (),
                file => {
                    let split: Vec<&str> = file.split(" ").collect();
                    count += split[0].parse::<u64>().unwrap();
                }
            }
        }
        if !broke && count <= 100000 {
            self.p1 += count;
        }

        count
    }

    fn get_dir_to_delete(&mut self, lines: &mut Lines) -> u64 {
        let mut count = 0_u64;
        while let Some(line) = lines.next() {
            match line {
                _back if line.contains("..") => break,
                _next_dir if line.contains("$ cd") => count += self.get_dir_to_delete(lines),
                _ls if line.contains("$ ls") => (),
                _dir if line.contains("dir") => (),
                file => {
                    let split: Vec<&str> = file.split(" ").collect();
                    count += split[0].parse::<u64>().unwrap();
                }
            }
        }
        if count >= self.required && (count < self.p2 || self.p2 == 0) {
            self.p2 = count;
        }

        count
    }
}
