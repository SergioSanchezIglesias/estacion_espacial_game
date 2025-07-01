use std::io::{ self, Write };

#[derive(Debug)]
pub struct Display;

impl Display {
    pub fn new() -> Self {
        Display {}
    }

    pub fn show_welcome(&self) {
        println!("Welcome to the game!");
    }
}