use chess::board::{moves::Move, Board, START_FEN};
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Uci,
    IsReady,
    UciNewGame,
    Position(Vec<String>),
    Go,
    Quit,
}

fn parse_command(input: &str) -> Option<Command> {
    let mut parts = input.split(" ");

    match parts.next() {
        Some(str) => match str {
            "uci\n" => Some(Command::Uci),
            "isready\n" => Some(Command::IsReady),
            "ucinewgame\n" => Some(Command::UciNewGame),
            "position" => Some(Command::Position(
                parts.map(|s| s.to_string()).collect::<Vec<String>>(),
            )),
            "go\n" => Some(Command::Go),
            "quit\n" => Some(Command::Quit),
            _ => None,
        },
        _ => None,
    }
}

fn process_command(command: &Command, board: &mut Board) -> Option<String> {
    match command {
        Command::Uci => Some(String::from(
            "id name Chress\nid author Luc de Cafmeyer\nuciok",
        )),
        Command::IsReady => Some(String::from("readyok")),
        Command::UciNewGame => {
            *board = Board::new(START_FEN).unwrap();
            Some(String::from("readyok"))
        }
        Command::Position(moves) => {
            for mv in moves {
                let mv = mv.trim();

                if mv == "startpos" {
                    board.load_from_fen(START_FEN).unwrap();
                    continue;
                }

                let mv = Move::from_long_algebraic(mv)
                    .expect("UCI move should be in long algebraic notation");

                board.make_move(mv).expect("UCI move should be legal");
            }
            None
        }
        Command::Go => {
            // Play a random legal move
            let legal_moves = board.legal_moves();
            let random_index = thread_rng().gen_range(0..legal_moves.len());
            let random_move = legal_moves[random_index];
            Some(format!("bestmove {}", random_move.long_algebraic()))
        }
        Command::Quit => None,
    }
}

fn main() {
    let mut board = Board::new(START_FEN).unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let Some(command) = parse_command(&input) else {
            continue;
        };
        let response = process_command(&command, &mut board);

        if command == Command::Quit {
            break;
        }

        if let Some(response) = response {
            println!("{}", response);
        }
    }
}
