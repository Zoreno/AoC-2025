run:
    cargo aoc --day 1
    cargo aoc --day 2
    cargo aoc --day 3
    cargo aoc --day 4
    cargo aoc --day 5
    cargo aoc --day 6
    cargo aoc --day 7

run-day DAY:
    cargo aoc --day {{DAY}}

test:
    cargo nextest run

setup:
    cargo install --locked cargo-aoc cargo-nextest

lint:
    cargo clippy
