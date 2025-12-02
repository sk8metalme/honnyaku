---
name: /kiro:spec-tasks
description: Generate tasks.md from design specification in Michi workflow format
---

# Generate Tasks from Design Specification

**Important**: Generate output in the language specified in `.kiro/project.json`.

## Overview

This command generates a detailed task breakdown (`tasks.md`) from the design specification (`design.md`) following the **Michi Workflow Format**.

**Critical**: The generated tasks.md MUST follow the Michi Phase structure (Phase 0.1-5, A, B) with Story headers. DO NOT generate AI-DLC checkbox format (`- [ ] 1.`, `- [ ] 1.1`).

## Usage

```
/kiro:spec-tasks
```

## Input Files

1. `.kiro/specs/{{FEATURE_NAME}}/design.md` - Design specification
2. `.kiro/specs/{{FEATURE_NAME}}/requirements.md` - Requirements specification
3. `.kiro/project.json` - Project metadata

## Output File

`.kiro/specs/{{FEATURE_NAME}}/tasks.md`

## Required Format: Michi Workflow Structure

### Header Section

```markdown
# Task Breakdown: {{FEATURE_NAME}}

## Project Information

- **Feature Name**: {{FEATURE_NAME}}
- **Start Date**: YYYY-MM-DD (Day of week) Day 1
- **End Date**: YYYY-MM-DD (Day of week)
- **Total Business Days**: X business days (excluding weekends)
- **Total Effort Estimate**: X person-days
- **Holidays**: Excludes Saturdays and Sundays
- **Note**: This task breakdown is calculated in business days (excluding weekends and holidays)
```

### Phase Structure (Required)

The tasks.md MUST include these phases in order:

1. **Phase 0.1: Requirements**
2. **Phase 0.2: Design**
3. **Phase 0.3: Test Type Selection** (optional)
4. **Phase 0.4: Test Specification** (optional)
5. **Phase 1: Environment Setup** (optional)
6. **Phase 2: TDD Implementation** (required - main development phase)
7. **Phase A: PR Pre-merge Tests** (optional - CI/CD automated)
8. **Phase 3: Additional QA** (optional)
9. **Phase B: Release Preparation Tests** (optional)
10. **Phase 4: Release Preparation**
11. **Phase 5: Release**

### Phase Header Format

Each phase MUST use this header format:

```markdown
## Phase X.Y: Phase Name (Label)

**Period**: Day X-Y
**Effort**: X person-days
**Status**: Not Started
```

### Story Header Format

Each story within a phase MUST use this format:

```markdown
### Story X.Y.Z: Story Title

- **Assignee**: @Role
- **Effort**: X person-days
- **Description**: What this story accomplishes
- **Deliverables**:
  - Deliverable 1
  - Deliverable 2
- **Acceptance Criteria**:
  - [ ] Criterion 1
  - [ ] Criterion 2
- **Tasks**:
  - [ ] Task X.Y.Z.1: Task description
  - [ ] Task X.Y.Z.2: Task description
- **Dependencies**: Story X.Y.W (if applicable)
```

## Detailed Phase Guidelines

### Phase 0.1: Requirements

```markdown
## Phase 0.1: Requirements (Requirements)

**Period**: Day 1
**Effort**: 0.5 person-days
**Status**: Not Started

### Story 0.1.1: Requirements Document Creation

- **Assignee**: @PM
- **Effort**: 0.5 person-days
- **Description**: Create feature requirements document
- **Deliverables**:
  - `.kiro/specs/{{FEATURE_NAME}}/requirements.md`
  - Confluence page (Requirements)
- **Acceptance Criteria**:
  - [ ] Functional requirements defined
  - [ ] Non-functional requirements defined
  - [ ] Use cases documented
  - [ ] PM approved
```

### Phase 0.2: Design

```markdown
## Phase 0.2: Design (Design)

**Period**: Day 1-2
**Effort**: 0.5-1.0 person-days
**Status**: Not Started

### Story 0.2.1: Basic Design

- **Assignee**: @Architect
- **Effort**: 0.5-1.0 person-days
- **Description**: Architecture design, API design, class design
- **Deliverables**:
  - `.kiro/specs/{{FEATURE_NAME}}/design.md`
  - Confluence page (Design Document)
- **Acceptance Criteria**:
  - [ ] System architecture diagram created
  - [ ] Layer structure defined
  - [ ] API specifications detailed
  - [ ] Class design completed
  - [ ] Architect approved
```

### Phase 2: TDD Implementation (Main Development)

This is the most detailed phase. Break down into multiple stories based on the design.md:

```markdown
## Phase 2: TDD Implementation (Implementation)

**Period**: Day X-Y
**Effort**: X person-days
**Status**: Not Started

### Story 2.1: Project Setup

- **Assignee**: @Developer
- **Effort**: 0.5 person-days
- **Description**: Project setup and dependency configuration
- **Tasks**:
  - [ ] Task 2.1.1: Create project
  - [ ] Task 2.1.2: Add dependencies
  - [ ] Task 2.1.3: Create configuration files
  - [ ] Task 2.1.4: Create package structure
  - [ ] Task 2.1.5: Verify build
- **Deliverables**:
  - Project configuration files
  - Package structure
- **Acceptance Criteria**:
  - [ ] Project builds successfully
  - [ ] Application starts
  - [ ] Package structure matches design

### Story 2.2: [Feature Component] Implementation

- **Assignee**: @Developer
- **Effort**: X person-days
- **Description**: Implement [component] with TDD approach
- **Tasks**:
  - [ ] Task 2.2.1: Write failing tests (RED)
  - [ ] Task 2.2.2: Implement minimum code (GREEN)
  - [ ] Task 2.2.3: Refactor (REFACTOR)
  - [ ] Task 2.2.4: Integration tests
- **Acceptance Criteria**:
  - [ ] All unit tests pass
  - [ ] Code coverage >= 95%
  - [ ] Design patterns followed

(Continue with Story 2.3, 2.4, etc. based on design.md components)
```

