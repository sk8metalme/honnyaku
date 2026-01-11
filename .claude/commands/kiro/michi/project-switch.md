---
name: /michi:project-switch
description: Switch between projects
---

# Project Switch Command

**Important**: Generate output in language specified in .kiro/project.json.

## Usage

```
/michi:project-switch <project_id>
```

**Parameters**:
- `project_id`: Project ID (e.g., customer-a-service-1, michi)

**Examples**:
```
/michi:project-switch michi
/michi:project-switch customer-a-service-1
```

## Execution Steps

1. Identify GitHub repository corresponding to project ID
2. Clone locally (if not cloned) or checkout
3. Load and display .kiro/project.json
4. Display corresponding Confluence project page URL

## Language Handling

- Read language from .kiro/project.json
- Generate all output in the specified language
- Default to English if language field is missing
