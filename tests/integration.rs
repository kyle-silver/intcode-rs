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

#[test]
fn day2_part2_proc() {
    let mut answer = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = read("res/02.txt");
            program[1] = noun;
            program[2] = verb;
            let mut comp = ProcIntCode::new(program, vec![]);
            comp.run();
            if comp.mem(0) == 19690720 {
                answer = 100 * noun + verb;
                println!("Day 02, Part 2: {}", 100 * noun + verb);
                break;
            }
        }
    }
    assert_eq!(5696, answer);
}

#[test]
fn day5_part1_proc() {
    let program = read("res/05.txt");
    let mut comp = ProcIntCode::new(program, vec![1]);
    comp.run();
    println!("Day 05, Part 1: {:?}", comp.out());
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 14522484], *comp.out());
}

#[test]
fn day5_part2_proc() {
    let program = read("res/05.txt");
    let mut comp = ProcIntCode::new(program, vec![5]);
    comp.run();
    println!("Day 05, Part 1: {:?}", comp.out());
    assert_eq!(vec![4655956], *comp.out());
}

fn unique_perms(max: i64, digit_low: i32, digit_high: i32) -> Vec<Vec<i64>> {
    (0..=max)
        .map(|n| format!("{:05}", n))
        .filter(|digits| {
            for i in digit_low..=digit_high {
                if digits.contains(&i.to_string()) == false {
                    return false;
                }
            }
            return true;
        })
        .map(|digits| digits.chars().collect())
        .map(|chars: Vec<char>| chars.iter().map(|d| d.to_string().parse().unwrap()).collect())
        .collect()
}

#[test]
fn day7_part1_proc() {
    let unique_perms = unique_perms(43210, 0, 4);
    let program = read("res/07.txt");
    let mut res: Vec<(&Vec<i64>, i64)> = vec![];
    for perm in unique_perms.iter() {
        let mut output = 0;
        for digit in perm.iter() {
            let mut comp = ProcIntCode::new(program.clone(), vec![*digit, output]);
            comp.run();
            output = *comp.out().get(0).unwrap();
        }
        res.push((perm, output));
    }
    let max = res.iter()
        .map(|(_, final_out)| final_out)
        .max()
        .unwrap();
    println!("Day 07, Part 1: {}", max);
    assert_eq!(880726, *max);
}

fn halted<T: IntCodeComputer>(comps: &Vec<T>) -> bool {
    comps.iter()
        .map(|comp| match comp.state() {
            State::Halted => true,
            _ => false,
        })
        .fold(false, |acc, c| acc || c)
}

#[test]
fn day7_part2_proc() {
    let unique_perms = unique_perms(98765, 5, 9);
    let program = read("res/07.txt");
    let mut res: Vec<(&Vec<i64>, i64)> = vec![];
    for perm in unique_perms.iter() {
        let mut output = 0;
        let mut comps: Vec<ProcIntCode> = Vec::new();
        for digit in perm.iter() {
            comps.push(ProcIntCode::new(program.clone(), vec![*digit]));
        }
        while halted(&comps) == false {
            for comp in comps.iter_mut() {
                comp.push(output);
                comp.run();
                output = comp.out()[comp.out().len()-1];
            }
        }
        res.push((perm, output));
    }
    let max = res.iter()
        .map(|(_, final_out)| final_out)
        .max()
        .unwrap();
    println!("Day 07, Part 2: {}", max);
    assert_eq!(4931744, *max);
}

#[test]
fn day9_part1_proc() {
    let program = read("res/09.txt");
    let mut comp = ProcIntCode::new(program.clone(), vec![1]);
    comp.run();
    println!("Day 09, Part 1: {:?}", comp.out());
    assert_eq!(vec![3380552333], *comp.out());
}

#[test]
fn day9_part2_proc() {
    let program = read("res/09.txt");
    let mut comp = ProcIntCode::new(program.clone(), vec![2]);
    comp.run();
    println!("Day 09, Part 2: {:?}", comp.out());
    assert_eq!(vec![78831], *comp.out());
}