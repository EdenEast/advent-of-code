#![allow(unused)]

pub use libaoc::Solution;

pub mod day_01;

pub const ALL: [&dyn Solution; 1] = [&day_01::Day01];
