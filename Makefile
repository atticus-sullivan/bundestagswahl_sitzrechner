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
	# fixup weird encodings
	test "text/plain; charset=utf-8" = "$$(file -bi "data/2013_gesamtergebnis.csv")" || iconv -f ISO-8859-1 -t UTF-8 -o data/2013_gesamtergebnis.csv data/2013_gesamtergebnis.csv
	# translate csv to xml
	python3 utils/csv_to_xml.py data/2013_gesamtergebnis.csv data/2013_gesamtergebnis.xml
	python3 utils/csv_to_xml.py data/2017_gesamtergebnis.csv data/2017_gesamtergebnis.xml

downloadDocs:
	for file in ./docs/*url ; do test -f "docs/$$(tail -n1 "$${file}")" && continue ; curl -o "docs/$$(tail -n1 "$${file}")" "$$(head -n1 "$${file}")" ; done
