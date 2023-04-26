use blockbuster::{i, j, l, o, s, t, z, Piece, RelPoint};
use std::{error, io};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Terminal,
};

#[derive(Default, Debug)]
struct Game {
    playfield: [[i16; 10]; 11],
    moving_piece: Option<Piece>,
    moving_piece_rotation: usize,
    next_piece: Option<Piece>,
    hold_piece: Option<Piece>,
}

enum Rotation {
    Clockwise,
    CounterClockwise,
}

enum TetrisDirection {
    Up,
    Down,
    Left,
    Right,
}

const CLOCKWISE: Rotation = Rotation::Clockwise;
const COUNTER_CLOCKWISE: Rotation = Rotation::CounterClockwise;
const UP: TetrisDirection = TetrisDirection::Up;
const DOWN: TetrisDirection = TetrisDirection::Down;
const LEFT: TetrisDirection = TetrisDirection::Left;
const RIGHT: TetrisDirection = TetrisDirection::Right;

impl Game {
    pub fn add_piece(&mut self, piece: Piece) {
        self.playfield[piece.center.0][piece.center.1] = piece.color;
        for point in piece.positions[0].iter() {
            self.playfield[(point.0 + piece.center.0 as i64) as usize]
                [(point.1 + piece.center.1 as i64) as usize] = piece.color;
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

    fn update_piece_location(&mut self) -> Result<(), &str> {
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
                return Err("Not enough room to move piece");
            }
            new_positions[id] = RelPoint(new_x as i64, new_y as i64);
        }
        for new_position in new_positions.iter() {
            self.playfield[new_position.0 as usize][new_position.1 as usize] = piece.color;
        }
        self.playfield[piece.center.0][piece.center.1] = piece.color;
        Ok(())
    }

    pub fn rotate_piece(&mut self, direction: Rotation) {
        self.clear_piece_location();
        match direction {
            Rotation::Clockwise => {
                if self.moving_piece_rotation == 0 {
                    self.moving_piece_rotation = 3
                } else {
                    self.moving_piece_rotation = self.moving_piece_rotation - 1
                }
            }
            Rotation::CounterClockwise => {
                self.moving_piece_rotation = (self.moving_piece_rotation + 1) % 4
            }
        }
        // TODO: implement wall kicks here
        self.update_piece_location().unwrap();
    }

    pub fn move_piece(&mut self, direction: TetrisDirection) -> Result<(), Box<dyn error::Error>> {
        self.clear_piece_location();
        match direction {
            TetrisDirection::Up => self.moving_piece.as_mut().unwrap().move_center(-1, 0),
            TetrisDirection::Down => self.moving_piece.as_mut().unwrap().move_center(1, 0),
            TetrisDirection::Left => self.moving_piece.as_mut().unwrap().move_center(0, -1),
            TetrisDirection::Right => self.moving_piece.as_mut().unwrap().move_center(0, 1),
        }
        self.update_piece_location()?;
        return Ok(());
    }

    pub fn lock_piece(&mut self) {
        self.moving_piece = None;
        self.moving_piece_rotation = 0;
    }
}

fn main() {
    let mut game = Game::default();
    game.add_piece(i());
    // game.move_piece(RIGHT).unwrap();
    // game.move_piece(RIGHT).unwrap();
    // game.move_piece(DOWN).unwrap();
    // game.rotate_piece(COUNTER_CLOCKWISE);
    // for line in game.playfield.iter() {
    //     println!("{:?}", line);
    // }
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    for event in io::stdin().keys() {
        match event {
            Ok(Key::Char('q')) => break,
            Ok(Key::Char('j')) => game.move_piece(DOWN).unwrap(),
            Ok(Key::Char('k')) => game.rotate_piece(CLOCKWISE),
            Ok(Key::Char('h')) => game.move_piece(LEFT).unwrap(),
            Ok(Key::Char('l')) => game.move_piece(RIGHT).unwrap(),
            _ => (),
        }
        terminal
            .draw(|f| {
                let field = game.playfield.map(|row| {
                    Row::new(row.map(|el| {
                        let color = match el {
                            0 => Color::Black,
                            1 => Color::Cyan,
                            2 => Color::Blue,
                            3 => Color::Red,
                            4 => Color::Yellow,
                            5 => Color::Green,
                            6 => Color::Magenta,
                            7 => Color::LightRed,
                            _ => Color::White,
                        };
                        Cell::from("").style(Style::default().bg(color))
                    }))
                });
                let table = Table::new(field)
                    // You can set the style of the entire Table.
                    .style(Style::default().fg(Color::White))
                    // As any other widget, a Table can be wrapped in a Block.
                    .block(Block::default().title("Tetris"))
                    // Columns widths are constrained in the same way as Layout...
                    .widths(&[
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                    ])
                    // ...and they can be separated by a fixed spacing.
                    .column_spacing(0)
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));
                f.render_widget(table, f.size());
            })
            .unwrap();
    }
}
