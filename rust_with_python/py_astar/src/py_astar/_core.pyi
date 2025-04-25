# encoding: utf-8
# module py_astar._core
# from /data/srcs/Repos/rust_study/rust_with_python/py_astar/src/py_astar/_core.abi3.so
# by generator 1.147
"""astar wrapper for python"""

# no imports

# functions

def version(*args, **kwargs):  # real signature unknown
    pass

# classes

class AStar(object):
    """astar impl"""

    def py_allow_diagonal(self, *args, **kwargs):  # real signature unknown
        pass

    def py_astar(self, *args, **kwargs):  # real signature unknown
        pass

    def py_set_allow_diagonal(self, *args, **kwargs):  # real signature unknown
        pass

    def __init__(self, *args, **kwargs):  # real signature unknown
        pass

    @staticmethod  # known case of __new__
    def __new__(*args, **kwargs):  # real signature unknown
        """Create and return a new object.  See help(type) for accurate signature."""
        pass

class PathFindError(object):
    # no doc
    def __init__(self, *args, **kwargs):  # real signature unknown
        pass

    @staticmethod  # known case of __new__
    def __new__(*args, **kwargs):  # real signature unknown
        """Create and return a new object.  See help(type) for accurate signature."""
        pass

    details = property(
        lambda self: object(), lambda self, v: None, lambda self: None
    )  # default

class Pos(object):
    # no doc
    def __init__(self, *args, **kwargs):  # real signature unknown
        pass

    @staticmethod  # known case of __new__
    def __new__(*args, **kwargs):  # real signature unknown
        """Create and return a new object.  See help(type) for accurate signature."""
        pass

    x = property(
        lambda self: object(), lambda self, v: None, lambda self: None
    )  # default

    y = property(
        lambda self: object(), lambda self, v: None, lambda self: None
    )  # default

# variables with complex values

__all__ = [
    "version",
    "AStar",
    "PathFindError",
    "Pos",
]

__loader__ = None  # (!) real value is '<_frozen_importlib_external.ExtensionFileLoader object at 0x7fe94c6e5510>'

__spec__ = None  # (!) real value is "ModuleSpec(name='py_astar._core', loader=<_frozen_importlib_external.ExtensionFileLoader object at 0x7fe94c6e5510>, origin='/data/srcs/Repos/rust_study/rust_with_python/py_astar/src/py_astar/_core.abi3.so')"
