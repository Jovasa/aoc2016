use std::fs::File;
use std::io::BufReader;


enum OpCode {
    COPY,
    JUMP,
    INC,
    DEC
}


struct Op {
    opcode: OpCode,
    first_reg: Option<char>,
    first_val: Option<i32>,
    second_reg: Option<char>,
    second_vla: Option<i32>,
}


fn main() {
    let reader = BufReader::new(File::open("data/day12.txt").unwrap());

    let mut register = [0; 4];


}