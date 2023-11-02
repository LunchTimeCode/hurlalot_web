@_list:
	just --list --unsorted


alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log

watch:
	cargo watch -c -- just verify

run:
   trunk serve --open

test:
    cargo test --target wasm32-unknown-unknown

# Perform all verifications (compile, test, lint etc.)
verify: lint 

# Run the static code analysis
lint:
	leptosfmt src --check
	cargo fmt --check
	cargo clippy

clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

fmt:
  leptosfmt src -c leptosfmt.toml
  cargo fmt
