use std::fs;
use intcode_rs::*;
use intcode_rs::procedural_comp::ProcIntCode;
use intcode_rs::polymorphic_comp::PolyIntCode;

fn read(file_name: &str) -> Vec<i64> {
    fs::read_to_string(file_name)
        .unwrap()
        .as_str()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn day2_part1(comp: &mut impl IntCodeComputer) -> i64 {
    comp.run();
    comp.mem(0)
}

#[test]
fn d2p1() {
    let mut program = read("res/02.txt");
    program[1] = 12;
    program[2] = 2;
    let mut proc = ProcIntCode::new(program.clone(), vec![]);
    let mut poly = PolyIntCode::new(program.clone(), vec![]);
    let ans_proc = day2_part1(&mut proc);
    let ans_poly = day2_part1(&mut poly);
    println!("Day 02, Part 1: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(4484226, ans_proc);
    assert_eq!(4484226, ans_poly);
}

fn day2_part2(compfn: fn(Vec<i64>) -> Box<dyn IntCodeComputer>) -> i64 {
    let program = read("res/02.txt");
    let mut answer = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;
            let mut comp = compfn(program);
            comp.run();
            if comp.mem(0) == 19690720 {
                answer = 100 * noun + verb;
                break;
            }
        }
    }
    answer
}

#[test]
fn d2p2() {
    let ans_proc = day2_part2(|program| Box::new(ProcIntCode::new(program, vec![])));
    let ans_poly = day2_part2(|program| Box::new(PolyIntCode::new(program, vec![])));
    println!("Day 02, Part 2: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(5696, ans_proc);
    assert_eq!(5696, ans_poly);
}

fn day5_part1(mut comp: impl IntCodeComputer) -> Vec<i64> {
    comp.run();
    comp.out().clone()
}

#[test]
fn d5p1() {
    let program = read("res/05.txt");
    let proc = ProcIntCode::new(program.clone(), vec![1]);
    let poly = PolyIntCode::new(program.clone(), vec![1]);
    let ans_proc = day5_part1(proc);
    let ans_poly = day5_part1(poly);
    println!("Day 05, Part 1: {:?} proc / {:?} poly", ans_proc, ans_poly);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 14522484], ans_proc);
    assert_eq!(ans_proc, ans_poly);
}

fn day5_part2(mut comp: impl IntCodeComputer) -> i64 {
    comp.run();
    comp.out().get(0).unwrap().clone()
}

#[test]
fn d5p2() {
    let program = read("res/05.txt");
    let proc = ProcIntCode::new(program.clone(), vec![5]);
    let poly = PolyIntCode::new(program.clone(), vec![5]);
    let ans_proc = day5_part2(proc);
    let ans_poly = day5_part2(poly);
    println!("Day 05, Part 2: {:?} proc / {:?} poly", ans_proc, ans_poly);
    assert_eq!(4655956, ans_proc);
    assert_eq!(4655956, ans_poly);
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

fn day7_part1(program: &str, perms: Vec<Vec<i64>>, compfn: Box<dyn Fn(Vec<i64>) -> Box<dyn IntCodeComputer>>) -> i64 {
    let mut res: Vec<(&Vec<i64>, i64)> = vec![];
    let program = read(program);
    for perm in perms.iter() {
        let mut output = 0;
        for digit in perm.iter() {
            let mut comp = compfn(program.clone());
            comp.push(*digit);
            comp.push(output);
            comp.run();
            output = *comp.out().get(0).unwrap();
        }
        res.push((perm, output));
    }
    *res.iter()
        .map(|(_, final_out)| final_out)
        .max()
        .unwrap()
}

#[test]
fn d7p1() {
    let unique_perms = unique_perms(43210, 0, 4);
    let ans_proc = day7_part1("res/07.txt", unique_perms.clone(), Box::new(|program| {
        Box::new(ProcIntCode::new(program, vec![]))
    }));
    let ans_poly = day7_part1("res/07.txt", unique_perms, Box::new(|program| {
        Box::new(PolyIntCode::new(program, vec![]))
    }));
    println!("Day 02, Part 1: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(880726, ans_proc);
    assert_eq!(880726, ans_poly);
}

fn halted(comps: &Vec<Box<dyn IntCodeComputer>>) -> bool {
    comps.iter()
        .map(|comp| match comp.state() {
            State::Halted => true,
            _ => false,
        })
        .fold(false, |acc, c| acc || c)
}

fn day7_part2(program: &str, perms: Vec<Vec<i64>>, compfn: Box<dyn Fn(Vec<i64>) -> Box<dyn IntCodeComputer>>) -> i64 {
    let program = read(program);
    let mut res: Vec<(&Vec<i64>, i64)> = vec![];
    for perm in perms.iter() {
        let mut output = 0;
        let mut comps: Vec<Box<dyn IntCodeComputer>> = Vec::new();
        for digit in perm.iter() {
            let mut comp = compfn(program.clone());
            comp.push(*digit);
            comps.push(comp);
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
    *res.iter()
        .map(|(_, final_out)| final_out)
        .max()
        .unwrap()
}

#[test]
fn d7p2() {
    let unique_perms = unique_perms(98765, 5, 9);
    let ans_proc = day7_part2("res/07.txt", unique_perms.clone(), Box::new(|program| {
        Box::new(ProcIntCode::new(program, vec![]))
    }));
    let ans_poly = day7_part2("res/07.txt", unique_perms, Box::new(|program| {
        Box::new(PolyIntCode::new(program, vec![]))
    }));
    println!("Day 02, Part 2: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(4931744, ans_proc);
    assert_eq!(4931744, ans_poly);
}

fn day9(mut comp: impl IntCodeComputer) -> i64 {
    comp.run();
    *comp.out().get(0).unwrap()
}

#[test]
fn d9p1() {
    let program = read("res/09.txt");
    let proc = ProcIntCode::new(program.clone(), vec![1]);
    let poly = PolyIntCode::new(program.clone(), vec![1]);
    let ans_proc = day9(proc);
    let ans_poly = day9(poly);
    println!("Day 02, Part 1: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(3380552333, ans_proc);
    assert_eq!(3380552333, ans_poly);
}

#[test]
fn d9p2() {
    let program = read("res/09.txt");
    let proc = ProcIntCode::new(program.clone(), vec![2]);
    let poly = PolyIntCode::new(program.clone(), vec![2]);
    let ans_proc = day9(proc);
    let ans_poly = day9(poly);
    println!("Day 02, Part 1: {} proc / {} poly", ans_proc, ans_poly);
    assert_eq!(78831, ans_proc);
    assert_eq!(78831, ans_poly);
}