use crate::IntProgram;
use crate::Result;

#[test]
fn constructor() {
    let prg = IntProgram::from(vec!(1,0,0,0,99)).unwrap();
    for (i, item) in vec!(1,0,0,0,99).iter().enumerate() {
        assert_eq!(item.clone(), prg.at(i));
    }
}

#[test]
fn add() {
    let mut prg = IntProgram::from(vec!(1,0,0,0,99)).unwrap();
    assert_eq!(prg.step(), Result::Ok);
    assert_eq!(prg.at(0), 2);
}

#[test]
fn mul() {
    let mut prg = IntProgram::from(vec!(2,3,0,3,99)).unwrap();
    assert_eq!(prg.step(), Result::Ok);
    assert_eq!(prg.at(3), 6);
}

#[test]
fn mul2() {
    let mut prg = IntProgram::from(vec!(2,4,4,5,99,0)).unwrap();
    assert_eq!(prg.step(), Result::Ok);
    assert_eq!(prg.at(5), 9801);
}

#[test]
fn combined() {
    let mut prg = IntProgram::from(vec!(1,1,1,4,99,5,6,0,99)).unwrap();
    prg.run();
    assert_eq!(prg.at(0), 30);
    assert_eq!(prg.at(4), 2);
}

#[test]
fn large() {
    let mut prg = IntProgram::from(vec!(1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0)).unwrap();
    prg.run();
    assert_eq!(prg.at(0), 3790689);
}

#[test]
fn io() {
    let mut prg = IntProgram::from(vec!(3,0,4,0,99)).unwrap();
    assert_eq!(prg.step(), Result::Input);
    prg.input(412);
    assert_eq!(prg.step(), Result::Output);
    assert_eq!(prg.output(), 412);
    assert_eq!(prg.step(), Result::Stop);
}

#[test]
fn position_mode() {
    let mut prg = IntProgram::from(vec!(1002,4,3,4,33)).unwrap();
    assert_eq!(prg.step(), Result::Ok);
    assert_eq!(prg.step(), Result::Stop);
    assert_eq!(prg.at(4), 99);
}

#[test]
fn eq_pos() {
    let mut prg = IntProgram::from(vec!(3,9,8,9,10,9,4,9,99,-1,8)).unwrap();
    let mut prg2 = prg.clone();
    prg.step();
    prg2.step();
    prg.input(8);
    prg2.input(7);
    prg.step();
    prg2.step();
    prg.step();
    prg2.step();
    assert_eq!(prg.output(), 1);
    assert_eq!(prg2.output(), 0);
}

#[test]
fn eq_imm() {
    let mut prg = IntProgram::from(vec!(3,3,1108,-1,8,3,4,3,99)).unwrap();
    let mut prg2 = prg.clone();
    prg.step();
    prg2.step();
    prg.input(8);
    prg2.input(7);
    prg.step();
    prg2.step();
    prg.step();
    prg2.step();
    assert_eq!(prg.output(), 1);
    assert_eq!(prg2.output(), 0);
}

#[test]
fn lt_pos() {
    let mut prg = IntProgram::from(vec!(3,9,7,9,10,9,4,9,99,-1,8)).unwrap();
    let mut prg2 = prg.clone();
    prg.step();
    prg2.step();
    prg.input(8);
    prg2.input(7);
    prg.step();
    prg2.step();
    prg.step();
    prg2.step();
    assert_eq!(prg.output(), 0);
    assert_eq!(prg2.output(), 1);
}

#[test]
fn lt_imm() {
    let mut prg = IntProgram::from(vec!(3,3,1107,-1,8,3,4,3,99)).unwrap();
    let mut prg2 = prg.clone();
    prg.step();
    prg2.step();
    prg.input(8);
    prg2.input(7);
    prg.step();
    prg2.step();
    prg.step();
    prg2.step();
    assert_eq!(prg.output(), 0);
    assert_eq!(prg2.output(), 1);
}

#[test]
fn if_pos() {
    let mut prg = IntProgram::from(vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9)).unwrap();
    let mut prg2 = prg.clone();
    prg.run();
    prg2.run();
    prg.input(0);
    prg2.input(10);
    prg.run();
    prg2.run();
    assert_eq!(prg.output(), 0);
    assert_eq!(prg2.output(), 1);
}

#[test]
fn if_imm() {
    let mut prg = IntProgram::from(vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1)).unwrap();
    let mut prg2 = prg.clone();
    prg.run();
    prg2.run();
    prg.input(0);
    prg2.input(10);
    prg.run();
    prg2.run();
    assert_eq!(prg.output(), 0);
    assert_eq!(prg2.output(), 1);
}

#[test]
fn if_large() {
    let mut prg = IntProgram::from(vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99)).unwrap();
    let mut prg2 = prg.clone();
    let mut prg3 = prg.clone();
    prg.run();
    prg2.run();
    prg3.run();
    prg.input(7);
    prg2.input(8);
    prg3.input(9);
    prg.run();
    prg2.run();
    prg3.run();
    assert_eq!(prg.output(), 999);
    assert_eq!(prg2.output(), 1000);
    assert_eq!(prg3.output(), 1001);
}