#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Clone, Copy)]
pub enum Instruction {
    Cpy(RegOrImm, Register),
    Inc(Register),
    Dec(Register),
    Jnz(RegOrImm, isize),
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn from_str(s: &str) -> Self {
        match s {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "d" => Self::D,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RegOrImm {
    Reg(Register),
    Imm(i64),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match &line[..3] {
            "cpy" => {
                let (src, dst) = line[4..].split_once(" ").expect("Invalid input");
                let src = src
                    .parse()
                    .map(RegOrImm::Imm)
                    .unwrap_or_else(|_| RegOrImm::Reg(Register::from_str(src)));
                let dst = Register::from_str(dst);
                Instruction::Cpy(src, dst)
            }
            "inc" => Instruction::Inc(Register::from_str(&line[4..])),
            "dec" => Instruction::Dec(Register::from_str(&line[4..])),
            "jnz" => {
                let (src, offset) = line[4..].split_once(" ").expect("Invalid input");
                let src = src
                    .parse()
                    .map(RegOrImm::Imm)
                    .unwrap_or_else(|_| RegOrImm::Reg(Register::from_str(src)));
                let offset = offset.parse().expect("Invalid input");
                Instruction::Jnz(src, offset)
            }
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn execute_with_state(program: &[Instruction], mut state: [i64; 4]) -> i64 {
    let mut ip = 0;

    while (ip as usize) < program.len() {
        use Instruction::*;
        match program[ip as usize] {
            Cpy(RegOrImm::Reg(src), dst) => state[dst as usize] = state[src as usize],
            Cpy(RegOrImm::Imm(val), dst) => state[dst as usize] = val,
            Inc(reg) => state[reg as usize] += 1,
            Dec(reg) => state[reg as usize] -= 1,
            Jnz(RegOrImm::Reg(reg), offset) if state[reg as usize] != 0 => ip += offset - 1,
            Jnz(RegOrImm::Imm(imm), offset) if imm != 0 => ip += offset - 1,
            Jnz(_, _) => {}
        }
        ip += 1;
    }

    state[0]
}

pub fn part1(input: &Input) -> i64 {
    execute_with_state(input, [0; 4])
}

pub fn part2(input: &Input) -> i64 {
    execute_with_state(input, [0, 0, 1, 0])
}
