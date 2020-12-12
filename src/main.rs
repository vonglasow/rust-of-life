extern crate ansi_term;
extern crate clap;
extern crate rand;

mod cell;
mod position;
mod printer;
mod world;

#[cfg(not(test))]
fn main() {
    use clap::App;
    use printer::CliPrinter;
    use std::{thread, time};
    use world::World;

    let matches = App::new("Rust of life")
        .version("v1.0-beta")
        .args_from_usage(
            "
             <width> 'Width of the world'
             <length> 'Length of the world'
        ",
        )
        .get_matches();

    let mut width: i16 = 20;
    let mut length: i16 = 20;
    if let Some(o) = matches.value_of("width") {
        width = o.parse().unwrap();
    }

    if let Some(o) = matches.value_of("length") {
        length = o.parse().unwrap();
    }
    let mut world = World::create_2d(width, length);
    CliPrinter::print_world(&world);
    for _i in 0..1000 {
        world = world.next_gen();
        CliPrinter::print_world(&world);
        thread::sleep(time::Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
