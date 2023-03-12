use blockbuster::{i, j, t, l, o, Piece, RelPoint};

#[derive(Default, Debug)]
struct Game {
    playfield: [[i16; 10]; 11],
    moving_piece: Option<Piece>,
    moving_piece_rotation: usize,
}

impl Game {
    // pula
    pub fn add_piece(&mut self, piece: Piece) {
        self.playfield[piece.center.0][piece.center.1] = 1;
        for point in piece.positions[0].iter() {
            self.playfield[(point.0 + piece.center.0 as i64) as usize]
                [(point.1 + piece.center.1 as i64) as usize] = 1;
        }
        self.moving_piece_rotation = 0;
        self.moving_piece = Some(piece);
    }
    fn clear_piece_location(&mut self) {
        let piece = self.moving_piece.as_mut().unwrap();
        self.playfield[piece.center.0][piece.center.1] = 0;
        for point in piece.positions[self.moving_piece_rotation].iter() {
            self.playfield[(point.0 + piece.center.0 as i64) as usize]
                [(point.1 + piece.center.1 as i64) as usize] = 0;
        }
    }

    fn update_piece_location(&mut self) -> Result<(), ()> {
        let piece = self.moving_piece.as_mut().unwrap();
        let mut new_positions: [RelPoint; 3] = [
            RelPoint::default(),
            RelPoint::default(),
            RelPoint::default(),
        ];
        for (id, point) in piece.positions[self.moving_piece_rotation]
            .iter()
            .enumerate()
        {
            let new_x = (point.0 + piece.center.0 as i64) as usize;
            let new_y = (point.1 + piece.center.1 as i64) as usize;
            if new_x > self.playfield.len() - 1 || new_y > self.playfield[0].len() - 1 {
                return Err(());
            }
            new_positions[id] = RelPoint(new_x as i64, new_y as i64);
        }
        for new_position in new_positions.iter() {
            self.playfield[new_position.0 as usize][new_position.1 as usize] = 1;
        }
        self.playfield[piece.center.0][piece.center.1] = 1;
        Ok(())
    }

    pub fn rotate_piece(&mut self, clockwise: bool) {
        self.clear_piece_location();
        if clockwise {
            todo!()
        } else {
            self.moving_piece_rotation = (self.moving_piece_rotation + 1) % 4;
        }
        self.update_piece_location().unwrap();
    }

    pub fn lower_piece(&mut self) -> Result<(), ()> {
        self.clear_piece_location();
        self.moving_piece.as_mut().unwrap().move_center(1, 0);
        if let Err(_) = self.update_piece_location() {
            self.moving_piece.as_mut().unwrap().move_center(-1, 0);
            self.update_piece_location().unwrap();
            return Err(());
        }
        return Ok(());
    }
}

fn main() {
    let mut game = Game::default();
    game.add_piece(o());
    // game.lower_piece().unwrap();
    // game.rotate_piece(false);
    for line in game.playfield.iter() {
        println!("{:?}", line);
    }
}
