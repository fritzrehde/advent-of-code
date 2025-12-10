from functools import cache
from typing import Tuple
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    grid = [[cell for cell in row] for row in puzzle_input.splitlines()]
    num_rows = len(grid)
    num_cols = len(grid[0])

    def cells_iter():
        for r in range(0, num_rows):
            for c in range(0, num_cols):
                yield (r, c)

    def find_start() -> Tuple[int, int]:
        for r, c in cells_iter():
            if grid[r][c] == "S":
                return r, c
        raise Exception("unreachable")

    DIR_TO_RC_DIFF = {
        "north": (-1, 0),
        "east": (0, 1),
        "south": (1, 0),
        "west": (0, -1),
    }

    def in_bounds(cell: Tuple[int, int]) -> bool:
        r, c = cell
        return 0 <= r < num_rows and 0 <= c < num_cols

    def step_in_dir(cell: Tuple[int, int], direction: str) -> Tuple[int, int]:
        r, c = cell
        r_diff, c_diff = DIR_TO_RC_DIFF[direction]
        new_r, new_c = r + r_diff, c + c_diff
        return new_r, new_c

    # dfs
    @cache
    def timelines_from(start: Tuple[int, int]) -> int:
        if not in_bounds(start):
            # leaving grid counts as completing a timeline
            return 1

        r, c = start

        match grid[r][c]:
            case "." | "S":
                # go down
                cell_below = step_in_dir(start, direction="south")
                return timelines_from(cell_below)
            case "^":
                # go left and right
                cell_left, cell_right = (
                    step_in_dir(start, direction)
                    for direction in ("west", "east")
                )
                # spec does not clarify what should happen if junction sits at left or right border
                assert in_bounds(cell_left) and in_bounds(cell_right)
                return timelines_from(cell_left) + timelines_from(cell_right)
            case _:
                raise Exception("unreachable")

    start = find_start()
    num_timelines = timelines_from(start)

    return str(num_timelines)


if __name__ == "__main__":
    run(solve)
