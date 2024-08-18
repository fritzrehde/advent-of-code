#!/usr/bin/env python3

from collections import deque
import itertools
from puzzle_utils import puzzle, run
from more_itertools import sliding_window


@puzzle
def solve_custom_sliding_window_list(puzzle_input: str) -> str:
    def count_increases(iterable) -> int:
        prev = None
        increases_count = 0
        for curr in iterable:
            if prev is not None:
                if curr > prev:
                    increases_count += 1
            prev = curr
        return increases_count

    def sliding_window(arr, window_len: int):
        for window_start in range(0, len(arr) - window_len + 1):
            yield arr[window_start : window_start + window_len]

    numbers = list(map(int, puzzle_input.splitlines()))
    sums = map(sum, sliding_window(numbers, window_len=3))
    return str(count_increases(sums))


@puzzle
def solve_custom_sliding_window_iterable(puzzle_input: str) -> str:
    def count_increases(iterable) -> int:
        prev = None
        increases_count = 0
        for curr in iterable:
            if prev is not None:
                if curr > prev:
                    increases_count += 1
            prev = curr
        return increases_count

    def sliding_window(iterable, window_len: int):
        it = iter(iterable)
        window = deque(itertools.islice(it, window_len - 1))
        for elem in it:
            window.append(elem)
            yield tuple(window)
            window.popleft()

    # We don't need to collect to list here, because sliding_window can take
    # any iterable.
    numbers = map(int, puzzle_input.splitlines())
    sums = map(sum, sliding_window(numbers, window_len=3))

    return str(count_increases(sums))


# Alternative solution using more_itertools's sliding window instead of
# implementing my own.
@puzzle
def solve_moreitertools_sliding_window(puzzle_input: str) -> str:
    def count_increases(iterable) -> int:
        prev = None
        increases_count = 0
        for curr in iterable:
            if prev is not None:
                if curr > prev:
                    increases_count += 1
            prev = curr
        return increases_count

    numbers = list(map(int, puzzle_input.splitlines()))
    sums = map(sum, sliding_window(numbers, 3))
    return str(count_increases(sums))


if __name__ == "__main__":
    run(solve_custom_sliding_window_list)
