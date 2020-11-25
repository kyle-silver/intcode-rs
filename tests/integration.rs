use std::fs;
use intcode_rs::*;
use intcode_rs::procedural_comp::ProcIntCode;

fn read(file_name: &str) -> Vec<i64> {
    fs::read_to_string(file_name)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[test]
fn day2_part1_proc() {
    let mut program = read("res/02.txt");
    program[1] = 12;
    program[2] = 2;
    let mut comp = ProcIntCode::new(program, vec![]);
    let state = comp.run();
    println!("Day 02, Part 1: {}", comp.mem(0));
    assert_eq!(State::Halted, state);
    assert_eq!(4484226, comp.mem(0));
}