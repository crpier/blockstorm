use blockbuster::{draw_game, Game, Piece, PieceType, COUNTER_CLOCKWISE, DOWN, LEFT, RIGHT};
use std::io;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

fn main() {
    let mut game = Game::default();
    game.add_piece_to_field(Piece::new(PieceType::I));
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(DOWN).unwrap();
    game.move_moving_piece(LEFT).unwrap();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    draw_game(&mut terminal, &game).unwrap();

    for event in io::stdin().keys() {
        match event {
            Ok(Key::Char('q')) => break,
            Ok(Key::Char('j')) => {
                match game.move_moving_piece(DOWN) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            }
            Ok(Key::Char('k')) => {
                match game.rotate_moving_piece(&COUNTER_CLOCKWISE) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            }
            Ok(Key::Char('h')) => {
                match game.move_moving_piece(LEFT) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            }
            Ok(Key::Char('l')) => {
                match game.move_moving_piece(RIGHT) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            }
            Ok(Key::Char('d')) => {
                game.place_moving_piece();
            }

            _ => (),
        }
        draw_game(&mut terminal, &game).unwrap();
    }
}
