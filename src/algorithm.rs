use std::cmp::Ordering;

use crate::view::View;

pub struct Algorithm {
    pub view: View,
    pub method: u32,
}

impl Algorithm {
    pub fn sort(&mut self) {
        match self.method {
            1 => self.insertion_sort(),
            2 => self.bubble_sort(),
            3 => self.merge_sort(),
            _ => println!("Type a valid sort method number."),
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
        let s = &mut self.view;
        let mut n = s.data.len();

        s.display();

        while n > 1 {
            let mut index = 0;
            for p in 1..n {
                if s.data[p - 1] > s.data[p] {
                    s.data.swap(p - 1, p);
                    index = p;

                    s.display();
                }
            }
            n = index;
        }
    }

    fn merge_sort(&mut self) {
        let s = &mut self.view;
        let n = s.data.len();
        let mut inner_index = 1;

        s.display();

        while inner_index < n {
            let mut start_index = 0;
            while start_index + inner_index < n {
                let mut end_index = start_index + 2 * inner_index;
                if end_index > n {
                    end_index = n;
                }
                merge(
                    &mut s.data,
                    start_index,
                    start_index + inner_index,
                    end_index,
                );

                s.display();

                start_index += 2 * inner_index;
            }
            inner_index *= 2;
        }

        fn merge(s: &mut [u32], left: usize, mid: usize, right: usize) {
            let s_left = s[left..mid].to_vec();
            let s_right = s[mid..right].to_vec();

            let mut left_iter = s_left.iter().peekable();
            let mut right_iter = s_right.iter().peekable();
            let mut index = left;

            loop {
                match (left_iter.peek(), right_iter.peek()) {
                    (Some(left_value), Some(right_value)) => match left_value.cmp(right_value) {
                        Ordering::Less => s[index] = *left_iter.next().unwrap(),
                        Ordering::Greater => s[index] = *right_iter.next().unwrap(),
                        Ordering::Equal => s[index] = *left_iter.next().unwrap(),
                    },
                    (Some(_), None) => s[index] = *left_iter.next().unwrap(),
                    (None, Some(_)) => s[index] = *right_iter.next().unwrap(),
                    _ => break,
                }
                index += 1;
            }
        }
    }
}
