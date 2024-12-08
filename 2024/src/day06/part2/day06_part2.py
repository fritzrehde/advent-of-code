import sys
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
        if rows[r][c] in dir_drow_dcol:
            start_dir = rows[r][c]
            start_r = r
            start_c = c

    def explore(start_r, start_c, start_dir) -> bool:
        "Return whether a cycle was detected."

        visited_states = set()
        r, c, dir = start_r, start_c, start_dir
        while True:
            # If we visit the same state (cell and dir) again, then we've found
            # a cycle.
            if (r, c, dir) in visited_states:
                return True

            visited_states.add((r, c, dir))

            if (
                next_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)
            ) is not None:
                next_r, next_c = next_r_c
                if rows[next_r][next_c] == "#":
                    dir = turn_right(dir)
                    # NOTE: We don't do a step here, so we're in the same cell again
                    # next iteration facing a different direction (that's a diff
                    # state, so it's fine).
                else:
                    r, c = next_r, next_c
            else:
                # Out of bounds: we've escaped grid without encountering cycle.
                return False

    def dbg_grid(rows, file=sys.stderr):
        for row in rows:
            for e in row:
                print(e, end="", file=file)
            print(file=file)
        print(file=file)

    obstacles = set()
    for r, c in all_coords(rows):
        is_starting_cell = (r, c) == (start_r, start_c)
        if is_starting_cell or rows[r][c] == "#":
            continue

        cell = rows[r][c]
        rows[r][c] = "#"
        found_cycle = explore(start_r, start_c, start_dir)
        if found_cycle:
            obstacles.add((r, c))

        # Restore cell.
        rows[r][c] = cell

    for r, c in obstacles:
        rows[r][c] = "O"

    dbg_grid(rows)

    res = len(obstacles)

    return str(res)


if __name__ == "__main__":
    run(solve)
