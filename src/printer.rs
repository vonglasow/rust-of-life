use crate::cell::Cell;

pub struct CliPrinter;

impl CliPrinter {
    pub fn print_cell(&self, cell: Cell) {
        use ansi_term::Color::Blue;
        use ansi_term::Color::Cyan;
        use ansi_term::Style;

        let color;
        if cell.alive {
            color = Blue;
        } else {
            color = Cyan;
        }
        print!("{}", Style::new().on(color).paint("  "));
    }
}

#[cfg(test)]
mod cli_printer_tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn printer_print_cell() {
        let printer = CliPrinter;
        let position = Position::create_2d(1, 1);
        let cell = Cell {
            position: position,
            alive: true,
        };
        printer.print_cell(cell);
        let cell = Cell {
            position: position,
            alive: false,
        };
        printer.print_cell(cell);
    }
}
