import inspect
import os
from functools import wraps
import sys
import re


def puzzle(solve_func):
    @wraps(solve_func)
    def wrapper(puzzle_input):
        return solve_func(puzzle_input)

    caller_name = solve_func.__name__
    caller_frame = inspect.stack()[1]

    solve_file_path = caller_frame.filename
    globals_scope = caller_frame.frame.f_globals
    test_fn_prefix = caller_name

    def codegen_test_fn(id, puzzle_input, expected_output):
        # Generate a test function.
        def test(benchmark):
            observed_output = benchmark(solve_func, puzzle_input)
            assert expected_output == observed_output

        test_fn_name = (
            f"test_{id}" if id != "" else "test"
        ) + f"_{test_fn_prefix}"
        codegen_fn(test_fn_name, test, globals_scope)

    # Generate the main test function.
    puzzle_input = get_main_puzzle_input(solve_file_path)
    expected_output = get_main_expected_output(solve_file_path)
    codegen_test_fn("", puzzle_input, expected_output)

    # Generate the additional example test functions.
    for id, puzzle_input, expected_output in get_example_tests(solve_file_path):
        codegen_test_fn(id, puzzle_input, expected_output)

    # TODO: would be cool to be able to also generate the main() here as well, to reduce boilerplate.

    return wrapper


def codegen_fn(fn_name, fn, globals_scope):
    """Put a function in the global namespace of the module this method was called from."""
    globals_scope[fn_name] = fn


def run(solve_func):
    caller_frame = inspect.stack()[1]
    solve_file_path = caller_frame.filename
    puzzle_input = get_main_puzzle_input(solve_file_path)
    result = solve_func(puzzle_input)
    print(result)


def read_file(file_path: str) -> str:
    with open(file_path, "r") as file:
        return file.read()


def get_main_puzzle_input(solve_file_path: str) -> str:
    PUZZLE_INPUT_FILE = os.path.join(
        os.path.dirname(os.path.dirname(solve_file_path)), "puzzle_input.txt"
    )
    return read_file(PUZZLE_INPUT_FILE)


def get_main_expected_output(solve_file_path: str) -> str:
    EXPECTED_OUTPUT_FILE = os.path.join(
        os.path.dirname(solve_file_path), "expected_output.txt"
    )
    return read_file(EXPECTED_OUTPUT_FILE).rstrip("\n")


def get_example_tests(solve_file_path: str):
    """For each example test, return the test id, puzzle input and expected output."""

    partX_dir = os.path.dirname(solve_file_path)
    tests_dir = os.path.join(partX_dir, "tests")
    ids = set(
        m.group(1)
        for file in os.listdir(tests_dir)
        if (
            m := re.search(
                r"^(?:expected_output|puzzle_input(\d+)\.txt)$", file
            )
        )
        is not None
    )
    for id in ids:
        expected_output_file = os.path.join(
            tests_dir, f"expected_output{id}.txt"
        )
        puzzle_input_file = os.path.join(tests_dir, f"puzzle_input{id}.txt")

        if os.path.isfile(expected_output_file) and os.path.isfile(
            puzzle_input_file
        ):
            yield (
                id,
                read_file(puzzle_input_file),
                read_file(expected_output_file).rstrip("\n"),
            )
        else:
            print(
                f"error: did not find puzzle input and expected output files for id {id}",
                file=sys.stderr,
            )
            exit(1)
