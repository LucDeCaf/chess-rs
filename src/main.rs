use chess::board::{Board, Move};

// Starting position
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

fn process_command(command: &Command, board: &mut Board) -> String {
    match command {
        Command::Uci => "id name Chress\nid author Luc de Cafmeyer\nuciok".to_owned(),
        Command::IsReady => "readyok".to_owned(),
        Command::UciNewGame => "readyok".to_owned(),
        Command::Position(moves) => {
            for mv in moves.split(' ') {
                if mv == "startpos" {
                    board.load_from_fen(START_FEN);
                    continue;
                }
                board.make_move(&Move::from_long_algebraic(mv).unwrap());
            }
            "".to_owned()
        }
        Command::Go => "bestmove 0000".to_owned(),
        Command::Quit => "".to_owned(),
        Command::Invalid => "".to_owned(),
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

        println!("{}", response);
    }
}
