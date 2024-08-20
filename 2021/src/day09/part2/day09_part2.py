import functools
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    grid = [[int(cell) for cell in row] for row in puzzle_input.splitlines()]

    def cell_value(cell) -> int:
        (row_idx, col_idx) = cell
        return grid[row_idx][col_idx]

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

    def adjacent_cells(cell):
        (row_idx, col_idx) = cell
        for row_diff, col_diff in map(
            lambda dir: step_in_dir[dir], ("north", "east", "south", "west")
        ):
            adj_row_idx = row_idx + row_diff
            adj_col_idx = col_idx + col_diff
            # Check in-bounds.
            if 0 <= adj_row_idx < row_count and 0 <= adj_col_idx < col_count:
                yield (adj_row_idx, adj_col_idx)

    def adjacent_cell_values(cell):
        return map(cell_value, adjacent_cells(cell))

    def is_low_point(row_idx: int, col_idx):
        cell_val = cell_value((row_idx, col_idx))
        return all(
            cell_val < adjacent_cell_val
            for adjacent_cell_val in adjacent_cell_values((row_idx, col_idx))
        )

    def low_points():
        for row_idx in range(0, row_count):
            for col_idx in range(0, col_count):
                if is_low_point(row_idx, col_idx):
                    yield (row_idx, col_idx)

    def risk_level(cell):
        return cell_value(cell) + 1

    BASIN_EDGE = 9

    def cells_in_basin(start_cell):
        basin_cells = set()

        def dfs_inner(cell):
            # Stop exploring at a basin's edge.
            if cell_value(cell) == BASIN_EDGE:
                return

            basin_cells.add(cell)

            for neighbour in adjacent_cells(cell):
                if neighbour not in basin_cells:
                    dfs_inner(neighbour)

        dfs_inner(start_cell)
        return basin_cells

    def basins():
        for low_point in low_points():
            yield cells_in_basin(low_point)

    def print_basins(basins):
        cell_to_basin_id = dict()
        for id, basin in enumerate(basins):
            for cell in basin:
                cell_to_basin_id[cell] = id

        for row_idx in range(0, row_count):
            for col_idx in range(0, col_count):
                print(
                    cell_to_basin_id.get((row_idx, col_idx), "."),
                    end="",
                )
            print()

    def largest_three(iterable):
        def rev(item):
            return -item

        return sorted(iterable, key=rev)[:3]

    result = functools.reduce(
        lambda a, b: a * b, largest_three(map(len, basins()))
    )

    return str(result)


if __name__ == "__main__":
    run(solve)
