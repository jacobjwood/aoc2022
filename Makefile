make:
	mkdir day$(day)
	cp template.py day$(day)/day$(day).py
	sed -i '' 's/input.txt/day$(day)\/input.txt/g' day$(day)/day$(day).py
	touch day$(day)/input.txt
	python3 setup.py $(day)
