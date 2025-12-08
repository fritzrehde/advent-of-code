from typing import Tuple
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

    def non_overlapping_ranges():
        # greedy: out of remaining ranges, pick range with earliest start x, and discard any next ranges y where x's end is after y's start, and then pick max(x.end, y.end) as end of merged range.

        ranges.sort(key=lambda lo_hi: (lo_hi[0], lo_hi[1]))

        if len(ranges) == 0:
            return

        curr_start, curr_end = ranges[0]
        for start, end in ranges[1:]:
            if curr_end >= start:
                # merge ranges
                curr_end = max(curr_end, end)
            else:
                # ends current range
                yield (curr_start, curr_end)
                # setup next range
                curr_start, curr_end = start, end

        # ends final range
        yield curr_start, curr_end

    def in_range_count(r: Tuple[int, int]) -> int:
        start, end = r
        return end - start + 1

    in_any_range_count = sum(map(in_range_count, non_overlapping_ranges()))

    return str(in_any_range_count)


if __name__ == "__main__":
    run(solve)
