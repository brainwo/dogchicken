use crate::Animal;

#[derive(Clone, Debug)]
pub struct GameManager {
    pub animals: Vec<Animal>,
    pub debug: bool,
}
