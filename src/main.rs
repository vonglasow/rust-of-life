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
}
