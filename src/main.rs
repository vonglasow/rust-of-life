extern crate ansi_term;
extern crate rand;
extern crate clap;

mod printer;
mod position;
mod cell;
mod world;

#[cfg(not(test))]
fn main() {
    use printer::CliPrinter;
    use world::World;
    use std::thread;
    use clap::App;
    use std::time::Duration;

    let matches = App::new("Rust of life")
        .version("v1.0-beta")
         .args_from_usage("
             <width> 'Width of the world'
             <length> 'Length of the world'
        ")
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
        thread::sleep(Duration::from_millis(500));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(1 == 1);
    }
}
