#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug,Clone)]
pub enum Tile {
    Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9,
    Sou1, Sou2, Sou3, Sou4, Sou5, Sou6, Sou7, Sou8, Sou9,
    Wan1, Wan2, Wan3, Wan4, Wan5, Wan6, Wan7, Wan8, Wan9,
    WindTon, WindNan, WindSha, WindPei,
    DragonHaku, DragonHatsu, DragonChun,
}

impl Tile {
    fn all_tiles() -> Vec<Tile> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        const ALL_TILES: [Tile; 136] = [
            Tile::Pin1, Tile::Pin1, Tile::Pin1, Tile::Pin1,
            Tile::Pin2, Tile::Pin2, Tile::Pin2, Tile::Pin2,
            Tile::Pin3, Tile::Pin3, Tile::Pin3, Tile::Pin3,
            Tile::Pin4, Tile::Pin4, Tile::Pin4, Tile::Pin4,
            Tile::Pin5, Tile::Pin5, Tile::Pin5, Tile::Pin5,
            Tile::Pin6, Tile::Pin6, Tile::Pin6, Tile::Pin6,
            Tile::Pin7, Tile::Pin7, Tile::Pin7, Tile::Pin7,
            Tile::Pin8, Tile::Pin8, Tile::Pin8, Tile::Pin8,
            Tile::Pin9, Tile::Pin9, Tile::Pin9, Tile::Pin9,
            Tile::Sou1, Tile::Sou1, Tile::Sou1, Tile::Sou1,
            Tile::Sou2, Tile::Sou2, Tile::Sou2, Tile::Sou2,
            Tile::Sou3, Tile::Sou3, Tile::Sou3, Tile::Sou3,
            Tile::Sou4, Tile::Sou4, Tile::Sou4, Tile::Sou4,
            Tile::Sou5, Tile::Sou5, Tile::Sou5, Tile::Sou5,
            Tile::Sou6, Tile::Sou6, Tile::Sou6, Tile::Sou6,
            Tile::Sou7, Tile::Sou7, Tile::Sou7, Tile::Sou7,
            Tile::Sou8, Tile::Sou8, Tile::Sou8, Tile::Sou8,
            Tile::Sou9, Tile::Sou9, Tile::Sou9, Tile::Sou9,
            Tile::Wan1, Tile::Wan1, Tile::Wan1, Tile::Wan1,
            Tile::Wan2, Tile::Wan2, Tile::Wan2, Tile::Wan2,
            Tile::Wan3, Tile::Wan3, Tile::Wan3, Tile::Wan3,
            Tile::Wan4, Tile::Wan4, Tile::Wan4, Tile::Wan4,
            Tile::Wan5, Tile::Wan5, Tile::Wan5, Tile::Wan5,
            Tile::Wan6, Tile::Wan6, Tile::Wan6, Tile::Wan6,
            Tile::Wan7, Tile::Wan7, Tile::Wan7, Tile::Wan7,
            Tile::Wan8, Tile::Wan8, Tile::Wan8, Tile::Wan8,
            Tile::Wan9, Tile::Wan9, Tile::Wan9, Tile::Wan9,
            Tile::WindTon, Tile::WindTon, Tile::WindTon, Tile::WindTon,
            Tile::WindNan, Tile::WindNan, Tile::WindNan, Tile::WindNan,
            Tile::WindSha, Tile::WindSha, Tile::WindSha, Tile::WindSha,
            Tile::WindPei, Tile::WindPei, Tile::WindPei, Tile::WindPei,
            Tile::DragonHaku, Tile::DragonHaku, Tile::DragonHaku, Tile::DragonHaku,
            Tile::DragonHatsu, Tile::DragonHatsu, Tile::DragonHatsu, Tile::DragonHatsu,
            Tile::DragonChun, Tile::DragonChun, Tile::DragonChun, Tile::DragonChun
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
