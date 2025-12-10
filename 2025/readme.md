Setup:
```sh
uv python install pypy
```

Run tests for all days with:
```sh
# cd to 2025 dir
uv run -- pytest
```

Run tests for a specific day with:
```sh
# cd to 2025 dir
uv run -- pytest src/dayXX

# or
cd 2025/src/dayXX
uv run -- pytest
```

Run tests where `NAME` is a substring of test name:
```sh
uv run -- pytest src/dayXX -k test_01
```
