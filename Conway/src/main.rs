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
    board.print_board();
    board.update();
    board.print_board();
    board.update();
    board.print_board();
    board.update();
    board.print_board();
    board.update();
    board.print_board();
}
