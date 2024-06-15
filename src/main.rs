use chess::board::{Board, START_FEN};
use chess::moves::Move;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Uci,
    IsReady,
    UciNewGame,
    Position(String),
    Go,
    Quit,
    Invalid,
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.split(' ');

    match parts.next().unwrap() {
        "uci\n" => Command::Uci,
        "isready\n" => Command::IsReady,
        "ucinewgame\n" => Command::UciNewGame,
        "position\n" => Command::Position(parts.collect::<String>()),
        "go\n" => Command::Go,
        "quit\n" => Command::Quit,
        _ => Command::Invalid,
    }
}

fn process_command(command: &Command, board: &mut Board) -> Option<&'static str> {
    match command {
        Command::Uci => Some("id name Chress\nid author Luc de Cafmeyer\nuciok\n"),
        Command::IsReady => Some("readyok\n"),
        Command::UciNewGame => Some("readyok\n"),
        Command::Position(moves) => {
            for mv in moves.split(' ') {
                if mv == "startpos" {
                    board.load_from_fen(START_FEN);
                    continue;
                }
                board.make_move(Move::from_long_algebraic(mv).unwrap())
            }
            None
        }
        Command::Go => None,
        Command::Quit => None,
        Command::Invalid => None,
    }
}

fn main() {
    let mut board = Board::new();
    // board.load_from_fen(START_FEN);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(&input);
        let response = process_command(&command, &mut board);

        if command == Command::Quit {
            break;
        }

        if let Some(response) = response {
            print!("{}", response);
        }
    }
}
