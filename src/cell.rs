use position::Position;

pub struct Cell {
    pub position: Position,
    pub alive: bool,
    pub neighbors_positions: Vec<Position>,
}

impl Cell {
    pub fn compute_state(&self, neighbors_alive: i8) -> Cell {
        return Cell {alive: neighbors_alive == 3 || self.alive && neighbors_alive == 2, .. self.clone()};
    }
}

impl Clone for Cell {
    fn clone (&self) -> Cell {
        return Cell{neighbors_positions: self.neighbors_positions.to_vec(), .. *self};
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cell_struct() {
        use cell::Cell;
        use position::Position;
        let position = Position::create_2d(1, 1);
        let c = Cell {position: position, alive: true, neighbors_positions: vec![]};
        assert_eq!(true, c.alive);
    }

    #[test]
    fn cell_struct_is_alive_will_die() {
        use cell::Cell;
        use position::Position;
        let position = Position::create_2d(1, 1);
        let c = Cell {position: position, alive: true, neighbors_positions: vec![]};
        assert_eq!(true, c.alive);
        let newc = c.compute_state(1);
        assert_eq!(false, newc.alive);
        assert_eq!(c.position, newc.position);
    }

    #[test]
    fn cell_struct_is_alive_stay_alive() {
        use cell::Cell;
        use position::Position;
        let position = Position::create_2d(1, 1);
        let c = Cell {position: position, alive: true, neighbors_positions: vec![]};
        assert_eq!(true, c.alive);
        let newc = c.compute_state(3);
        assert_eq!(true, newc.alive);
        assert_eq!(c.position, newc.position);
    }

    #[test]
    fn cell_struct_is_dead_will_live() {
        use cell::Cell;
        use position::Position;
        let position = Position::create_2d(1, 1);
        let c = Cell {position: position, alive: false, neighbors_positions: vec![]};
        assert_eq!(false, c.alive);
        let newc = c.compute_state(3);
        assert_eq!(true, newc.alive);
        assert_eq!(c.position, newc.position);
    }

    #[test]
    fn cell_struct_is_dead_stay_dead() {
        use cell::Cell;
        use position::Position;
        let position = Position::create_2d(1, 1);
        let c = Cell {position: position, alive: false, neighbors_positions: vec![]};
        assert_eq!(false, c.alive);
        let newc = c.compute_state(2);
        assert_eq!(false, newc.alive);
        assert_eq!(c.position, newc.position);
    }
}
