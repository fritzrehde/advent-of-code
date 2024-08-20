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
        Return the state of the line, along with the offending character that
        caused the line to be corrupted, if any.
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
            return (LineState.Incomplete, None)

    points = {
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137,
    }

    lines = puzzle_input.splitlines()

    corrupted_line_offending_chars = (
        c
        for (state, c) in map(line_state, lines)
        if state == LineState.Corrupted
    )
    result = sum(map(lambda c: points[c], corrupted_line_offending_chars))

    return str(result)


if __name__ == "__main__":
    run(solve)
