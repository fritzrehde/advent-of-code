from typing import Tuple
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    grid = [[cell for cell in row] for row in puzzle_input.splitlines()]
    rows = len(grid)
    cols = len(grid[0])

    FILLED = "@"
    EMPTY = "."

    def adjacent_cells(cell: Tuple[int, int]):
        r, c = cell
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                nr, nc = r + dr, c + dc
                if dr == 0 and dc == 0:
                    continue
                in_bounds = 0 <= nr < rows and 0 <= nc < cols
                if in_bounds:
                    # passing value should indicate to caller they don't need to bounds check
                    yield (nr, nc, grid[nr][nc])

    def accessible_cells():
        for r in range(0, rows):
            for c in range(0, cols):
                cell = (r, c)
                # only count filled cells
                if grid[r][c] == FILLED:
                    adj_filled_count = sum(
                        (
                            1
                            for (_adj_r, _adj_c, adj_cell) in adjacent_cells(
                                cell
                            )
                            if adj_cell == FILLED
                        ),
                        start=0,
                    )

                    accessible = adj_filled_count < 4
                    if accessible:
                        yield cell

    return str(sum(1 for _ in accessible_cells()))


if __name__ == "__main__":
    run(solve)
