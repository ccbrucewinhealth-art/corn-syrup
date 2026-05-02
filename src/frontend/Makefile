SHELL := /usr/bin/env bash
.PHONY: install build dev loop all-in-one clean
install:
	npm install
build:
	bash ./util_compile.sh
dev:
	bash ./util_corn-syrup.sh
loop:
	bash ./util_corn-syrup-loop-exec.sh
all-in-one:
	bash ./util_all-in-one-compile.sh
clean:
	rm -rf dist node_modules corn-syrup-frontend-dist.tar.gz
