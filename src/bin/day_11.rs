use core::str::Lines;
use std::cell::RefCell;
use std::num::ParseIntError;
use std::ops::Div;
use std::ptr;
use std::rc::Rc;
use std::str::FromStr;

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
    throw_to_monkey_id: Box<dyn Fn(&Item) -> i32>,
    friends: Rc<RefCell<Vec<RefCell<Monkey>>>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMonkeyError;

// Functions takes in strings for a monkey's behavior:
// ex: starting_items, operation,
// and parse it to give the respective fields needed for a Monkey Struct
struct StringMonkeyParser;

impl StringMonkeyParser {
    fn starting_items(s: &str) -> Result<Vec<Item>, ParseMonkeyError> {
        //   Starting items: 79, 98
        let s: Vec<&str> = s.split(':').collect();

        let s = s.get(1).ok_or(ParseMonkeyError)?;

        let s: Result<Vec<i32>, ParseIntError> = s
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .map(|worry_level| worry_level.parse::<i32>())
            .collect();

        let s = match s {
            Ok(s) => s,
            Err(_) => return Err(ParseMonkeyError),
        };

        let s = s
            .iter()
            .map(|worry_level| Item {
                worry_level: *worry_level,
            })
            .collect();

        Ok(s)
    }

    fn operation(s: &str) -> Result<Box<dyn Fn(&mut Item)>, ParseMonkeyError> {
        let mut s = s.split_whitespace().rev();

        let number = s.next().ok_or(ParseMonkeyError)?;

        let number = match number.parse::<i32>() {
            Ok(number) => number,
            Err(_) => return Err(ParseMonkeyError),
        };

        let operation = s.next().ok_or(ParseMonkeyError)?;

        let add_operation = Box::new(move |item: &mut Item| {
            item.worry_level += number;
            item.worry_level = item.worry_level.div(3);
        });

        let multiply_operation = Box::new(move |item: &mut Item| {
            item.worry_level *= number;
            item.worry_level = item.worry_level.div(3);
        });

        match operation {
            "+" => return Ok(add_operation),
            "*" => return Ok(multiply_operation),
            _ => return Err(ParseMonkeyError),
        }
    }

    fn throw_to_monkey_id(s: &mut Lines) -> Result<Box<dyn Fn(&Item) -> i32>, ParseMonkeyError> {
        let mod_by = s.next().ok_or(ParseMonkeyError)?;
        let mod_by = Self::last_number_in_string(&mod_by)?;

        let true_monkey_index = s.next().ok_or(ParseMonkeyError)?;
        let true_monkey_index = Self::last_number_in_string(&true_monkey_index)?;

        let false_monkey_index = s.next().ok_or(ParseMonkeyError)?;
        let false_monkey_index = Self::last_number_in_string(&false_monkey_index)?;

        Ok(Box::new(move |item: &Item| {
            if item.worry_level % mod_by == 0 {
                true_monkey_index
            } else {
                false_monkey_index
            }
        }))
    }

    fn last_number_in_string(s: &str) -> Result<i32, ParseMonkeyError> {
        let mut s = s.split_whitespace().rev();
        let s = s.next().ok_or(ParseMonkeyError)?;

        return match s.parse::<i32>() {
            Ok(number) => Ok(number),
            Err(_) => return Err(ParseMonkeyError),
        };
    }
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        let mut s = s.lines();
        s.next();

        let starting_items = s.next().ok_or(ParseMonkeyError)?;
        let starting_items = StringMonkeyParser::starting_items(starting_items)?;

        let operation = s.next().ok_or(ParseMonkeyError)?;
        let operation = StringMonkeyParser::operation(operation)?;

        let throw_to_monkey_id = StringMonkeyParser::throw_to_monkey_id(&mut s)?;

        Ok(Monkey {
            items: starting_items,
            operation,
            throw_to_monkey_id,
            friends: Rc::new(RefCell::new(vec![])),
        })
    }
}

