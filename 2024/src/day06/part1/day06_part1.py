from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    rows = [[e for e in row] for row in puzzle_input.splitlines()]
    row_count = len(rows)
    col_count = len(rows[0])

    def all_coords(rows: List[List]):
        row_count = len(rows)
        col_count = len(rows[0])
        for r in range(0, row_count):
            for c in range(0, col_count):
                yield r, c

    def in_bounds(rows: List[List], row: int, col: int) -> bool:
        row_count = len(rows)
        col_count = len(rows[0])
        return row in range(0, row_count) and col in range(0, col_count)

    def steps_in_dir(
        start_row: int, start_col: int, drow: int, dcol: int, steps: int
    ):
        row = start_row + drow * steps
        col = start_col + dcol * steps
        if in_bounds(rows, row, col):
            return (row, col)
        else:
            return None

    # Ordered clockwise.
    dir_drow_dcol = {
        "^": (-1, 0),
        ">": (0, 1),
        "v": (1, 0),
        "<": (0, -1),
    }
    dirs = list(dir_drow_dcol.keys())
    dir_to_idx = {dir: i for i, dir in enumerate(dirs)}

    def turn_right(dir: str) -> str:
        i = dir_to_idx[dir]
        return dirs[(i + 1) % len(dirs)]

    # Find starting position and dir.
    for r, c in all_coords(rows):
        match rows[r][c]:
            case "^" | ">" | "v" | "<":
                start_dir = rows[r][c]
                start_r = r
                start_c = c
            case _:
                pass

    visited_cells = set()

    r, c, dir = start_r, start_c, start_dir
    while True:
        visited_cells.add((r, c))
        # NOTE: I didn't realise I could unpack the tuple directly with * here.
        if (next_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)) is not None:
            next_r, next_c = next_r_c
            if rows[next_r][next_c] == "#":
                dir = turn_right(dir)
                # NOTE: We don't do a step here, so we're in the same cell again
                # next iteration facing a different direction. It's fine to add
                # this cell to the set twice.
            else:
                r, c = next_r, next_c
        else:
            # Out of bounds.
            break

    res = len(visited_cells)

    return str(res)


if __name__ == "__main__":
    run(solve)
