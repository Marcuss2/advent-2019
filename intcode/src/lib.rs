use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum State {
    Running,
    AwaitingInput,
    AwaitingOutput,
    Stopped,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Result {
    Ok,
    Stop,
    Input,
    Output,
}

#[derive(Debug, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
enum Instruction {
    Add = 1,
    Mul = 2,
    In = 3,
    Out = 4,
    JIT = 5,
    JIF = 6,
    LT = 7,
    EQ = 8,
    Stop = 99,
}

impl Instruction {
    fn needed_codes(&self) -> i32 {
        match self {
            Instruction::Add => 4,
            Instruction::Mul => 4,
            Instruction::In => 2,
            Instruction::Out => 2,
            Instruction::JIT => 3,
            Instruction::JIF => 3,
            Instruction::LT => 4,
            Instruction::EQ => 4,
            Instruction::Stop => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntProgram {
    intcodes : Vec<i32>,
    pc : i32,
    state : State,
}


impl IntProgram {
    
    ///
    /// Crates an intcode program with the provided vector
    /// 
    /// Returns None if vector is empty or has less than 4 codes and first code isn't 99
    /// 
    pub fn from(intcodes : Vec<i32>) -> Option<IntProgram> {
        if intcodes.len() == 0 {
            return None
        }
        let program = IntProgram {
            intcodes,
            pc : 0,
            state : State::Running,
        };
        Some(program)
    }

    fn get_value(&self, value : i32, mode : Mode) -> i32 {
        match mode {
            Mode::Position => self.intcodes[value as usize],
            Mode::Immediate => value,
        }
    }

    fn decode(code : i32) -> (Instruction, [Mode; 3]) {
        let instruction = Instruction::try_from((code % 100) as u8).expect("Tryfrom failed");
        let mode1 = Mode::try_from(((code / 100) % 10) as u8).expect("Tryfrom failed");
        let mode2 = Mode::try_from(((code / 1000) % 10) as u8).expect("Tryfrom failed");
        let mode3 = Mode::try_from(((code / 10000) % 10) as u8).expect("Tryfrom failed");
        let mode = [mode1, mode2, mode3];
        (instruction, mode)
    }

    fn execute(&mut self, instruction : &Instruction, codes : &[i32; 3], modes : &[Mode; 3]) {
        match instruction {
            Instruction::Add => self.intcodes[codes[2] as usize] = self.get_value(codes[0], modes[0]) + self.get_value(codes[1], modes[1]),
            Instruction::Mul => self.intcodes[codes[2] as usize] = self.get_value(codes[0], modes[0]) * self.get_value(codes[1], modes[1]),
            Instruction::In => self.state = State::AwaitingInput,
            Instruction::Out => self.state = State::AwaitingOutput,
            Instruction::JIT => self.pc = if self.get_value(codes[0], modes[0]) != 0 {self.get_value(codes[1], modes[1]) - 3} else {self.pc},
            Instruction::JIF => self.pc = if self.get_value(codes[0], modes[0]) == 0 {self.get_value(codes[1], modes[1]) - 3} else {self.pc},
            Instruction::LT => self.intcodes[codes[2] as usize] = if self.get_value(codes[0], modes[0]) < self.get_value(codes[1], modes[1]) {1} else {0},
            Instruction::EQ => self.intcodes[codes[2] as usize] = if self.get_value(codes[0], modes[0]) == self.get_value(codes[1], modes[1]) {1} else {0},
            Instruction::Stop => self.state = State::Stopped,
        }
    }

    pub fn input(&mut self, in1 : i32) {
        if self.state != State::AwaitingInput {
            panic!("No input needed");
        }
        self.state = State::Running;
        let (_, modes) = IntProgram::decode(self.intcodes[self.pc as usize - 2]);
        if modes[0] == Mode::Immediate {
            self.intcodes[self.pc as usize - 1] = in1;
        } else {
            let i = self.intcodes[self.pc as usize - 1] as usize;
            self.intcodes[i] = in1;
        }
    }

    pub fn output(&mut self) -> i32 {
        if self.state != State::AwaitingOutput {
            panic!("No output needed");
        }
        self.state = State::Running;
        let (_, modes) = IntProgram::decode(self.intcodes[self.pc as usize - 2]);
        if (modes[0]) == Mode::Immediate {
            return self.intcodes[self.pc as usize - 1]
        }
        self.intcodes[self.intcodes[self.pc as usize - 1] as usize]
    }

    fn state_to_result(state : State) -> Result {
        let s = match state {
            State::Running => Result::Ok,
            State::AwaitingInput => Result::Input,
            State::AwaitingOutput => Result::Output,
            State::Stopped => Result::Stop,
        };
        s
    }

    pub fn get_result(&self) -> Result {
        IntProgram::state_to_result(self.state)
    }

    ///
    /// Performs one step
    /// 
    /// Returns Result based on result of the step
    /// 
    pub fn step(&mut self) -> Result {
        if self.state != State::Running {
            return self.get_result()
        }

        let (instruction, modes) = IntProgram::decode(self.intcodes[self.pc as usize]);
        if self.intcodes.len() < self.pc as usize + instruction.needed_codes() as usize {
            self.state = State::Stopped;
            return Result::Stop
        }
        let mut codes = [0, 0, 0];
        for i in 0..(instruction.needed_codes() - 1) {
            codes[i as usize] = self.intcodes[self.pc as usize + i as usize + 1];
        }

        self.execute(&instruction, &codes, &modes);

        self.pc += instruction.needed_codes();

        self.get_result()
    }

    ///
    /// Returns true if program is able to perform next step
    /// 
    pub fn running(&self) -> bool {
        self.state != State::Stopped
    }

    ///
    /// Runs for as long as it can
    /// 
    /// Returns amount of steps performed before it was stopped
    /// 
    pub fn run(&mut self) -> i32 {
        let mut steps : i32 = 0;
        if !self.running() {
            return steps
        }

        while self.step() == Result::Ok {
            steps += 1;
        }
        steps
    }

    ///
    /// Returns intcode at target position
    /// 
    pub fn at(&self, pos : usize) -> i32 {
        self.intcodes[pos]
    }
}
