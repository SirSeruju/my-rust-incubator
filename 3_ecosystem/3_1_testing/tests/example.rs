use std::assert_eq;
use std::io::{prelude::*, BufReader};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command, Stdio};

use cucumber::writer::out::WriteStrExt;
use cucumber::{given, then, when, World};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug)]
enum GuessGame {
    BeforeExecution(Command),
    Execute(ChildStdin, BufReader<ChildStdout>, BufReader<ChildStderr>),
}

impl GuessGame {
    fn set_number(&mut self, string: String) {
        match self {
            GuessGame::BeforeExecution(c) => c.arg(string),
            GuessGame::Execute { .. } => {
                panic!("failed to set number to guess, game already started!")
            }
        };
    }

    fn start_game(&mut self) {
        match self {
            GuessGame::BeforeExecution(c) => {
                let mut c = c.spawn().expect("failed to start a game with error:");
                *self = GuessGame::Execute(
                    c.stdin.take().unwrap(),
                    BufReader::new(c.stdout.take().unwrap()),
                    BufReader::new(c.stderr.take().unwrap()),
                )
            }
            GuessGame::Execute(..) => panic!("failed to start a game, already started!"),
        }
    }
    fn get_stdout(&mut self) -> String {
        match self {
            GuessGame::BeforeExecution(_) => panic!("failed to get output, game is not started!"),
            GuessGame::Execute(_, reader, _) => {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .expect("failed to read line with error:");
                line.trim().to_string()
            }
        }
    }
    fn get_stderr(&mut self) -> String {
        match self {
            GuessGame::BeforeExecution(_) => panic!("failed to get output, game is not started!"),
            GuessGame::Execute(_, _, reader) => {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .expect("failed to read line with error:");
                line.trim().to_string()
            }
        }
    }
    fn send_stdin(&mut self, string: String) {
        match self {
            GuessGame::BeforeExecution(_) => panic!("failed to send input, game is not started!"),
            GuessGame::Execute(stdin, _, _) => {
                stdin.write_line(string).unwrap();
            }
        }
    }
    fn guess(&mut self, string: String) {
        match self {
            GuessGame::BeforeExecution(_) => panic!("failed to guess, game is not started!"),
            GuessGame::Execute(..) => {
                self.send_stdin(string + "\n");
            }
        }
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct GuessGameWorld {
    guess_game: GuessGame,
}

impl GuessGameWorld {
    fn new() -> Self {
        let mut c = Command::new("cargo");
        c.args(["run", "-q", "--"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        Self {
            guess_game: GuessGame::BeforeExecution(c),
        }
    }
}

#[given(expr = "number for guess is {word}")]
fn guessed_number(world: &mut GuessGameWorld, string: String) {
    world.guess_game.set_number(string);
}

#[then("game is started")]
fn start_game(world: &mut GuessGameWorld) {
    world.guess_game.start_game();
}

#[then("it prints welcome message")]
fn welcome_message(world: &mut GuessGameWorld) {
    assert_eq!(
        "Guess the number!".to_string(),
        world.guess_game.get_stdout()
    );
}

#[then("it asks number")]
fn asks_number(world: &mut GuessGameWorld) {
    assert_eq!(
        "Please input your guess.".to_string(),
        world.guess_game.get_stdout()
    );
}

#[when(expr = "we guess {word}")]
fn guess(world: &mut GuessGameWorld, string: String) {
    world.guess_game.guess(string);
}

#[then(expr = "it prints we asked {int}")]
fn we_asked(world: &mut GuessGameWorld, number: u32) {
    assert_eq!(
        format!("You guessed: {}", number),
        world.guess_game.get_stdout()
    );
}

#[then("it prints number too small")]
fn too_small(world: &mut GuessGameWorld) {
    assert_eq!("Too small!".to_string(), world.guess_game.get_stdout());
    // for some reason it double prints "Please input your guess." after this
    // dont know how to fix it, so check if so
    assert_eq!(
        "Please input your guess.".to_string(),
        world.guess_game.get_stdout()
    );
}

#[then("it prints number too big")]
fn too_big(world: &mut GuessGameWorld) {
    assert_eq!("Too big!".to_string(), world.guess_game.get_stdout());
    // TODO: FIX IT
    // for some reason it double prints "Please input your guess." after this
    // dont know how to fix it, so check if so
    assert_eq!(
        "Please input your guess.".to_string(),
        world.guess_game.get_stdout()
    );
}

#[then("it paniced")]
fn paniced(world: &mut GuessGameWorld) {
    assert!(world
        .guess_game
        .get_stderr()
        .contains("thread 'main' panicked at "));
}

#[then("it prints we win!")]
fn win(world: &mut GuessGameWorld) {
    assert_eq!("You win!".to_string(), world.guess_game.get_stdout());
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GuessGameWorld::run("tests/features"));
}
