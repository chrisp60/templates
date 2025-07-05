# add --release flags whenever ready
build-site:
    cd site && bun install && bun vite build

# add --release flags whenever ready
run-server:
    cargo run

serve: build-site run-server
