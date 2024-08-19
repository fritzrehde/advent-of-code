from dataclasses import dataclass
from typing import List
from puzzle_utils import puzzle, run
from more_itertools import last


@dataclass
class Cell:
    num: int
    marked: bool

    def __init__(self, num: int):
        self.num = num
        self.marked = False

    def mark(self):
        self.marked = True


@dataclass
class Board:
    rows: List[List[Cell]]
    # A board can only win once, after which it will always be in a winning
    # state.
    has_won: bool

    def __init__(self, lines: List[str]):
        self.rows = [[Cell(int(num)) for num in row.split()] for row in lines]
        self.has_won = False

    def cells_in_row(self, row_idx: int):
        return self.rows[row_idx]

    def cells_in_col(self, col_idx: int):
        # TODO: very inefficient, we're duplicating the whole matrix every time.
        return list(zip(*self.rows))[col_idx]

    def get_unmarked_cells(self):
        for row in self.rows:
            for cell in row:
                if not cell.marked:
                    yield cell

    def first_win_after_mark(self, marked_num: int) -> bool:
        """
        Mark all cells on the board that have the given value (there may be
        multiple), and return whether this board has won.
        """

        def latest_marked_coords():
            for row_idx, row in enumerate(self.rows):
                for col_idx, cell in enumerate(row):
                    if cell.num == marked_num:
                        cell.mark()
                        yield (row_idx, col_idx)

        def first_win():
            for row_idx, col_idx in latest_marked_coords():
                if all(cell.marked for cell in self.cells_in_row(row_idx)):
                    return True
                if all(cell.marked for cell in self.cells_in_col(col_idx)):
                    return True

        if self.has_won:
            _ = latest_marked_coords()
            # Has already won before => not first win.
            return False
        else:
            if first_win():
                self.has_won = True
                return True
            else:
                # Hasn't won before, and also hasn't won now.
                return False


@puzzle
def solve(puzzle_input: str) -> str:
    sections = puzzle_input.split("\n\n")

    numbers = list(map(int, sections[0].split(",")))
    boards = [Board(board.splitlines()) for board in sections[1:]]

    def wins():
        for number in numbers:
            for board in boards:
                if board.first_win_after_mark(number):
                    unmarked_sum = sum(
                        map(
                            lambda cell: cell.num,
                            board.get_unmarked_cells(),
                        )
                    )
                    yield (number, unmarked_sum)

    (number, unmarked_sum) = last(wins())

    result = unmarked_sum * number
    return str(result)


if __name__ == "__main__":
    run(solve)
