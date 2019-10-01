extern crate ansi_term;
extern crate rand;

mod cell;
mod position;
mod printer;

#[cfg(not(test))]
fn main() {
    use cell::Cell;
    use position::Position;
    use printer::CliPrinter;
    use rand::Rng;

    let mut rng = rand::thread_rng();

    println!("Game of Life");
    for y in 0..10 {
        for x in 0..10 {
            let position = Position::create_2d(x, y);
            let cell = Cell {
                position: position,
                alive: true,
            };
            let new_cell = cell.compute_state(rng.gen_range(1, 4));
            let printer = CliPrinter;
            printer.print_cell(cell);
            printer.print_cell(new_cell);
        }
        println!("");
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
