use graph_traversal::Data;

fn main() {
    let data = Data::from_iter([
        (1, vec![10, 2, 3, 5, 8]),
        (10, vec![11, 12, 13]),
        (2, vec![4]),
        (3, vec![4]),
        (5, vec![6, 9]),
        (8, vec![9]),
        (11, vec![14, 15]),
        (4, vec![7]),
        (6, vec![7]),
    ]);

    for node in [1, 10, 3] {
        let descendants: Vec<_> = data
            .descendants_iter([&node])
            .map(|(_level, id)| id)
            .collect();
        println!("descendants of {} - {:?}", node, descendants);
    }
}
