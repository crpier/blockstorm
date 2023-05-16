use blockbuster::{
    draw_game, draw_game_over, Event, Game, MinoesError, OutOfBoundsError, OverlappingMinoesError,
    Piece, PieceType, DOWN,
};
use std::io;
use termion::raw::IntoRawMode;
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

    loop {
        let event = game.event_receiver.recv().unwrap();
        match event {
            Event::Quit => break,
            Event::MovePiece(direction) => {
                if !game_ended {
                    match game.move_moving_piece(direction) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Event::RotatePiece(rotation) => {
                if !game_ended {
                    match game.rotate_moving_piece(&rotation) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Event::HardDropPiece => {
                match game.hard_drop_moving_piece() {
                    Ok(_) => (),
                    Err(MinoesError::OverlappingMinoes(OverlappingMinoesError)) => {
                        draw_game_over(&mut terminal).unwrap();
                        game_ended = true;
                    }
                    Err(_) => panic!("Unexpected error"),
                };
            }
            Event::TimePassed => {
                if !game_ended {
                    match game.move_moving_piece(DOWN) {
                        Ok(_) => (),
                        Err(OutOfBoundsError) => match game.hard_drop_moving_piece() {
                            Ok(_) => (),
                            Err(MinoesError::OverlappingMinoes(OverlappingMinoesError)) => {
                                draw_game_over(&mut terminal).unwrap();
                                game_ended = true;
                            }
                            Err(_) => panic!("Unexpected error"),
                        },
                    };
                }
            }
        }
        if !game_ended {
            draw_game(&mut terminal, &game).unwrap();
        }
    }
}
