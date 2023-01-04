use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

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

struct Item {
    worry_level: i32,
}

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

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(&mut Item)>,
    throw_to_monkey: Box<dyn Fn(&Item) -> i32>,
    friends: Rc<RefCell<Vec<RefCell<Monkey>>>>,
}

impl Monkey {
    fn new_dummy(friends: &Rc<RefCell<Vec<RefCell<Monkey>>>>) -> Monkey {
        let item = Item { worry_level: 1 };
        let items = vec![item];
        let operation = |item: &mut Item| item.worry_level = item.worry_level + 3;
        let mod_by = 2;
        let monkey_true = 1;
        let monkey_false = 2;

        let throw_to_monkey = move |item: &Item| {
            if item.worry_level % mod_by == 0 {
                monkey_true
            } else {
                monkey_false
            }
        };

        Monkey {
            items,
            operation: Box::new(operation),
            throw_to_monkey: Box::new(throw_to_monkey),
            friends: Rc::clone(&friends),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn throw(&mut self) {
        let mut item_to_throw = self.items.remove(0);

        (self.operation)(&mut item_to_throw);

        let monkey = (self.throw_to_monkey)(&item_to_throw);

        let monkeys = &self.friends.borrow();
        let mut monkey = monkeys[monkey as usize].borrow_mut();

        monkey.add_item(item_to_throw);
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monkey_throws_to_monkey() {
        let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

        for _ in 0..3 {
            let monkey = Monkey::new_dummy(&monkeys);

            monkeys.as_ref().borrow_mut().push(RefCell::new(monkey));
        }

        // monkey 0 throws item by:
        // - removing first item in its list of items
        //      - new_dummy creates one item in items with a worry level of 1 so item so the first item removed has a worry level of 1.
        // 1. perform operation on item:
        // - new_dummy Monkey method operation takes the item's worry level and + 3
        // - item should now have worry level = 4
        // 2. test which monkey to throw to based on inspecting the item's worry level after operation
        // - monkey to throw to is decided on if worry level is divisible by 2 in new_dummy
        // - 4 % 2 == 0 so it goes to the monkey_true in the new_dummy Monkey method
        // - monkey_true == 1 in the new_dummy
        // - item should leave monkey 0 and go to monkey 1.

        let monkey_0 = &monkeys.borrow()[0];

        monkey_0.borrow_mut().throw();

        assert_eq!(monkey_0.borrow().items.len(), 0);

        let monkey_1 = &monkeys.borrow()[1];

        assert_eq!(monkey_1.borrow().items.len(), 2);
        assert_eq!(monkey_1.borrow().items[1].worry_level, 4);
    }
}
