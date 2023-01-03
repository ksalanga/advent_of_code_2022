use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{fs, thread};

struct CPU {
    cycle: usize,
    x_register: i32,
    transmitter: Sender<(usize, i32)>,
}

impl CPU {
    fn new(transmitter: Sender<(usize, i32)>) -> CPU {
        CPU {
            cycle: 1,
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

    let mut cpu = CPU::new(tx);

    let receiver_handle = thread::spawn(move || signal_strength_receiver(rx));

    for instruction in cpu_instructions.lines() {
        if cpu.cycle > 220 {
            break;
        }

        cpu.execute(instruction);
    }

    drop(cpu);

    receiver_handle.join().unwrap()
}

fn signal_strength_receiver(receiver: Receiver<(usize, i32)>) {
    let mut total_signal_strength = 0;
    let mut i = 0;

    for cpu_state in receiver {
        let (cycle, x_register) = cpu_state;

        if cycle % (20 + 40 * i) == 0 {
            let signal_strength: i32 = cycle as i32 * x_register;
            total_signal_strength += signal_strength;
            i += 1;
        }
    }

    println!("Final Signal Strength: {}", total_signal_strength);
}
