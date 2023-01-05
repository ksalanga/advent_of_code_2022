use core::str::Lines;
use std::cell::RefCell;
use std::fs;
use std::num::ParseIntError;
use std::ops::Div;
use std::ptr;
use std::rc::Rc;
use std::str::FromStr;

struct Item {
    worry_level: i32,
}

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(&mut Item)>,
    throw_to_monkey_id: Box<dyn Fn(&Item) -> i32>,
    friends: Rc<RefCell<Vec<RefCell<Monkey>>>>,
    inspection_count: usize,
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
        fn remove_spaces(string: &str) -> String {
            string.replace(" ", "")
        }

        fn is_alphabetic(string: &str) -> bool {
            for ch in string.chars() {
                if !ch.is_alphabetic() {
                    return false;
                }
            }
            true
        }

        //  Operation: new = old + 3
        let s: Vec<&str> = s.split("=").collect();

        let equation = s.get(1).ok_or(ParseMonkeyError)?;
        let equation = remove_spaces(equation);

        let operation = move |item: &mut Item| {
            let ops = ['+', '-', '*', '/'];
            let values: Vec<i32> = equation
                .split(&ops)
                .map(|v| {
                    if is_alphabetic(v) {
                        item.worry_level
                    } else {
                        v.trim().parse().unwrap()
                    }
                })
                .collect();
            let operands: Vec<_> = equation.matches(&ops).collect();

            let (&(mut curr), values) = values.split_first().unwrap();
            for (op, &value) in operands.into_iter().zip(values) {
                match op {
                    "+" => curr = curr + value,
                    "-" => curr = curr - value,
                    "*" => curr = curr * value,
                    "/" => curr = curr / value,
                    _ => unreachable!(),
                }
            }

            item.worry_level = curr.div(3);
        };

        Ok(Box::new(operation))
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
            inspection_count: 0,
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
            inspection_count: 0,
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

    fn inspect_and_throw_all_items(&mut self) {
        while !self.items.is_empty() {
            self.inspection_count += 1;
            self.throw();
        }
    }
}

fn spawn_monkeys() -> Rc<RefCell<Vec<RefCell<Monkey>>>> {
    let file_path_from_src = "./inputs/day_11/example.txt";
    let monkeys: String = fs::read_to_string(file_path_from_src).unwrap();

    let monkey_strings: Vec<&str> = monkeys.split("\n\n").collect();

    let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

    for monkey_string in monkey_strings {
        let mut monkey = monkey_string.parse::<Monkey>().unwrap();

        monkey.friends = Rc::clone(&monkeys);

        monkeys.borrow_mut().push(RefCell::new(monkey));
    }

    monkeys
}

