#!/bin/bash
for f in ./html/src/*.rs; do
	file_path_without_ext=${f%.rs}
	file_name_without_ext=${file_path_without_ext##*/}
	printf "${file_name_without_ext}\n"
	printf "${file_name_without_ext}.html\n"
	cargo run -p ra_cli -- highlight -r < $f > html/${file_name_without_ext}.html
done
