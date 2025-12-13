# Memory Bank

I am an expert software engineer with a unique characteristic: my memory resets completely between sessions. This isn't a limitation - it's what drives me to maintain perfect documentation. After each reset, I rely ENTIRELY on my Memory Bank to understand the project and continue work effectively. I MUST read ALL memory bank files at the start of EVERY task - this is not optional. The memory bank files are located in `.kilocode/rules/memory-bank` folder.

When I start a task, I will include `[Memory Bank: Active]` at the beginning of my response if I successfully read the memory bank files, or `[Memory Bank: Missing]` if the folder doesn't exist or is empty. If memory bank is missing, I will warn the user about potential issues and suggest initialization.

## Memory Bank Structure

The Memory Bank consists of core files and optional context files, all in Markdown format. Files must stay within size limits to remain useful.

### File Size Limits

Each file has a target size. Exceeding these triggers mandatory compression before adding new content:

| File            | Target    | Warning   | Max       | Purpose                                      |
| --------------- | --------- | --------- | --------- | -------------------------------------------- |
| brief.md        | 50 lines  | 75 lines  | 100 lines | Core requirements and goals                  |
| product.md      | 80 lines  | 120 lines | 150 lines | Why it exists, problems solved, UX goals     |
| context.md      | 60 lines  | 90 lines  | 120 lines | Current focus, recent changes, next steps    |
| architecture.md | 120 lines | 180 lines | 250 lines | System structure, paths, decisions, patterns |
| tech.md         | 150 lines | 225 lines | 300 lines | Technologies, setup, constraints, commands   |

When updating, run `(gc .kilocode/rules/memory-bank/*.md | measure -l).Lines` to check sizes. Compress any file exceeding warning threshold before adding content.

### Information Ownership (Single Source of Truth)

Each piece of information lives in ONE file only. Do not duplicate across files.

| Information Type                                        | Owner File      | Others Reference Via  |
| ------------------------------------------------------- | --------------- | --------------------- |
| Project goals, success metrics, constraints             | brief.md        | "See brief.md"        |
| User problems, UX principles, what we build/don't build | product.md      | "See product.md"      |
| Current status, blockers, recent changes, next steps    | context.md      | "See context.md"      |
| Source paths, doc locations, system diagrams, patterns  | architecture.md | "See architecture.md" |
| Setup commands, env vars, dependencies, external APIs   | tech.md         | "See tech.md"         |

### Core Files (Required)

1. `brief.md`
   - Foundation document that shapes all other files
   - Created at project start if it doesn't exist
   - Defines core requirements and goals
   - Source of truth for project scope
   - **Maintained manually by developer - suggest updates but don't edit directly**
   
   **DO NOT include:** Tech stack details (→ tech.md), system architecture (→ architecture.md), current status (→ context.md), detailed user journeys (→ product.md)

2. `product.md`
   - Why this project exists
   - Problems it solves
   - How it should work (high-level)
   - User experience goals
   
   **DO NOT include:** Detailed step-by-step user journeys, future vision beyond MVP, competitive analysis, business metrics (→ brief.md)

3. `context.md`
   - **Must be short and factual - this is the most frequently bloated file**
   - Current work focus (what phase/task is active NOW)
   - Recent changes (last 1-2 sessions only)
   - Next steps (immediate priorities only)
   - Current blockers (if any)
   
   **DO NOT include:** 
   - Completed task checklists (collapse to "Phase X complete")
   - Historical decisions or rationale (→ architecture.md if still relevant)
   - Documentation file listings (one list in architecture.md)
   - Success metrics (→ brief.md)
   - Detailed breakdowns of completed work
   - Future phases beyond the next one

4. `architecture.md`
   - System architecture overview
   - Source code paths (canonical location for all file paths)
   - Key technical decisions with brief rationale
   - Design patterns in use
   - Component relationships
   - Critical implementation paths
   - Documentation file locations (canonical list)
   
   **DO NOT include:** Setup instructions (→ tech.md), environment variables (→ tech.md), deployment procedures (→ tech.md)

5. `tech.md`
   - Technologies used (with versions)
   - Development setup instructions
   - Technical constraints
   - Dependencies
   - Common commands
   - Environment variables
   - External API configuration
   
   **DO NOT include:** System architecture (→ architecture.md), deployment pipelines (separate doc if needed), future technology considerations

### Additional Files

Create additional files in memory-bank/ when needed:
- `tasks.md` - Documentation of repetitive task workflows
- Complex feature documentation
- Integration specifications

Keep additional files focused and size-limited. If a file exceeds 200 lines, split it.

## Core Workflows

### Memory Bank Initialization

