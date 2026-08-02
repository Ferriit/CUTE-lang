CXX := g++
CXX_SRC := $(shell find src -name "*.cpp")

BUILD_DIR := build/

.PHONY: build_dir clean

default: build

build: build_dir
	${CXX} ${CXX_SRC} -o ${BUILD_DIR}main

run: build
	./build/main

build_dir:
	mkdir -p ${BUILD_DIR}

clean:
	rm -rf ${BUILD_DIR}
