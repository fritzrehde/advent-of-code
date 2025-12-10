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

    def step_in_dir(
        cell: Tuple[int, int], direction: str
    ) -> Tuple[int, int] | None:
        r, c = cell
        r_diff, c_diff = DIR_TO_RC_DIFF[direction]
        new_r, new_c = r + r_diff, c + c_diff
        new_cell = new_r, new_c

        if not in_bounds(new_cell):
            return None

        return new_cell

    visited = set()

    # dfs
    def try_go_downwards(start: Tuple[int, int]):
        # check if we are in bounds
        if not in_bounds(start):
            return

        if start in visited:
            return

        visited.add(start)

        r, c = start

        match grid[r][c]:
            case "." | "S":
                # try going down
                if (
                    new_cell := step_in_dir(start, direction="south")
                ) is not None:
                    try_go_downwards(new_cell)
            case "^":
                # try going left and right
                for direction in ("west", "east"):
                    if (new_cell := step_in_dir(start, direction)) is not None:
                        try_go_downwards(new_cell)

    start = find_start()
    try_go_downwards(start)

    num_visited_junctions = sum(
        1 for r, c in cells_iter() if grid[r][c] == "^" and (r, c) in visited
    )

    return str(num_visited_junctions)


if __name__ == "__main__":
    run(solve)
