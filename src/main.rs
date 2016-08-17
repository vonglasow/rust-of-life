extern crate ansi_term;

mod printer;
mod position;
mod cell;

#[cfg(not(test))]
fn main() {
    use cell::Cell;
    use printer::CliPrinter;
    use position::Position;

    println!("Game of Life");
    for x in 0..10 {
        let position = Position::create_2d(1, x);
        let cell = Cell {position: position, alive: true};
        let new_cell = cell.compute_state(25);
        let printer = CliPrinter;
        printer.print_cell(cell);
        printer.print_cell(new_cell);
    }

    println!("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