impl Monkey {
    fn new_dummy(friends: &Rc<RefCell<Vec<RefCell<Monkey>>>>) -> Monkey {
        let item = Item { worry_level: 1 };
        let items = vec![item];
        let operation = |item: &mut Item| item.worry_level = item.worry_level + 3;
        let mod_by = 2;
        let monkey_true = 1;
        let monkey_false = 2;

        let throw_to_monkey_id = move |item: &Item| {
            if item.worry_level % mod_by == 0 {
                monkey_true
            } else {
                monkey_false
            }
        };

        Monkey {
            items,
            operation: Box::new(operation),
            throw_to_monkey_id: Box::new(throw_to_monkey_id),
            friends: Rc::clone(&friends),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn throw(&mut self) {
        let mut item_to_throw = self.items.remove(0);

        (self.operation)(&mut item_to_throw);

        let target_monkey_id = (self.throw_to_monkey_id)(&item_to_throw);

        let monkeys = &self.friends.borrow();

        let target_monkey = &monkeys[target_monkey_id as usize];

        if ptr::eq(target_monkey.as_ptr(), self) {
            return self.items.push(item_to_throw);
        }

        let mut target_monkey = target_monkey.borrow_mut();

        target_monkey.add_item(item_to_throw);
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_down() {
        let dividend = 1862;
        let divisor = 3;

        let result = dividend.div(divisor);
        assert_eq!(result, 620);
    }

    #[test]
    fn parse_throw_to_true_monkey_id_operation() {
        let monkey_2_throw_operations = "  Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3";

        let mut monkey_2_throw_operations = monkey_2_throw_operations.lines();

        let throw_to_monkey_id =
            StringMonkeyParser::throw_to_monkey_id(&mut monkey_2_throw_operations);

        assert!(throw_to_monkey_id.is_ok());

        let monkey_2_item = Item { worry_level: 2080 };

        let throw_to_monkey_id = throw_to_monkey_id.unwrap()(&monkey_2_item);

        assert_eq!(throw_to_monkey_id, 1);
    }

    #[test]
    fn parse_throw_to_false_monkey_id_operation() {
        let monkey_2_throw_operations = "  Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3";

        let mut monkey_2_throw_operations = monkey_2_throw_operations.lines();

        let throw_to_monkey_id =
            StringMonkeyParser::throw_to_monkey_id(&mut monkey_2_throw_operations);

        assert!(throw_to_monkey_id.is_ok());

        let monkey_2_item = Item { worry_level: 1200 };

        let throw_to_monkey_id = throw_to_monkey_id.unwrap()(&monkey_2_item);

        assert_eq!(throw_to_monkey_id, 3);
    }

    #[test]
    fn parse_multiply_operation() {
        let operation = "Operation: new = old * 19";
        let operation = StringMonkeyParser::operation(operation);

        assert!(operation.is_ok());

        let mut item = Item { worry_level: 98 };
        operation.unwrap()(&mut item);

        assert_eq!(item.worry_level, 620);
    }

    #[test]
    fn parse_add_operation() {
        let operation = "Operation: new = old + 6";
        let operation = StringMonkeyParser::operation(operation);

        assert!(operation.is_ok());

        let mut item = Item { worry_level: 54 };
        operation.unwrap()(&mut item);

        assert_eq!(item.worry_level, 20);
    }

    #[test]
    fn split() {
        let s = " 79, 98";
        let s: Vec<&str> = s
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect();

        assert_eq!(s.len(), 2);
    }

    #[test]
    fn parse_starting_items() {
        let monkey_str = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey: Result<Monkey, ParseMonkeyError> = Monkey::from_str(monkey_str);

        assert!(monkey.is_ok());

        let monkey = monkey.unwrap();

        let monkey_items = monkey.items;

        assert_eq!(monkey_items.len(), 2);
        assert_eq!(monkey_items[0].worry_level, 79);
        assert_eq!(monkey_items[1].worry_level, 98);
    }

    #[test]
    fn parse_incorrect_starting_items() {
        let monkey_str = "Monkey 0:
        Starting items: 79, 98, F
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey: Result<Monkey, ParseMonkeyError> = Monkey::from_str(monkey_str);

        assert!(monkey.is_err());
    }

    #[test]
    fn parsed_monkey_throws_to_monkey() {
        todo!()
    }

    #[test]
    fn dummy_monkey_throws_to_monkey() {
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
