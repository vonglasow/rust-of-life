struct Cell {
    x: i64,
    y: i64,
    alive: bool
}

impl Cell {
}

#[cfg(not(test))]
fn main() {
    println!("Game of Life");
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
}
