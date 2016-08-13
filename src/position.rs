#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Position {
    pub fn create_2d (x: i16, y: i16) -> Position {
        return Position{x: x, y: y, z: 0};
    }
}

#[cfg(test)]
mod position_test {
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
}
