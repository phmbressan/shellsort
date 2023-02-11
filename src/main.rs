use std::io;
use std::cmp::Ordering;
use std::{thread, time::Duration};
use rand::{distributions::Uniform, Rng};


fn main() {
    let (data_inputs, view_inputs, model_inputs) = (
        Controller::get_data_inputs(), 
        Controller::get_view_inputs(), 
        Controller::get_model_inputs());

    let view = View {
        data: Model::generate_data(data_inputs.0, data_inputs.1),
        stack_char: view_inputs.0,
        time: view_inputs.1
    };

    Model {sortable: view, method_number: model_inputs}.sort();
}

struct Controller {}

impl Controller {

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

    fn get_data_inputs() -> (u32, (u32, u32)) {
        (
            Self::get_value("vector length", "Length of the vector: "),
            (Self::get_value("lower range", "Lower data range: "),
            Self::get_value("upper range", "Upper data range: "))
        )
    }

    fn get_view_inputs() -> (String, u32) {
        (
            Self::get_stack_char(),
            Self::get_value("sort period", "Time interval in seconds: "),
        )
    }

    fn get_model_inputs() -> u32 {
        Self::get_value("sort method", "Chosen sorting method number:\n
        1. Insertion Sort
        2. Bubble Sort
        3. Merge Sort
        ")
    }
}

struct View {
    data: Vec<u32>, 
    stack_char: String,
    time: u32
}

impl View {
    fn display_vert(&self) {
        thread::sleep(Duration::from_millis((self.time*1000).into()));
        for value in &self.data {
            print!("{}", &View::build_stack(&self.stack_char, *value));
        }
        println!();
    }

    fn display_horz(&self) {
        let vec_iter = self.data.iter();
        let max_val = *vec_iter.max().unwrap();
        let mut vec_string = String::new();
        thread::sleep(Duration::from_millis((self.time*1000).into()));
        for curr_val in (1..=max_val).rev() {
            for value in &self.data {
                if *value < curr_val {
                    vec_string += &" ".repeat(self.stack_char.len()+2);
                } else {
                    vec_string += " ";
                    vec_string += &self.stack_char;
                    vec_string += " ";
                }
            }
            vec_string += "\n";
        }
        println!("{}", vec_string);
        println!();
    }
    
    fn build_stack(stack_char: &str, size: u32) -> String {
        match size {
            0 => String::from(".\n"),
            _ => stack_char.repeat(size.try_into().unwrap()) + "\n"
        }
    }
}

struct Model {
    sortable: View,
    method_number: u32
}

impl Model {
    fn generate_data(length: u32, range: (u32, u32)) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let bound = Uniform::new(range.0, range.1);
        (0..length).map(|_| rng.sample(&bound)).collect()
    }

    fn sort(&mut self) {
        match self.method_number {
            1 => self.insertion_sort(),
            2 => self.bubble_sort(),
            3 => self.merge_sort(),
            _ => println!("Type a valid sort method number.")
        }
    }

    fn insertion_sort(&mut self) {
        let s = &mut self.sortable;
        s.display_horz();

        for p in 1..s.data.len() {
            let mut i = p;
            while i > 0 && s.data[i - 1] > s.data[i] {
                s.data.swap(i - 1, i);
                i -= 1;
                s.display_horz();
            }
        }
    }

    fn bubble_sort(&mut self) {
        self.sortable.display_horz();
        let mut n = self.sortable.data.len();

        while n > 1 {
            let mut index = 0;
            for p in 1..n {
                if self.sortable.data[p - 1] > self.sortable.data[p] {
                    self.sortable.data.swap(p - 1, p);
                    index = p;
                }
                self.sortable.display_horz();
            }
            n = index;
        }
        
    }

    fn merge_sort(&mut self) {
        let vsize = self.sortable.data.len();
        let mut inner_index = 1;
        self.sortable.display_horz();
    
        while inner_index < vsize {
            let mut start_bin_index = 0;
            while start_bin_index + inner_index < vsize {
                let mut end_bin_index = start_bin_index + 2 * inner_index;
                if end_bin_index > vsize {
                    end_bin_index = vsize;
                }
                self.merge(start_bin_index, start_bin_index+inner_index, end_bin_index);
                start_bin_index += 2 * inner_index;
            }
            inner_index *= 2;
            self.sortable.display_horz();
        }
    }
    
    fn merge(&mut self, left: usize, mid: usize, right: usize) {
        let v = &mut self.sortable.data;
        let v_left = v[left..mid].to_vec();
        let v_right = v[mid..right].to_vec();
    
        let mut left_iter = v_left.iter().peekable();
        let mut right_iter = v_right.iter().peekable();
        let mut index = left;
    
        loop {
            match (left_iter.peek(), right_iter.peek()) {
                (Some(left_value), Some(right_value)) => match left_value.cmp(right_value) {
                    Ordering::Less => v[index] = *left_iter.next().unwrap(),
                    Ordering::Greater => v[index] = *right_iter.next().unwrap(),
                    Ordering::Equal => v[index] = *left_iter.next().unwrap()
                },
                (Some(_), None) => v[index] = *left_iter.next().unwrap(),
                (None, Some(_)) => v[index] = *right_iter.next().unwrap(),
                _ => break
            }
            index += 1
        }
    
    }
}