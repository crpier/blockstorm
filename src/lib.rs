use std::{error, io};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Terminal,
};

#[derive(Debug, Default)]
pub struct CenterPoint(pub usize, pub usize);

#[derive(Debug, Default, Clone, Copy)]
pub struct RelPoint(pub i16, pub i16);

#[derive(Debug)]
pub struct Piece {
    center: CenterPoint,
    current_rotation_id: u8,
    rotations: [[RelPoint; 3]; 4],
    pub color: i16,
}

pub enum PieceType {
    // Clockwise,
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
impl Piece {
    pub fn new(piece_type: PieceType) -> Piece {
        let center: CenterPoint;
        let pos_0: [RelPoint; 3];
        let pos_1: [RelPoint; 3];
        let pos_2: [RelPoint; 3];
        let pos_3: [RelPoint; 3];
        let color: i16;

        match piece_type {
            PieceType::I => {
                center = CenterPoint(1, 5);
                pos_0 = [RelPoint(0, -2), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-2, 0), RelPoint(-1, 0), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -2), RelPoint(0, -1), RelPoint(0, 1)];
                pos_3 = [RelPoint(-2, 0), RelPoint(-1, 0), RelPoint(1, 0)];
                color = 1;
            }
            PieceType::J => {
                center = CenterPoint(1, 5);
                pos_0 = [RelPoint(0, -1), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_1 = [RelPoint(-1, -1), RelPoint(-1, 0), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -1), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_3 = [RelPoint(-1, -1), RelPoint(-1, 0), RelPoint(1, 0)];
                color = 2;
            }
            PieceType::L => {
                center = CenterPoint(1, 4);
                pos_0 = [RelPoint(-1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, -1)];
                pos_2 = [RelPoint(-1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, -1)];
                color = 3;
            }
            PieceType::O => {
                center = CenterPoint(1, 4);
                pos_0 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_2 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                color = 4;
            }
            PieceType::S => {
                center = CenterPoint(0, 4);
                pos_0 = [RelPoint(1, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(1, 1)];
                pos_2 = [RelPoint(1, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(1, 1)];
                color = 5;
            }
            PieceType::T => {
                center = CenterPoint(1, 4);
                pos_0 = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(0, 1)];
                color = 6;
            }
            PieceType::Z => {
                center = CenterPoint(0, 4);
                pos_0 = [RelPoint(0, -1), RelPoint(1, 0), RelPoint(1, 1)];
                pos_1 = [RelPoint(-1, 1), RelPoint(0, 1), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -1), RelPoint(1, 0), RelPoint(1, 1)];
                pos_3 = [RelPoint(-1, 1), RelPoint(0, 1), RelPoint(1, 0)];
                color = 7;
            }
        };
        Piece {
            center,
            current_rotation_id: 0,
            rotations: [pos_0, pos_1, pos_2, pos_3],
            color,
        }
    }
    pub fn move_piece(&mut self, move_x: i16, move_y: i16) {
        let new_center_x = (self.center.0 as i16 + move_x) as usize;
        let new_center_y = (self.center.1 as i16 + move_y) as usize;
        self.center.0 = new_center_x;
        self.center.1 = new_center_y;
    }
}

#[derive(Default, Debug)]
pub struct Game {
    pub playfield: [[i16; 10]; 11],
    pub moving_piece: Option<Piece>,
    pub moving_piece_rotation: usize,
    // next_piece: Option<Piece>,
    // hold_piece: Option<Piece>,
}

pub enum Rotation {
    // Clockwise,
    CounterClockwise,
}

pub enum TetrisDirection {
    // Up,
    Down,
    Left,
    Right,
}

// const CLOCKWISE: Rotation = Rotation::Clockwise;
pub const COUNTER_CLOCKWISE: Rotation = Rotation::CounterClockwise;
// const UP: TetrisDirection = TetrisDirection::Up;
pub const DOWN: TetrisDirection = TetrisDirection::Down;
pub const LEFT: TetrisDirection = TetrisDirection::Left;
pub const RIGHT: TetrisDirection = TetrisDirection::Right;

impl Game {
    pub fn add_piece(&mut self, piece: Piece) {
        self.playfield[piece.center.0][piece.center.1] = piece.color;
        for point in piece.rotations[0].iter() {
            self.playfield[(point.0 + piece.center.0 as i16) as usize]
                [(point.1 + piece.center.1 as i16) as usize] = piece.color;
        }
        self.moving_piece_rotation = 0;
        self.moving_piece = Some(piece);
    }
    fn clear_piece_location(&mut self) {
        let piece = self.moving_piece.as_mut().unwrap();
        self.playfield[piece.center.0][piece.center.1] = 0;
        for point in piece.rotations[self.moving_piece_rotation].iter() {
            self.playfield[(point.0 + piece.center.0 as i16) as usize]
                [(point.1 + piece.center.1 as i16) as usize] = 0;
        }
    }

    fn update_piece_location(&mut self) -> Result<(), &str> {
        let piece = self.moving_piece.as_mut().unwrap();
        let mut new_positions: [RelPoint; 3] = [
            RelPoint::default(),
            RelPoint::default(),
            RelPoint::default(),
        ];
        for (id, point) in piece.rotations[self.moving_piece_rotation]
            .iter()
            .enumerate()
        {
            let new_x = (point.0 + piece.center.0 as i16) as usize;
            let new_y = (point.1 + piece.center.1 as i16) as usize;
            if new_x > self.playfield.len() - 1 || new_y > self.playfield[0].len() - 1 {
                return Err("Not enough room to move piece");
            }
            new_positions[id] = RelPoint(new_x as i16, new_y as i16);
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
            // Rotation::Clockwise => {
            //     if self.moving_piece_rotation == 0 {
            //         self.moving_piece_rotation = 3
            //     } else {
            //         self.moving_piece_rotation = self.moving_piece_rotation - 1
            //     }
            // }
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
            TetrisDirection::Down => self.moving_piece.as_mut().unwrap().move_piece(1, 0),
            TetrisDirection::Left => self.moving_piece.as_mut().unwrap().move_piece(0, -1),
            TetrisDirection::Right => self.moving_piece.as_mut().unwrap().move_piece(0, 1),
        }
        self.update_piece_location()?;
        return Ok(());
    }
}

pub fn draw_game(
    terminal: &mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
    game: &Game,
) -> Result<(), Box<dyn error::Error>> {
    terminal.draw(|f| {
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
                    8 => Color::DarkGray,
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
    })?;
    Ok(())
}
