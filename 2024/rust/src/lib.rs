#![allow(unused)]

pub use libaoc::Solution;

pub mod day_01;
pub mod day_02;

pub const ALL: [&dyn Solution; 2] = [&day_01::Day01, &day_02::Day02];
