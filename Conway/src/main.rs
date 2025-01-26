use core::time;
use std::thread;

pub mod conway;
fn main() {
    use conway::ConwayBoard;
    let mut board = ConwayBoard::new(11, 11);
    board.print_board();
    board.oposite(5, 5);
    board.oposite(6, 5);
    board.oposite(5, 6);
    board.oposite(4, 5);
    board.oposite(5, 4);
    let one_sec = time::Duration::from_millis(500);
    for _ in 0..=10 {
        board.print_board();
        board.update();
        thread::sleep(one_sec);
        print!("\x1B[2J");
    }
}
