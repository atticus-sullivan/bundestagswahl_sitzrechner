# SPDX-FileCopyrightText: 2025 Lukas Heindl
#
# SPDX-License-Identifier: MIT

.PHONY: genDemo build downloadData downloadDocs

genDemo:
	cd demo && vhs main.tape

build:
	cargo build --release
	cp ./target/release/sitzrechner ./sitzrechner

downloadData:
	for file in ./data/*url ; do test -f "data/$$(tail -n1 "$${file}")" && continue ; curl -o "data/$$(tail -n1 "$${file}")" "$$(head -n1 "$${file}")" ; done

downloadDocs:
	for file in ./docs/*url ; do test -f "docs/$$(tail -n1 "$${file}")" && continue ; curl -o "docs/$$(tail -n1 "$${file}")" "$$(head -n1 "$${file}")" ; done
