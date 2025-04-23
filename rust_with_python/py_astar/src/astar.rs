use pathfinding::prelude::astar;
use std::error::Error;
use std::fmt;
use pyo3::{pyclass, pymethods,PyResult,PyErr};

#[derive(Debug)]
#[pyclass]
pub struct PathFindError {
    details: String,
}
impl fmt::Display for PathFindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for PathFindError {}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[pyclass]
pub struct Pos(pub i16, pub i16);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u32,
}
// Used to make writing tests easier
impl PartialEq<(Pos, u32)> for Successor {
    fn eq(&self, other: &(Pos, u32)) -> bool {
        self.pos == other.0 && self.cost == other.1
    }
}

#[pyclass]
pub struct Board {
    pub width: u8,                  // 地图的宽度(列数)
    pub height: u8,                 // 地图的高度(行数)
    pub data: Vec<Vec<Option<u8>>>, // 二维网格数据，存储每个格子的值
    pub allow_diagonal: bool,       // 是否允许对角线移动
}

impl Board {
    pub fn new(board_lines: Vec<&str>, allow_diagonal: bool) -> Result<Self, PathFindError> {
        let width = board_lines[0].len() as u8;
        let height = board_lines.len() as u8;
        let mut data = Vec::new();
        for board_line in board_lines {
            let mut row: Vec<Option<u8>> = Vec::new();
            for c in board_line.chars() {
                match c {
                    'X' => row.push(None),
                    '1'..='9' => row.push(Some(c as u8 - b'0')),
                    _ => {
                        return Err(PathFindError {
                            details: format!("Invalid character in line: {}", board_line),
                        })
                    }
                }
            }
            if row.len() != width as usize {
                return Err(PathFindError {
                    details: format!("Invalid line length: {}", board_line),
                });
            }
            data.push(row);
        }
        Ok(Board {
            width,
            height,
            data,
            allow_diagonal,
        })
    }

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if self.allow_diagonal {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                } else {
                    // Omit diagonal moves (and moving to the same position)
                    if (dx + dy).abs() != 1 {
                        continue;
                    }
                }
                let new_position = Pos(position.0 + dx, position.1 + dy);
                if new_position.0 < 0
                    || new_position.0 >= self.width.into()
                    || new_position.1 < 0
                    || new_position.1 >= self.height.into()
                {
                    continue;
                }
                let board_value = self.data[new_position.1 as usize][new_position.0 as usize];
                if let Some(board_value) = board_value {
                    successors.push(Successor {
                        pos: new_position,
                        cost: board_value as u32,
                    });
                }
            }
        }

        successors
    }
    pub fn path_to_str(path: &Vec<Pos>) -> String {
        let mut result = String::new();
        for p in path {
            // 格式化为 "(x,y): cost" 的形式
            let s = format!("({},{}); ", p.0, p.1,);
            result.push_str(&s);
        }
        // 移除最后一个分号和空格
        if !result.is_empty() {
            result.truncate(result.len() - 2);
        }
        result
    }
}
#[pyclass]
pub struct AStar {
    board: Board,
}

impl AStar {
    pub fn new(board: Board) -> Self {
        AStar { board }
    }
}
impl From<Board> for AStar {
    fn from(board: Board) -> Self {
        AStar { board }
    }
}
impl TryFrom<Vec<&str>> for AStar {
    type Error = PathFindError;

    fn try_from(board_lines: Vec<&str>) -> Result<Self, Self::Error> {
        Board::new(board_lines, false).map(AStar::from)
    }
}
#[pymethods]
impl AStar {
    #[new]
    pub fn py_new(board_lines: Vec<String>) -> PyResult<Self> {
        let board_lines_ref: Vec<&str> = board_lines.iter().map(|s| s.as_str()).collect();
        AStar::try_from(board_lines_ref).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    pub fn astar(&self, start: Pos, goal: Pos) -> Option<Vec<Pos>> {
        let result = astar(
            &start,
            |p| {
                self.board
                    .get_successors(p)
                    .iter()
                    .map(|s| (s.pos, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| ((p.0 - goal.0).abs() + (p.1 - goal.1).abs()) as u32,
            |p| *p == goal,
        );

        result.map(|(path, _)| path)
    }
    pub fn set_allow_diagonal(&mut self, allow_diagonal: bool) {
        self.board.allow_diagonal = allow_diagonal;
    }
    pub fn allow_diagonal(&self) -> bool {
        self.board.allow_diagonal
    }
}