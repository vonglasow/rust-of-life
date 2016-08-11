extern crate ansi_term;

mod printer;

struct Cell {
    x: i64,
    y: i64,
    alive: bool,
}

impl Cell {
    fn compute_state(&self, neighbors_alive: i64) -> Cell {
        if self.alive == true && (neighbors_alive == 2 || neighbors_alive == 3) {
            return Cell { alive: true, .. *self };
        }

        if self.alive == false && neighbors_alive == 3 {
            return Cell { alive: true, .. *self };
        }

        return Cell { alive: false, .. *self};
    }
}

#[cfg(not(test))]
fn main() {
    use Cell;
    use printer::CliPrinter;

    println!("Game of Life");
    let cell = Cell {x: 1, y: 1, alive: true};
    println!("Cell state is {}, x:{} , y:{}", cell.alive, cell.x, cell.y);
    println!("Change state of cell");
    let new_cell = cell.compute_state(25);
    println!("Old Cell state is {}, x:{} , y:{}", cell.alive, cell.x, cell.y);
    println!("New Cell state is {}, x:{} , y:{}", new_cell.alive, new_cell.x, new_cell.y);
    let printer = CliPrinter;
    printer.print_cell(cell);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }

    #[test]
    fn cell_struct() {
        use Cell;
        let c = Cell {x: 1, y: 1, alive: true};
        assert_eq!(true, c.alive);
        assert_eq!(1, c.x);
        assert_eq!(1, c.y);
    }

    #[test]
    fn cell_struct_is_alive_will_die() {
        use Cell;
        let c = Cell {x: 1, y: 1, alive: true};
        assert_eq!(true, c.alive);
        assert_eq!(1, c.x);
        assert_eq!(1, c.y);
        let newc = c.compute_state(1);
        assert_eq!(false, newc.alive);
    }

    #[test]
    fn cell_struct_is_alive_stay_alive() {
        use Cell;
        let c = Cell {x: 1, y: 1, alive: true};
        assert_eq!(true, c.alive);
        assert_eq!(1, c.x);
        assert_eq!(1, c.y);
        let newc = c.compute_state(3);
        assert_eq!(true, newc.alive);
    }

    #[test]
    fn cell_struct_is_dead_will_live() {
        use Cell;
        let c = Cell {x: 10, y: 1, alive: false};
        assert_eq!(false, c.alive);
        assert_eq!(10, c.x);
        assert_eq!(1, c.y);
        let newc = c.compute_state(3);
        assert_eq!(true, newc.alive);
        assert_eq!(10, newc.x);
        assert_eq!(1, newc.y);
    }

    #[test]
    fn printer_print_cell() {
        use Cell;
        use printer::CliPrinter;
        let printer = CliPrinter;
        let cell = Cell{x: 1, y: 1, alive: true};
        printer.print_cell(cell);
    }
}
