---
title: Michi Core Principles
description: Core principles for GitHub SSoT and multi-project management
---

# Michi Core Principles

## Development Guidelines
- Think in English, but generate responses in Japanese (思考は英語、回答の生成は日本語で行うように)

## Language
All generated documents should be in: **ja**

Reference the language field in .kiro/project.json.

## Single Source of Truth (SSoT)

### GitHub as SSoT
- **All specifications are managed in GitHub** (.kiro/specs/)
- Confluence is **reference and approval only** (editing is GitHub only)
- Avoid duplicate management

### Data Flow
```
GitHub (.kiro/specs/)  ← Source of truth (editable)
    ↓ sync
Confluence ← Display and approval (read-only)
```

## Multi-Project Management

### Project Identification
- All operations reference .kiro/project.json
- Dynamically use project ID, JIRA key, Confluence labels
- Project ID: {{PROJECT_ID}}

### Naming Conventions

#### Confluence Pages
- Format: `[{projectName}] {document_type}`
- Example: `[{{PROJECT_ID}}] Requirements`

#### JIRA Epic/Story
- Format: `[{JIRA_KEY}] {title}`
- Use project metadata from .kiro/project.json

## Agent Directory
- Agent configuration: claude
- Rules location: claude/rules/
- Commands location: claude/commands/

## Feature Development
- Feature name: {{FEATURE_NAME}}
- Spec location: .kiro/specs/{{FEATURE_NAME}}/
