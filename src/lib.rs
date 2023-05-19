use rand::seq::SliceRandom;
use std::{
    error::{self, Error},
    fmt, io,
    sync::mpsc::{self, Receiver},
    thread::{self, sleep},
    time::{Duration, Instant},
};
use termion::{event::Key, input::TermRead};

use termion::raw::RawTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Terminal,
};

#[derive(Debug)]
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
pub const DOWN: TetrisDirection = TetrisDirection::Down;
pub const LEFT: TetrisDirection = TetrisDirection::Left;
pub const RIGHT: TetrisDirection = TetrisDirection::Right;

pub const KICKS: [(i16, i16); 5] = [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)];

#[derive(Debug)]
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
pub const COUNTER_CLOCKWISE: Rotation = Rotation::CounterClockwise;
pub const CLOCKWISE: Rotation = Rotation::Clockwise;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(pub i16, pub i16);

#[derive(Debug, Default, Clone, Copy)]
pub struct RelPoint(pub i16, pub i16);

#[derive(Debug, Default)]
pub struct OutOfBoundsError;
impl Error for OutOfBoundsError {}
impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds")
    }
}
#[derive(Debug, Default)]
pub struct OverlappingMinoesError;
impl Error for OverlappingMinoesError {}
impl fmt::Display for OverlappingMinoesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds")
    }
}
impl From<OutOfBoundsError> for MinoesError {
    fn from(_: OutOfBoundsError) -> Self {
        MinoesError::OutOfBounds(OutOfBoundsError)
    }
}
#[derive(Debug)]
pub enum MinoesError {
    OutOfBounds(OutOfBoundsError),
    OverlappingMinoes(OverlappingMinoesError),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Piece {
    center: Point,
    current_rotation_id: usize,
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
                pos_2 = [RelPoint(1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_3 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, 1)];
                color = 2;
            }
            PieceType::L => {
                center = Point(1, 4);
                pos_0 = [RelPoint(-1, -1), RelPoint(0, -1), RelPoint(0, 1)];
                pos_1 = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(1, -1)];
                pos_2 = [RelPoint(0, -1), RelPoint(0, 1), RelPoint(1, 1)];
                pos_3 = [RelPoint(-1, 1), RelPoint(-1, 0), RelPoint(1, 0)];
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
                self.center.0 as i16 + rel_point.0,
                self.center.1 as i16 + rel_point.1,
            );
            points[i] = point;
        }
        points[3] = self.center;
        Ok(points)
    }

    fn move_piece_by(&mut self, x: i16, y: i16) {
        self.center = Point(self.center.0 + x, self.center.1 + y);
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

#[derive(Debug)]
pub enum Event {
    TimePassed,
    MovePiece(TetrisDirection),
    RotatePiece(Rotation),
    HardDropPiece,
    HoldPiece,
    Quit,
}

#[derive(Debug)]
pub struct Game {
    pub event_receiver: Receiver<Event>,
    pub ghost_piece: Option<Piece>,
    pub moving_piece: Piece,
    pub playfield: [[i16; 10]; 22],
    pub score: u16,
    pub level: u16,
    pub lines_cleared: u16,
    pub next_pieces: Vec<Piece>,
    pub held_piece: Option<Piece>,
}

impl Default for Game {
    fn default() -> Game {
        let (event_sender, event_receiver) = mpsc::channel();
        let key_sender = event_sender.clone();
        event_sender
            .send(Event::TimePassed)
            .expect("Could not send message");

        thread::spawn(move || {
            let mut last_action_time = Instant::now();
            loop {
                let elapsed = Instant::now().duration_since(last_action_time);
                let time_elapsed = elapsed >= Duration::from_millis(1000);

                if time_elapsed {
                    event_sender
                        .send(Event::TimePassed)
                        .expect("Could not send message");
                    last_action_time = Instant::now();
                }

                sleep(Duration::from_millis(10));
            }
        });

        thread::spawn(move || {
            for event in io::stdin().keys() {
                match event {
                    Ok(Key::Char('q')) => {
                        key_sender
                            .send(Event::Quit)
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('j')) => {
                        key_sender
                            .send(Event::MovePiece(TetrisDirection::Down))
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('k')) => {
                        key_sender
                            .send(Event::RotatePiece(CLOCKWISE))
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('h')) => {
                        key_sender
                            .send(Event::MovePiece(TetrisDirection::Left))
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('l')) => {
                        key_sender
                            .send(Event::MovePiece(TetrisDirection::Right))
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('d')) => {
                        key_sender
                            .send(Event::HardDropPiece)
                            .expect("Could not send message");
                    }
                    Ok(Key::Char('c')) => {
                        key_sender
                            .send(Event::HoldPiece)
                            .expect("Could not send message");
                    }

                    _ => (),
                }
            }
        });

        let mut rng = rand::thread_rng();
        let mut next_pieces = vec![
            Piece::new(PieceType::I),
            Piece::new(PieceType::J),
            Piece::new(PieceType::L),
            Piece::new(PieceType::O),
            Piece::new(PieceType::S),
            Piece::new(PieceType::T),
            Piece::new(PieceType::Z),
        ];
        next_pieces.shuffle(&mut rng);

        return Self {
            event_receiver,
            ghost_piece: None,
            moving_piece: next_pieces.pop().unwrap(),
            playfield: [[0; 10]; 22],
            score: 0,
            level: 1,
            lines_cleared: 0,
            next_pieces,
            held_piece: None,
        };
    }
}

impl Game {
    pub fn get_next_piece_in_queue(&mut self, pop: bool) -> Piece {
        let mut next_bag = Vec::new();
        if self.next_pieces.last().is_none() {
            let mut rng = rand::thread_rng();
            next_bag = vec![
                Piece::new(PieceType::I),
                Piece::new(PieceType::J),
                Piece::new(PieceType::L),
                Piece::new(PieceType::O),
                Piece::new(PieceType::S),
                Piece::new(PieceType::T),
                Piece::new(PieceType::Z),
            ];

            next_bag.shuffle(&mut rng);
        }
        self.next_pieces.append(&mut next_bag);

        if pop {
            self.next_pieces.pop().unwrap()
        } else {
            self.next_pieces.last().unwrap().clone()
        }
    }

    fn fill_piece_points(&mut self, piece: &Piece) -> Result<(), MinoesError> {
        let piece_points = piece.get_piece_points().unwrap();
        for point in piece_points.iter() {
            if point.0 < 0 || point.1 < 0 {
                return Err(MinoesError::OutOfBounds(OutOfBoundsError));
            }
            if point.0 >= self.playfield.len() as i16 || point.1 >= self.playfield[0].len() as i16 {
                return Err(MinoesError::OutOfBounds(OutOfBoundsError));
            }
            if self.playfield[point.0 as usize][point.1 as usize] == piece.color {
                continue;
            }

            if self.playfield[point.0 as usize][point.1 as usize] > 0 {
                return Err(MinoesError::OverlappingMinoes(OverlappingMinoesError));
            }
            self.playfield[point.0 as usize][point.1 as usize] = piece.color;
        }
        Ok(())
    }

    fn clear_piece_points(&mut self, piece: &Piece) -> Result<(), OutOfBoundsError> {
        let piece_points = piece.get_piece_points().unwrap();
        for point in piece_points.iter() {
            if point.0 < 0 || point.1 < 0 {
                return Err(OutOfBoundsError);
            }
            if point.0 >= self.playfield.len() as i16 || point.1 >= self.playfield[0].len() as i16 {
                return Err(OutOfBoundsError);
            }
            if self.playfield[point.0 as usize][point.1 as usize] != piece.color {
                continue;
            }
            self.playfield[point.0 as usize][point.1 as usize] = 0;
        }
        Ok(())
    }

    // TODO: remove pub
    pub fn piece_is_in_allowed_position(&self, piece: &Piece) -> Result<(), MinoesError> {
        let piece_points = match piece.get_piece_points() {
            Ok(points) => points,
            Err(_) => return Err(MinoesError::OutOfBounds(OutOfBoundsError)),
        };

        for point in piece_points.iter() {
            if point.0 < 0 || point.1 < 0 {
                return Err(MinoesError::OutOfBounds(OutOfBoundsError));
            }
            if point.0 >= self.playfield.len() as i16 || point.1 >= self.playfield[0].len() as i16 {
                return Err(MinoesError::OutOfBounds(OutOfBoundsError));
            }
            if self.playfield[point.0 as usize][point.1 as usize] > 0 {
                return Err(MinoesError::OverlappingMinoes(OverlappingMinoesError));
            }
        }
        return Ok(());
    }

    // TODO: make private
    pub fn piece_is_out_of_bounds(&self, piece: &Piece) -> bool {
        let piece_points = match piece.get_piece_points() {
            Ok(points) => points,
            Err(_) => return true,
        };

        for point in piece_points.iter() {
            if point.0 < 0 || point.1 < 0 {
                return true;
            }
            if point.0 >= self.playfield.len() as i16 || point.1 >= self.playfield[0].len() as i16 {
                return true;
            }
        }
        return false;
    }

    pub fn piece_is_overlapping_with(&self, piece: &Piece, checked: i16) -> bool {
        let piece_points = piece.get_piece_points().unwrap();

        for point in piece_points.iter() {
            if self.playfield[point.0 as usize][point.1 as usize] == checked {
                return true;
            }
        }
        return false;
    }

    fn fill_field_with_dropped_points(&mut self, points: [Point; 4]) {
        for point in points.iter() {
            self.playfield[point.0 as usize][point.1 as usize] = 8;
        }
    }

    fn update_ghost_piece(&mut self) {
        if let Some(ghost_piece) = &self.ghost_piece {
            self.clear_piece_points(&ghost_piece.clone()).unwrap();
        }
        let mut ghost_piece = self.moving_piece.clone();
        ghost_piece.color = -1;

        let mut lowered_counter = 0;
        while !self.piece_is_out_of_bounds(&ghost_piece)
            && !self.piece_is_overlapping_with(&ghost_piece, 8)
        {
            ghost_piece.move_piece(&TetrisDirection::Down);
            lowered_counter += 1;
        }

        // Move the ghost piece up one step, since we lowered it one step too far
        if lowered_counter != 0 {
            ghost_piece.move_piece(&TetrisDirection::Up);
        }
        match self.fill_piece_points(&ghost_piece) {
            Ok(_) => {}
            Err(_) => {}
        }
        self.ghost_piece = Some(ghost_piece);
        // Re-draw the moving piece, since we might have overwritten it
        // While drawing the ghost piece, and then gave up
        self.fill_piece_points(&self.moving_piece.clone()).unwrap();
    }

    pub fn add_piece_to_field(&mut self, piece: Piece) -> Result<(), MinoesError> {
        self.moving_piece = piece;
        self.fill_piece_points(&self.moving_piece.clone())?;
        self.update_ghost_piece();
        return Ok(());
    }

    pub fn rotate_moving_piece(&mut self, direction: &Rotation) -> Result<(), OutOfBoundsError> {
        let mut ok = false;
        let old_piece = self.moving_piece.clone();
        self.clear_piece_points(&old_piece).unwrap();
        for (kick_x, kick_y) in KICKS.iter() {
            self.moving_piece.move_piece_by(*kick_x, *kick_y);
            self.moving_piece.rotate_piece(direction);
            if let Ok(_) = self.piece_is_in_allowed_position(&self.moving_piece.clone()) {
                ok = true;
                break;
            }
            self.moving_piece.move_piece_by(-*kick_x, -*kick_y);
            self.moving_piece.rotate_piece(&direction.other_direction());
        }
        if !ok {
            self.fill_piece_points(&old_piece).unwrap();
            return Err(OutOfBoundsError);
        }
        self.fill_piece_points(&self.moving_piece.clone()).unwrap();
        self.update_ghost_piece();
        return Ok(());
    }

    pub fn move_moving_piece(
        &mut self,
        direction: TetrisDirection,
    ) -> Result<(), OutOfBoundsError> {
        self.clear_piece_points(&self.moving_piece.clone()).unwrap();
        self.moving_piece.move_piece(&direction);
        if let Err(_) = self.piece_is_in_allowed_position(&self.moving_piece.clone()) {
            self.moving_piece
                .move_piece(&direction.opposite_direction());
            self.fill_piece_points(&self.moving_piece.clone()).unwrap();
            return Err(OutOfBoundsError);
        }
        self.update_ghost_piece();
        self.fill_piece_points(&self.moving_piece.clone()).unwrap();
        return Ok(());
    }

    // There is no need for a separate lock function, since a lock is really a hard drop from
    // lowest possible height
    pub fn hard_drop_moving_piece(&mut self) -> Result<(), MinoesError> {
        self.clear_piece_points(&self.moving_piece.clone())?;
        self.clear_piece_points(&self.ghost_piece.unwrap().clone())?;
        self.moving_piece = self.ghost_piece.unwrap().clone();
        self.ghost_piece = None;
        self.fill_field_with_dropped_points(self.moving_piece.get_piece_points().unwrap());
        let cleared_lines_count = self.clear_filled_lines();
        self.adjust_level(cleared_lines_count);
        self.adjust_score(cleared_lines_count);
        let next_piece = self.get_next_piece_in_queue(true);
        self.add_piece_to_field(next_piece)?;
        return Ok(());
    }

    pub fn hold_moving_piece(&mut self) -> Result<(), MinoesError> {
        self.clear_piece_points(&self.moving_piece.clone())?;
        self.clear_piece_points(&self.ghost_piece.unwrap().clone())?;
        match self.held_piece {
            Some(piece) => {
                let old_held_piece = piece;
                self.held_piece = Some(self.moving_piece.clone());
                self.moving_piece = old_held_piece;
            }
            None => {
                self.held_piece = Some(self.moving_piece.clone());
                self.moving_piece = self.get_next_piece_in_queue(true);
            }
        }
        self.fill_piece_points(&self.moving_piece.clone()).unwrap();
        self.add_piece_to_field(self.moving_piece)?;
        return Ok(());
    }

    fn adjust_level(&mut self, cleared_lines: usize) {
        self.lines_cleared += cleared_lines as u16;
        if self.lines_cleared >= self.level * 10 {
            self.level += 1;
        }
    }

    fn adjust_score(&mut self, cleared_lines: usize) {
        match cleared_lines {
            1 => self.score += 100 * self.level,
            2 => self.score += 300 * self.level,
            3 => self.score += 500 * self.level,
            4 => self.score += 800 * self.level,
            _ => {}
        }
    }

    #[allow(unused_assignments)]
    pub fn clear_filled_lines(&mut self) -> usize {
        let mut cleared_lines: usize = 0;
        for last_line in (0..self.playfield.len()).rev() {
            while self.playfield[last_line].iter().all(|x| *x == 8) {
                cleared_lines += 1;
                for i in (1..=last_line).rev() {
                    self.playfield[i] = self.playfield[i - 1];
                }
            }
        }
        return cleared_lines;
    }
}

pub fn draw_game(
    terminal: &mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
    game: &mut Game,
) -> Result<(), Box<dyn error::Error>> {
    terminal.draw(|f| {
        let vertical_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length((game.playfield.len() + 2).try_into().unwrap()),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(f.size());
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(20),
                    Constraint::Length((game.playfield[0].len() * 2 + 2).try_into().unwrap()),
                    Constraint::Length(21),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(vertical_chunk[0]);
        let piece_info_section = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(10),
                    Constraint::Percentage(45),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let mut next_piece_field = game.playfield.clone().map(|row| row.map(|_cell| 0));
        let piece = game.get_next_piece_in_queue(false);
        let points = piece.get_piece_points().unwrap();
        for point in points {
            next_piece_field[(point.0 - piece.center.0 + 3) as usize]
                [(point.1 - piece.center.1 + 4) as usize] = piece.color;
        }
        let next_piece_rows = next_piece_field.map(|row| {
            Row::new(row.map(|el| {
                let color = match el {
                    // Pieces
                    n if n.rem_euclid(10) == 1 => Color::Cyan,
                    n if n.rem_euclid(10) == 2 => Color::Blue,
                    n if n.rem_euclid(10) == 3 => Color::Red,
                    n if n.rem_euclid(10) == 4 => Color::Yellow,
                    n if n.rem_euclid(10) == 5 => Color::Green,
                    n if n.rem_euclid(10) == 6 => Color::Magenta,
                    n if n.rem_euclid(10) == 7 => Color::LightRed,
                    // Pieces already placed
                    n if n.rem_euclid(10) == 8 => Color::DarkGray,
                    // Ghost piece
                    // Empty tile
                    n if n.rem_euclid(10) == 0 => Color::Black,
                    _ => Color::White,
                };
                Cell::from("").style(Style::default().bg(color))
            }))
        });
        let next_piece_table = Table::new(next_piece_rows)
            // You can set the style of the entire Table.
            .style(Style::default().fg(Color::White))
            // As any other widget, a Table can be wrapped in a Block.
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
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .column_spacing(0)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .title("Next piece")
                    .title_alignment(tui::layout::Alignment::Center)
                    .borders(Borders::ALL),
            );
        f.render_widget(next_piece_table, piece_info_section[0]);

        let mut held_piece_field = game.playfield.clone().map(|row| row.map(|_cell| 0));
        match game.held_piece {
            Some(piece) => {
                let points = piece.get_piece_points().unwrap();
                for point in points {
                    held_piece_field[(point.0 - piece.center.0 + 3) as usize]
                        [(point.1 - piece.center.1 + 4) as usize] = piece.color;
                }
            }
            None => {}
        };
        let held_piece_rows = held_piece_field.map(|row| {
            Row::new(row.map(|el| {
                let color = match el {
                    // Pieces
                    n if n.rem_euclid(10) == 1 => Color::Cyan,
                    n if n.rem_euclid(10) == 2 => Color::Blue,
                    n if n.rem_euclid(10) == 3 => Color::Red,
                    n if n.rem_euclid(10) == 4 => Color::Yellow,
                    n if n.rem_euclid(10) == 5 => Color::Green,
                    n if n.rem_euclid(10) == 6 => Color::Magenta,
                    n if n.rem_euclid(10) == 7 => Color::LightRed,
                    // Pieces already placed
                    n if n.rem_euclid(10) == 8 => Color::DarkGray,
                    // Ghost piece
                    // Empty tile
                    n if n.rem_euclid(10) == 0 => Color::Black,
                    _ => Color::White,
                };
                Cell::from("").style(Style::default().bg(color))
            }))
        });
        let held_piece_table = Table::new(held_piece_rows)
            // You can set the style of the entire Table.
            .style(Style::default().fg(Color::White))
            // As any other widget, a Table can be wrapped in a Block.
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
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .column_spacing(0)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .title("Hold")
                    .title_alignment(tui::layout::Alignment::Center)
                    .borders(Borders::ALL),
            );
        f.render_widget(held_piece_table, piece_info_section[2]);

        let field_rows = game.playfield.map(|row| {
            Row::new(row.map(|el| {
                let color = match el {
                    // Pieces
                    n if n.rem_euclid(10) == 1 => Color::Cyan,
                    n if n.rem_euclid(10) == 2 => Color::Blue,
                    n if n.rem_euclid(10) == 3 => Color::Red,
                    n if n.rem_euclid(10) == 4 => Color::Yellow,
                    n if n.rem_euclid(10) == 5 => Color::Green,
                    n if n.rem_euclid(10) == 6 => Color::Magenta,
                    n if n.rem_euclid(10) == 7 => Color::LightRed,
                    // Pieces already placed
                    n if n.rem_euclid(10) == 8 => Color::DarkGray,
                    // Ghost piece
                    n if n < 0 => Color::White,
                    // Empty tile
                    n if n.rem_euclid(10) == 0 => Color::Black,
                    _ => Color::White,
                };
                Cell::from("").style(Style::default().bg(color))
            }))
        });
        let playfield_table = Table::new(field_rows)
            // You can set the style of the entire Table.
            .style(Style::default().fg(Color::White))
            // As any other widget, a Table can be wrapped in a Block.
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
            .column_spacing(0)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(playfield_table, chunks[1]);
        let text = vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Score")]),
            Spans::from(Span::styled(
                game.score.to_string(),
                Style::default().fg(Color::Red),
            )),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Level")]),
            Spans::from(Span::styled(
                game.level.to_string(),
                Style::default().fg(Color::Red),
            )),
        ];
        let score_section = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
            .split(chunks[2]);
        let score_paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Score")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(score_paragraph, score_section[0]);
    })?;
    Ok(())
}

pub fn draw_game_over(
    terminal: &mut Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
) -> Result<(), Box<dyn error::Error>> {
    terminal.draw(|f| {
        let block = Paragraph::new("Game Over");
        f.render_widget(block, f.size());
    })?;
    Ok(())
}
