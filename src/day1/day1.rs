use std::env;
use std::fs;

#[derive(Debug)]
struct Dial {
    direction: i32,
}
// dial points from 0 to 100. If it goes to 100 it goes back to

impl Dial {
    fn turn(&mut self, input: &str) {
        let inp = input.replace("L", "-").replace("R", "");
        let num: i32 = inp.parse().unwrap();
        self.direction = self.direction + num;
        //self.direction = self.direction % 100;

        if self.direction >= 0 {
            self.direction = self.direction % 100;
        } else {
            self.direction = (100 + (self.direction %  100)) % 100;
        }
    }

    fn turn_with_info(&mut self, input: &str) -> i32 {
        let inp = input.replace("L", "-").replace("R", "");
        let num: i32 = inp.parse().unwrap();
        let bungo = self.direction == 0;
        self.direction = self.direction + num;
        let g: i32;
        if self.direction > 0 {
            g = self.direction / 100;
        } else {
            if bungo{
                g = -1 * self.direction / 100;
            }
            else {
                g = -1 * self.direction / 100 + 1;
            }
            
        }

        if self.direction >= 0 {
            self.direction = self.direction % 100;
        } else {
            self.direction = (100 + (self.direction %  100)) % 100;
        }

        return g;
    }
}

pub fn part_1(file_path: String) {
    let contents: String =
        fs::read_to_string(file_path).expect("Should ahve been able to read the file");
    let rotation_schedules: Vec<&str> = contents.split('\n').collect();
    let mut dial = Dial { direction: 50 };
    //println!("With text:\n  {rotation_schedules:?}");
    let mut counter = 0;
    for fung in rotation_schedules {
        dial.turn(fung);
        if dial.direction == 0 {
            counter = counter + 1;
        }
    }
    println!("{dial:?}");
    println!("The counter is at {counter}");
}

pub fn part_2(file_path: String) {
    let contents: String =
        fs::read_to_string(file_path).expect("Should ahve been able to read the file");
    let rotation_schedules: Vec<&str> = contents.split('\n').collect();
    let mut dial = Dial { direction: 50 };
    //println!("With text:\n  {rotation_schedules:?}");
    let mut counter: i32 = 0;
    for fung in rotation_schedules {
        counter = counter + dial.turn_with_info(fung);
    }
    println!("{dial:?}");
    println!("The counter is at {counter}");
}