### Phase 4: Release Preparation

```markdown
## Phase 4: Release Preparation (Release Preparation)

**Period**: Day Y-Z
**Effort**: X person-days
**Status**: Not Started

### Story 4.1: Production Environment Setup

- **Assignee**: @InfraEngineer
- **Effort**: X person-days
- **Description**: Production infrastructure setup and configuration
- **Acceptance Criteria**:
  - [ ] Server provisioned
  - [ ] Database configured
  - [ ] SSL certificate configured
  - [ ] Monitoring configured

### Story 4.2: Release Documentation

- **Assignee**: @PM
- **Effort**: X person-days
- **Description**: Release notes, manual, operation procedures
- **Acceptance Criteria**:
  - [ ] Release notes created
  - [ ] User manual created
  - [ ] Operation procedures created
```

### Phase 5: Release

```markdown
## Phase 5: Release (Release)

**Period**: Day Z
**Effort**: X person-days
**Status**: Not Started

### Story 5.1: Staging Deployment

- **Assignee**: @InfraEngineer
- **Effort**: X person-days
- **Description**: Deploy to staging and final verification
- **Acceptance Criteria**:
  - [ ] Staging deployment successful
  - [ ] Smoke tests pass
  - [ ] Final verification completed

### Story 5.2: Production Release

- **Assignee**: @InfraEngineer, @PM
- **Effort**: X person-days
- **Description**: Production release execution
- **Acceptance Criteria**:
  - [ ] Production deployment successful
  - [ ] Smoke tests pass
  - [ ] Monitoring verified
  - [ ] Stakeholders notified
```

## Timeline Section

Include a timeline at the end:

```markdown
## Timeline (Business Days)

**Assumption**: Calculated in business days (excluding weekends)

### Week 1: Specification Phase

Day 1 (Mon):

- Story 0.1.1: Requirements Document Creation
- Story 0.2.1: Basic Design (start)

Day 2-3 (Tue-Wed):

- Story 0.2.1: Basic Design (complete)
- Technical review and PM approval

### Week 2-N: Implementation Phase

(Continue based on project scope)
```

## Estimate Summary Section

```markdown
## Estimate Summary

| Phase                        | Story Points | Effort (days) | Stories |
| ---------------------------- | ------------ | ------------- | ------- |
| Phase 0.1: Requirements      | X SP         | X days        | X       |
| Phase 0.2: Design            | X SP         | X days        | X       |
| Phase 2: TDD Implementation  | X SP         | X days        | X       |
| Phase 4: Release Preparation | X SP         | X days        | X       |
| Phase 5: Release             | X SP         | X days        | X       |
| **Total**                    | **X SP**     | **X days**    | **X**   |

**Conversion Rate**: 1 SP ≈ 0.5 person-days
```

## JIRA Labels Section

```markdown
## JIRA Labels

Each story is automatically assigned the following labels:

- `requirements` - Phase 0.1: Requirements
- `design` - Phase 0.2: Design
- `test-type-selection` - Phase 0.3: Test Type Selection
- `test-spec` - Phase 0.4: Test Specification
- `environment-setup` - Phase 1: Environment Setup
- `implementation` / `tdd-implementation` - Phase 2: TDD Implementation
- `phase-a` / `pr-tests` - Phase A: PR Pre-merge Tests
- `additional-qa` / `testing` - Phase 3: Additional QA
- `phase-b` / `release-tests` - Phase B: Release Preparation Tests
- `release-prep` / `release-preparation` - Phase 4: Release Preparation
- `release` - Phase 5: Release
```

## Important Notes

1. **DO NOT** use AI-DLC checkbox format (`- [ ] 1.`, `- [ ] 1.1`)
2. **MUST** use Phase headers (`## Phase X.Y:`)
3. **MUST** use Story headers (`### Story X.Y.Z:`)
4. **MUST** include day numbering (Day 1, Day 2, etc.)
5. **MUST** include effort estimates in person-days
6. **MUST** include acceptance criteria with checkboxes
7. Break down Phase 2 stories based on the components in design.md
8. Ensure each Story has clear dependencies documented

## Execution Steps

1. Read `.kiro/specs/{{FEATURE_NAME}}/design.md`
2. Read `.kiro/specs/{{FEATURE_NAME}}/requirements.md`
3. Analyze design components and create Story breakdown
4. Generate `tasks.md` following the Michi Workflow Format above
5. Save to `.kiro/specs/{{FEATURE_NAME}}/tasks.md`

## Validation Checklist

Before finalizing, verify:

- [ ] Phase headers exist (Phase 0.1, 0.2, 2, 4, 5 at minimum)
- [ ] Story headers follow format (`### Story X.Y.Z:`)
- [ ] Day numbering is present
- [ ] Effort estimates are included
- [ ] Acceptance criteria use checkboxes `- [ ]`
- [ ] No AI-DLC format (`- [ ] 1.`, `- [ ] 1.1`) is used
- [ ] Timeline section is included
- [ ] Estimate summary table is included
