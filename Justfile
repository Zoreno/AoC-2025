run:
    cargo aoc --day 1
    cargo aoc --day 2
    cargo aoc --day 3

run-day DAY:
    cargo aoc --day {{DAY}}

test:
    cargo nextest run

setup:
    cargo install --locked cargo-aoc cargo-nextest

lint:
    cargo clippy
