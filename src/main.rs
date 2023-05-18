use blockbuster::{
    draw_game, draw_game_over, Event, Game, MinoesError, OutOfBoundsError, OverlappingMinoesError,
    DOWN,
};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::io;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

fn main() {
    let mut game = Game::default();
    let mut game_ended = false;

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

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
            Event::HoldPiece => {
                match game.hold_moving_piece() {
                    Ok(_) => (),
                    Err(_) => (),
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
            draw_game(&mut terminal, &mut game).unwrap();
        }
    }
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
