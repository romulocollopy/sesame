dev:
	DEBUG=True ./scripts/init-server.sh

test:
	cargo watch -x test
