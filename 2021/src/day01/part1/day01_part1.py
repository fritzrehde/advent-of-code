from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    prev_depth = None
    depth_increases_count = 0

    for curr_depth in map(int, puzzle_input.splitlines()):
        if prev_depth is not None:
            if curr_depth > prev_depth:
                depth_increases_count += 1
        prev_depth = curr_depth

    return str(depth_increases_count)


if __name__ == "__main__":
    run(solve)
