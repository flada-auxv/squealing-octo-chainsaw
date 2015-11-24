extern crate rand;

use std::slice::Iter;

mod tile;
use tile::Tile;

#[derive(Debug)]
pub struct Table {
    players: Vec<Player>,
    tiles: Vec<Tile>,
}
impl Table {
    pub fn new() -> Table {
        Table {
            players: vec![Player::new(), Player::new(), Player::new(), Player::new()],
            tiles: Tile::shuffled_all_tiles(),
        }
    }
    fn draw(&mut self, num: usize) -> Vec<Tile> {
        let mut ret = vec![];
        for _ in 0..num {
            ret.push(self.tiles.remove(0))
        }
        ret
    }
    pub fn start(&mut self) {
        for i in 0..4 {
            self.players[i].tehai = Some(self.draw(13))
        }
    }
}

#[derive(Debug,Clone)]
pub struct Player {
    name: String,
    tehai: Option<Vec<Tile>>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            name: "player".to_string(),
            tehai: None,
        }
    }
}
