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

    visited_cells = set()

    def search_visited_obstacle_right(start_r, start_c, start_dir) -> bool:
        r, c, dir = start_r, start_c, turn_right(start_dir)

        # If we're directly facing an obstacle, turn right again.
        while (next_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)) and rows[
            next_r_c[0]
        ][next_r_c[1]] == "#":
            dir = turn_right(dir)

        if dir == start_dir:
            # We turned right 3 times, returning us to our original facing dir.
            return True

        # Anytime we visit a cell, we need to check to our right whether there
        # is an obstacle in line of sight that we have already "bumped" into
        # (i.e. visited the cell right before the obstacle).
        while (
            next_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)
        ) is not None:
            next_r, next_c = next_r_c
            if rows[next_r][next_c] == "#":
                is_starting_cell = r == start_r and c == start_c
                # This the first obstacle in line of sight, don't visit any
                # more afterwards.
                return (not is_starting_cell) and (r, c) in visited_cells
            r, c = next_r, next_c
        return False

    obstacles = set()

    r, c, dir = start_r, start_c, start_dir
    while True:
        visited_cells.add((r, c))

        if search_visited_obstacle_right(r, c, dir):
            # Place an obstacle in-front of us, so we're forced to go right,
            # which causes us to go to a cell we've been to facing the same way,
            # resulting in a cycle.
            if (
                infront_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)
            ) is not None:
                infront_r, infront_c = infront_r_c
                # Can't put an obstacle on starting cell.
                if not (infront_r == start_r and infront_c == start_c):
                    obstacles.add((infront_r, infront_c))

        if (next_r_c := steps_in_dir(r, c, *dir_drow_dcol[dir], 1)) is not None:
            next_r, next_c = next_r_c
            if rows[next_r][next_c] == "#":
                dir = turn_right(dir)
                # NOTE: We don't do a step here, so we're in the same cell again
                # next iteration facing a different direction.
            else:
                r, c = next_r, next_c
        else:
            # Out of bounds.
            break

    def dbg_grid(rows, file=sys.stderr):
        for row in rows:
            for e in row:
                print(e, end="", file=file)
            print(file=file)
        print(file=file)

    for r, c in obstacles:
        rows[r][c] = "O"

    dbg_grid(rows)

    res = len(obstacles)

    return str(res)


if __name__ == "__main__":
    run(solve)
