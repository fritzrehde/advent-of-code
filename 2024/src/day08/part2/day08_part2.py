from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    rows: List[List[str]] = [
        [c for c in row] for row in puzzle_input.splitlines()
    ]
    row_count = len(rows)
    col_count = len(rows[0])

    def in_bounds(row, col):
        return row in range(0, row_count) and col in range(0, col_count)

    def antinodes(x1, y1, x2, y2):
        dx = x2 - x1
        dy = y2 - y1
        for i in range(0, max(row_count, col_count)):
            x = x1 + i * dx
            y = y1 + i * dy
            if in_bounds(x, y):
                yield (x, y)

    def all_cells():
        for row in range(0, row_count):
            for col in range(0, col_count):
                yield (row, col)

    all_antinodes = set()
    for r1, c1 in all_cells():
        if rows[r1][c1] == ".":
            continue
        for r2, c2 in all_cells():
            if r1 == r2 and c1 == c2:
                continue
            if rows[r1][c1] == rows[r2][c2]:
                for r, c in antinodes(r1, c1, r2, c2):
                    all_antinodes.add((r, c))

    res = len(all_antinodes)

    return str(res)


if __name__ == "__main__":
    run(solve)
