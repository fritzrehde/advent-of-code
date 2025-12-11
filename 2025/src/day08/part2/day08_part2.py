from typing import Dict
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

    num_groups = len(points)  # every node starts as its own group
    nodes_seen = set()

    # go from smallest to largest dist
    dists = sorted(
        (
            (dist_squared(pt_a, pt_b), pt_a_id, pt_b_id)
            for i, (pt_a_id, pt_a) in enumerate(points)
            for (pt_b_id, pt_b) in points[i + 1 :]
        )
    )
    for _dist, a_id, b_id in dists:
        for x in (a_id, b_id):
            nodes_seen.add(x)

        a_root = find_root(a_id)
        b_root = find_root(b_id)

        if a_root == b_root:
            # same group already
            pass
        else:
            # merge a and b's groups: either parent becomes the other's parent's parent
            # a's parent becomes b's parent's parent
            pt_to_parent[b_root] = a_root
            num_groups -= 1  # b's group no longer exists

        # if there every node is in a single group, we're done
        if len(nodes_seen) == len(points) and num_groups == 1:
            _a_id, (ax, _ay, _az) = points[a_id]
            _b_id, (bx, _by, _bz) = points[b_id]
            res = ax * bx
            return str(res)

    raise Exception("unreachable")


if __name__ == "__main__":
    run(solve)
