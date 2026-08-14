# Target vocabulary (build-server / start-server / wait-for-server / stop-server)
# matches the commons TestServer orchestration. Recipes are single shell lines
# for the 3.81 Make that ships with macOS.

SHELL := bash

# Port for the server (override: make run-server PORT=9090).
PORT ?= 8098

BIN      := target/release/esdk-test-server
PID_FILE := .server.pid
LOG_FILE := .server.log

# Bootstrap-then-delegate coordinates for `make test-server`. The commons
# repository coordinates live in commons-configuration.json next to this
# Makefile; COMMONS_BRANCH overrides the configured branch at invocation time.
MAKEFILE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
REPO_ROOT    := $(abspath $(MAKEFILE_DIR)/../..)
COMMONS_CONFIGURATION := $(MAKEFILE_DIR)/commons-configuration.json
COMMONS_BRANCH ?=
CLONE_DIR      ?= $(MAKEFILE_DIR)/.commons-clone
CLONE_ORCH_DIR := $(CLONE_DIR)/esdk/test-server/orchestrator

.PHONY: help build-server run-server start-server wait-for-server stop-server \
        test test-server check-java fmt clippy check clean

help: ## Show this help
	@echo "ESDK TestServer (Dafny-Rust) — make targets"
	@grep -E '^[a-zA-Z0-9_-]+:.*## ' "$(lastword $(MAKEFILE_LIST))" \
		| sort | awk 'BEGIN{FS=":.*## "}{printf "  %-18s %s\n", $$1, $$2}'
	@echo ""
	@echo "  PORT=$(PORT)"

build-server: ## Build the server (release)
	cargo build --release

run-server: build-server ## Run the server in the FOREGROUND on PORT (Ctrl-C to stop)
	$(BIN) $(PORT)

start-server: build-server ## Start the server in the BACKGROUND on PORT (writes .server.pid)
	@$(BIN) $(PORT) >"$(LOG_FILE)" 2>&1 & echo $$! >"$(PID_FILE)"; \
		echo "started esdk-test-server (pid $$(cat $(PID_FILE))) on port $(PORT)"

wait-for-server: ## Block until the server accepts connections on PORT (120s timeout)
	@for i in $$(seq 1 120); do \
		if (exec 3<>"/dev/tcp/127.0.0.1/$(PORT)") 2>/dev/null; then echo "server ready on $(PORT)"; exit 0; fi; \
		sleep 1; \
	done; \
	echo "timed out waiting for port $(PORT)"; [ -f "$(LOG_FILE)" ] && tail -n 40 "$(LOG_FILE)"; exit 1

stop-server: ## Stop the background server and free PORT
	@if [ -f "$(PID_FILE)" ]; then kill "$$(cat $(PID_FILE))" 2>/dev/null || true; rm -f "$(PID_FILE)"; fi; \
		pids=$$(lsof -ti tcp:$(PORT) 2>/dev/null || true); \
		if [ -n "$$pids" ]; then kill $$pids 2>/dev/null || true; fi; \
		echo "stopped server on port $(PORT)"

test: ## Run the server's tests
	cargo test

check-java: ## Verify JAVA_HOME points at a JDK 21+ (the commons orchestrator needs it)
	@if [ -z "$$JAVA_HOME" ] || [ ! -x "$$JAVA_HOME/bin/java" ]; then \
		echo "ERROR: JAVA_HOME must point at a JDK 21+ for the commons orchestrator." >&2; exit 1; \
	fi; \
	v=$$("$$JAVA_HOME/bin/java" -version 2>&1 | grep -i version | head -1 | sed -E 's/.*version .?([0-9]+).*/\1/'); \
	if [ -z "$$v" ] || [ "$$v" -lt 21 ] 2>/dev/null; then \
		echo "ERROR: JDK 21+ required, JAVA_HOME has major version '$$v'." >&2; exit 1; \
	fi

# Bootstrap-then-delegate (the single orchestrated entry point): parse the
# commons coordinates, clone commons at the branch head, and run the
# orchestrator core in the clone with this working tree as the live Rust
# library + server source. The core builds and launches every configured
# Language_Server, runs the full Tests matrix, and tears down; its exit code
# propagates. Needs AWS credentials and a JDK 21+.
test-server: check-java ## Run the complete cross-language TestServer via the commons orchestrator; COMMONS_BRANCH=<b> overrides
	@set -eo pipefail; \
	if ! coords=$$(python3 -c 'import json,sys; c=json.load(open(sys.argv[1]))["commonsRepository"]; print(c["url"]); print(c["branch"])' "$(COMMONS_CONFIGURATION)" 2>/dev/null); then \
		echo "ERROR: missing or unparseable $(COMMONS_CONFIGURATION); halting before any clone." >&2; exit 1; \
	fi; \
	{ read -r url; read -r branch; } <<< "$$coords"; \
	if [ -n "$(strip $(COMMONS_BRANCH))" ]; then branch="$(strip $(COMMONS_BRANCH))"; reason="invocation-override"; else reason="configuration-entry"; fi; \
	echo "==> Cloning commons at '$$branch' ($$reason) into $(CLONE_DIR)"; \
	rm -rf "$(CLONE_DIR)"; \
	if ! git clone --depth 1 --single-branch --branch "$$branch" "$$url" "$(CLONE_DIR)"; then \
		echo "ERROR: failed to clone $$url at branch $$branch; no Tests will run." >&2; exit 1; \
	fi; \
	if [ ! -d "$(CLONE_ORCH_DIR)" ]; then \
		echo "ERROR: branch $$branch of $$url has no orchestrator at esdk/test-server/orchestrator." >&2; exit 1; \
	fi; \
	echo "==> Delegating: context=language:rust-dafny languageRepoRoot=$(REPO_ROOT)"; \
	cd "$(CLONE_ORCH_DIR)" && ./gradlew --console=plain run \
		--args="context=language:rust-dafny languageRepoRoot=$(REPO_ROOT) commonsOrigin.url=$$url commonsOrigin.branch=$$branch commonsOrigin.reason=$$reason"

fmt: ## Check formatting (this package only; --all would sweep in the aws-esdk path dependency)
	cargo fmt -- --check

clippy: ## Lint with warnings denied
	cargo clippy --all-targets -- -D warnings

check: fmt clippy build-server test ## fmt + clippy + build + test (the CI gate)

clean: ## Remove build output, server scratch files, and the commons clone
	cargo clean; rm -f "$(PID_FILE)" "$(LOG_FILE)"; rm -rf "$(CLONE_DIR)"