fn main() {
    let monkeys = spawn_monkeys();

    for round in 0..20 {
        for monkey in monkeys.borrow().iter() {
            monkey.borrow_mut().inspect_and_throw_all_items();
        }
    }

    let mut highest_monkey_inspections: Vec<usize> = Vec::new();

    for monkey in monkeys.borrow().iter() {
        highest_monkey_inspections.push(monkey.borrow().inspection_count);
    }

    highest_monkey_inspections.sort();
    highest_monkey_inspections.reverse();

    let monkey_business = highest_monkey_inspections[0] * highest_monkey_inspections[1];

    println!("monkey business: {}", monkey_business);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_by_empty_space_line() {
        let file_path_from_src = "./inputs/day_11/input.txt";
        let monkeys: String = fs::read_to_string(file_path_from_src).unwrap();

        let monkey_strings: Vec<&str> = monkeys.split("\n\n").collect();

        //     let input = "Monkey 0:
        //     Starting items: 79, 98
        //     Operation: new = old * 19
        //     Test: divisible by 23
        //       If true: throw to monkey 2
        //       If false: throw to monkey 3

        //   Monkey 1:
        //     Starting items: 54, 65, 75, 74
        //     Operation: new = old + 6
        //     Test: divisible by 19
        //       If true: throw to monkey 2
        //       If false: throw to monkey 0

        //   Monkey 2:
        //     Starting items: 79, 60, 97
        //     Operation: new = old * old
        //     Test: divisible by 13
        //       If true: throw to monkey 1
        //       If false: throw to monkey 3

        //   Monkey 3:
        //     Starting items: 74
        //     Operation: new = old + 3
        //     Test: divisible by 17
        //       If true: throw to monkey 0
        //       If false: throw to monkey 1
        //   ";

        let output: Vec<&str> = monkeys.split("\n\n").collect();

        assert_eq!(output.len(), 4);
    }

    #[test]
    fn remove_spaces() {
        fn remove_spaces(string: &str) -> String {
            string.replace(" ", "")
        }

        let s = "  Operation: new = old + 3";

        //  Operation: new = old + 3
        let s: Vec<&str> = s.split("=").collect();

        let operation = s[1];

        let operation = remove_spaces(operation);

        assert_eq!(operation, "old+3");
    }
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
        let monkey_0 = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey_1 = "Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0";

        let monkey_2 = "Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

        let monkey_3 = "Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";

        let monkey_strings = vec![monkey_0, monkey_1, monkey_2, monkey_3];

        let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

        for monkey_string in monkey_strings {
            let mut monkey = monkey_string.parse::<Monkey>().unwrap();

            monkey.friends = Rc::clone(&monkeys);

            monkeys.borrow_mut().push(RefCell::new(monkey));
        }

        let monkey_0 = &monkeys.borrow()[0];

        monkey_0.borrow_mut().throw();

        assert_eq!(monkey_0.borrow().items.len(), 1);

        let monkey_3 = &monkeys.borrow()[3];

        assert_eq!(monkey_3.borrow().items.len(), 2);
        assert_eq!(monkey_3.borrow().items[1].worry_level, 500);
    }

    #[test]
    fn monkey_throw_all_items() {
        let monkey_0 = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey_1 = "Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0";

        let monkey_2 = "Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

        let monkey_3 = "Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";

        let monkey_strings = vec![monkey_0, monkey_1, monkey_2, monkey_3];

        let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

        for monkey_string in monkey_strings {
            let mut monkey = monkey_string.parse::<Monkey>().unwrap();

            monkey.friends = Rc::clone(&monkeys);

            monkeys.borrow_mut().push(RefCell::new(monkey));
        }

        let monkey_0 = &monkeys.borrow()[0];

        monkey_0.borrow_mut().inspect_and_throw_all_items();

        assert_eq!(monkey_0.borrow().items.len(), 0);

        let monkey_3 = &monkeys.borrow()[3];

        assert_eq!(monkey_0.borrow().inspection_count, 2);
        assert_eq!(monkey_3.borrow().items.len(), 3);
        assert_eq!(monkey_3.borrow().items[1].worry_level, 500);
        assert_eq!(monkey_3.borrow().items[2].worry_level, 620);
    }

    #[test]
    fn monkeys_throw_all_items_for_20_rounds() {
        let monkey_0 = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey_1 = "Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0";

        let monkey_2 = "Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

        let monkey_3 = "Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";

        let monkey_strings = vec![monkey_0, monkey_1, monkey_2, monkey_3];

        let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

        for monkey_string in monkey_strings {
            let mut monkey = monkey_string.parse::<Monkey>().unwrap();

            monkey.friends = Rc::clone(&monkeys);

            monkeys.borrow_mut().push(RefCell::new(monkey));
        }

        for round in 0..20 {
            for monkey in monkeys.borrow().iter() {
                monkey.borrow_mut().inspect_and_throw_all_items();
            }
        }

        let monkey_inspection_counts: Vec<usize> = vec![101, 95, 7, 105];

        let mut i = 0;

        for monkey in monkeys.borrow().iter() {
            assert_eq!(
                monkey.borrow().inspection_count,
                monkey_inspection_counts[i]
            );
            i += 1;
        }
    }

    #[test]
    fn monkeys_throw_all_items_for_1_round() {
        let monkey_0 = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let monkey_1 = "Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0";

        let monkey_2 = "Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

        let monkey_3 = "Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";

        let monkey_strings = vec![monkey_0, monkey_1, monkey_2, monkey_3];

        let monkeys: Rc<RefCell<Vec<RefCell<Monkey>>>> = Rc::new(RefCell::new(Vec::new()));

        for monkey_string in monkey_strings {
            let mut monkey = monkey_string.parse::<Monkey>().unwrap();

            monkey.friends = Rc::clone(&monkeys);

            monkeys.borrow_mut().push(RefCell::new(monkey));
        }

        for monkey in monkeys.borrow().iter() {
            monkey.borrow_mut().inspect_and_throw_all_items();
        }

        let monkey_inspection_counts: Vec<usize> = vec![2, 4, 3, 5];

        let mut i = 0;

        for monkey in monkeys.borrow().iter() {
            assert_eq!(
                monkey.borrow().inspection_count,
                monkey_inspection_counts[i]
            );
            i += 1;
        }
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
