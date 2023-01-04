// Parsing:
// split the input by the blank spaces:
// ("\n\r\n")

// ex:

// splitting by "\n\r\n" on input:

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
//
// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0

// gets us an array of size 2:
// [Monkey 0: Starting items:...  , Monkey 1: Starting items:...]

// Structs:

// Item Struct
// fields:
// - worry level: i32

// Monkey Struct
// fields:
// - items: Vec<Item>
// - operation: Closure fn(&mut Item)
// - test: Closure fn(&Item) -> bool
// - throw: Closure fn(test Closure, Item, &Vec<Monkey>) -> bool
// - friends: &Vec<&Monkey>

// methods:
// - new(behavior: &str, friends: &mut Vec<&Monkey>) -> Monkey
//      - get the behavior.lines()
//      - get starting items in behavior line 1 (Starting items:)
//      - create operation closure in behavior line 2: (either * or +) and then / 3 rounded down
//      - create test closure in behavior line 3: closure fn = Item.value % parsed_num == 0
//      - get monkey_true_index = line 4
//      - get monkey_false_index = line 5
//      - create throw closure:
//          - if test closure true: add Item to Vec[monkey_true_index]
//          - else: add Item to Vec[monkey_false_index]
//      - friends vector is a reference to the friends input parameter

// - inspect_items(&mut self)
//      - for each item in items:
//          - self.operation(&mut Item)
//          - self.throw(self.test, Item, self.friends)

fn main() {
    todo!()
}
