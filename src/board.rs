use crate::layouts::Layouts;
use bevy::prelude::Component;
use std::mem;

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Black,
    White,
}

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum Square {
    Empty,
    Bishop(Colour),
    King(Colour),
    Knight(Colour),
    Pawn(Colour),
    Queen(Colour),
    Rook(Colour),
}

#[derive(PartialEq)]
pub struct Board {
    layout: Vec<Vec<Square>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            layout: Layouts::standard(),
        }
    }
}

impl Board {
    pub fn new(layout: Vec<Vec<Square>>) -> Self {
        Self { layout }
    }

    pub fn get_layout(&self) -> &Vec<Vec<Square>> {
        &self.layout
    }

    pub fn move_piece(&mut self, old_pos: &Position, new_pos: &Position) -> Result<(), &str> {
        if self.layout.get(old_pos.y).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout[old_pos.y].get(old_pos.x).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout.get(new_pos.y).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }
        if self.layout[new_pos.y].get(new_pos.x).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }
        let moved_piece = mem::replace(&mut self.layout[old_pos.y][old_pos.x], Square::Empty);
        self.layout[new_pos.y][new_pos.x] = moved_piece;
        Ok(())
    }
}
