# uv 会吧当前工程以 edit mode 加入到 .venv 中, 不需要再手动添加
# import sys
# sys.path.append('../src')

from py_astar._core import version, AStar, Pos
import pytest

@pytest.fixture
def sample_board1():
    """测试用地图数据1"""
    return [
        "21397X2",
        "1X19452",
        "62251X1",
        "1612179",
        "1348512",
        "61453X1",
        "7861243"
    ]

@pytest.fixture
def sample_board2():
    """测试用地图数据2（含障碍）"""
    return ["1"*3+"X"+"1"*6] * 10

class TestAStar:
    def test_astar_path_found(self, sample_board1):
        """测试能找到有效路径的情况"""
        print(version())
        astar = AStar(sample_board1)
        result = astar.py_astar(Pos(0, 1), Pos(6, 2))
        path = [(pos.x, pos.y) for pos in result]
        print(path)
        assert path == [
            (0, 1), (0, 0), (1, 0), (2, 0), (2, 1),
            (2, 2), (2, 3), (3, 3), (4, 3), (4, 2),
            (4, 1), (5, 1), (6, 1), (6, 2)
        ]

    def test_astar_simple_path(self, sample_board2):
        """测试简单路径查找"""
        print(version())
        astar = AStar(sample_board2)
        result = astar.py_astar(Pos(0, 1), Pos(1, 2))
        path = [(pos.x, pos.y) for pos in result]
        print(path)
        assert path == [(0, 1), (0, 2), (1, 2)]

    def test_astar_path_not_found(self, sample_board2):
        """测试找不到路径的情况"""
        astar = AStar(sample_board2)
        with pytest.raises(RuntimeError) as excinfo:
            astar.py_astar(Pos(0, 1), Pos(7, 8))
        assert str(excinfo.value) == 'Path not found'