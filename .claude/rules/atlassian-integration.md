---
title: Atlassian Integration
description: Integration rules for Confluence and JIRA via MCP
---

# Atlassian Integration

## Development Guidelines
- Think in English, but generate responses in Japanese (思考は英語、回答の生成は日本語で行うように)

## Language
All generated documents should be in: **ja**

## MCP Integration

### Confluence Sync
- Sync specifications from .kiro/specs/ to Confluence
- Use MCP server for Confluence operations
- Project labels: Check .kiro/project.json

### JIRA Sync
- Create Epic and Stories from .kiro/specs/{{FEATURE_NAME}}/tasks.md
- Use JIRA project key: {{PROJECT_ID}}
- Link Confluence pages automatically

## Project Metadata
- Project ID: {{PROJECT_ID}}
- Kiro directory: .kiro
- Agent directory: claude

## Workflow
1. Create spec in .kiro/specs/{{FEATURE_NAME}}/
2. Sync to Confluence via MCP
3. Create JIRA Epic/Stories
4. Link Confluence pages to JIRA
