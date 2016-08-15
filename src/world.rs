use cell::Cell;
use position::Position;

pub struct World {
    pub cells: Vec<Vec<Cell>>,
    pub age: i64,
}

impl World {
    pub fn create_2d(width: i16, length: i16) -> World {
        extern crate rand;
        use rand::{Rng};
        use cell::Cell;
        use position::Position;

        let mut cells: Vec<Vec<Cell>> = vec![];
        let mut rng = rand::thread_rng();
        for y in 0..length {
            let mut cell_row: Vec<Cell> = vec![];
            for x in 0..width {
                let position = Position::create_2d(x, y);
                let alive = rng.gen_weighted_bool(2);
                let cell = Cell {
                    position: position,
                    alive: alive,
                    neighbors_positions: position.get_neighbors_positions_2d(width, length)
                };
                cell_row.push(cell);
            }
            cells.push(cell_row);
        }
        return World {cells: cells, age: 0};
    }

    pub fn next_gen(&self) -> World {
        let mut cells: Vec<Vec<Cell>> = vec![];
        for y in 0..self.cells.len() {
            let cell_row = self.cells[y].to_vec();
            let mut new_cell_row: Vec<Cell> = vec![];
            for cell in &cell_row {
                new_cell_row.push(
                    cell.compute_state(
                        self.count_alive(cell.neighbors_positions.to_vec())
                    )
                );
            }
            cells.push(new_cell_row);
        }
        return World{cells: cells, age: self.age + 1};
    }

    pub fn count_alive(&self, positions: Vec<Position>) -> i8 {
        let mut number_alive = 0;
        for position in &positions {
            if self.cells[position.y as usize][position.x as usize].alive {
                number_alive += 1;
            }
        }
        return number_alive;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_2d() {
        use world::World;
        let world = World::create_2d(20, 20);
        assert_eq!(9, world.cells[0][9].position.x);
        assert_eq!(0, world.cells[0][9].position.y);
        assert_eq!(0, world.cells[1][0].position.x);
        assert_eq!(1, world.cells[1][0].position.y);
        assert_eq!(20, world.cells[0].len());
        let ref c = world.cells[0][19];
    }

    #[test]
    fn test_next_gen() {
        use world::World;
        let world = World::create_2d(10, 10);
        let new_world = world.next_gen();
        assert_eq!(1, new_world.age);
        assert_eq!(world.cells.len(), new_world.cells.len());
    }

    #[test]
    fn test_count_alive() {
        use world::World;
        use position::Position;
        use cell::Cell;
        let cell = Cell {
            position: Position {
                x: 0,
                y: 0,
                z: 0,
            },
            alive: true,
            neighbors_positions: vec![],
        };
        let neighbor1 = Cell {
            position: Position {
                x: 1,
                y: 0,
                z: 0,
            },
            alive: true,
            neighbors_positions: vec![],
        };
        let neighbor2 = Cell {
            position: Position {
                x: 0,
                y: 1,
                z: 0,
            },
            alive: false,
            neighbors_positions: vec![],
        };
        let neighbor3 = Cell {
            position: Position {
                x: 1,
                y: 1,
                z: 0,
            },
            alive: true,
            neighbors_positions: vec![],
        };
        let cells = vec![
            vec![cell,neighbor1],
            vec![neighbor2, neighbor3]
        ];
        let world = World {cells: cells, age: 0};
        let neighbors_positions = vec![
            Position::create_2d(1, 0),
            Position::create_2d(0, 1),
            Position::create_2d(1, 1),
        ];
        assert_eq!(2, world.count_alive(neighbors_positions));
    }
}
