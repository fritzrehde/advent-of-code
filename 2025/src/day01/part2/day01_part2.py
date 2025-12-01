from puzzle_utils import puzzle, run
from enum import Enum


@puzzle
def solve(puzzle_input: str) -> str:
    zeroes = 0
    location = 50
    for line in puzzle_input.splitlines():
        left = line[0] == "L"
        ticks = int(line[1:])

        step = 1 if not left else -1

        # how many steps to move to reach 0
        dist_to_zero = (100 - location) % 100 if step == 1 else location % 100
        if dist_to_zero == 0:
            # we are already at 0, need a full lap to reach it again
            dist_to_zero = 100

        if ticks >= dist_to_zero:
            # we will reach 0 during this move
            zeroes += 1
            ticks -= dist_to_zero
            location = 0

            # after reaching 0, each full lap adds another zero
            full_laps = ticks // 100
            zeroes += full_laps
            ticks %= 100

        # we should be unable to reach 0 again in the remaining ticks
        location = (location + step * ticks) % 100

    return str(zeroes)


if __name__ == "__main__":
    run(solve)
