from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    # Origin (0,0) is top left.
    rows: List[List[str]] = [
        [e for e in row] for row in puzzle_input.splitlines()
    ]

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

    def all_cells(rows: List[List]):
        row_count = len(rows)
        col_count = len(rows[0])
        for r in range(0, row_count):
            for c in range(0, col_count):
                yield r, c

    def in_bounds(rows: List[List], row: int, col: int) -> bool:
        row_count = len(rows)
        col_count = len(rows[0])
        return row in range(0, row_count) and col in range(0, col_count)

    def coords_in_dir_out_of_bounds(
        start_row: int, start_col: int, drow: int, dcol: int
    ):
        row, col = start_row, start_col
        while True:
            yield (row, col)
            row += drow
            col += dcol

    word = "XMAS"

    res = 0
    for r, c in all_cells(rows):
        # For each cell, check if it starts the word in all directions (could
        # start the same word in multiple directions).
        for drow, dcol in dir_drow_dcol.values():
            # zip with infinite oob coord iterator will ensure we read exactly
            # len(word) chars in this direction.
            if all(
                map(
                    # TODO: I wish python just let me specify: (a, (r, c))
                    lambda a_rc: in_bounds(rows, a_rc[1][0], a_rc[1][1])
                    and a_rc[0] == rows[a_rc[1][0]][a_rc[1][1]],
                    zip(word, coords_in_dir_out_of_bounds(r, c, drow, dcol)),
                )
            ):
                res += 1

    return str(res)


if __name__ == "__main__":
    run(solve)
