# Prompt Templates

This directory contains reusable prompt templates for common LLM-assisted development tasks.

## Purpose

Prompt templates help maintain consistency in LLM interactions by providing standardized formats for common tasks.

## Template Structure

Each prompt template follows this structure:

```markdown
# [Task Type] - [Specific Task]

## Context
[Relevant background information and constraints]

## Requirements
- [Requirement 1]
- [Requirement 2]

## Input Format
[Description of expected input format]

## Output Format
[Description of expected output format]

## Examples
[Example inputs and outputs]

## Validation Criteria
[How to verify the output is correct]
```

## Available Templates

| Template             | Purpose                       |
| -------------------- | ----------------------------- |
| `code_generation.md` | General code generation tasks |
| `bug_fixing.md`      | Debugging and bug fixing      |
| `refactoring.md`     | Code refactoring tasks        |
| `documentation.md`   | Documentation generation      |
| `test_generation.md` | Test case generation          |

## Usage Guidelines

1. **Select the appropriate template** for your task
2. **Customize the context** with project-specific information
3. **Specify requirements** clearly and unambiguously
4. **Provide examples** when possible
5. **Validate output** against the criteria

## Best Practices

- Keep prompts focused on single tasks
- Provide complete context without overwhelming detail
- Use consistent terminology
- Specify coding standards and conventions
- Include error handling requirements
- Document expected behavior for edge cases

## Template Maintenance

- Review templates periodically for effectiveness
- Update templates based on lessons learned
- Add new templates for recurring task types
- Deprecate ineffective templates