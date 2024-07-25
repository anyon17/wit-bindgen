use std::fs;
use wit_parser::{Resolve, WorldItem};

fn main() {
    let mut resolve = Resolve::default();
    let contents = fs::read_to_string("erc20.wit").unwrap();
    let ids = resolve.push_str("erc20.wit", &contents).unwrap();

    let world_id = resolve.select_world(&ids, None).unwrap();
    let world = &resolve.worlds[world_id];
    let exports = &world.exports;

    for (_, item) in exports {
        match item {
            WorldItem::Interface { id, stability } => {
                let interface = &resolve.interfaces[*id];
                let functions = &interface.functions;

                for (_, func) in functions {
                    println!("{:?}", func);
                }
            }
            WorldItem::Function(_) | WorldItem::Type(_) => todo!(),
        }
    }

    println!("{:?}", world.name);
}
