// we a have a packet that contains a block of: lists of integers, integers, or nothing

// enum PacketDatum:
// - List(Box<Vec<PacketDatum>>)
// - Integer(i32)

// [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
/// [1,1,5,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

// [[1],[2,3,4]] => Vec<PacketDatum>: [List, List]
// [[1],4] => Vec<PacketDatum>: [List, Integer]

// can only compare lists with lists and integers with integers.
// if we compare a list with an integer, that integer, needs to become a list.

fn main() {
    todo!();
}
