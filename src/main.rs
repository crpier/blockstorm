use blockbuster::{
    draw_game, draw_game_over, Event, Game, MinoesError, OutOfBoundsError, OverlappingMinoesError,
    DOWN,
};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

const LOCK_DELAY: Duration = Duration::from_millis(500);

fn main() {
    let mut game = Game::default();
    let mut game_ended = false;

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let mut last_piece_move = Instant::now();

    loop {
        let event = game.event_receiver.recv().unwrap();
        match event {
            Event::TimePassed => {
                if !game_ended {
                    match game.move_moving_piece(DOWN) {
                        Ok(_) => (),
                        Err(OutOfBoundsError) => {
                            let delay_passed = Instant::now() - last_piece_move > LOCK_DELAY;
                            if delay_passed {
                                last_piece_move = Instant::now();
                                match game.hard_drop_moving_piece() {
                                    Ok(_) => (),
                                    Err(MinoesError::OverlappingMinoes(OverlappingMinoesError)) => {
                                        draw_game_over(&mut terminal, &mut game).unwrap();
                                        game_ended = true;
                                    }
                                    Err(_) => panic!("Unexpected error"),
                                }
                            }
                        }
                    };
                }
            }
            Event::Quit => break,
            Event::MovePiece(direction) => {
                if !game_ended {
                    match direction {
                        blockbuster::TetrisDirection::Down => {}
                        _ => {
                            last_piece_move = Instant::now();
                        }
                    }
                    match game.move_moving_piece(direction) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Event::RotatePiece(rotation) => {
                if !game_ended {
                    last_piece_move = Instant::now();
                    match game.rotate_moving_piece(&rotation) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                }
            }
            Event::HardDropPiece => {
                if !game_ended {
                    last_piece_move = Instant::now();
                    match game.hard_drop_moving_piece() {
                        Ok(_) => (),
                        Err(MinoesError::OverlappingMinoes(OverlappingMinoesError)) => {
                            draw_game_over(&mut terminal, &mut game).unwrap();
                            game_ended = true;
                        }
                        Err(_) => panic!("Unexpected error"),
                    };
                }
            }
            Event::HoldPiece => {
                last_piece_move = Instant::now();
                if !game_ended {
                    match game.hold_moving_piece() {
                        Ok(_) => (),
                        Err(_) => (),
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
