from itertools import count
from typing import List

from more_itertools import sliding_window
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:

    def safe(levels: List[int]) -> bool:
        def valid_diff(xy) -> bool:
            x, y = xy
            return 1 <= abs(x - y) <= 3

        adjacent_pairs = list(sliding_window(levels, 2))

        asc = all(map(lambda xy: xy[0] <= xy[1], adjacent_pairs))
        desc = all(map(lambda xy: xy[0] >= xy[1], adjacent_pairs))
        valid_diffs = all(map(valid_diff, adjacent_pairs))

        return (asc or desc) and valid_diffs

    def kinda_safe(levels: List[int]) -> bool:
        def rm_idx(i: int) -> List[int]:
            copy = levels[:]
            del copy[i]
            return copy

        n = len(levels)
        return safe(levels) or any(map(safe, map(rm_idx, range(0, n))))

    def count(iterable):
        return sum(1 for _ in iterable)

    levels = map(
        lambda line: list(map(int, line.split())), puzzle_input.splitlines()
    )
    res = count(filter(kinda_safe, levels))

    return str(res)


if __name__ == "__main__":
    run(solve)
