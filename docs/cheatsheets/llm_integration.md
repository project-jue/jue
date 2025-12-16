# LLM Integration Cheat Sheet

## LLM Integration Overview

### Integration Principles
- **Context Management**: Provide complete, relevant context
- **Scope Control**: Limit change scope explicitly
- **Validation**: Verify all LLM output immediately
- **Pattern Consistency**: Maintain canonical examples

### Integration Workflow
1. **Context Preparation**: Gather all relevant files and information
2. **Prompt Formulation**: Create clear, specific instructions
3. **Execution**: Run LLM with prepared context
4. **Validation**: Verify output against requirements
5. **Integration**: Apply validated changes

## Context Packaging

### Effective Context Structure
```markdown
## Task Description
Clear, concise explanation of what needs to be done

## Relevant Files
- `path/to/file1.rs` - Description of file
- `path/to/file2.rs` - Description of file

## Key Requirements
- Requirement 1
- Requirement 2
- Constraint 1

## Examples
```rust
// Canonical example showing desired pattern
```

## Current State
Description of current implementation status
```

### Context Size Management
- Keep individual prompts under 4000 tokens
- Break large tasks into smaller subtasks
- Use file references instead of full content when possible
- Prioritize most relevant information

## Prompt Engineering

### Prompt Structure
```markdown
# Task
[Clear, specific task description]

# Requirements
- [Requirement 1]
- [Requirement 2]

# Constraints
- [Constraint 1]
- [Constraint 2]

# Examples
[Relevant code examples]

# Files to Modify
- `path/to/file.rs` - [Description of changes needed]
```

### Prompt Best Practices
- Be explicit about what should change vs. stay the same
- Specify coding standards and conventions
- Include error handling requirements
- Provide test case expectations
- Use consistent terminology

## Safety Checks

### Pre-Execution Checks
- Verify LLM understands the task
- Confirm context is complete and accurate
- Check for ambiguous requirements
- Validate example correctness

### Post-Execution Checks
- Compile the generated code
- Run linter and formatter
- Execute unit tests
- Verify integration tests
- Check for regressions

## Pattern Management

### Canonical Examples
```rust
// Error handling pattern
fn safe_operation() -> Result<(), Error> {
    // Operation implementation
    Ok(())
}

// Logging pattern
fn log_operation() {
    log::debug!("Operation started");
    // Operation
    log::info!("Operation completed");
}

// State transition pattern
fn transition_state(current: State) -> State {
    match current {
        State::Initial => State::Processing,
        State::Processing => State::Complete,
        State::Complete => State::Complete,
    }
}
```

### Pattern Documentation
- Store patterns in `docs/patterns/`
- Include usage examples
- Document when to use each pattern
- Show anti-patterns to avoid

## LLM-Specific Considerations

### Token Management
- Monitor token usage in prompts and responses
- Use compression for repetitive content
- Reference external documentation when possible
- Break complex tasks into smaller steps

### Response Validation
- Check for hallucinations and inconsistencies
- Verify all references are correct
- Confirm coding style compliance
- Validate logical correctness

## Integration Tools

### Common Integration Methods
```bash
# CLI integration
llm-cli --prompt "prompt_file.md" --context "context.json"

# API integration
curl -X POST https://api.llm.com/v1/completions \
  -H "Content-Type: application/json" \
  -d '{"prompt": "prompt text", "context": "context data"}'

# Local integration
python integrate_llm.py --task "task_description" --files "file_list.txt"
```

### Tool Configuration
```json
{
  "model": "gpt-4",
  "temperature": 0.2,
  "max_tokens": 2000,
  "top_p": 1.0,
  "frequency_penalty": 0.0,
  "presence_penalty": 0.0,
  "stop_sequences": ["\n\n", "##"]
}
```

## Debugging LLM Integration

### Common Issues and Solutions

| Issue                   | Solution                                        |
| ----------------------- | ----------------------------------------------- |
| Hallucinated references | Provide complete context, verify all references |
| Inconsistent formatting | Specify exact formatting requirements           |
| Scope creep             | Explicitly limit change scope                   |
| Performance issues      | Optimize prompt structure, reduce token count   |
| Safety violations       | Implement strict validation checks              |

### Debugging Techniques
```rust
// Add validation hooks
fn validate_llm_output(output: &str) -> Result<(), ValidationError> {
    // Check for required patterns
    // Verify no unsafe operations
    // Confirm style compliance
    Ok(())
}

// Add logging for LLM interactions
fn log_llm_interaction(prompt: &str, response: &str) {
    log::debug!("LLM Prompt: {}", prompt);
    log::debug!("LLM Response: {}", response);
}
```

## Best Practices

### Integration Guidelines
- Start with small, well-defined tasks
- Build comprehensive test suites
- Implement rollback mechanisms
- Monitor LLM performance over time
- Document integration patterns

### Maintenance
- Regularly update canonical examples
- Review and refine prompts
- Monitor for drift in LLM behavior
- Update safety checks as needed
- Document lessons learned

### Security
- Sanitize all LLM inputs and outputs
- Implement rate limiting
- Monitor for prompt injection
- Validate all generated code
- Use sandboxed execution when possible