extern crate rand;

fn main() {
    let mut table = jan_jan_jan::Table::new();
    table.start();
    println!("{:?}", table);
}

mod jan_jan_jan {
    use std::slice::Iter;

    #[derive(Debug,Clone)]
    enum Tile {
        Pin_1,
        Pin_2,
        Pin_3,
        Pin_4,
        Pin_5,
        Pin_6,
        Pin_7,
        Pin_8,
        Pin_9,
        Sou_1,
        Sou_2,
        Sou_3,
        Sou_4,
        Sou_5,
        Sou_6,
        Sou_7,
        Sou_8,
        Sou_9,
        Wan_1,
        Wan_2,
        Wan_3,
        Wan_4,
        Wan_5,
        Wan_6,
        Wan_7,
        Wan_8,
        Wan_9,
        Wind_Ton,
        Wind_Nan,
        Wind_Sha,
        Wind_Pei,
        Dragon_Haku,
        Dragon_Hatsu,
        Dragon_Chun,
    }
    impl Tile {
        fn all_tiles() -> Vec<Tile> {
            const ALL_TILES: [Tile; 136] = [Tile::Pin_1,
                                            Tile::Pin_1,
                                            Tile::Pin_1,
                                            Tile::Pin_1,
                                            Tile::Pin_2,
                                            Tile::Pin_2,
                                            Tile::Pin_2,
                                            Tile::Pin_2,
                                            Tile::Pin_3,
                                            Tile::Pin_3,
                                            Tile::Pin_3,
                                            Tile::Pin_3,
                                            Tile::Pin_4,
                                            Tile::Pin_4,
                                            Tile::Pin_4,
                                            Tile::Pin_4,
                                            Tile::Pin_5,
                                            Tile::Pin_5,
                                            Tile::Pin_5,
                                            Tile::Pin_5,
                                            Tile::Pin_6,
                                            Tile::Pin_6,
                                            Tile::Pin_6,
                                            Tile::Pin_6,
                                            Tile::Pin_7,
                                            Tile::Pin_7,
                                            Tile::Pin_7,
                                            Tile::Pin_7,
                                            Tile::Pin_8,
                                            Tile::Pin_8,
                                            Tile::Pin_8,
                                            Tile::Pin_8,
                                            Tile::Pin_9,
                                            Tile::Pin_9,
                                            Tile::Pin_9,
                                            Tile::Pin_9,
                                            Tile::Sou_1,
                                            Tile::Sou_1,
                                            Tile::Sou_1,
                                            Tile::Sou_1,
                                            Tile::Sou_2,
                                            Tile::Sou_2,
                                            Tile::Sou_2,
                                            Tile::Sou_2,
                                            Tile::Sou_3,
                                            Tile::Sou_3,
                                            Tile::Sou_3,
                                            Tile::Sou_3,
                                            Tile::Sou_4,
                                            Tile::Sou_4,
                                            Tile::Sou_4,
                                            Tile::Sou_4,
                                            Tile::Sou_5,
                                            Tile::Sou_5,
                                            Tile::Sou_5,
                                            Tile::Sou_5,
                                            Tile::Sou_6,
                                            Tile::Sou_6,
                                            Tile::Sou_6,
                                            Tile::Sou_6,
                                            Tile::Sou_7,
                                            Tile::Sou_7,
                                            Tile::Sou_7,
                                            Tile::Sou_7,
                                            Tile::Sou_8,
                                            Tile::Sou_8,
                                            Tile::Sou_8,
                                            Tile::Sou_8,
                                            Tile::Sou_9,
                                            Tile::Sou_9,
                                            Tile::Sou_9,
                                            Tile::Sou_9,
                                            Tile::Wan_1,
                                            Tile::Wan_1,
                                            Tile::Wan_1,
                                            Tile::Wan_1,
                                            Tile::Wan_2,
                                            Tile::Wan_2,
                                            Tile::Wan_2,
                                            Tile::Wan_2,
                                            Tile::Wan_3,
                                            Tile::Wan_3,
                                            Tile::Wan_3,
                                            Tile::Wan_3,
                                            Tile::Wan_4,
                                            Tile::Wan_4,
                                            Tile::Wan_4,
                                            Tile::Wan_4,
                                            Tile::Wan_5,
                                            Tile::Wan_5,
                                            Tile::Wan_5,
                                            Tile::Wan_5,
                                            Tile::Wan_6,
                                            Tile::Wan_6,
                                            Tile::Wan_6,
                                            Tile::Wan_6,
                                            Tile::Wan_7,
                                            Tile::Wan_7,
                                            Tile::Wan_7,
                                            Tile::Wan_7,
                                            Tile::Wan_8,
                                            Tile::Wan_8,
                                            Tile::Wan_8,
                                            Tile::Wan_8,
                                            Tile::Wan_9,
                                            Tile::Wan_9,
                                            Tile::Wan_9,
                                            Tile::Wan_9,
                                            Tile::Wind_Ton,
                                            Tile::Wind_Ton,
                                            Tile::Wind_Ton,
                                            Tile::Wind_Ton,
                                            Tile::Wind_Nan,
                                            Tile::Wind_Nan,
                                            Tile::Wind_Nan,
                                            Tile::Wind_Nan,
                                            Tile::Wind_Sha,
                                            Tile::Wind_Sha,
                                            Tile::Wind_Sha,
                                            Tile::Wind_Sha,
                                            Tile::Wind_Pei,
                                            Tile::Wind_Pei,
                                            Tile::Wind_Pei,
                                            Tile::Wind_Pei,
                                            Tile::Dragon_Haku,
                                            Tile::Dragon_Haku,
                                            Tile::Dragon_Haku,
                                            Tile::Dragon_Haku,
                                            Tile::Dragon_Hatsu,
                                            Tile::Dragon_Hatsu,
                                            Tile::Dragon_Hatsu,
                                            Tile::Dragon_Hatsu,
                                            Tile::Dragon_Chun,
                                            Tile::Dragon_Chun,
                                            Tile::Dragon_Chun,
                                            Tile::Dragon_Chun];

            ALL_TILES.to_vec()
        }

        fn shuffled_all_tiles() -> Vec<Tile> {
            use rand::{thread_rng, Rng};

            let mut rng = thread_rng();
            let mut ret = Tile::all_tiles();
            rng.shuffle(&mut ret);
            ret
        }
    }


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
    struct Player {
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
}
