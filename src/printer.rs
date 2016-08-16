use cell::Cell;

pub struct CliPrinter;

impl CliPrinter {
    pub fn print_cell(&self, cell: Cell) {
        use ansi_term::Style;
        use ansi_term::Color::Blue;
        use ansi_term::Color::Cyan;

        let color;
        if cell.alive {
            color = Blue;
        } else {
            color = Cyan;
        }
        println!("{}", Style::new().on(color).paint("  "));
    }
}

#[cfg(test)]
mod cli_printer_tests {
    #[test]
    fn printer_print_cell() {
        use printer::CliPrinter;
        use cell::Cell;
        use position::Position;

        let printer = CliPrinter;
        let position = Position::create_2d(1, 1);
        let cell = Cell{position: position, alive: true};
        printer.print_cell(cell);
        let cell = Cell{position: position, alive: false};
        printer.print_cell(cell);
    }
}
