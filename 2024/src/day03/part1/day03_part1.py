from typing import List, Tuple
from puzzle_utils import puzzle, run
import re


@puzzle
def solve(puzzle_input: str) -> str:
    def all_mul_insts():
        return map(
            lambda xy: map(int, xy),
            re.findall(r"mul\((\d+),(\d+)\)", puzzle_input),
        )

    def exec_mul(xy):
        x, y = xy
        return x * y

    res = sum(map(exec_mul, all_mul_insts()))

    return str(res)


if __name__ == "__main__":
    run(solve)
