start:
	cargo run -r

dev: 
	cargo run

test:
	cargo test -r -- -Z unstable-options --report-time

setup: 
	docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
	docker run -p 6379:6379 -d eqalpha/keydb