extern crate ansi_term;
extern crate rand;

mod printer;
mod position;
mod cell;
mod world;

#[cfg(not(test))]
fn main() {
    use cell::Cell;
    use printer::CliPrinter;
    use position::Position;
    use world::World;
    use std::process::Command;
    use std::thread;

    let mut world = World::create_2d(20, 20);
    CliPrinter::print_world(&world);
    for i in 0..1000 {
        world = world.next_gen();
        CliPrinter::print_world(&world);
        thread::sleep_ms(500);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
