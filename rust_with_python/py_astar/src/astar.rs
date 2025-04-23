use pyo3::prelude::*;
use std::collections::{BinaryHeap, HashMap};

/// A*算法节点结构
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
}

/// 用于优先队列的排序节点
#[derive(Eq, PartialEq)]
struct PriorityNode {
    node: Node,
    f_score: i32,
}

impl Ord for PriorityNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for PriorityNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A*路径查找器
#[pyclass]
pub struct AStarPathfinder {
    grid: Vec<Vec<bool>>, // true表示可通过
    start: Node,
    end: Node,
    width: i32,
    height: i32,
}

/// A*路径查找实现
impl AStarPathfinder {
    pub fn new(grid: Vec<Vec<bool>>, start: (i32, i32), end: (i32, i32)) -> Self {
        // 先计算尺寸再移动所有权
        let width = if !grid.is_empty() {
            grid[0].len() as i32
        } else {
            0
        };
        let height = grid.len() as i32;
        AStarPathfinder {
            grid,
            start: Node {
                x: start.0,
                y: start.1,
            },
            end: Node { x: end.0, y: end.1 },
            width,
            height,
        }
    }
    /// 启发式函数（曼哈顿距离）
    fn heuristic(&self, node: (i32, i32)) -> i32 {
        (self.end.x - node.0).abs() + (self.end.y - node.1).abs()
    }

    /// 获取有效邻居节点
    fn get_neighbors(&self, node: &Node) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = node.x + dx;
                let y = node.y + dy;
                if x >= 0 && x < self.width && y >= 0 && y < self.height {
                    if self.grid[y as usize][x as usize] {
                        neighbors.push(Node { x, y });
                    }
                }
            }
        }
        neighbors
    }

    /// 重建路径
    fn reconstruct_path(&self, came_from: &HashMap<Node, Node>, current: Node) -> Vec<(i32, i32)> {
        let mut path = vec![(current.x, current.y)];
        let mut current = current;
        while let Some(node) = came_from.get(&current) {
            path.push((node.x, node.y));
            current = *node;
        }
        path.reverse();
        path
    }

    /// 执行路径查找
    pub fn find_path(&self) -> Option<Vec<(i32, i32)>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        g_score.insert(self.start, 0);
        open_set.push(PriorityNode {
            node: self.start,
            f_score: self.heuristic((self.start.x, self.start.y)),
        });

        while let Some(PriorityNode { node, .. }) = open_set.pop() {
            if node == self.end {
                return Some(self.reconstruct_path(&came_from, node));
            }

            for neighbor in self.get_neighbors(&node) {
                let tentative_g = g_score.get(&node).unwrap_or(&i32::MAX) + 1;
                if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, node);
                    g_score.insert(neighbor, tentative_g);
                    let f_score = tentative_g + self.heuristic((neighbor.x, neighbor.y));
                    open_set.push(PriorityNode {
                        node: neighbor,
                        f_score,
                    });
                }
            }
        }

        None
    }
}

/// A* 路径查找python接口
#[pymethods]
impl AStarPathfinder {
    #[new]
    pub fn py_new(grid: Vec<Vec<bool>>, start: (i32, i32), end: (i32, i32)) -> Self {
        AStarPathfinder::new(grid, start, end)
    }
    pub fn py_find_path(&self) -> Option<Vec<(i32, i32)>> {
        self.find_path()
    }
}
