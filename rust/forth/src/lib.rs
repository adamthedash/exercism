use std::collections::HashMap;

use crate::Error::{DivisionByZero, InvalidWord, StackUnderflow, UnknownWord};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    vars: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            vars: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn parse_tokens(&mut self, input: &Vec<String>) -> std::result::Result<Vec<String>, Error> {
        // Recursively parses and replaces substrings
        let mut out = vec![];

        let mut i = 0;
        while i < input.len() {
            if input[i].parse::<Value>().is_ok() {
                // Ignore numbers during parsing
                out.push(String::from(&input[i]));
            } else if input[i] == ":" {
                // Find ;
                let mut j = 2;
                while i + j < input.len() && input[i + j] != ";" { j += 1 };
                if i + j >= input.len() { return Err(InvalidWord); }

                // Pull out substring and parse it
                let sub_inst = input[i + 2..i + j].to_vec();
                let parsed_sub = self.parse_tokens(&sub_inst);

                // Pull var name out
                let var_name = &input[i + 1];
                if var_name.parse::<Value>().is_ok() {
                    return Err(InvalidWord);
                }

                // Error pass up
                match parsed_sub {
                    Ok(parsed) => {
                        self.vars.insert(String::from(var_name), parsed);
                    }
                    Err(e) => { return Err(e); }
                }

                i += 2 + sub_inst.len();
            } else if self.vars.contains_key(&input[i]) {
                // replace word with contents
                for item in self.vars.get(&input[i]).unwrap() {
                    out.push(item.clone())
                }
            } else if vec!["+", "-", "*", "/"].contains(&&*input[i]) {
                // Ignore ops during parsing
                out.push(String::from(&input[i]));
            } else if vec!["dup", "drop", "swap", "over"].contains(&&*input[i]) {
                // Fallback special terms
                out.push(String::from(&input[i]));
            } else {
                return Err(UnknownWord);
            }

            i += 1
        }

        return Ok(out);
    }

    fn compute(&mut self, input: &Vec<String>) -> std::result::Result<(), Error> {
        // Executes the program
        for x in input {
            if vec!["+", "-", "*", "/"].contains(&x.as_str()) {
                if self.stack.len() < 2 {
                    return Err(StackUnderflow);
                }

                // Binary ops
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();

                if x == "+" {
                    self.stack.push(a + b)
                } else if x == "-" {
                    self.stack.push(a - b)
                } else if x == "*" {
                    self.stack.push(a * b)
                } else if x == "/" {
                    if b == 0 {
                        return Err(DivisionByZero);
                    }
                    self.stack.push(a / b)
                }
            } else if x == "dup" {
                if self.stack.len() < 1 {
                    return Err(StackUnderflow);
                }
                self.stack.push(self.stack.last().unwrap().clone())
            } else if x == "drop" {
                if self.stack.len() < 1 {
                    return Err(StackUnderflow);
                }
                self.stack.pop();
            } else if x == "swap" {
                if self.stack.len() < 2 {
                    return Err(StackUnderflow);
                }
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();

                self.stack.push(b);
                self.stack.push(a);
            } else if x == "over" {
                if self.stack.len() < 2 {
                    return Err(StackUnderflow);
                }
                let a = self.stack.get(self.stack.len() - 2).unwrap();
                self.stack.push(a.clone());
            } else {
                // Number
                self.stack.push(x.parse::<Value>().unwrap())
            }
        }

        Ok(())
    }

    fn preprocess(input: &str) -> Vec<String> {
        input
            .split_whitespace() // Split by spaces
            .map(str::to_lowercase) // Lowercase each word
            .collect() // Collect into a vector of &str
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = Self::preprocess(input);
        println!("input tokens: {:?}", input);

        let parsed_input = self.parse_tokens(&input);
        println!("parsed tokens: {:?}", parsed_input);

        if parsed_input.is_err() {
            return Err(parsed_input.err().unwrap());
        }

        println!("vars: {:?}", self.vars);

        let compute_result = self.compute(&parsed_input.unwrap());
        println!("stack: {:?}", self.stack);

        if compute_result.is_err() {
            return Err(compute_result.err().unwrap());
        }


        return Ok(());
    }
}
