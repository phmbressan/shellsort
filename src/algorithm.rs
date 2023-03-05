use std::cmp::Ordering;

use crate::view::View;


pub struct Algorithm {
    pub view: View,
    pub method: u32
}

impl Algorithm {
    
    pub fn sort(&mut self) {
        match self.method {
            1 => self.insertion_sort(),
            2 => self.bubble_sort(),
            3 => self.merge_sort(),
            _ => println!("Type a valid sort method number.")
        }
    }

    fn insertion_sort(&mut self) {
        let s = &mut self.view;
        s.display();

        for p in 1..s.data.len() {
            let mut i = p;
            while i > 0 && s.data[i - 1] > s.data[i] {
                s.data.swap(i - 1, i);
                i -= 1;
                s.display();
            }
        }
    }

    fn bubble_sort(&mut self) {
        self.view.display();
        let mut n = self.view.data.len();

        while n > 1 {
            let mut index = 0;
            for p in 1..n {
                if self.view.data[p - 1] > self.view.data[p] {
                    self.view.data.swap(p - 1, p);
                    index = p;
                }
                self.view.display();
            }
            n = index;
        }
        
    }

    fn merge_sort(&mut self) {
        let vsize = self.view.data.len();
        let mut inner_index = 1;
        self.view.display();
    
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
            self.view.display();
        }
    }
    
    fn merge(&mut self, left: usize, mid: usize, right: usize) {
        let v = &mut self.view.data;
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