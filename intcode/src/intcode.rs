
#[derive(PartialEq)]
pub enum State {
    Running,
    Stopped,
}

pub struct IntProgram {
    intcodes : Vec<usize>,
    step : usize,
    state : State,
}

impl IntProgram {
    
    ///
    /// Crates an intcode program with the provided vector
    /// 
    /// Returns None if vector is empty or has less than 4 codes and first code isn't 99
    /// 
    pub fn from(intcodes : Vec<usize>) -> Option<IntProgram> {
        if intcodes.len() == 0 {
            return None
        }
        if intcodes[0] != 99 && intcodes.len() < 4 {
            return None
        }
        let program = IntProgram {
            intcodes,
            step : 0,
            state : State::Running,
        };
        Some(program)
    }

    fn execute(&mut self, (code, in1, in2, out) : (usize, usize, usize, usize)) {
        match code {
            1 => self.intcodes[out] = self.intcodes[in1] + self.intcodes[in2],
            2 => self.intcodes[out] = self.intcodes[in1] * self.intcodes[in2],
            _ => self.state = State::Stopped,
        }
    }

    ///
    /// Performs one step
    /// 
    /// Returns false if no action was performed
    /// 
    pub fn step(&mut self) -> bool {
        if self.state == State::Stopped {
            return false
        }

        if self.intcodes[self.step * 4] == 99 {
            self.state = State::Stopped;
            return true
        }

        let instruction = (
            self.intcodes[self.step * 4],
            self.intcodes[self.step * 4 + 1],
            self.intcodes[self.step * 4 + 2],
            self.intcodes[self.step * 4 + 3],
        );

        self.execute(instruction);

        self.step += 1;

        if self.intcodes.len() < self.step * 4 + 3 {
            self.state = State::Stopped;
        }

        true
    }

    ///
    /// Returns true if program is able to perform next step
    /// 
    pub fn running(&self) -> bool {
        self.state == State::Running
    }

    ///
    /// Runs for as long as it can
    /// 
    /// Returns amount of steps performed before it was stopped
    /// 
    pub fn run(&mut self) -> usize {
        let mut steps : usize = 0;
        if !self.running() {
            return steps
        }

        while self.step() {
            steps += 1;
        }
        steps
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for int in self.intcodes.iter() {
            string.push_str(&int.to_string()[..]);
            string.push(',');
        }
        string.pop();
        string
    }

    ///
    /// Returns intcode at target position
    /// 
    pub fn at(&self, pos : usize) -> usize {
        self.intcodes[pos]
    }
}