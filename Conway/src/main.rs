use core::time;
use std::thread;

pub mod conway;
fn main() {
    use conway::ConwayBoard;
    let mut board = ConwayBoard::new(21, 21);
    board.print_board();
    board.oposite(10, 10);
    board.oposite(11, 10);
    board.oposite(9, 10);
    board.oposite(10, 9);
    board.oposite(9, 11);
    let one_sec = time::Duration::from_millis(250);
    for _ in 0..=100 {
        board.print_board();
        board.update();
        thread::sleep(one_sec);
        //clears the screen
        print!("\x1B[2J");
    }
}
