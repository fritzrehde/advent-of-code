from enum import Enum
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    open_to_closed = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }

    class CharState(Enum):
        Opening = 0
        Closing = 1

    def open_or_closed(c: str) -> CharState:
        if c in ("(", "[", "{", "<"):
            return CharState.Opening
        elif c in (")", "]", "}", ">"):
            return CharState.Closing
        else:
            raise Exception(f"unexpected char: {c}")

    class LineState(Enum):
        Legal = 0
        Incomplete = 1
        # Chunk closes with the wrong character.
        Corrupted = 2

    def line_state(line: str):
        """
        Return the state of the line. For corrupted lines, also return the
        offending character that caused the line to be invalid. For incomplete
        strings, also return the remaining closing chars that would make the
        line legal/valid.
        """
        openings = []
        for c in line:
            match open_or_closed(c):
                case CharState.Opening:
                    openings.append(c)
                case CharState.Closing:
                    if len(openings) == 0:
                        raise Exception(
                            "assume there are no lines with too many closing chars"
                        )
                    else:
                        latest_opening = openings.pop()
                        if c != open_to_closed[latest_opening]:
                            return (LineState.Corrupted, c)

        if len(openings) == 0:
            return (LineState.Legal, None)
        else:
            return (
                LineState.Incomplete,
                map(
                    lambda opening: open_to_closed[opening], reversed(openings)
                ),
            )

    points = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    }

    lines = puzzle_input.splitlines()

    incomplete_line_remaining = (
        remaining
        for (state, remaining) in map(line_state, lines)
        if state == LineState.Incomplete
    )

    def calc_score(remaining):
        score = 0
        for c in remaining:
            score *= 5
            score += points[c]
        return score

    def sorted_middle(iterable):
        # Assume the iterable has an odd length.
        arr = sorted(iterable)
        middle_idx = len(arr) // 2
        return arr[middle_idx]

    result = sorted_middle(map(calc_score, incomplete_line_remaining))

    return str(result)


if __name__ == "__main__":
    run(solve)
