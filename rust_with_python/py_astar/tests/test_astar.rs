use _core::{Board, Pos,AStar};

#[test]
fn test_astar_cls() {
    let board = vec![
        "21397X2",
        "1X19452",
        "62251X1",
        "1612179",
        "1348512",
        "61453X1",
        "7861243"];
    let astar: AStar = board.try_into().unwrap();
    let start = Pos(0, 1);
    let goal = Pos(6, 2);
    let result = astar.astar(start, goal);
    let result = result.expect("No path found");
    println!("path is {:}", Board::path_to_str(&result));
    assert_eq!(result, vec![
        Pos(0, 1),
        Pos(0, 0),
        Pos(1, 0),
        Pos(2, 0),
        Pos(2, 1),
        Pos(2, 2),
        Pos(2, 3),
        Pos(3, 3),
        Pos(4, 3),
        Pos(4, 2),
        Pos(4, 1),
        Pos(5, 1),
        Pos(6, 1),
        Pos(6, 2)])
}