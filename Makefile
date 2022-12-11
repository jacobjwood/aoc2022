make:
	if ! [ -d ./day$(day) ]; \
	then \
		mkdir day$(day); \
		cp template.py day$(day)/day$(day).py; \
		sed -i '' 's/input.txt/day$(day)\/input.txt/g' day$(day)/day$(day).py; \
		touch day$(day)/input.txt; \
		python3 setup.py $(day); \
	else \
		echo "Directory for day $(day) already exists"; \
	fi
rust:
	echo "Making day $(day) with Cargo"
	if ! [ -d ./day$(day) ]; \
	then \
		cargo new day$(day); \
		touch day$(day)/input.txt; \
		python3 setup.py $(day); \
		cp template.rs day$(day)/src/main.rs; \
	else \
		echo "Directory for day $(day) already exists"; \
	fi
