use std::fmt;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone)]
pub enum Tile {
    Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9,
    Sou1, Sou2, Sou3, Sou4, Sou5, Sou6, Sou7, Sou8, Sou9,
    Wan1, Wan2, Wan3, Wan4, Wan5, Wan6, Wan7, Wan8, Wan9,
    WindTon, WindNan, WindSha, WindPei,
    DragonHaku, DragonHatsu, DragonChun,
}

impl Tile {
    fn all_tiles() -> Vec<Tile> {
        use tile::Tile::*;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        const ALL_TILES: [Tile; 136] = [
            Pin1, Pin1, Pin1, Pin1,
            Pin2, Pin2, Pin2, Pin2,
            Pin3, Pin3, Pin3, Pin3,
            Pin4, Pin4, Pin4, Pin4,
            Pin5, Pin5, Pin5, Pin5,
            Pin6, Pin6, Pin6, Pin6,
            Pin7, Pin7, Pin7, Pin7,
            Pin8, Pin8, Pin8, Pin8,
            Pin9, Pin9, Pin9, Pin9,
            Sou1, Sou1, Sou1, Sou1,
            Sou2, Sou2, Sou2, Sou2,
            Sou3, Sou3, Sou3, Sou3,
            Sou4, Sou4, Sou4, Sou4,
            Sou5, Sou5, Sou5, Sou5,
            Sou6, Sou6, Sou6, Sou6,
            Sou7, Sou7, Sou7, Sou7,
            Sou8, Sou8, Sou8, Sou8,
            Sou9, Sou9, Sou9, Sou9,
            Wan1, Wan1, Wan1, Wan1,
            Wan2, Wan2, Wan2, Wan2,
            Wan3, Wan3, Wan3, Wan3,
            Wan4, Wan4, Wan4, Wan4,
            Wan5, Wan5, Wan5, Wan5,
            Wan6, Wan6, Wan6, Wan6,
            Wan7, Wan7, Wan7, Wan7,
            Wan8, Wan8, Wan8, Wan8,
            Wan9, Wan9, Wan9, Wan9,
            WindTon, WindTon, WindTon, WindTon,
            WindNan, WindNan, WindNan, WindNan,
            WindSha, WindSha, WindSha, WindSha,
            WindPei, WindPei, WindPei, WindPei,
            DragonHaku, DragonHaku, DragonHaku, DragonHaku,
            DragonHatsu, DragonHatsu, DragonHatsu, DragonHatsu,
            DragonChun, DragonChun, DragonChun, DragonChun
        ];

        ALL_TILES.to_vec()
    }

    pub fn shuffled_all_tiles() -> Vec<Tile> {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut ret = Tile::all_tiles();
        rng.shuffle(&mut ret);
        ret
    }
}
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use tile::Tile::*;

        match *self {
            Pin1 => write!(f, "1p"),
            Pin2 => write!(f, "2p"),
            Pin3 => write!(f, "3p"),
            Pin4 => write!(f, "4p"),
            Pin5 => write!(f, "5p"),
            Pin6 => write!(f, "6p"),
            Pin7 => write!(f, "7p"),
            Pin8 => write!(f, "8p"),
            Pin9 => write!(f, "9p"),
            Sou1 => write!(f, "1s"),
            Sou2 => write!(f, "2s"),
            Sou3 => write!(f, "3s"),
            Sou4 => write!(f, "4s"),
            Sou5 => write!(f, "5s"),
            Sou6 => write!(f, "6s"),
            Sou7 => write!(f, "7s"),
            Sou8 => write!(f, "8s"),
            Sou9 => write!(f, "9s"),
            Wan1 => write!(f, "1w"),
            Wan2 => write!(f, "2w"),
            Wan3 => write!(f, "3w"),
            Wan4 => write!(f, "4w"),
            Wan5 => write!(f, "5w"),
            Wan6 => write!(f, "6w"),
            Wan7 => write!(f, "7w"),
            Wan8 => write!(f, "8w"),
            Wan9 => write!(f, "9w"),
            WindTon => write!(f, "東"),
            WindNan => write!(f, "南"),
            WindSha => write!(f, "西"),
            WindPei => write!(f, "北"),
            DragonHaku => write!(f, "白"),
            DragonHatsu => write!(f, "發"),
            DragonChun => write!(f, "中"),
        }
    }
}
