# Project Jue V2: Core-World Specification

## Directory Structure

This directory contains the V2 Core-World specifications:

- [`core_spec_v2.0.md`](core_spec_v2.0.md) - Main Core-World specification
- [`proof_system.md`](proof_system.md) - Proof system details
- [`api_reference.md`](api_reference.md) - Public API reference
- [`conformance_tests.md`](conformance_tests.md) - Conformance test suite

## Overview

Core-World V2 remains the immutable formal kernel of Project Jue. The primary change from V1 is explicit acknowledgment that capabilities are an operational concern, not a semantic one. Core-World continues to provide timeless mathematical meaning through βη-equivalence.

## Key Changes from V1

1. **Capability Boundary**: Explicitly state that capabilities belong to Physics/Jue layers
2. **Minimal Axioms**: Frozen set of primitives (λ, Nat, Pair)
3. **Serialization Format**: Standardized binary format for CoreExpr and Proof
4. **Performance**: Optimized normalization for large terms

## Relationship to V2 Architecture

Core-World serves as the "court of final appeal" for semantic equivalence. When Jue-World compiles code with capabilities, it must still prove that the capability-wrapped operations preserve the underlying Core meaning.