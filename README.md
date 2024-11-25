# Advent of Code 2024 Solutions

[![Build Status](https://github.com/akaritakai/AdventOfCode2024/actions/workflows/main.yml/badge.svg)](https://github.com/akaritakai/AdventOfCode2024/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/akaritakai/AdventOfCode2024.svg)](https://codecov.io/gh/akaritakai/AdventOfCode2024)
![Stars](https://img.shields.io/badge/gold%20stars%20⭐-0-yellow)
![Days Completed](https://img.shields.io/badge/days%20completed-0-green)

This repo contains my Advent of Code 2024 solutions in Rust. After providing it with your puzzle inputs (or your
session token), running the program will print out the answers to all currently solved days of the puzzle. A Docker image is provided to ensure compatibility with machines that do not want to install dependencies.

The goal of this repo is to provide fast, highly tested, and easy-to-use solutions.

This repo may see changes in the future to improve runtime. If you have any suggestions, issues running the code, or
find a correctness error: please open an issue or pull request.

### Example output:
```
```

## Docker Instructions

1. Follow the instructions below for providing your puzzle input.
2. Run `docker build -t aoc2024 .`
3. Run `docker run --rm --name aoc2024-run aoc2024`

## Providing Your Puzzle Input

There are two supported methods for inputting your puzzle data into this application.

### Automatic Puzzle Fetcher (via Session Cookie)

First, get your cookie session data.

You will need to log into the Advent of Code website and then inspect your cookies.
If you are using Chrome, you can follow the directions [here](https://developers.google.com/web/tools/chrome-devtools/storage/cookies).

You will be looking for a cookie called `session`. It will contain a long sequence of hexadecimal digits.

Place that data into a file called `cookie.txt` in the project directory.

The application will use that data to automatically fetch your puzzle input for each day.

### Manual Input

This code will also look in a particular location on your local machine for puzzle input.

In the project directory, it will check a directory called `puzzle`.
Within that directory it will expect Day 1's input to be in a file called `01`, Day 2's input to be in a file called `02`, etc.

You can find your puzzle input for a given day by logging into the Advent of Code website and then navigating to the URL
for that puzzle's input.

The URL for your puzzle input will be at:
```
https://adventofcode.com/2024/day/${DAY}/input
```
where `${DAY}` is the day number of the puzzle.

As an example, Day 1's input is at https://adventofcode.com/2024/day/1/input,
Day 2's input is at https://adventofcode.com/2024/day/2/input, etc.
