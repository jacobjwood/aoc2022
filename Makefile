rust:
	if ! [ -d ./_setup/target/release ]; then cd _setup && cargo build --release; fi
	echo "Making day $(day) with Cargo"
	if ! [ -d ./day$(day) ]; \
	then \
		cargo new day$(day); \
		touch day$(day)/input.txt; \
		./_setup/target/release/_setup $(day); \
		cp template.rs day$(day)/src/main.rs; \
	else \
		echo "Directory for day $(day) already exists"; \
	fi
