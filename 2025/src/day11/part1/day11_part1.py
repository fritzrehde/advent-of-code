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

    start = "you"
    end = "out"

    # directed, unweighted graph

    # n = number of nodes = O(10^3)
    # m = number of edges = O(10^4)

    # find number of paths from start to each node
    # dfs
    @cache
    def num_paths_to_node(node: str) -> int:
        if node == start:
            return 1

        v = node
        # given node is v, find all u where (u,v) is edge, and sum num paths to all u's
        return sum(num_paths_to_node(u) for u in predecessors[v])

    return str(num_paths_to_node(end))


if __name__ == "__main__":
    run(solve)
