use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {

    let std_in = io::stdin();
    let mut ints = HashSet::new();

    'outer: for line in std_in.lock().lines() {
        let line_str = line.unwrap();
        let line_int = line_str.parse::<u32>().unwrap(); 
        let needs1 = 2020 - line_int;
        //println!("input: {}, needs1: {}", line_int, needs1);
        for x in &ints {
            if*x > needs1{
                continue;
            }
            let needs = needs1 - x;
            //println!("needs2: {}", needs);
            if ints.contains(&needs){
                println!("vals - {}, {}, {}, result: {}", line_int, x, needs, line_int * needs * x);
                break 'outer;
            }
        }

        ints.insert(line_int);
    }
    println!("I'm DOne!!");
}
