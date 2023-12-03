set dotenv-load := true

work day:
		nvim day-{{day}}

create day:
		cargo generate --path ./daily-template --name day-{{day}}
		bash -c "curl 'https://adventofcode.com/$YEAR/day/{{day}}/input' -H 'cookie: session=$AOC_SESSION' -H 'user-agent: one time download script. Author: cameron_barnes@outlook.com' --compressed > day-{{day}}/input.txt"

bench-all:
		cargo bench -q > benchmarks.txt
bench day part:
		cargo bench --bench day-{{day}}-bench part{{part}} >> day-{{day}}.bench.txt

test-all:
		cargo nextest r
test day part:
		cargo nextest r part{{part}} -p day-{{day}}
test-day day:
		cargo nextest r -p day-{{day}}

run day part:
		cargo run -p day-{{day}} --bin part{{part}}
