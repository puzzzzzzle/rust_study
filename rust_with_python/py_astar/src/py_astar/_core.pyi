# encoding: utf-8
# module py_astar._core
# from /data/srcs/Repos/rust_study/rust_with_python/py_astar/src/py_astar/_core.abi3.so
# by generator 1.147
"""
A Python module implemented in Rust. The name of this function must match
the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
import the module.
"""
# no imports

# functions

def version(*args, **kwargs): # real signature unknown
    pass

# classes

class AStar(object):
    # no doc
    def astar(self, *args, **kwargs): # real signature unknown
        pass

    def py_allow_diagonal(self, *args, **kwargs): # real signature unknown
        pass

    def py_set_allow_diagonal(self, *args, **kwargs): # real signature unknown
        pass

    def __init__(self, *args, **kwargs): # real signature unknown
        pass

    @staticmethod # known case of __new__
    def __new__(*args, **kwargs): # real signature unknown
        """ Create and return a new object.  See help(type) for accurate signature. """
        pass


class PathFindError(object):
    # no doc
    def __init__(self, *args, **kwargs): # real signature unknown
        pass

    @staticmethod # known case of __new__
    def __new__(*args, **kwargs): # real signature unknown
        """ Create and return a new object.  See help(type) for accurate signature. """
        pass

    details = property(lambda self: object(), lambda self, v: None, lambda self: None)  # default



class Pos(object):
    # no doc
    def __init__(self, *args, **kwargs): # real signature unknown
        pass

    @staticmethod # known case of __new__
    def __new__(*args, **kwargs): # real signature unknown
        """ Create and return a new object.  See help(type) for accurate signature. """
        pass

    x = property(lambda self: object(), lambda self, v: None, lambda self: None)  # default

    y = property(lambda self: object(), lambda self, v: None, lambda self: None)  # default



# variables with complex values

__all__ = [
    'version',
    'AStar',
    'PathFindError',
    'Pos',
]

__loader__ = None # (!) real value is '<_frozen_importlib_external.ExtensionFileLoader object at 0x7f6268af5490>'

__spec__ = None # (!) real value is "ModuleSpec(name='py_astar._core', loader=<_frozen_importlib_external.ExtensionFileLoader object at 0x7f6268af5490>, origin='/data/srcs/Repos/rust_study/rust_with_python/py_astar/src/py_astar/_core.abi3.so')"

