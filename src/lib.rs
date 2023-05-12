use std::{
    error::{self, Error},
    fmt, io,
};
use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Terminal,
};

pub enum TetrisDirection {
    Up,
    Down,
    Left,
    Right,
}

impl TetrisDirection {
    pub fn opposite_direction(&self) -> TetrisDirection {
        match self {
            TetrisDirection::Up => TetrisDirection::Down,
            TetrisDirection::Down => TetrisDirection::Up,
            TetrisDirection::Left => TetrisDirection::Right,
            TetrisDirection::Right => TetrisDirection::Left,
        }
    }
}

// const CLOCKWISE: Rotation = Rotation::Clockwise;
pub const COUNTER_CLOCKWISE: Rotation = Rotation::CounterClockwise;
// const UP: TetrisDirection = TetrisDirection::Up;
pub const DOWN: TetrisDirection = TetrisDirection::Down;
pub const LEFT: TetrisDirection = TetrisDirection::Left;
pub const RIGHT: TetrisDirection = TetrisDirection::Right;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(pub usize, pub usize);

#[derive(Debug, Default, Clone, Copy)]
pub struct RelPoint(pub i16, pub i16);

#[derive(Debug, Default)]
pub struct Piece {
    center: Point,
    current_rotation_id: usize,
    rotations: [[RelPoint; 3]; 4],
    pub color: i16,
}

#[derive(Debug, Default)]
pub struct OutOfBoundsError;

impl Error for OutOfBoundsError {}
impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds")
    }
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
        let center: Point;
        let pos_0: [RelPoint; 3];
        let pos_1: [RelPoint; 3];
        let pos_2: [RelPoint; 3];
        let pos_3: [RelPoint; 3];
        let color: i16;

        match piece_type {
            PieceType::I => {
                center = Point(1, 5);
                pos_0 = [RelPoint(0, -2), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-2, 0), RelPoint(-1, 0), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -2), RelPoint(0, -1), RelPoint(0, 1)];
                pos_3 = [RelPoint(-2, 0), RelPoint(-1, 0), RelPoint(1, 0)];
                color = 1;
            }
            PieceType::J => {
                center = Point(1, 5);
                pos_0 = [RelPoint(0, -1), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_1 = [RelPoint(-1, -1), RelPoint(-1, 0), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -1), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_3 = [RelPoint(-1, -1), RelPoint(-1, 0), RelPoint(1, 0)];
                color = 2;
            }
            PieceType::L => {
                center = Point(1, 4);
                pos_0 = [RelPoint(-1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, -1)];
                pos_2 = [RelPoint(-1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, -1)];
                color = 3;
            }
            PieceType::O => {
                center = Point(1, 4);
                pos_0 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_2 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(-1, 1)];
                color = 4;
            }
            PieceType::S => {
                center = Point(0, 4);
                pos_0 = [RelPoint(1, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(1, 1)];
                pos_2 = [RelPoint(1, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(0, 1), RelPoint(1, 1)];
                color = 5;
            }
            PieceType::T => {
                center = Point(1, 4);
                pos_0 = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(1, 0)];
                pos_2 = [RelPoint(0, -1), RelPoint(1, 0), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(0, 1)];
                color = 6;
            }
            PieceType::Z => {
                center = Point(0, 4);
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

    pub fn get_piece_points(&self) -> Result<[Point; 4], OutOfBoundsError> {
        let mut points = [Point(0, 0); 4];
        let current_rotation = self.rotations[self.current_rotation_id];
        for i in 0..3 {
            let rel_point = current_rotation[i];
            if self.center.0 as i16 + rel_point.0 < 0 {
                return Err(OutOfBoundsError);
            }
            let point = Point(
                (self.center.0 as i16 + rel_point.0) as usize,
                (self.center.1 as i16 + rel_point.1) as usize,
            );
            points[i] = point;
        }
        points[3] = self.center;
        Ok(points)
    }

    fn move_piece_by(&mut self, x: i16, y: i16) {
        self.center = Point(
            (self.center.0 as i16 + x) as usize,
            (self.center.1 as i16 + y) as usize,
        );
    }

    pub fn move_piece(&mut self, direction: &TetrisDirection) {
        match direction {
            TetrisDirection::Up => {
                self.move_piece_by(-1, 0);
            }
            TetrisDirection::Down => {
                self.move_piece_by(1, 0);
            }
            TetrisDirection::Left => {
                self.move_piece_by(0, -1);
            }
            TetrisDirection::Right => {
                self.move_piece_by(0, 1);
            }
        }
    }

    pub fn rotate_piece(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::CounterClockwise => {
                self.current_rotation_id = (self.current_rotation_id + 1) % 4;
            }
            Rotation::Clockwise => {
                if self.current_rotation_id == 0 {
                    // can't do modulo because current_rotation_id is unsigned
                    self.current_rotation_id = 3;
                } else {
                    self.current_rotation_id -= 1;
                }
            }
        }
    }
}

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Rotation {
    pub fn other_direction(&self) -> Rotation {
        match self {
            Rotation::Clockwise => Rotation::CounterClockwise,
            Rotation::CounterClockwise => Rotation::Clockwise,
        }
    }
}

#[derive(Default, Debug)]
pub struct Game {
    pub playfield: [[i16; 10]; 11],
    pub moving_piece: Piece,
    // TODO: remove this field
    pub moving_piece_rotation: usize,
    // next_piece: Option<Piece>,
    // hold_piece: Option<Piece>,
}

impl Game {
    fn fill_piece_points(&mut self) {
        let piece_points = self.moving_piece.get_piece_points().unwrap();
        for point in piece_points.iter() {
            self.playfield[point.0][point.1] = self.moving_piece.color;
        }
    }

    fn clear_piece_points(&mut self) {
        let piece_points = self.moving_piece.get_piece_points().unwrap();
        for point in piece_points.iter() {
            self.playfield[point.0][point.1] = 0;
        }
    }

    fn moving_piece_is_within_bounds(&self) -> bool {
        let piece_points = match self.moving_piece.get_piece_points() {
            Ok(points) => points,
            Err(_) => return false,
        };

        for point in piece_points.iter() {
            if point.0 >= self.playfield.len() || point.1 >= self.playfield[0].len() {
                return false;
            }
        }
        return true;
    }

    pub fn add_piece_to_field(&mut self, piece: Piece) {
        self.fill_piece_points();
        self.moving_piece = piece;
    }

    pub fn rotate_moving_piece(&mut self, direction: &Rotation) -> Result<(), OutOfBoundsError> {
        self.clear_piece_points();
        self.moving_piece.rotate_piece(direction);
        if !self.moving_piece_is_within_bounds() {
            self.moving_piece.rotate_piece(&direction.other_direction());
            self.fill_piece_points();
            return Err(OutOfBoundsError);
        }
        self.fill_piece_points();
        return Ok(());
    }

    pub fn move_piece(&mut self, direction: TetrisDirection) -> Result<(), OutOfBoundsError> {
        self.clear_piece_points();
        self.moving_piece.move_piece(&direction);
        if !self.moving_piece_is_within_bounds() {
            self.moving_piece
                .move_piece(&direction.opposite_direction());
            self.fill_piece_points();
            return Err(OutOfBoundsError);
        }
        self.fill_piece_points();
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
                    // Empty tile
                    0 => Color::Black,
                    // Pieces
                    1 => Color::Cyan,
                    2 => Color::Blue,
                    3 => Color::Red,
                    4 => Color::Yellow,
                    5 => Color::Green,
                    6 => Color::Magenta,
                    7 => Color::LightRed,
                    // Pieces already placed
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