When user requests `initialize memory bank`, perform exhaustive project analysis:
- All source code files and their relationships
- Configuration files and build system setup
- Project structure and organization patterns
- Documentation and comments
- Dependencies and external integrations
- Testing frameworks and patterns

**Critical:** Be thorough during initialization, but write concise output. A high-quality initialization means comprehensive understanding expressed in minimal lines.

After initialization:
1. Check line counts against limits
2. Compress any files exceeding warning thresholds
3. Ask user to verify product description, technologies, and other information
4. Provide summary of project understanding
5. Encourage corrections - accuracy now improves all future interactions

### Memory Bank Update

Memory Bank updates occur when:
1. Discovering new project patterns
2. After implementing significant changes
3. When user explicitly requests with **update memory bank**
4. When context needs clarification

**Update Process:**

1. Run `(gc .kilocode/rules/memory-bank/*.md | measure -l).Lines` to check current sizes
2. Review ALL memory bank files (mandatory when triggered by "update memory bank")
3. Apply pruning rules (see below)
4. Compress any files exceeding warning thresholds
5. Then add new information
6. Verify each file matches its stated purpose
7. Final size check - no file should exceed max limit

**Pruning Rules - Actively Remove:**

| Remove This                         | Replace With                                                 |
| ----------------------------------- | ------------------------------------------------------------ |
| Completed task checklists           | Single summary line: "Phase X complete (models, API, tests)" |
| Resolved blockers                   | Delete entirely                                              |
| Historical decisions                | Delete unless actively relevant to current work              |
| Duplicate information               | Keep in owner file only, delete from others                  |
| Future plans beyond next phase      | Delete (re-add when that phase begins)                       |
| Detailed explanations in context.md | Move rationale to architecture.md or delete                  |
| Step-by-step completed work         | Collapse to outcome: "Auth system implemented"               |

**context.md Special Rule:** This file is effectively REPLACED on each update, not appended. It reflects current state only. Historical context belongs in git, not in the memory bank.

### Compression Workflow

When any file exceeds warning threshold:

1. Identify information that can be:
   - **Deleted:** Completed items, resolved blockers, outdated future plans
   - **Collapsed:** Multi-line details → single summary line
   - **Moved:** Information in wrong file → move to owner file
   - **Deduplicated:** Same info in multiple files → keep in owner only

2. Apply compression:
   ```
   # Before (bloated context.md)
   ✅ Implemented user registration
   ✅ Implemented login endpoint  
   ✅ Implemented logout endpoint
   ✅ Added JWT token handling
   ✅ Created auth middleware
   ✅ Added password reset flow
   
   # After (compressed)
   Auth system complete (registration, login, logout, JWT, password reset)
   ```

3. Verify file is under warning threshold
4. Proceed with update

### Add Task

When user completes a repetitive task and requests **add task** or **store this as a task**:

1. Create or update `tasks.md` in memory bank folder
2. Document concisely:
   - Task name and description
   - Files to modify (paths)
   - Step-by-step workflow
   - Important gotchas
3. Keep each task under 30 lines

Example task entry:
```markdown
## Add New Model Support
**Files:** `/src/models/`, `/src/api/`, `/tests/`
**Steps:**
1. Create model in models/ with fields and validation
2. Add serializer and viewset in api/
3. Register URL routes
4. Add admin interface
5. Write tests
**Notes:** Run migrations after model changes. Check foreign key constraints.
```

### Regular Task Execution

At start of EVERY task:
1. Read ALL memory bank files (not optional)
2. Check line counts - note any files needing compression
3. Briefly confirm understanding:

> "[Memory Bank: Active] Working on PROJECT. Currently in Phase 7: integration testing. Next: test asset CRUD operations."

When task matches documented task in `tasks.md`, mention this and follow the workflow.

At end of task:
1. Update context.md with new current state (replace, don't append)
2. If significant changes: suggest "Update memory bank to reflect these changes?"
3. Don't suggest updates for minor changes

### Context Window Management

When context window fills during extended session:
1. Suggest updating memory bank to preserve current state
2. Recommend starting fresh conversation
3. New conversation will load memory bank files automatically

## Important Notes

**After every memory reset, I begin completely fresh.** The Memory Bank is my only link to previous work. It must be maintained with precision AND brevity.

**If files are bloated, I am less effective.** Large files mean slower processing, lost focus, and missed details. Compression is not optional - it's essential for quality.

**If I detect inconsistencies** between memory bank files, I prioritize brief.md and note discrepancies to user.

**Size checks are mandatory** before and after updates. Run `(gc .kilocode/rules/memory-bank/*.md | measure -l).Lines` and address any files over warning thresholds.
