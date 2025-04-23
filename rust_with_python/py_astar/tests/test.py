# uv 会吧当前工程以 edit mode 加入到 .venv 中, 不需要再手动添加
# import sys
# sys.path.append('../src')

from py_astar._core import version, AStar, Pos

if __name__ == '__main__':
    print(version())
    board = [
        "21397X2",
        "1X19452",
        "62251X1",
        "1612179",
        "1348512",
        "61453X1",
        "7861243"]
    astar = AStar(board)
    start = Pos(0, 1)
    goal = Pos(6, 2)
    result = astar.astar(start, goal)
    print([(pos.x, pos.y) for pos in result])


