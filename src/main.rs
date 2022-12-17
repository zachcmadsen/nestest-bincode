use std::{
    fs::{self, File},
    io::BufWriter,
};

use bincode::Encode;

#[derive(Encode)]
struct State {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    cycles: u64,
}

impl State {
    fn from_log_line(log_line: &str) -> State {
        let pc = u16::from_str_radix(&log_line[0..4], 16).unwrap();

        let mut registers = log_line[48..74].split_ascii_whitespace();
        let a =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let x =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let y =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let p =
            u8::from_str_radix(&registers.next().unwrap()[2..], 16).unwrap();
        let s =
            u8::from_str_radix(&registers.next().unwrap()[3..], 16).unwrap();

        let cycles = log_line[90..].parse::<u64>().unwrap();

        State {
            pc,
            a,
            x,
            y,
            p,
            s,
            cycles,
        }
    }
}

fn main() {
    let nestest_log = fs::read_to_string("nestest.log").unwrap();

    let states: Vec<State> =
        nestest_log.lines().map(State::from_log_line).collect();

    let out_file = File::create("nestest.log.bincode").unwrap();
    let mut buf_writer = BufWriter::new(out_file);
    bincode::encode_into_std_write(
        states,
        &mut buf_writer,
        bincode::config::standard(),
    )
    .unwrap();
}
