extern crate mahjong;

use mahjong::Table;

fn main() {
    let mut table = Table::new();
    table.start();
    println!("{:?}", table);
}
