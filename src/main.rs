use blockbuster::{draw_game, Game, Piece, COUNTER_CLOCKWISE, DOWN, LEFT, RIGHT, PieceType};
use std::io;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

fn main() {
    let mut game = Game::default();
    game.add_piece(Piece::new(PieceType::T));
    game.move_piece(DOWN).unwrap();
    // game.move_piece(DOWN).unwrap();
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
            Ok(Key::Char('k')) => game.rotate_piece(COUNTER_CLOCKWISE),
            Ok(Key::Char('h')) => game.move_piece(LEFT).unwrap(),
            Ok(Key::Char('l')) => game.move_piece(RIGHT).unwrap(),
            _ => (),
        }
        draw_game(&mut terminal, &game).unwrap();
    }
}
