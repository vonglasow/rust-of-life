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
}
