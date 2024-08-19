from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    (horizontal_pos, depth) = (0, 0)

    def make_move(move: str):
        nonlocal horizontal_pos, depth
        (command, x_str) = move.split()
        match (command, int(x_str)):
            case ("forward", x):
                horizontal_pos += x
            case ("down", x):
                depth += x
            case ("up", x):
                depth -= x

    for line in puzzle_input.splitlines():
        make_move(line)

    result = horizontal_pos * depth
    return str(result)


if __name__ == "__main__":
    run(solve)
