use std::fs;
use std::sync::mpsc::Sender;
// struct CPU
// field:
// cycle: usize
// x_register: i32

// method:
// execute(instruction: str)
// case addx:
// cycle += 1
// send (cycle, x_register) message
// cycle += 1
// x_register is new value
// send (cycle, x_register) message
// emit cycle:
// case noop:
// cycle += 1
// send (cycle, x_register) message

// I wanna use threads
// std::sync::mpsc guarantees that messages sent by the sender will be received in order by the receiver.

// Main transmitter Thread:
// create the mpsc channel
// spawns the receiver thread
// and give the receiver thread ownership of receiver on mpsc channel

// creates CPU struct
// reads lines from file,
// CPU struct will send to receiver (cycle, x_register) tuple during execute() of those lines.

// wait for the receiver thread to close.

// Signal Strength receiver Thread:
// for each item received:
// receives tuple of (usize, i32) representing the cycle and register value.
// signal strength = 0.
// if cycle is >= 220
// break out
// sets i = 0.
// if cycle value is % 20 + 40i.
// increment i by 1
// add register value to signal strength

struct CPU {
    cycle: usize,
    x_register: i32,
    transmitter: Sender<(usize, i32)>,
}

impl CPU {
    fn new(transmitter: Sender<(usize, i32)>) -> CPU {
        CPU {
            cycle: 0,
            x_register: 1,
            transmitter,
        }
    }

    fn execute(instruction: &str) {
        todo!()
    }
}

fn main() {
    let file_path_from_src = "./inputs/day_9/input.txt";
    let cpu_instructions: String = fs::read_to_string(file_path_from_src).unwrap();
}
