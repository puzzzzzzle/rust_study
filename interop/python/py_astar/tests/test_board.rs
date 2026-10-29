use _core::{Board, Pos};
#[test]
fn test_onebyoneboard_nosuccessors() {
    let board = Board::new(vec!["1"], false).unwrap();
    let result = board.get_successors(&Pos(0, 0));
    assert_eq!(result.len(), 0);
}

#[test]
fn test_twobytwoboardwithobstacle() {
    let board = Board::new(vec![
        "21",
        "1X"], false).unwrap();
    let result = board.get_successors(&Pos(0, 1));
    assert_eq!(result, vec![(Pos(0, 0), 2)]);
}

#[test]
fn test_twobytwoboardwithobstacleanddiagonal() {
    let board = Board::new(vec![
        "21",
        "1X"], true).unwrap();
    let result = board.get_successors(&Pos(0, 1));
    assert_eq!(result, vec![(Pos(0, 0), 2), (Pos(1, 0), 1)]);
}

#[test]
fn test_bigboardmovingfrommiddle() {
    let board = Board::new(vec![
        "21941",
        "1X587",
        "238X1",
        "18285",
        "13485"], false).unwrap();
    let result = board.get_successors(&Pos(2, 2));
    assert_eq!(result, vec![(Pos(1, 2), 3), (Pos(2, 1), 5), (Pos(2, 3), 2)]);
}

#[test]
fn test_surroundedbywalls() {
    let board = Board::new(vec![
        "21941",
        "1XX87",
        "2X8X1",
        "18X85",
        "13485"], false).unwrap();
    let result = board.get_successors(&Pos(2, 2));
    assert_eq!(result.len(), 0);
}