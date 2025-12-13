# Project Jue - Comprehensive Test Makefile
# ========================================

# Default target
all: help

# Help message
help:
	@echo "Project Jue Test Suite"
	@echo "======================="
	@echo ""
	@echo "Available targets:"
	@echo ""
	@echo "  all                - Show this help message"
	@echo "  test               - Run all tests (default)"
	@echo "  test-core          - Run Core-World tests"
	@echo "  test-physics       - Run Physics Layer tests"
	@echo "  test-dan           - Run Dan-World tests"
	@echo "  test-jue           - Run Jue-World tests"
	@echo "  test-integration   - Run integration tests"
	@echo "  test-beta          - Run beta reduction tests"
	@echo "  test-normalization - Run normalization tests"
	@echo "  test-evaluation    - Run evaluation tests"
	@echo "  test-proof         - Run proof system tests"
	@echo "  test-comprehensive - Run comprehensive tests"
	@echo "  test-fast          - Run fast tests (excluding slow ones)"
	@echo "  test-slow          - Run slow tests only"
	@echo "  test-property      - Run property-based tests"
	@echo "  test-bench         - Run benchmarks"
	@echo "  test-clean         - Clean test artifacts"
	@echo "  build              - Build the project"
	@echo "  build-release      - Build release version"
	@echo "  clippy             - Run clippy linter"
	@echo "  fmt                - Format code with rustfmt"
	@echo "  clean              - Clean build artifacts"
	@echo ""

# Default test target
test: test-all

# Run all tests
test-all:
	@echo "Running all tests..."
	cargo test --all
	@echo "Running integration tests..."
	cargo test --test *

# Core-World tests
test-core:
	@echo "Running Core-World tests..."
	cargo test -p core_world

# Physics Layer tests
test-physics:
	@echo "Running Physics Layer tests..."
	cargo test -p physics_layer

# Dan-World tests
test-dan:
	@echo "Running Dan-World tests..."
	cargo test --test test_dan_world_comprehensive_integration
	cargo test --test test_dan_world_core_world_integration
	cargo test --test test_event_loop_foundations

# Jue-World tests
test-jue:
	@echo "Running Jue-World tests..."
	cargo test --test test_jue_world_basic
	cargo test --test test_jue_world_integration

# Integration tests
test-integration:
	@echo "Running integration tests..."
	cargo test --test test_complete_system_integration
	cargo test --test test_core_jue_integration
	cargo test --test test_core_physics_integration
	cargo test --test test_jue_physics_integration

# Beta reduction tests
test-beta:
	@echo "Running beta reduction tests..."
	cargo test --test test_beta_reduction_comprehensive
	cargo test --test test_double_application_reduction
	cargo test --test test_identity_function_reduction
	cargo test --test test_nested_lambda_reduction
	cargo test --test test_variable_capture_avoidance

# Normalization tests
test-normalization:
	@echo "Running normalization tests..."
	cargo test --test test_normalization_comprehensive
	cargo test --test test_already_normal_form
	cargo test --test test_normalization_idempotent
	cargo test --test test_normalization_preserves_structure

# Evaluation tests
test-evaluation:
	@echo "Running evaluation tests..."
	cargo test --test test_evaluation_relation_comprehensive
	cargo test --test test_application_elimination_simple
	cargo test --test test_application_elimination_complex
	cargo test --test test_normal_form_detection

# Proof system tests
test-proof:
	@echo "Running proof system tests..."
	cargo test --test test_proof_system_comprehensive
	cargo test --test test_proof_system_integration

# Comprehensive tests
test-comprehensive:
	@echo "Running comprehensive tests..."
	cargo test --test test_integration_comprehensive
	cargo test --test test_core_kernel_eval_consistency
	cargo test --test test_eval_normalization_consistency

# Fast tests (exclude slow ones)
test-fast:
	@echo "Running fast tests..."
	cargo test --all -- --test-threads=1 --skip slow

# Slow tests only
test-slow:
	@echo "Running slow tests..."
	cargo test --all -- --test-threads=1 slow

# Property-based tests
test-property:
	@echo "Running property-based tests..."
	cargo test --all --features proptest

# Benchmarks
test-bench:
	@echo "Running benchmarks..."
	cargo bench

# Clean test artifacts
test-clean:
	@echo "Cleaning test artifacts..."
	cargo clean
	rm -rf target/debug/deps/test_*
	rm -rf target/release/deps/test_*

# Build the project
build:
	@echo "Building project..."
	cargo build

# Build release version
build-release:
	@echo "Building release version..."
	cargo build --release

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# PHONY targets (don't represent files)
.PHONY: all help test test-all test-core test-physics test-dan test-jue \
		test-integration test-beta test-normalization test-evaluation \
		test-proof test-comprehensive test-fast test-slow test-property \
		test-bench test-clean build build-release clippy fmt clean