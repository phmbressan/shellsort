use crate::control::ShellSort;

pub mod algorithm;
pub mod control;
pub mod view;

fn main() {
    ShellSort::new().start();
}
