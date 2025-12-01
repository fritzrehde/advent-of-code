from puzzle_utils import puzzle, run
from enum import Enum


@puzzle
def solve(puzzle_input: str) -> str:
    zeroes = 0
    location = 50
    for line in puzzle_input.splitlines():
        left = line[0] == "L"
        ticks = int(line[1:])

        if not left:
            # right
            location = (location + ticks) % 100
        else:
            # left
            location = (location - ticks) % 100

        if location == 0:
            zeroes += 1

    return str(zeroes)


if __name__ == "__main__":
    run(solve)
