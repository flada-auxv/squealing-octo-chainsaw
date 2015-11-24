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
        let mut player1 = Player::new();
        let mut player2 = Player::new();
        let mut player3 = Player::new();
        let mut player4 = Player::new();
        Table {
            players: vec![player1, player2, player3, player4],
            tiles: Tile::shuffled_all_tiles(),
        }
    }
    fn draw(&mut self, num: usize) -> Vec<Tile> {
        let mut ret = vec![];
        for i in 0..num {
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
