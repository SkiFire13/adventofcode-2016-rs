#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Clone, Copy)]
pub enum Instruction {
    Cpy(RegOrImm, RegOrImm),
    Inc(RegOrImm),
    Dec(RegOrImm),
    Jnz(RegOrImm, RegOrImm),
    Tgl(RegOrImm),
}

#[derive(Clone, Copy)]
pub enum RegOrImm {
    Reg(usize),
    Imm(i64),
}

fn parse_reg(src: &str) -> usize {
    match src {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register"),
    }
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
                    .unwrap_or_else(|_| RegOrImm::Reg(parse_reg(src)));
                let dst = RegOrImm::Reg(parse_reg(dst));
                Instruction::Cpy(src, dst)
            }
            "inc" => Instruction::Inc(RegOrImm::Reg(parse_reg(&line[4..]))),
            "dec" => Instruction::Dec(RegOrImm::Reg(parse_reg(&line[4..]))),
            "jnz" => {
                let (src, offset) = line[4..].split_once(" ").expect("Invalid input");
                let src = src
                    .parse()
                    .map(RegOrImm::Imm)
                    .unwrap_or_else(|_| RegOrImm::Reg(parse_reg(src)));
                let offset = offset
                    .parse()
                    .map(RegOrImm::Imm)
                    .unwrap_or_else(|_| RegOrImm::Reg(parse_reg(offset)));
                Instruction::Jnz(src, offset)
            }
            "tgl" => Instruction::Tgl(
                line[4..]
                    .parse()
                    .map(RegOrImm::Imm)
                    .unwrap_or_else(|_| RegOrImm::Reg(parse_reg(&line[4..]))),
            ),
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn execute_with_state(program: &mut [Instruction], mut state: [i64; 4]) -> i64 {
    let mut ip = 0;

    while (ip as usize) < program.len() {
        use Instruction::*;
        use RegOrImm::*;
        let value_of = |reg_or_imm| match reg_or_imm {
            Reg(reg) => state[reg],
            Imm(imm) => imm,
        };
        match program[ip as usize] {
            Cpy(src, Reg(dst)) => state[dst] = value_of(src),
            Cpy(_, Imm(_)) => {}
            Inc(Reg(reg)) => state[reg] += 1,
            Dec(Reg(reg)) => state[reg] -= 1,
            Inc(Imm(_)) | Dec(Imm(_)) => {}
            Jnz(cond, offset) if value_of(cond) != 0 => ip += value_of(offset) - 1,
            Jnz(_, _) => {}
            Tgl(delta) => {
                let addr = ip + value_of(delta);
                if addr >= 0 && addr < program.len() as i64 {
                    program[addr as usize] = match program[addr as usize] {
                        Inc(reg_or_imm) => Dec(reg_or_imm),
                        Dec(reg_or_imm) => Inc(reg_or_imm),
                        Tgl(reg_or_imm) => Inc(reg_or_imm),
                        Jnz(left, right) => Cpy(left, right),
                        Cpy(left, right) => Jnz(left, right),
                    };
                }
            }
        }
        ip += 1;
    }

    state[0]
}

pub fn part1(input: &Input) -> i64 {
    execute_with_state(&mut input.clone(), [7, 0, 0, 0])
}

pub fn part2(input: &Input) -> i64 {
    execute_with_state(&mut input.clone(), [12, 0, 0, 0])
}
