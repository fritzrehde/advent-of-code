from collections import defaultdict
import itertools
from typing import Optional
from puzzle_utils import puzzle, run


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
    segments_to_digit[tuple(sorted(segments))] = digit


def get_digit_from_segments(segments) -> int:
    return segments_to_digit[tuple(sorted(segments))]


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


def item_in_set(set):
    """
    Extract the only item from a set, asserting that the set only contains one
    item.
    """
    assert len(set) == 1
    return next(iter(set))


def get_real_to_buggy_segment_map(all_unique_digits):
    def get_guessable_digits():
        # Digits that use a unique number of segments are guessable.
        for number_segments in all_unique_digits:
            if (digit := guess_digit(number_segments)) is not None:
                assert digit in [1, 4, 7, 8]
                yield (digit, set(number_segments))

    # Map digits to the segments they are made up of in the buggy input.
    digit_to_buggy_segments = dict(get_guessable_digits())

    # Map the real/true segment to its representation in the buggy input.
    real_to_buggy_segment = dict()

    def find_segment_a():
        # When we subtract digit 7 from 1, we are left with segment a.
        return item_in_set(
            digit_to_buggy_segments[7] - digit_to_buggy_segments[1]
        )

    real_to_buggy_segment["a"] = find_segment_a()

    def find_digit_3():
        # The digits with 5 segments are 2, 3, 5. x, y, z will correspond to
        # 2, 3, 5, though not necessarily in that order. Use these to find
        # digit 3.
        (x, y, z) = list(
            map(
                set,
                filter(lambda segments: len(segments) == 5, all_unique_digits),
            )
        )

        # The union between 2 and 5 contains all (7) segments, while the
        # unions between 2 and 3, and 3 and 5, contain only 6 segments.
        match (
            len(set.union(x, y)),
            len(set.union(x, z)),
            len(set.union(y, z)),
        ):
            case (6, 6, 7):
                return x
            case (6, 7, 6):
                return y
            case (7, 6, 6):
                return z

    digit_to_buggy_segments[3] = find_digit_3()

    def find_segment_g():
        # When we subtract 4's segments from 3's segments, and further
        # subtract segment a, we are left with segment g.
        return item_in_set(
            digit_to_buggy_segments[3]
            - digit_to_buggy_segments[4]
            - {real_to_buggy_segment["a"]}
        )

    real_to_buggy_segment["g"] = find_segment_g()

    def find_segment_b():
        # When we subtract 3's segments from 4's segments, we are left with
        # segment b.
        return item_in_set(
            digit_to_buggy_segments[4] - digit_to_buggy_segments[3]
        )

    real_to_buggy_segment["b"] = find_segment_b()

    def find_segment_f_c():
        # The digits with 6 segments are 0, 6, 9. x, y, z will correspond to
        # 0, 6, 9, though not necessarily in that order.
        (x, y, z) = list(
            map(
                set,
                filter(lambda segments: len(segments) == 6, all_unique_digits),
            )
        )

        # When we intersect 1's segments with x, y, z, only 6 will have 1
        # segment left, and this segment will be segment f, while the other
        # segment, which both 0 and 9 have additionally, will be c.
        diffs = [
            set.intersection(x, digit_to_buggy_segments[1]),
            set.intersection(y, digit_to_buggy_segments[1]),
            set.intersection(z, digit_to_buggy_segments[1]),
        ]

        segment_f = item_in_set(next(filter(lambda s: len(s) == 1, diffs)))
        segment_f_and_c = next(filter(lambda s: len(s) == 2, diffs))
        segment_c = item_in_set(segment_f_and_c - set(segment_f))

        return (segment_f, segment_c)

    (real_to_buggy_segment["f"], real_to_buggy_segment["c"]) = (
        find_segment_f_c()
    )

    def find_segment_d():
        # When we subtract segments b, c, f from digit 4, we are left
        # with segment d.
        segment_set = {
            real_to_buggy_segment[real_segment]
            for real_segment in ["a", "b", "g", "f", "c"]
        }
        return item_in_set(digit_to_buggy_segments[4] - segment_set)

    real_to_buggy_segment["d"] = find_segment_d()

    def find_segment_e():
        # When we subtract segments a, b, c, d, f, g from digit 8, we are
        # left with segment e.
        segment_set = {
            real_to_buggy_segment[real_segment]
            for real_segment in ["a", "b", "c", "d", "f", "g"]
        }
        return item_in_set(digit_to_buggy_segments[8] - segment_set)

    real_to_buggy_segment["e"] = find_segment_e()

    return real_to_buggy_segment


@puzzle
def solve(puzzle_input: str) -> str:
    def output_numbers():
        for entry in puzzle_input.splitlines():
            (input_digits, output_digits) = map(str.split, entry.split("|"))

            all_unique_digits = set(
                map(
                    lambda segments: "".join(sorted(segments)),
                    itertools.chain(input_digits, output_digits),
                )
            )

            real_to_buggy_segment = get_real_to_buggy_segment_map(
                all_unique_digits
            )

            # Invert hashmap (which works because key and values have 1-to-1
            # relationship).
            buggy_to_real_segment = {
                buggy: real for (real, buggy) in real_to_buggy_segment.items()
            }

            def buggy_to_real_segments(buggy_segments):
                return map(
                    lambda buggy_segment: buggy_to_real_segment[buggy_segment],
                    buggy_segments,
                )

            def real_output_digits():
                for buggy_segments in output_digits:
                    real_segments = buggy_to_real_segments(buggy_segments)
                    output_digit = get_digit_from_segments(real_segments)
                    yield str(output_digit)

            output_number = int("".join(real_output_digits()))
            yield output_number

    return str(sum(output_numbers()))


if __name__ == "__main__":
    run(solve)
