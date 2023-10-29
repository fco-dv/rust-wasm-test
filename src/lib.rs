mod utils;
use std::fmt;

use wasm_bindgen::prelude::*;

extern crate js_sys;
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     let alert_message = format!("{name} Hello, wasm-game-of-life!");
//     alert(&alert_message);
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe{
        let width:u32=64;
        let height: u32=64;
        // let cells: Vec<_> = (0..width*height)
        //  .map(|idx| {
        //     if idx%2 == 0 || idx%7 == 0 {
        //         Cell::Alive
        //     }else {
        //         Cell::Dead
        //     }
        //  } )
        //  .collect();

        let cells: Vec<_> = (0..width*height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        //single space ship  c/2 orthogonal
        // can be seen as a vector of offset of row and cols
        // defined from the top left of the space
        let row_col_offsets = vec![(0,1),(0,4),(1,0),(2,0), (2,4),(3,0),(3,1),(3,2), (3,3) ];
        // init cells to Dead
        let mut cells: Vec<_> = (0..width*height)
        .map(|_| Cell::Dead)
        .collect();
        let spaceship_origin_idx = (1,1);
        for offsets in row_col_offsets.iter() {
            let _row = offsets.0+spaceship_origin_idx.0;
            let _col = offsets.1+spaceship_origin_idx.1;
            let idx = (_row*width+_col) as usize;
            cells[idx] = Cell::Alive;

        }
        Universe { width: (width), height: (height), cells: (cells) }

    }

    pub fn render(&self)->String {
        self.to_string()
    }

    fn get_index(&self, row:u32, col:u32) -> usize {
        //(row*self.width+col).try_into().unwrap()
        (row*self.width+col) as usize
    }

    fn live_neighbour_count(&self, row:u32, col:u32) -> u8 {
        let mut count = 0;
        //iterate on the 8  neighbourhood 
        for delta_row in [self.height -1, 0, 1].iter().cloned() {
            for delta_col in [ self.width -1 , 0, 1 ].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row)%self.height;
                let neighbour_col = (col + delta_col)%self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8; 
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_count = self.live_neighbour_count(row, col);
                
                let next_cell= match (cell, live_count) {
                    (Cell::Alive, x ) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell{ 
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width:u32) {
        self.width = width;
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect()

    }

    pub fn set_height(&mut self, height:u32) {
        self.height = height;
        self.cells = (0..self.width*self.height).map(|_| Cell::Dead).collect()

    }

}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                let symbol = if *cell ==  Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
      Ok(())  
    }
}