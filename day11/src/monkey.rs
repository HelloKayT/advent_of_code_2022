use std::{fmt, collections::VecDeque};
pub struct Monkey {
    pub items: VecDeque<u64>,
    // returns the new worry level
    operation: Box<dyn Fn(u64) -> u64>,
    pub test_divisor: u64,
    // returns which monkey to throw to
    test: Box<dyn Fn(u64) -> u64>,
    pub items_inspected: u64
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
         .field("items", &self.items)
         .finish()
    } 
}

impl  Monkey {
    pub fn new(monkey_lines: Vec<String>) -> Self {
        let op_fn = Self::get_op(&monkey_lines[2]);
        let (test_divisor, test_fn) = Self::get_test(&monkey_lines[3..]);
        let mut items_vec = VecDeque::new();
        let items_str = monkey_lines[1].split(":").collect::<Vec<&str>>()[1];
        let items: Vec<&str> = items_str.split(",").map(|x|x.trim()).collect();

        for num in items {
            items_vec.push_back(num.parse().unwrap());
        } 

        Monkey { 
            items: items_vec, 
            operation: Box::new(op_fn), 
            test: Box::new(test_fn),
            test_divisor: test_divisor,
            items_inspected: 0
        }
        
    }

    // return monkey index to send to and item
    pub fn process_item(&mut self, item: u64, mod_worry_by: u64) -> (u64, u64) {
        self.items_inspected += 1;
        let mut new_worry = self.run_op(item);
        new_worry %= mod_worry_by;
        let next_monkey_index = self.run_test(new_worry);
        return (next_monkey_index, new_worry);
    }


    pub fn run_op(&self, input: u64) -> u64 {
        (self.operation)(input)
    }

    pub fn run_test(&self, input: u64) -> u64 {
        (self.test)(input)
    }

    fn get_op(operation_str: &str) -> impl Fn(u64) -> u64 {
        let tokens: Vec<&str> = operation_str.split_whitespace().collect();
        let op_1 = String::from(tokens[3]);
        let operator = String::from(tokens[4]);
        let op_2 = String::from(tokens[5]);

        let return_op = move |item_worry: u64| {
            let num_1 = match &op_1[..] {
                "old" => { item_worry },
                num_str => {num_str.parse().unwrap()} 
            };
            let num_2 = match &op_2[..] {
                "old" => { item_worry },
                num_str => {num_str.parse().unwrap()}
            };


            let result = match &operator[..] {
                "*" => {num_1.wrapping_mul(num_2)},
                "+" => {num_1.wrapping_add(num_2)},
                "-" => {num_1.wrapping_sub(num_2)},
                "/" => {num_1.wrapping_div(num_2)},
                &_ => panic!("unsupported operator"),
            };

            return result;
        };

        return return_op;
    }

    fn get_test(test_str: &[String]) -> (u64, impl Fn(u64) -> u64) {
        let divisor_tokens: Vec<&str> = (test_str[0]).split_whitespace().collect();
        let divisor: u64 = divisor_tokens[divisor_tokens.len()-1].parse()
            .expect("couldn't parse");

        let true_tokens: Vec<&str> = (test_str[1]).split_whitespace().collect();
        let true_monkey: u64 = true_tokens[true_tokens.len()-1].parse()
            .expect("couldn't parse");

        let false_tokens: Vec<&str> = (test_str[2]).split_whitespace().collect();
        let false_monkey: u64 = false_tokens[false_tokens.len()-1].parse()
            .expect("couldn't parse");

        let return_test = move |test_value: u64| {
            if (test_value % divisor) == 0 {
                return true_monkey;
            }
            else {
                return false_monkey;
            }
        };
        return (divisor, return_test);
    }

}

#[cfg(test)]
mod tests {
    use super::Monkey;

    #[test]
    fn basic_monkey_test() {
        let test_monkey_lines = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old * 19"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
        ];

        let test_monkey = Monkey::new(test_monkey_lines);
        assert!(test_monkey.items == vec![79, 98]);
        let op_result = test_monkey.run_op(3);
        assert!(op_result == (3 * 19));
        let test_result_true = test_monkey.run_test(23);
        assert!(test_result_true == 2);
        let test_result_false = test_monkey.run_test(1);
        assert!(test_result_false == 3);
    }
}