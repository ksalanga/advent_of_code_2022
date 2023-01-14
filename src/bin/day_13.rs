use packet_13_22::datum::PacketDatum;
use std::fs;

fn main() {
    let file_path_from_src = "./inputs/day_13/input.txt";
    let packet_pairs: String = fs::read_to_string(file_path_from_src).unwrap();

    let packet_pairs = packet_pairs.split("\n\r\n");

    let mut right_order: usize = 0;

    for (idx, packet_pair) in packet_pairs.enumerate() {
        let mut packet_pair = packet_pair.lines();

        let packet_1: PacketDatum = packet_pair.next().unwrap().parse().unwrap();
        let packet_2: PacketDatum = packet_pair.next().unwrap().parse().unwrap();

        if packet_1 < packet_2 {
            right_order += idx + 1;
        }
    }

    println!("Right order pairs: {}", right_order);
}
