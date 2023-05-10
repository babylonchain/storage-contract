DOCKER := $(shell which docker)
CUR_DIR := $(shell pwd)
CUR_BASENAME := $(shell basename $(CUR_DIR))
# Native arch
BUILDARCH := $(shell uname -m)

build-optimized-amd64:
	docker run --rm -v "$(CUR_DIR)":/code \
  --mount type=volume,source="$(CUR_BASENAME)_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.13

build-optimized-arm64:
	docker run --rm -v "$(CUR_DIR)":/code \
  --mount type=volume,source="$(CUR_BASENAME)_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.13
