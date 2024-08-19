from collections import defaultdict
from puzzle_utils import puzzle, run
from more_itertools import ilen
import re


@puzzle
def solve(puzzle_input: str) -> str:
    # grid[y][x] represents the number of lines that would cover coordinate
    # (x,y).
    grid = defaultdict(lambda: defaultdict(lambda: 0))

    def all_points():
        for row in grid.values():
            for point in row.values():
                yield point

    def print_grid():
        max_y = max(grid.keys()) + 1
        max_x = max(x for row in grid.values() for x in row.keys()) + 1
        for y in range(0, max_y):
            for x in range(0, max_x):
                print(p if (p := grid[y][x]) != 0 else ".", end="")
            print()

    def bi_range(from_incl: int, to_incl: int):
        """
        Generates a range between endpoints, figuring in which direction
        (towards infinity or negative infinity) to iterate.
        """
        if from_incl <= to_incl:
            return range(from_incl, to_incl + 1)
        else:
            return range(from_incl, to_incl - 1, -1)

    def horiztonal_or_vertical_line(x1, y1, x2, y2) -> bool:
        return x1 == x2 or y1 == y2

    for line in puzzle_input.splitlines():
        if m := re.search(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$", line):
            (x1, y1, x2, y2) = map(int, m.groups())
            if horiztonal_or_vertical_line(x1, y1, x2, y2):
                for y in bi_range(y1, y2):
                    for x in bi_range(x1, x2):
                        grid[y][x] += 1
            else:
                for x, y in zip(bi_range(x1, x2), bi_range(y1, y2)):
                    grid[y][x] += 1

    result = ilen(filter(lambda point: point >= 2, all_points()))

    return str(result)


if __name__ == "__main__":
    run(solve)
