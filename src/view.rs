use std::{thread, time::Duration};

pub struct View {
    pub data: Vec<u32>,
    pub stack_char: String,
    pub period: u32,
}

impl View {
    pub fn display(&self) {
        let max_val = self.data.iter().max().map_or(&0, |max| max);
        let mut vec_string = String::new();

        thread::sleep(Duration::from_millis((self.period * 1000).into()));

        for curr_val in (1..=*max_val).rev() {
            for value in &self.data {
                if *value < curr_val {
                    vec_string += &" ".repeat(self.stack_char.len() + 2);
                } else {
                    vec_string += " ";
                    vec_string += &self.stack_char;
                    vec_string += " ";
                }
            }
            vec_string += "\n";
        }
        println!("{vec_string}\n");
    }
}
