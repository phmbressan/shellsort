use crate::control::ShellSort;

pub mod algorithm;
pub mod control;
pub mod view;

fn main() {
    let mut shell_sort = ShellSort::new();
    shell_sort.start();
}