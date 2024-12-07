from typing import Counter
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    left, right = zip(
        *(map(lambda line: map(int, line.split()), puzzle_input.splitlines()))
    )
    right_counter = Counter(right)

    res = sum(map(lambda l: l * right_counter[l], left))

    return str(res)


if __name__ == "__main__":
    run(solve)
