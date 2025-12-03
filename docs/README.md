# Jue Compiler Documentation

This directory contains comprehensive documentation for the Jue compiler project, organized into logical categories for easy navigation.

## Documentation Structure

```
docs/
├── README.md                          # This file
├── architecture/                      # Architecture documentation
│   ├── README.md                      # Architecture overview
│   ├── compiler/                      # Compiler-specific architecture
│   │   ├── README.md                  # Compiler architecture overview
│   │   ├── JUEC and JUERUN.md         # Component separation
│   │   └── Cranelift Vs LLVM.md        # Backend comparison
│   ├── runtime/                       # Runtime-specific architecture
│   │   ├── README.md                  # Runtime architecture overview
│   │   ├── HOMOICONIC_FEATURES_DOCUMENTATION.md
│   │   └── AST_Primitives.md          # AST primitives
│   └── system/                        # System-level architecture
│       ├── README.md                  # System architecture overview
│       ├── COMPREHENSIVE_ENGINEERING_PLAN.md
│       ├── ENGINEERING_PLAN_SUMMARY.md
│       └── ROADMAP.md                 # Project roadmap
├── development/                       # Development documentation
│   ├── README.md                      # Development overview
│   ├── process/                       # Development processes
│   │   └── README.md                  # Process documentation overview
│   ├── testing/                       # Testing guidelines
│   │   ├── README.md                  # Testing documentation overview
│   │   ├── COMPREHENSIVE_TEST_SUITE_SUMMARY.md
│   │   ├── TEST_DRIVEN_DEVELOPMENT_GUIDELINES.md
│   │   └── [Other test suite documents]
│   └── contributing/                   # Contribution guidelines
│       └── README.md                  # Contributing documentation
├── phases/                            # Phase-specific documentation
│   ├── README.md                      # Phases overview
│   ├── PHASE_1_MVC.md                 # Phase 1: Minimal Viable Compiler
│   ├── PHASE_2_CORE_FEATURES.md        # Phase 2: Core Language Features
│   ├── PHASE_3_FULL_PIPELINE.md        # Phase 3: Full Compiler Pipeline
│   └── PHASE_4_TESTING_POLISH.md       # Phase 4: Testing and Polish
├── ast/                               # AST-related documentation
│   ├── README.md                      # AST documentation overview
│   ├── HIR_Lowering.md                # HIR lowering process
│   ├── HIR_MIR_Discussion.md          # HIR/MIR relationship
│   └── MIR_Design.md                  # MIR design details
└── pitch/                              # Pitch and presentation materials
    ├── README.md                      # Pitch materials overview
    ├── index.html                      # Pitch deck entry point
    ├── PitchDeck.md                    # Main pitch deck
    ├── PitchDeck_DARPA.md              # DARPA-specific pitch
    ├── Skeptic1.md                     # Skeptic response analysis
    └── Slides.md                       # Presentation slides