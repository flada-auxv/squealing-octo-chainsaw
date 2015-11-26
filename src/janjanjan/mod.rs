extern crate rand;

use std::fmt;

mod tile;
use tile::Tile;

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
impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "player1 => {:?}\nplayer2 => {:?}\nplayer3 => {:?}\nplayer4 => {:?}",
               self.players[0],
               self.players[1],
               self.players[2],
               self.players[3])
    }
}

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
impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: \"{}\", tehai: {:?}", self.name, self.tehai.clone().unwrap_or(vec!()))
    }
}
