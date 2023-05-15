use blockbuster::{
    draw_game, draw_game_over, Game, MinoesError, OverlappingMinoesError, Piece, PieceType,
    COUNTER_CLOCKWISE, DOWN, LEFT, RIGHT,
};
use std::io;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

fn main() {
    let mut game = Game::default();
    let mut game_ended = false;
    game.add_piece_to_field(Piece::new(PieceType::T)).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.move_moving_piece(DOWN).unwrap();
    // game.hard_drop_moving_piece();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    draw_game(&mut terminal, &game).unwrap();

    for event in io::stdin().keys() {
        match event {
            Ok(Key::Char('q')) => break,
            Ok(Key::Char('j')) => {
                if !game_ended {
                    match game.move_moving_piece(DOWN) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Ok(Key::Char('k')) => {
                if !game_ended {
                    match game.rotate_moving_piece(&COUNTER_CLOCKWISE) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Ok(Key::Char('h')) => {
                if !game_ended {
                    match game.move_moving_piece(LEFT) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Ok(Key::Char('l')) => {
                if !game_ended {
                    match game.move_moving_piece(RIGHT) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Ok(Key::Char('d')) => {
                match game.hard_drop_moving_piece() {
                    Ok(_) => (),
                    Err(MinoesError::OverlappingMinoes(OverlappingMinoesError)) => {
                        draw_game_over(&mut terminal).unwrap();
                        game_ended = true;
                    }
                    Err(_) => panic!("Unexpected error"),
                };
            }

            _ => (),
        }
        if !game_ended {
            draw_game(&mut terminal, &game).unwrap();
        }
    }
}
