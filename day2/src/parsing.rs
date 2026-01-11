use crate::logic::Game;

pub trait Parse {
    fn parse(str: &str) -> Self;
}

impl Parse for Game {
    fn parse(str: &str) -> Self {
        todo!()   
    }
}