from typing import Tuple
from functools import reduce
from typing import Iterable
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    rows = [list(row) for row in puzzle_input.splitlines()]
    num_rows = len(rows)
    num_cols = len(rows[0])

    def apply_op(op: str, args: Iterable[int]) -> int:
        match op:
            case "+":
                op_fn = lambda x, y: x + y
            case "*":
                op_fn = lambda x, y: x * y
            case _:
                raise Exception("unreachable")

        return reduce(op_fn, args)

    def cols() -> Iterable[Iterable[str]]:
        for c in range(0, num_cols):
            yield (rows[r][c] for r in range(0, num_rows))

    def questions():
        cols_it = iter(cols())
        for first_col_raw in cols_it:
            first_col = "".join(first_col_raw)
            op = first_col[-1]

            def args():
                yield int(first_col[:-1].strip())

                for col_raw in cols_it:
                    col = "".join(col_raw).strip()
                    if col == "":
                        break
                    yield int(col)

            yield (op, args())

    def eval_question(question: Tuple[str, Iterable[int]]) -> int:
        op, args = question
        return apply_op(op, args)

    res = sum(eval_question(q) for q in questions())

    return str(res)


if __name__ == "__main__":
    run(solve)
