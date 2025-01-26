use std::os::windows;

pub struct ConwayBoard {
    odd_frame: Vec<Vec<bool>>,
    even_frame: Vec<Vec<bool>>,
    on_odd_frame: bool,
}

const ACTIVE: char = '#';
const UNACTIVE: char = ' ';

impl ConwayBoard {
    pub fn new(height: usize, width: usize) -> Self {
        //[height] [width]
        ConwayBoard {
            odd_frame: vec![vec![false; width]; height],
            even_frame: vec![vec![false; width]; height],
            on_odd_frame: true,
        }
    }
    pub fn get_frame(&self) -> &Vec<Vec<bool>> {
        if self.on_odd_frame == true {
            return &self.odd_frame;
        }
        &self.even_frame
    }
    pub fn get_frame_mut(&mut self) -> &mut Vec<Vec<bool>> {
        if self.on_odd_frame == true {
            return &mut self.odd_frame;
        }
        &mut self.even_frame
    }
    pub fn update(&mut self) {
        let (current_grid, future_grid) = if self.on_odd_frame {
            (&self.odd_frame, &mut self.even_frame)
        } else {
            (&self.even_frame, &mut self.odd_frame)
        };
        for (h_index, height) in current_grid.iter().enumerate() {
            for (w_index, cell) in height.iter().enumerate() {
                let neighbors = Self::count_neighbors(current_grid, h_index, w_index);
                match (cell, neighbors) {
                    // live cell fewer than 2
                    (true, 0..2) => {
                        *future_grid
                            .get_mut(h_index)
                            .unwrap()
                            .get_mut(w_index)
                            .unwrap() = false;
                    }
                    (true, 2 | 3) => {
                        *future_grid
                            .get_mut(h_index)
                            .unwrap()
                            .get_mut(w_index)
                            .unwrap() = true;
                    }
                    (true, 4..) => {
                        *future_grid
                            .get_mut(h_index)
                            .unwrap()
                            .get_mut(w_index)
                            .unwrap() = false;
                    }
                    (false, 3) => {
                        *future_grid
                            .get_mut(h_index)
                            .unwrap()
                            .get_mut(w_index)
                            .unwrap() = true;
                    }
                    (_, _) => {
                        *future_grid
                            .get_mut(h_index)
                            .unwrap()
                            .get_mut(w_index)
                            .unwrap() = false;

                        // println!("this shouldn't appear");
                    }
                }
            }
        }
        self.on_odd_frame = !self.on_odd_frame;
    }
    fn count_neighbors(board: &Vec<Vec<bool>>, height: usize, width: usize) -> u8 {
        let mut neighbors = 0;
        let searches: [[isize; 2]; 8] = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];
        for [x, y] in searches {
            if let (Some(new_height), Some(new_width)) =
                (height.checked_add_signed(x), width.checked_add_signed(y))
            {
                if let Some(width_board) = board.get(new_height) {
                    if let Some(value) = width_board.get(new_width) {
                        if *value == true {
                            neighbors += 1;
                        }
                    }
                }
            }
        }
        return neighbors;
    }
    pub fn oposite(&mut self, height: usize, width: usize) {
        let frame = self.get_frame_mut();
        *frame.get_mut(height).unwrap().get_mut(width).unwrap() =
            !*frame.get_mut(height).unwrap().get_mut(width).unwrap();
    }
    pub fn print_board(&self) {
        for width in self.get_frame().iter() {
            for block in width.iter() {
                print!(" {} ", if *block == true { ACTIVE } else { UNACTIVE });
            }
            println!("");
        }
        println!("");
    }
}
