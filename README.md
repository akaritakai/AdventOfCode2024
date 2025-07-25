# Advent of Code 2024 Solutions

[![Build Status](https://github.com/akaritakai/AdventOfCode2024/actions/workflows/main.yml/badge.svg)](https://github.com/akaritakai/AdventOfCode2024/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/akaritakai/AdventOfCode2024.svg)](https://codecov.io/gh/akaritakai/AdventOfCode2024)
![Stars](https://img.shields.io/badge/gold%20stars%20⭐-40-yellow)
![Days Completed](https://img.shields.io/badge/days%20completed-20-green)

This repo contains my Advent of Code 2024 solutions in Rust. After providing it with your puzzle inputs (or your
session token), running the program will print out the answers to all currently solved days of the puzzle. A Docker image is provided to ensure compatibility with machines that do not want to install dependencies.

The goal of this repo is to provide fast, highly tested, and easy-to-use solutions.

This repo may see changes in the future to improve runtime. If you have any suggestions, issues running the code, or
find a correctness error: please open an issue or pull request.

### Example output:
```
Day 01 Part 1: 1666427
Day 01 Part 2: 24316233
Day 02 Part 1: 585
Day 02 Part 2: 626
Day 03 Part 1: 165225049
Day 03 Part 2: 108830766
Day 04 Part 1: 2560
Day 04 Part 2: 1910
Day 05 Part 1: 5374
Day 05 Part 2: 4260
Day 06 Part 1: 5564
Day 06 Part 2: 1976
Day 07 Part 1: 20665830408335
Day 07 Part 2: 354060705047464
Day 08 Part 1: 359
Day 08 Part 2: 1293
Day 09 Part 1: 6385338159127
Day 09 Part 2: 6415163624282
Day 10 Part 1: 468
Day 10 Part 2: 966
Day 11 Part 1: 184927
Day 11 Part 2: 220357186726677
Day 12 Part 1: 1396562
Day 12 Part 2: 844132
Day 13 Part 1: 35997
Day 13 Part 2: 82510994362072
Day 14 Part 1: 232589280
Day 14 Part 2: 7569
Day 15 Part 1: 1511865
Day 15 Part 2: 1519991
Day 16 Part 1: 93436
Day 16 Part 2: 486
Day 17 Part 1: 3,5,0,1,5,1,5,1,0
Day 17 Part 2: 107413700225434
Day 18 Part 1: 384
Day 18 Part 2: 36,10
Day 19 Part 1: 367
Day 19 Part 2: 724388733465031
Day 20 Part 1: 1409
Day 20 Part 2: 1012821
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
