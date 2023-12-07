use graph_traversal::Data;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("../../data/parent_children_map.json").unwrap();
    let reader = BufReader::new(file);
    let hashmap = serde_json::from_reader::<_, HashMap<String, Vec<String>>>(reader).unwrap();

    let data = Data::from_iter(hashmap.into_iter());

    let root_node = "Categorie";
    println!(
        "Number of descendants of 'Categorie' - {:?}",
        data.descendants_iter([root_node]).count()
    );

    let last_level = data
        .descendants_iter([root_node])
        .last()
        .map_or(0, |(level, _)| level);
    println!("Number of levels from 'Categorie' - {}", last_level);

    let descendants: Vec<_> = data
        .descendants_iter(["Imperatori_romani"])
        .map(|(_level, node)| node)
        .collect();
    println!("Descendants of 'Imperatori romani' - {:#?}", descendants);

    let n_descendants = data.descendants_iter(["Letteratura"]).count();
    println!("Descendants of 'Letteratura' - {:#?}", n_descendants);

    let n_descendants = data.descendants_iter(["Letteratura_italiana"]).count();
    println!(
        "Descendants of 'Letteratura_italiana' - {:#?}",
        n_descendants
    );

    let n_descendants = data.descendants_iter(["Letteratura_latina"]).count();
    println!("Descendants of 'Letteratura_latina' - {:#?}", n_descendants);

    let n_descendants = data
        .descendants_iter([
            "Letteratura_latina",
            "Letteratura_italiana",
            "Imperatori_romani",
        ])
        .count();
    println!(
        "Descendants of 'Letteratura_latina', 'Letteratura_italiana', and 'Imperatori_romani' - {}",
        n_descendants
    );
}
