use std::io;

const ARGS1: [u16; 6] = [2, 3, 6, 17, 19, 20];     // ops that have one arg
const ARGS2: [u16; 6] = [1, 7, 8, 14, 15, 16];     // ops that have two ARGS
const ARGS3: [u16; 7] = [4, 5, 9, 10, 11, 12, 13]; // ops that have three ARGS
const JMP: [u16; 5] = [6, 7, 8, 17, 18];
const USEREGA: [u16; 12] = [1, 3, 4, 5, 9, 10, 11, 12, 13, 14, 15, 20];

/// State of execution
enum State {
    Running,
    Halted,
    Error
}

/// Input buffer
struct Buffer {
    data: Vec<u8>, // Input as bytes
    index: i32     // Current char index
}

impl Buffer {
    /// New input buffer
    fn new() -> Buffer {
        return Buffer {
            data: Vec::new(),
            index: -1
        }
    }
    /// Current elem; inc index
    fn next(&mut self) -> Option<usize> {
        if (self.index == -1) || (self.index >= self.data.len() as i32) {
            return None;
        } else {
            self.index += 1;
            return Some((self.data[(self.index-1) as usize]) as usize);
        }
    }
    /// Add data
    fn allocate(&mut self, data: Vec<u8>) {
        self.data = data;
        self.index = 0;
    }
}

/// Virtual machine
pub struct Vm {
    state: State,         // Current state
    memory: Vec<u16>,     // Program addresses
    register: [usize; 8], // Registers
    stack: Vec<u16>,      // Stack
    buffer: Buffer,       // Input buffer as bytes
    iptr: usize,          // Instruction pointer
}


impl Vm {
    /// New vm
    pub fn new(memory: Vec<u16>) -> Vm {
        return Vm {
            state: State::Running,
            memory: memory,
            register: [0, 0, 0, 0, 0, 0, 0, 0],
            stack: Vec::new(),
            buffer: Buffer::new(),
            iptr: 0
        }
    }
    /// Read value
    fn read_val(&mut self, value: u16) -> isize {
        if value > 32775 {
            self.state = State::Error;
            panic!("Value > 32775");
        } else if value > 32767 {
            return self.register[(value % 32768) as usize] as isize;
        } else {
            return value as isize;
        }
    }
    /// Execute current instruction
    fn execute_once(&mut self) {
        let instruction = self.memory[self.iptr];          // Current instruction
        let mut inc = 1;
        let mut a: usize = 0;
        let mut b: usize = 0;
        let mut c: usize = 0;
        let mut rega = &mut 0;
        if ARGS1.contains(&instruction) {
            inc += 1;
            a = self.read_val(self.memory[self.iptr+1]) as usize;
        } else if ARGS2.contains(&instruction) {
            inc += 2;
            a = self.read_val(self.memory[self.iptr+1]) as usize;
            b = self.read_val(self.memory[self.iptr+2]) as usize;
        } else if ARGS3.contains(&instruction) {
            inc += 3;
            a = self.read_val(self.memory[self.iptr+1]) as usize;
            b = self.read_val(self.memory[self.iptr+2]) as usize;
            c = self.read_val(self.memory[self.iptr+3]) as usize;
        }
        if USEREGA.contains(&instruction) {
            rega = &mut self.register[(self.memory[self.iptr + 1] % 32768) as usize];
        }
        match instruction {
            0 => self.state = State::Halted,
            1 => *rega = b,
            2 => self.stack.push(a as u16),
            3 => match self.stack.pop() {
                Some(val) => *rega = val as usize,
                None => {
                    self.state = State::Error;
                    panic!("Stack empty");
                }
            },
            4 => *rega = if b == c {1} else {0},
            5 => *rega = if b > c {1} else {0},
            6 => self.iptr = a,
            7 => if a != 0 {self.iptr = b} else {self.iptr += 3},
            8 => if a == 0 {self.iptr = b} else {self.iptr += 3},
            9 => *rega = (b + c) % 32768,
            10 => *rega = (b * c) % 32768,
            11 => *rega = b % c,
            12 => *rega = b & c,
            13 => *rega = b | c,
            14 => *rega = ((!b) % 32768) as usize,
            15 => *rega = self.memory[b] as usize,
            16 => self.memory[a] = b as u16,
            17 => {
                self.stack.push((self.iptr + 2) as u16);
                self.iptr = a;
            },
            18 => self.iptr = match self.stack.pop() {
                Some(val) => val as usize,
                None      => {
                    self.state = State::Error;
                    panic!("Stack is empty");
                }
            },
            19 => print!("{}", a as u8 as char),
            20 => match self.buffer.next() {
                Some(val) => *rega = val,
                None      => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    self.buffer.allocate(input.as_bytes().to_vec());
                    *rega = self.buffer.next().unwrap();
                }
            },
            21 => self.iptr = self.iptr,
            _ => self.state = State::Error
        }
        if !JMP.contains(&instruction) {
            self.iptr += inc;
        }
    }
    /// Run the program
    pub fn execute(mut self) {
        while let State::Running = self.state {
            self.execute_once();
        }
    }
}
