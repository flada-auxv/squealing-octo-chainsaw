extern crate janjanjan;

use janjanjan::Table;

fn main() {
    let mut table = Table::new();
    table.start();
    println!("{:?}", table);
}
