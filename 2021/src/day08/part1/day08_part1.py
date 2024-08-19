from collections import defaultdict
from typing import Optional
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    all_segments = set(["a", "b", "c", "d", "e", "f", "g"])

    digit_to_segments = {
        0: ["a", "b", "c", "e", "f", "g"],
        1: ["c", "f"],
        2: ["a", "c", "d", "e", "g"],
        3: ["a", "c", "d", "f", "g"],
        4: ["b", "c", "d", "f"],
        5: ["a", "b", "d", "f", "g"],
        6: ["a", "b", "d", "e", "f", "g"],
        7: ["a", "c", "f"],
        8: ["a", "b", "c", "d", "e", "f", "g"],
        9: ["a", "b", "c", "d", "f", "g"],
    }

    def print_digit(digit: int):
        displayed_segments = """ aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg
"""
        hidden_segments = all_segments - set(digit_to_segments[digit])
        for hidden_segment in hidden_segments:
            displayed_segments = displayed_segments.replace(hidden_segment, ".")

        print(displayed_segments, end="")

    segments_to_digit = dict()
    for digit, segments in digit_to_segments.items():
        segments_to_digit[tuple(segments)] = digit

    segment_count_to_digits = defaultdict(lambda: [])
    for digit, segments in digit_to_segments.items():
        segment_count_to_digits[len(segments)].append(digit)

    def guess_digit(buggy_segments: str) -> Optional[int]:
        # Some digits can be uniquely identified solely based on how many
        # segments are in use, even if these are not the correct segments.
        match segment_count_to_digits[len(buggy_segments)]:
            case [digit]:
                return digit
            case _:
                return None

    # Digits that use a unique number of segments are guessable.
    guessable_digits = 0

    for line in puzzle_input.splitlines():
        (_input_numbers, output_numbers) = map(str.split, line.split("|"))
        for number_segments in output_numbers:
            if (digit := guess_digit(number_segments)) is not None:
                assert digit in [1, 4, 7, 8]
                guessable_digits += 1

    return str(guessable_digits)


if __name__ == "__main__":
    run(solve)
