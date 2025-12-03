from typing import Iterable
from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    def max_line_joltages():
        for line in puzzle_input.splitlines():
            digits = list(map(int, line))

            n = len(digits)
            K = 12
            assert n >= 12

            IMPOSSIBLE = -1

            # dp[i][k] represents the max k-digit number we can make using digits from 0 to i, inclusive.
            dp = [[IMPOSSIBLE for _ in range(K + 1)] for _ in range(n)]

            # base cases:
            for i in range(0, n):
                dp[i][0] = 0
            dp[0][1] = digits[0]

            # order of computation: increasing i, increasing k
            for i in range(1, n):
                for k in range(1, K + 1):
                    # recurrence:
                    # we either pick digit i given k-1 already picked, or we just use the first i-1 digits to get k
                    # dp[i][k] = max(concat(dp[i-1][k-1], digits[i]), dp[i-1][k])
                    dp[i][k] = max(
                        dp[i - 1][k - 1] * 10 + digits[i], dp[i - 1][k]
                    )

            # final answer: dp[n-1][K]
            joltage = dp[n - 1][K]
            yield joltage

    return str(sum(max_line_joltages(), start=0))


if __name__ == "__main__":
    run(solve)
