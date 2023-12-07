use graph_traversal::Data;

fn main() {
    let data = Data::from_iter([("A", "B"), ("A", "C"), ("B", "D"), ("C", "D"), ("C", "E")]);

    println!("Number of nodes: {}", data.get_n_nodes());

    println!("Children of 'A' - {:?}", data.get_children("A"));
    println!("Children of 'C' - {:?}", data.get_children("C"));
    println!("Children of 'D' - {:?}", data.get_children("D"));
    println!(
        "Children of a non existent node - {:?}",
        data.get_children("feshfeslj")
    );

    let root_node = "A";
    println!("level=1, node={}", root_node);
    for (level, node) in data.descendants_iter([root_node]) {
        println!("level={}, node={}", level, node);
    }
}
