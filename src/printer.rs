use Cell;

pub struct CliPrinter;

impl CliPrinter {
    pub fn print_cell(&self, cell: Cell) {
        use ansi_term::Style;
        use ansi_term::Color::Black;
        use ansi_term::Color::White;

        let color;
        if cell.alive {
            color = Black;
        } else {
            color = White;
        }
        println!("{}", Style::new().on(color).paint("  "));
    }
}
