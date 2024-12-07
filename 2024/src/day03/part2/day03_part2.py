from typing import List, Tuple
from puzzle_utils import puzzle, run
import re


@puzzle
def solve(puzzle_input: str) -> str:
    s = puzzle_input
    n = len(puzzle_input)

    all_mul_insts = []

    def add_mul_insts(l: int, r: int):
        "l inclusive, r: exclusive"
        all_mul_insts.extend(
            map(
                lambda xy: map(int, xy),
                re.findall(r"mul\((\d+),(\d+)\)", s[l:r]),
            )
        )

    l = 0
    while True:
        # Until where are we allowed to read?
        if (r := s.find("don't()", l)) != -1:
            add_mul_insts(l, r)
        else:
            # No limit, so read until the end and stop.
            add_mul_insts(l, n)
            break

        # From where onwards are we allowed to read?
        if (l := s.find("do()", r)) == -1:
            # Never allowed to read again.
            break

    def exec_mul(xy):
        x, y = xy
        return x * y

    res = sum(map(exec_mul, all_mul_insts))

    return str(res)


if __name__ == "__main__":
    run(solve)
