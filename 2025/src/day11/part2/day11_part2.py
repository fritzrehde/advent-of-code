from puzzle_utils import dbg
from typing import Tuple
from functools import cache
from typing import List
from collections import defaultdict
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    # predecessors[v] contains all u where (u,v) in graph
    predecessors = defaultdict(lambda: set())

    for line in puzzle_input.splitlines():
        u, vs = tuple(line.split(":"))
        u = u.strip()
        for to_node in vs.split():
            v = to_node.strip()
            predecessors[v].add(u)

    start = "svr"
    end = "out"

    # directed, unweighted graph

    # n = number of nodes = O(10^3)
    # m = number of edges = O(10^4)

    # find number of paths from start to each node
    @cache
    def num_paths(from_node: str, to_node: str) -> int:
        if to_node == from_node:
            return 1

        v = to_node
        # given node is v, find all u where (u,v) is edge, and sum num paths to all u's
        return sum(num_paths(from_node, u) for u in predecessors[v])

    res = 0
    for a, b in (("dac", "fft"), ("fft", "dac")):
        # find number of paths from start to each node that contain a and b in that order
        num_paths_start_to_a = num_paths(start, a)
        num_paths_a_to_b = num_paths(a, b)
        num_paths_b_to_end = num_paths(b, end)

        res += num_paths_start_to_a * num_paths_a_to_b * num_paths_b_to_end

    return str(res)


if __name__ == "__main__":
    run(solve)
