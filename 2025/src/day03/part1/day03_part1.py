from typing import Iterable
from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    def max_line_joltages():
        for line in puzzle_input.splitlines():
            digits = list(map(int, line))

            def leftmost_max(iterable: Iterable[int]):
                it = iter(enumerate(iterable))
                try:
                    curr_max_idx, curr_max_val = next(it)
                except StopIteration:
                    return None

                for i, x in it:
                    # keep leftmost on ties
                    if x > curr_max_val:
                        curr_max_val = x
                        curr_max_idx = i

                return curr_max_val, curr_max_idx

            # lo inclusive, hi exclusive.
            def leftmost_max_in_range(lo: int, hi: int, vals: List[int]):
                if (max_val_and_idx := leftmost_max(vals[lo:hi])) is not None:
                    max_val, max_val_idx = max_val_and_idx
                    return lo + max_val_idx
                else:
                    return None

            # optimal to always turn on leftmost max value first (as long as there
            # are elements right of that max), and then leftmost max value right of
            # the initial one
            pos1 = leftmost_max_in_range(lo=0, hi=len(digits) - 1, vals=digits)
            assert pos1 is not None
            pos2 = leftmost_max_in_range(
                lo=pos1 + 1, hi=len(digits), vals=digits
            )
            assert pos2 is not None

            joltage = digits[pos1] * 10 + digits[pos2]
            yield joltage

    return str(sum(max_line_joltages(), start=0))


def tests():
    assert solve("9999999999") == "99"
    assert solve("000091289") == "99"
    assert solve("99") == "99"
    assert solve("777777779") == "79"
    assert solve("7777777797") == "97"


if __name__ == "__main__":
    run(solve)
