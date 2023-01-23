mod board;
mod wfc;
use board::{get_input, SBoard};

fn main() {
    run();
}

fn run() {
    print!("Do u want to load an existing board?(y/n): ");
    let choice = get_input();
    let mut board = match choice {
        'y' => SBoard::load(),
        'n' => SBoard::new(),
        _ => panic!("Invalid choice!"),
    };
    SBoard::display(board.board);

    let mut Wfc = wfc::WFC::new();
    Wfc.load(board);

    loop{
        Wfc.run();
        SBoard::display(Wfc.board);
        println!("\n\n\n");
    }
}
