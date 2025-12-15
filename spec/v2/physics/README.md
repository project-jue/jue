# Project Jue V2: Physics-World Specification

## Directory Structure

This directory contains the V2 Physics-World specifications:

- [`physics_spec_v2.0.md`](physics_spec_v2.0.md) - Main Physics-World specification
- [`capability_system.md`](capability_system.md) - Capability system details
- [`instruction_set.md`](instruction_set.md) - VM instruction set reference
- [`scheduler.md`](scheduler.md) - Actor scheduler and concurrency model
- [`api_reference.md`](api_reference.md) - Public API for Jue-World integration

## Overview

Physics-World V2 transforms from a simple deterministic VM into a **capability-enforced runtime**. This is the most significant change in V2, creating a unified security model for all privileged operations.

## Key Changes from V1

1. **Capability System**: New core mechanism for managing power
2. **Enhanced Actor Model**: Actors carry capability sets and request logs
3. **Capability-Aware Instructions**: New opcodes for capability checking and requests
4. **Scheduler as Authority**: Scheduler mediates all capability grants/revocations
5. **Comptime Integration**: Sandboxed compile-time execution with restricted capabilities

## Relationship to V2 Architecture

Physics-World is the **final arbiter of power**. All capability requests flow through the scheduler, which enforces AIKR limits and provides structured feedback. This creates the causal friction necessary for learning.

## Implementation Priority

Physics-World V2 is the **critical path** for V2 implementation. The capability system must be operational before Jue-World can be updated to use it.