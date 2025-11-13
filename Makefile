# Config variables
TARGET ?= localhost
PORT   ?= 8080
NUM_SERVERS ?= 3
SERVER_BASE ?= node
TRANSACTION_FILE ?= kvscoordinator/example.txns

SERVER_TARGET = $(TARGET):$(PORT)
COORDINATOR_TARGET = $(TARGET)

.PHONY: all build clean \
        build-server build-interface build-coordinator \
        run-server run-coordinator run-all-local \
        run-cluster run-local run-single

all: build

build:
	cargo build --workspace

build-server:
	cargo build -p kvsserver

build-interface:
	cargo build -p kvsinterface

build-coordinator:
	cargo build -p kvscoordinator

clean:
	cargo clean

# RUN SERVER 
# Example:
#   make run-server TARGET=node1 PORT=8081
run-server:
	./target/debug/kvsserver --listen-on "$(SERVER_TARGET)"

# RUN COORDINATOR 
# Example:
#   make run-coordinator mode=localhost
#   make run-coordinator mode=cluster
#   make run-coordinator mode=single
run-coordinator:
ifeq ($(mode),localhost)
	./target/debug/kvscoordinator --localhost "$(TRANSACTION_FILE)"
else ifeq ($(mode),cluster)
	./target/debug/kvscoordinator --server-base "$(SERVER_BASE)" --num-servers "$(NUM_SERVERS)" "$(TRANSACTION_FILE)"
else ifeq ($(mode),single)
	./target/debug/kvscoordinator --ip-addr "$(COORDINATOR_TARGET)" "$(TRANSACTION_FILE)"
else
	@echo "Usage: make run-coordinator mode=[localhost|cluster|single]"
endif