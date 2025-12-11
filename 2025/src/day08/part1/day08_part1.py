from itertools import islice
from functools import reduce
from typing import Iterable
from functools import cache
from typing import Dict
from pprint import pprint
from collections import defaultdict
from typing import Tuple
from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    points = [
        (pt_id, tuple(map(int, line.split(","))))
        for pt_id, line in enumerate(puzzle_input.splitlines())
    ]

    # n = number of points = O(10^3)

    def dist_squared(pt_a, pt_b) -> float:
        ax, ay, az = pt_a
        bx, by, bz = pt_b
        return ((ax - bx) ** 2) + ((ay - by) ** 2) + ((az - bz) ** 2)

    # all children, recursively, are in the same group as the parent
    # a node that is its own parent is the "head"/id of the group containing all of its children
    pt_to_parent: Dict[int, int] = {pt_id: pt_id for pt_id, _pt in points}

    def find_root(node: int) -> int:
        parent = pt_to_parent[node]
        if node == parent:
            # root nodes are their own parents
            return node
        else:
            # path compression
            root = find_root(parent)
            pt_to_parent[node] = root
            return root

    # go from smallest to largest dist
    dists = islice(
        sorted(
            (
                (dist_squared(pt_a, pt_b), pt_a_id, pt_b_id)
                for i, (pt_a_id, pt_a) in enumerate(points)
                for (pt_b_id, pt_b) in points[i + 1 :]
            )
        ),
        1000,
    )
    for _dist, a_id, b_id in dists:
        a_root = find_root(a_id)
        b_root = find_root(b_id)

        if a_root == b_root:
            # same group already
            pass
        else:
            # merge a and b's groups: either parent becomes the other's parent's parent
            # a's parent becomes b's parent's parent
            pt_to_parent[b_root] = a_root

    pt_to_group_id = {pt_id: find_root(pt_id) for pt_id, _pt in points}

    groups = defaultdict(lambda: [])
    for pt, group_id in pt_to_group_id.items():
        groups[group_id].append(pt)

    def product(nums: Iterable[int]) -> int:
        return reduce(lambda a, b: a * b, nums)

    res = product(
        islice(sorted(map(len, groups.values()), key=lambda x: -x), 3)
    )

    return str(res)


if __name__ == "__main__":
    run(solve)
