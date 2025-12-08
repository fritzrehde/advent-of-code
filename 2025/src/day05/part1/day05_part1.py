from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    ranges_part, singles_part = puzzle_input.split("\n\n", 1)

    ranges = [
        tuple(map(int, line.split("-", maxsplit=1)))
        for line in ranges_part.splitlines()
    ]
    singles = [int(line) for line in singles_part.splitlines()]

    # n = number of ranges, approx 10^2
    # m = number of singles, approx 10^3

    def in_any_range(x: int) -> bool:
        # O(n)
        return any(lo <= x <= hi for lo, hi in ranges)

    # O(n * m) = O(10^5)
    singles_in_range = sum(1 for single in singles if in_any_range(single))

    return str(singles_in_range)


if __name__ == "__main__":
    run(solve)
