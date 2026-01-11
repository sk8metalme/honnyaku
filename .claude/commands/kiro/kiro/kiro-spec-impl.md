# kiro:spec-impl - Spec Implementation Command

## Overview

This command implements the specifications defined in `tasks.md` following TDD (Test-Driven Development) practices with automatic JIRA integration.

**Key Features:**

- Automatic JIRA info detection from `spec.json`
- Updates both Epic AND first Story status
- Creates PR and adds JIRA comments automatically

## Arguments

- `$ARGUMENTS`: Feature name (e.g., "user-auth")

**Note:** JIRA ticket keys are automatically detected from `.kiro/specs/{feature}/spec.json`. No need to specify them manually.

## Pre-requisites

1. `.kiro/specs/{feature}/tasks.md` must exist
2. `.kiro/specs/{feature}/spec.json` must contain JIRA info (run `michi jira:sync` first)
3. Environment variables must be configured:
   - `ATLASSIAN_URL`: JIRA instance URL
   - `ATLASSIAN_EMAIL`: JIRA user email
   - `ATLASSIAN_API_TOKEN`: JIRA API token
   - `GITHUB_TOKEN`: GitHub API token
   - `GITHUB_REPO`: GitHub repository (owner/repo format)

## Execution Flow

### 1. Parse Arguments

```
Input: $ARGUMENTS
Expected format: "<feature-name>"
Example: "user-auth"
```

Extract:

- `FEATURE_NAME`: Feature name from arguments

### 2. JIRA Info Detection (Automatic)

Read JIRA information from `.kiro/specs/$FEATURE_NAME/spec.json`:

```json
{
  "jira": {
    "epicKey": "PROJ-123",
    "storyKeys": ["PROJ-124", "PROJ-125"],
    "epicUrl": "https://..."
  }
}
```

**If JIRA info is missing:**

- Ask user: "JIRA info not found. Skip JIRA integration? (y/n)"
- If yes: Continue without JIRA integration
- If no: Stop and suggest running `michi jira:sync $FEATURE_NAME` first

### 3. Start Processing (JIRA Status Update)

**Automatically move Epic AND first Story to "In Progress":**

This is handled internally. The workflow will:

1. Transition Epic to "In Progress"
2. Transition first Story to "In Progress"

### 4. Implementation Phase (TDD)

Read the tasks from `.kiro/specs/$FEATURE_NAME/tasks.md` and implement each Story following TDD:

For each Story in tasks.md:

1. **Read Story Requirements**
   - Parse the Story section from tasks.md
   - Identify acceptance criteria and subtasks

2. **Write Tests First (Red)**
   - Create test files based on acceptance criteria
   - Run tests to verify they fail

3. **Implement Code (Green)**
   - Write minimal code to pass tests
   - Follow the existing code patterns in the project

4. **Refactor (Blue)**
   - Clean up code while keeping tests green
   - Apply project coding standards

5. **Commit Changes**
   - Create atomic commits for each Story
   - Use conventional commit format: `feat($FEATURE_NAME): Story title`

### 5. Code Review Phase (Automatic)

**Automatically review the implemented code:**

For each Story implementation:

1. **Design Review (/michi:design-review)** - **Recommended for UI Components**
   - **推奨**: UI/フロントエンドコードを実装した場合、`/michi:design-review` コマンドの使用を検討してください
   - デザイン品質のレビュー（アクセシビリティ、レスポンシブ、UX）
   - CSS、React、Vue、HTML、Tailwind の変更時に実行

   ```bash
   /michi:design-review
   ```

2. **Code Review (/review)**
   - Execute `/review` command automatically
   - Analyze the implementation for code quality issues
   - Check for best practices and potential bugs

3. **Fix Issues if Found**
   - If review identifies issues, automatically fix them
   - Commit the fixes
   - Re-run `/review` to verify fixes
   - Repeat until `/review` passes without critical issues

4. **Security Review (/security-review)**
   - Execute `/security-review` command automatically
   - Scan for security vulnerabilities
   - Check for common security issues (SQL injection, XSS, etc.)

5. **Fix Security Issues if Found**
   - If security issues are identified, automatically fix them
   - Commit the security fixes
   - Re-run `/security-review` to verify fixes
   - Repeat until `/security-review` passes without critical issues

**Review Loop:**
- Maximum iterations: 3 per review type
- If issues persist after 3 iterations, report to user and ask for manual intervention

### 6. PR Creation Confirmation

**Ask user before creating PR:**

After all reviews pass successfully, ask the user:

```
✅ All reviews completed successfully!
   - Code Review: PASSED
   - Security Review: PASSED

Would you like to create a Pull Request now? (y/n)
```

- **If yes**: Proceed to End Processing (Step 7)
- **If no**: Stop workflow, save current state
  - User can manually create PR later using:
    - `jj git push --bookmark <branch-name>`
    - `gh pr create --head <branch-name> --base main`

### 7. End Processing (PR Creation & JIRA Update)

**Automatically complete the workflow:**

This is handled internally. The workflow will:

1. Push the branch to remote
2. Create a Pull Request on GitHub
3. Move Epic AND first Story to "Ready for Review"
4. Add PR link as a comment on the Epic

## Usage Examples

```
/kiro:spec-impl user-auth
```

```
/kiro:spec-impl payment-gateway
```

## Workflow Comparison

### Before (3 steps with manual JIRA keys)

```bash
michi spec-impl:start user-auth PROJ-123
/kiro:spec-impl user-auth PROJ-123
michi spec-impl:complete user-auth PROJ-123
```

### After (1 command with auto-detection)

```
/kiro:spec-impl user-auth
# JIRA info auto-detected from spec.json
# Epic + first Story -> "In Progress"
# TDD implementation
# Automatic code review (/review)
#   - Fix issues if found
#   - Re-review until passed
# Automatic security review (/security-review)
#   - Fix security issues if found
#   - Re-review until passed
# Ask user: "Create PR now? (y/n)"
#   - If yes:
#     - PR creation
#     - Epic + first Story -> "Ready for Review"
#     - PR link commented on JIRA
#   - If no:
#     - Save state, exit workflow
```

## JIRA Status Mapping

Default status names (customizable in `.michi/config.json`):

| Phase | Default Status   | Targets            |
| ----- | ---------------- | ------------------ |
| Start | In Progress      | Epic + First Story |
| End   | Ready for Review | Epic + First Story |

To customize, add to `.michi/config.json`:

```json
{
  "jira": {
    "statusMapping": {
      "inProgress": "In Progress",
      "readyForReview": "Ready for Review"
    }
  }
}
```

## Error Handling

- **JIRA info not found**: Interactive prompt to skip JIRA integration
- **JIRA transition fails**: Check available transitions for the current status
- **Review fails (max iterations exceeded)**: Report issues to user and ask for manual intervention
- **Security review fails (max iterations exceeded)**: Report vulnerabilities to user and ask for manual intervention
- **User declines PR creation**: Save current state, workflow stops (user can create PR manually later)
- **PR creation fails**: Ensure the branch is pushed to remote
- **JIRA comment fails**: The PR URL is still returned for manual update

## Notes

- JIRA info is automatically detected from `spec.json` (populated by `michi jira:sync`)
- Both Epic AND first Story are updated together
- The branch name defaults to `feature/{feature-name}`
- Individual JIRA operations can still be done via `michi jira:transition` and `michi jira:comment`
- Code reviews (`/review` and `/security-review`) are executed automatically after implementation
- Review loop has a maximum of 3 iterations per review type to prevent infinite loops
- User can decline PR creation and manually create it later if needed
