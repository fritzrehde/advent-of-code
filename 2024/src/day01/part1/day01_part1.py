from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    left, right = zip(
        *(map(lambda line: map(int, line.split()), puzzle_input.splitlines()))
    )

    def dist(xy) -> int:
        x, y = xy
        return abs(x - y)

    res = sum(map(dist, zip(sorted(left), sorted(right))))

    return str(res)


if __name__ == "__main__":
    run(solve)
