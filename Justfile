run DAY:
    cargo aoc --day {{DAY}}

test:
    cargo nextest run

setup:
    cargo install --locked cargo-aoc cargo-nextest
