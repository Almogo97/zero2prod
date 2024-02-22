run:
	cargo watch -x run | bunyan

test-with-logs:
	RUST_LOG="zero2prod=debug,tower_http=debug,axum::rejection=trace" cargo test | bunyan

cov:
	RUST_LOG=trace cargo tarpaulin
