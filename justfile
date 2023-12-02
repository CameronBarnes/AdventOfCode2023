set dotenv-load := true

work day:
		nvim day-{{day}}

create day:
		cargo generate --path ./daily-template --name day-{{day}}
		bash -c "curl 'https://adventofcode.com/$YEAR/day/{{day}}/input' -H 'cookie: session=$AOC_SESSION' --compressed > day-{{day}}/input.txt"

bench-all:
		cargo bench -q > benchmarks.txt
bench day part:
		cargo bench --bench day-{{day}}-bench part{{part}} >> day-{{day}}.bench.txt
