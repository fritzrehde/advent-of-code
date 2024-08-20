from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    grid = [[int(cell) for cell in row] for row in puzzle_input.splitlines()]

    row_count = len(grid)
    col_count = len(grid[0])

    # The difference in (row, col) that needs to be applied to the current locatio
    # to result in a step being taken in the given direction.
    step_in_dir = {
        "north": (-1, 0),
        "south": (1, 0),
        "east": (0, 1),
        "west": (0, -1),
    }

    def adjacent_cells(row_idx: int, col_idx):
        for row_diff, col_diff in map(
            lambda dir: step_in_dir[dir], ("north", "east", "south", "west")
        ):
            adj_row_idx = row_idx + row_diff
            adj_col_idx = col_idx + col_diff
            # Check in-bounds.
            if 0 <= adj_row_idx < row_count and 0 <= adj_col_idx < col_count:
                yield grid[adj_row_idx][adj_col_idx]

    def is_low_point(row_idx: int, col_idx):
        cell_val = grid[row_idx][col_idx]
        return all(
            cell_val < adjacent_cell_val
            for adjacent_cell_val in adjacent_cells(row_idx, col_idx)
        )

    def low_points():
        for row_idx in range(0, row_count):
            for col_idx in range(0, col_count):
                if is_low_point(row_idx, col_idx):
                    yield (row_idx, col_idx)

    def risk_level(row_col_idx):
        (row_idx, col_idx) = row_col_idx
        return grid[row_idx][col_idx] + 1

    result = sum(map(risk_level, low_points()))

    return str(result)


if __name__ == "__main__":
    run(solve)
