from collections import defaultdict
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    timers = list(map(int, puzzle_input.rstrip("\n").split(",")))

    # Map each possible time a timer can have to the number of times (pun
    # unintended) this timer occurs. This hashmap will always contain at most 9
    # entries, which is the largest time a timer can have.
    timer_counts = defaultdict(lambda: 0)
    for timer in timers:
        timer_counts[timer] += 1

    DAYS = 256
    for _day in range(0, DAYS):
        new_timer_counts = defaultdict(lambda: 0)
        for timer, occurences in timer_counts.items():
            if timer == 0:
                timer = 6
                child_time = 8
                new_timer_counts[timer] += occurences
                new_timer_counts[child_time] += occurences
            else:
                timer -= 1
                new_timer_counts[timer] += occurences
        timer_counts = new_timer_counts

    total_fish = sum(timer_counts.values())

    return str(total_fish)


if __name__ == "__main__":
    run(solve)
