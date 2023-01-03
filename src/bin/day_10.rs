use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{fs, thread};
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

    fn execute(&mut self, instruction: &str) {
        let instruction: Vec<&str> = instruction.split_whitespace().collect();

        match instruction[0] {
            "addx" => self.addx(instruction[1].parse().unwrap()),
            "noop" => self.noop(),
            i => panic!("instruction {} does not exist", i),
        }
    }

    fn addx(&mut self, value: i32) {
        self.noop();

        self.cycle += 1;
        self.x_register += value;

        self.transmitter
            .send((self.cycle, self.x_register))
            .unwrap();
    }

    fn noop(&mut self) {
        self.cycle += 1;
        self.transmitter
            .send((self.cycle, self.x_register))
            .unwrap();
    }
}

fn main() {
    let file_path_from_src = "./inputs/day_9/input.txt";
    let cpu_instructions: String = fs::read_to_string(file_path_from_src).unwrap();

    let (tx, rx) = mpsc::channel::<(usize, i32)>();

    let cpu = CPU::new(tx);

    let receiver_handle = thread::spawn(move || signal_receiver(rx));

    for instruction in cpu_instructions.lines() {
        todo!()
    }

    receiver_handle.join().unwrap()
}

fn signal_receiver(receiver: Receiver<(usize, i32)>) {
    for cpu_state in receiver {
        let (cycle, x_register) = cpu_state;
        todo!()
    }
}
