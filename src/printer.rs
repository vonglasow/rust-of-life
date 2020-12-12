use cell::Cell;
use world::World;

pub struct CliPrinter;

impl CliPrinter {
    pub fn print_cell(cell: Cell) {
        use ansi_term::Color::Black;
        use ansi_term::Color::White;
        use ansi_term::Style;

        let color;
        if cell.alive {
            color = Black;
        } else {
            color = White;
        }
        print!("{}", Style::new().on(color).paint("  "));
    }

    pub fn print_world(world: &World) {
        use std::process::Command;

        println!("");
        Command::new("clear")
            .status()
            .expect("An error occured during clear");
        println!("World age : {}", world.age);
        for y in 0..world.cells.len() {
            println!("");
            for cell in &world.cells[y] {
                CliPrinter::print_cell(Cell { ..cell.clone() });
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn printer_print_cell() {
        use cell::Cell;
        use position::Position;
        use printer::CliPrinter;

        let position = Position::create_2d(1, 1);
        let cell = Cell {
            position: position,
            alive: true,
            neighbors_positions: vec![],
        };
        CliPrinter::print_cell(cell);
        let cell = Cell {
            position: position,
            alive: false,
            neighbors_positions: vec![],
        };
        CliPrinter::print_cell(cell);
    }

    #[test]
    fn printer_print_world() {
        use printer::CliPrinter;
        use world::World;

        let world = World::create_2d(10, 10);
        CliPrinter::print_world(&world);
    }
}
