#![warn(clippy::pedantic)]

mod graph;

fn main() {
    use graph::{Block, Board};

    use pathfinding::directed::astar::astar;

    // use pathfinding::directed::dijkstra::dijkstra

    /*
    let target_position = (1, 3);
    let initial = Board::new(
        (4, 5),
        vec![
            // target block
            Block::new((2, 0), (2, 2)),
            // other blocks
            Block::new((0, 0), (1, 2)),
            Block::new((1, 0), (1, 2)),
            Block::new((0, 3), (1, 2)),
            Block::new((1, 3), (1, 2)),
            Block::new((1, 2), (2, 1)),
            Block::new((2, 3), (1, 1)),
            Block::new((2, 4), (1, 1)),
            Block::new((3, 3), (1, 1)),
            Block::new((3, 4), (1, 1)),
        ],
    );
    */

    /*
    let initial = Board::new(
        (4, 5),
        vec![
            // target block
            Block::new((1, 0), (2, 2)),
            // other blocks
            Block::new((0, 0), (1, 2)),
            Block::new((3, 0), (1, 2)),
            Block::new((0, 2), (1, 2)),
            Block::new((3, 2), (1, 2)),
            Block::new((1, 2), (2, 1)),
            Block::new((0, 4), (1, 1)),
            Block::new((1, 3), (1, 1)),
            Block::new((2, 3), (1, 1)),
            Block::new((3, 4), (1, 1)),
        ],
    );
    */

    let target_position = (4, 2);
    let initial = Board::new((6, 6), vec![
        Block::new((3, 2), (2, 1), Some("A".into())),
        Block::new((1, 0), (2, 1), Some("B".into())),
        Block::new((0, 3), (3, 1), Some("C".into())),
        Block::new((4, 4), (2, 1), Some("D".into())),
        Block::new((0, 5), (2, 1), Some("E".into())),
        Block::new((3, 5), (2, 1), Some("F".into())),
        Block::new((0, 0), (1, 3), Some("G".into())),
        Block::new((1, 1), (1, 2), Some("H".into())),
        Block::new((2, 1), (1, 2), Some("I".into())),
        Block::new((2, 4), (1, 2), Some("J".into())),
        Block::new((3, 3), (1, 2), Some("K".into())),
        Block::new((4, 0), (1, 2), Some("L".into())),
        Block::new((5, 1), (1, 3), Some("M".into())),
    ]);

    println!("initial:\n{}", initial);

    /*
    let initial = Board::new((4, 4), vec![
        Block::new((0, 0), (1, 1)),
        Block::new((2, 0), (1, 1)),
        Block::new((1, 2), (1, 1)),
        Block::new((0, 1), (1, 1)),
        Block::new((2, 1), (1, 1)),
        Block::new((0, 2), (1, 1)),
        Block::new((3, 2), (1, 1)),
        Block::new((1, 1), (1, 1)),
        Block::new((1, 3), (1, 1)),
        Block::new((2, 3), (1, 1)),
        Block::new((0, 3), (1, 1)),
        Block::new((1, 0), (1, 1)),
        Block::new((3, 0), (1, 1)),
        Block::new((3, 3), (1, 1)),
        Block::new((3, 1), (1, 1)),
    ]);

    let target = Board::new((4, 4), vec![
        Block::new((0, 0), (1, 1)),
        Block::new((1, 0), (1, 1)),
        Block::new((2, 0), (1, 1)),
        Block::new((3, 0), (1, 1)),
        Block::new((0, 1), (1, 1)),
        Block::new((1, 1), (1, 1)),
        Block::new((2, 1), (1, 1)),
        Block::new((3, 1), (1, 1)),
        Block::new((0, 2), (1, 1)),
        Block::new((1, 2), (1, 1)),
        Block::new((2, 2), (1, 1)),
        Block::new((3, 2), (1, 1)),
        Block::new((0, 3), (1, 1)),
        Block::new((1, 3), (1, 1)),
        Block::new((2, 3), (1, 1)),
    ]);
    */

    // println!("initial:\n{}", initial);
    // println!("target:\n{}", target);

    let mut n = 0_f64;

    let res = astar(
        &initial,
        |node| node.neighbors().map(|node| (node, 1)),
        // |node| node.blocks.iter().enumerate().map(|(key, val)|
        // val.distance_from(target.blocks[key].position)).sum(),
        |node| node.blocks[0].distance_from(target_position),
        |node| {
            if n.log2().fract() == 0.0 {
                println!("checked {} permutations", n);
                println!("{}", node);
            }
            n += 1.0;

            node.blocks[0].position == target_position
        },
        /* |node| *node == target */
        /*
        |node| {
            if n.log2().fract() == 0.0 {
                println!("checked {} permutations", n);
                // println!("{}", node);
            }
            n += 1.0;
            *node == target
        },
        */
    );

    if let Some(res) = res {
        // println!("Done. Solution has length {}", res.0.len());

        println!("solution:");
        for (index, board) in res.0.into_iter().enumerate() {
            println!("{}:\n{}\n", index + 1, board);
        }

    /*
    println!(
        "{}",
        serde_json::to_string(&res.0).expect("serialization failed")
    );
    */
    } else {
        println!("No solution found");
    }
}
