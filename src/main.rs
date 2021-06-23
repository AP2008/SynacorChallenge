use itertools::Itertools;
use std::fs;
use std::io::{self, Read};

fn read_file(path: &str) -> Vec<u16> {
  let buf = fs::read(path).unwrap();
  return buf.iter()
    .tuples()
    .map(|(&a,&b)| (b as u16) << 8 | a as u16)
    .collect::<Vec<_>>();
}

fn read_val(register: &[usize], value: u16) -> usize{
    if value > 32767 {
        return register[(value % 32768) as usize] as usize;
    } else {
        return value as usize;
    }
}

fn execute_prog(memory: &mut Vec<u16>) {
    let mut register: [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut stack: Vec<u16> = Vec::new();
    let mut index = 0;
    while true {
//        println!("{}, {}, {}, {}, {}", index, memory[index], memory[index+1], memory[index+2], memory[index+3]);
        let remaining = memory.len() - index - 1;
        let mut addr = 0;
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        if remaining >= 3 {
            addr = read_val(&register, memory[index]);
            a    = read_val(&register, memory[index + 1]);
            b    = read_val(&register, memory[index + 2]);
            c    = read_val(&register, memory[index + 3]);
        } else if remaining >= 2 {
            addr = read_val(&register, memory[index]);
            a    = read_val(&register, memory[index + 1]);
            b    = read_val(&register, memory[index + 2]);
        } else if remaining >= 1 {
            addr = read_val(&register, memory[index]);
            a    = read_val(&register, memory[index + 1]);
        } else if remaining >= 0 {
            addr = read_val(&register, memory[index]);
        } else {
            break;
        }
//        println!("{}, {}, {}, {}, {}", index, addr, a , b, c);
        match addr {
            0 => break,
            1 => {
                register[(memory[index+1] % 32768) as usize] = b;
                index += 3;
            },
            2 => {
                stack.push(a as u16);
                index += 2;
            },
            3 => {
                register[(memory[index+1] % 32768) as usize] = stack.pop().unwrap() as usize;
                index += 2;
            },
            4 => {
                if b == c {
                    register[(memory[index+1] % 32768) as usize] = 1;
                } else {
                    register[(memory[index+1] % 32768) as usize] = 0;
                }
                index += 4;
            },
            5 => {
                if b > c {
                    register[(memory[index+1] % 32768) as usize] = 1;
                } else {
                    register[(memory[index+1] % 32768) as usize] = 0;
                }
                index += 4;
            },
            6 => {
                index = a;
            },
            7 => {
                if a != 0 {
                    index = b;
                } else {
                    index += 3;
                }
            },
            8 => {
                if a == 0 {
                    index = b;
                } else {
                    index += 3;
                }
            },
            9 => {
                register[(memory[index+1] % 32768) as usize] = (b + c) % 32768;
                index += 4;
            },
            10 => {
                register[(memory[index+1] % 32768) as usize] = (b * c) % 32768;
                index += 4;
            },
            11 => {
                register[(memory[index+1] % 32768) as usize] = b % c;
                index += 4;
            }
            12 => {
                register[(memory[index+1] % 32768) as usize] = b & c;
                index += 4;
            },
            13 => {
                register[(memory[index+1] % 32768) as usize] = b | c;
                index += 4;
            },
            14 => {
                register[(memory[index+1] % 32768) as usize] = ((!b) % 32768) as u16 as usize;
                index += 3;
            },
            15 => {
                register[(memory[index+1] % 32768) as usize] = memory[b] as usize;
                index += 3;
            },
            16 => {
                let temp = a;
                memory[temp as usize] = read_val(&register, memory[index+2]) as u16;
                index += 3;
            },
            17 => {
                stack.push((index + 2) as u16);
                index = read_val(&register, memory[index+1]);
            },
            18 => index = stack.pop().unwrap() as usize,
            19 => {
                print!("{}", a as u8 as char);
                index += 2;
            },
            20 => {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let mut start = memory[index+1] % 32768;
                for n in start..(8 - start) {
                    register[n as usize] = buffer.as_bytes()[(n-memory[index+1]) as usize] as usize;
                }
                println!("KAWA");
                index += 2;
            },
            21 => index += 1,
            _ => ()
        }
    }
}

fn main() {
    let mut memory = read_file("challenge.bin");
    execute_prog(&mut memory);
}
