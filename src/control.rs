use std::io;
use rand::{distributions::Uniform, Rng};

use crate::algorithm::Algorithm;
use crate::view::View;


pub struct ShellSort {
    algorithm: Algorithm
}

impl ShellSort {

    pub fn new() -> ShellSort {
        let data = Self::get_data();
        let view = Self::get_view(data);
        let algorithm = Self::get_algorithm(view);
        ShellSort{algorithm}
    }

    pub fn start(&mut self) {
        self.algorithm.sort();
    }

    fn get_data() -> Vec<u32> {
        let length = Self::get_value("vector length", "Length of the vector: ");
        let range = (
            Self::get_value("lower range", "Lower data range: "),
            Self::get_value("upper range", "Upper data range: ")
        );
        Self::generate_data(length, range)
    }

    fn get_view(data: Vec<u32>) -> View {
        let stack_char = Self::get_stack_char();
        let period = Self::get_value("sort period", "Time interval in seconds: ");
        View{data, stack_char, period}
    }

    fn get_algorithm(view: View) -> Algorithm {
        let method = Self::get_value(
        "sort method", "Chosen sorting method number:\n
        1. Insertion Sort
        2. Bubble Sort
        3. Merge Sort
        ");
        Algorithm{view, method}
    }

    fn generate_data(length: u32, range: (u32, u32)) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let bound = Uniform::new(range.0, range.1);
        (0..length).map(|_| rng.sample(&bound)).collect()
    }

    fn get_value(kind: &str, message: &str) -> u32 {
        loop {
            println!("\n{}", message);
            let mut value = String::new();
            io::stdin().read_line(&mut value).expect("Failed to read line");
            let value: u32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error: Check {kind} input");
                    continue
                }
            };
            return value;
        }
    }

    fn get_stack_char() -> String {
        println!("\nInput the stack character for the vector.");
        let mut stack = String::new();
        io::stdin().read_line(&mut stack).expect("Failed to read line");
        String::from(stack.trim())
    }
 
}

impl Default for ShellSort {
    fn default() -> Self {
        Self::new()
    }
}
