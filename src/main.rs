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
    let position = Position::create_2d(1, 1);
    let cell = Cell {position: position, alive: true};
    println!("Cell state is {}, x:{} , y:{}", cell.alive, cell.position.x, cell.position.y);
    println!("Change state of cell");
    let new_cell = cell.compute_state(25);
    println!("Old Cell state is {}, x:{} , y:{}", cell.alive, cell.position.x, cell.position.y);
    println!("New Cell state is {}, x:{} , y:{}", new_cell.alive, new_cell.position.x, new_cell.position.y);
    let printer = CliPrinter;
    printer.print_cell(cell);
    printer.print_cell(new_cell);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
