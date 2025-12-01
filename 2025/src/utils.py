import sys
from typing import List


def dbg_grid(rows, file=sys.stderr):
    for row in rows:
        for e in row:
            print(e, end="", file=file)
        print(file=file)

    print(file=file)


# TODO: dbg macro


dir_drow_dcol = {
    "north": (-1, 0),
    "east": (0, 1),
    "south": (1, 0),
    "west": (0, -1),
    "north-east": (-1, 1),
    "south-east": (1, 1),
    "south-west": (1, -1),
    "north-west": (-1, -1),
}


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


def coords_in_dir_unchecked_oob(
    start_row: int, start_col: int, drow: int, dcol: int
):
    row, col = start_row, start_col
    while True:
        yield (row, col)
        row += drow
        col += dcol
