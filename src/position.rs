#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Position {
    pub fn create_2d (x: i16, y: i16) -> Position {
        return Position{x: x, y: y, z: 0};
    }

    pub fn get_neighbors_positions_2d(&self, width: i16, length: i16) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];
        for i in -1..2 {
            let mut y = self.y + i;
            if y < 0 {
                y = length + i;
            } else if y > length - 1 {
                y = -1 + i;
            }
            for j in -1..2 {
                let mut x = self.x + j;
                if x < 0 {
                    x = width + j;
                } else if x > width - 1 {
                    x = -1 + j;
                }
                if x < 0 || x > width - 1 || (i == 0 && j == 0) {
                    continue;
                }
                positions.push(Position::create_2d(x, y));
            }
        }
        return positions;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_2d() {
        use position::Position;
        let position = Position::create_2d(10, 11);
        assert_eq!(10, position.x);
        assert_eq!(11, position.y);
        assert_eq!(0, position.z);
    }

    #[test]
    fn test_eq_equals() {
        use position::Position;
        let position1 = Position {x: 1, y: 2, z: 3};
        let position2 = Position {x: 1, y: 2, z: 3};
        assert_eq!(true, position1 == position2);
    }

    #[test]
    fn test_eq_not_equals() {
        use position::Position;
        let position1 = Position {x: 1, y: 2, z: 3};
        let position2 = Position {x: 2, y: 2, z: 3};
        assert_eq!(true, position1 != position2);
        let position2 = Position {x: 1, y: 1, z: 3};
        assert_eq!(true, position1 != position2);
        let position2 = Position {x: 1, y: 2, z: 2};
        assert_eq!(true, position1 != position2);
    }

    #[test]
    fn test_get_neighbors_positions_2d() {
        use position::Position;
        let position1 = Position::create_2d(0, 0);
        let neighbors = position1.get_neighbors_positions_2d(3, 3);
        assert_eq!(8, neighbors.len());

        assert_eq!(2, neighbors[0].x);
        assert_eq!(2, neighbors[0].y);

        assert_eq!(0, neighbors[1].x);
        assert_eq!(2, neighbors[1].y);

        assert_eq!(1, neighbors[2].x);
        assert_eq!(2, neighbors[2].y);

        assert_eq!(2, neighbors[3].x);
        assert_eq!(0, neighbors[3].y);

        assert_eq!(1, neighbors[4].x);
        assert_eq!(0, neighbors[4].y);

        assert_eq!(2, neighbors[5].x);
        assert_eq!(1, neighbors[5].y);

        assert_eq!(0, neighbors[6].x);
        assert_eq!(1, neighbors[6].y);

        assert_eq!(1, neighbors[7].x);
        assert_eq!(1, neighbors[7].y);
    }
}
