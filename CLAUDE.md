# AI-DLC and Spec-Driven Development

Kiro-style Spec Driven Development implementation on AI-DLC (AI Development Life Cycle)

## Project Context

### Paths
- Steering: `.kiro/steering/`
- Specs: `.kiro/specs/`

### Steering vs Specification

**Steering** (`.kiro/steering/`) - Guide AI with project-wide rules and context
**Specs** (`.kiro/specs/`) - Formalize development process for individual features

### Active Specifications
- Check `.kiro/specs/` for active specifications
- Use `/kiro:spec-status [feature-name]` to check progress

## Development Guidelines
- Think in English, generate responses in Japanese. All Markdown content written to project files (e.g., requirements.md, design.md, tasks.md, research.md, validation reports) MUST be written in the target language configured for this specification (see spec.json.language).

## Minimal Workflow
- Phase 0 (optional): `/kiro:steering`, `/kiro:steering-custom`
- Phase 1 (Specification):
  - `/kiro:spec-init "description"`
  - `/kiro:spec-requirements {feature}`
  - `/kiro:validate-gap {feature}` (optional: for existing codebase)
  - `/kiro:spec-design {feature} [-y]`
  - `/kiro:validate-design {feature}` (optional: design review)
  - `/kiro:spec-tasks {feature} [-y]`
- Phase 2 (Implementation): `/kiro:spec-impl {feature} [tasks]`
  - `/kiro:validate-impl {feature}` (optional: after implementation)
- Progress check: `/kiro:spec-status {feature}` (use anytime)

## Development Rules
- 3-phase approval workflow: Requirements → Design → Tasks → Implementation
- Human review required each phase; use `-y` only for intentional fast-track
- Keep steering current and verify alignment with `/kiro:spec-status`
- Follow the user's instructions precisely, and within that scope act autonomously: gather the necessary context and complete the requested work end-to-end in this run, asking questions only when essential information is missing or the instructions are critically ambiguous.

## Steering Configuration
- Load entire `.kiro/steering/` as project memory
- Default files: `product.md`, `tech.md`, `structure.md`
- Custom files are supported (managed via `/kiro:steering-custom`)

## Release Process

### Prerequisites
- All changes must be merged to `main` branch via PR
- Never commit directly to `main` branch
- Use feature branches: `fix/*`, `bugfix/*`, `feature/*`

### Version Update Checklist
When creating a new release, update ALL of the following files:

1. **`package.json`** - Frontend version
2. **`src-tauri/Cargo.toml`** - Backend version
3. **`src-tauri/tauri.conf.json`** - ⚠️ CRITICAL: Tauri config version (used for DMG filename)
4. **`CHANGELOG.md`** - Add release notes
5. **`src-tauri/Cargo.lock`** - Update with `cd src-tauri && cargo update -p honnyaku`

### Release Steps

#### 1. Create Feature Branch
```bash
git checkout -b fix/your-fix-name
# Make changes and commit
git add .
git commit -m "fix: description"
git push -u origin fix/your-fix-name
```

#### 2. Create and Merge PR
```bash
gh pr create --title "fix: description" --body "PR description"
gh pr merge <pr-number> --squash --delete-branch
```

#### 3. Update Versions on Main
```bash
git checkout main
git pull

# Update all version files (see checklist above)
# Example for version 0.5.3:
# - package.json: "version": "0.5.3"
# - src-tauri/Cargo.toml: version = "0.5.3"
# - src-tauri/tauri.conf.json: "version": "0.5.3"
# - CHANGELOG.md: Add ## [0.5.3] section

cd src-tauri && cargo update -p honnyaku && cd ..

git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json src-tauri/Cargo.lock CHANGELOG.md
git commit -m "docs: update CHANGELOG and version to vX.Y.Z"
git push origin main
```

#### 4. Create Tag and Release
```bash
# Create tag
git tag -a vX.Y.Z -m "Release vX.Y.Z

## Changes
- Change 1
- Change 2
"
git push origin vX.Y.Z

# Create GitHub Release (GitHub Actions will build automatically)
gh release create vX.Y.Z \
  --title "vX.Y.Z" \
  --notes "Release notes here" \
  --verify-tag
```

#### 5. Verify Build
- Check GitHub Actions: https://github.com/sk8metalme/honnyaku/actions
- Verify DMG filename is `honnyaku_X.Y.Z_aarch64.dmg` (not old version)
- Check release page: https://github.com/sk8metalme/honnyaku/releases

#### 6. Update Release Notes with Installation Instructions
After the release is created, ensure the release notes on https://github.com/sk8metalme/honnyaku/releases include the following installation instructions:

```markdown
## インストール手順

### 1. DMGファイルをダウンロード
- `honnyaku_X.Y.Z_aarch64.dmg` をダウンロード

### 2. アプリケーションフォルダに移動
1. DMGファイルをダブルクリック
2. honnyaku.appをアプリケーションフォルダにドラッグ＆ドロップ

### 3. Gatekeeperの隔離属性を削除（重要）
macOSのセキュリティ機能により、ダウンロードしたアプリは隔離されます。以下のコマンドで隔離属性を削除してください：

\`\`\`bash
xattr -d com.apple.quarantine /Applications/honnyaku.app
\`\`\`

このコマンドを実行しないと、アプリケーションが正しく起動しない場合があります。

### 4. アプリケーションを起動
アプリケーションフォルダから honnyaku を起動してください。
```

### Troubleshooting

#### Problem: DMG filename shows old version
**Cause**: `src-tauri/tauri.conf.json` version was not updated

**Solution**:
1. Update `tauri.conf.json` version
2. Commit and push
3. Delete tag and release: `gh release delete vX.Y.Z --yes && git tag -d vX.Y.Z && git push origin :refs/tags/vX.Y.Z`
4. Recreate tag and release

#### Problem: Build fails with version mismatch
**Cause**: `Cargo.lock` not updated

**Solution**:
```bash
cd src-tauri
cargo update -p honnyaku
cd ..
git add src-tauri/Cargo.lock
git commit -m "chore: update Cargo.lock"
git push origin main
# Recreate tag
```

#### Problem: Committed to main directly
**Cause**: Forgot to create feature branch

**Solution**:
```bash
# Reset last commit (keep changes staged)
git reset HEAD~1 --soft

# Create feature branch
git checkout -b fix/your-fix-name

# Re-commit
git commit -m "fix: description"
git push -u origin fix/your-fix-name

# Create PR
gh pr create --title "fix: description" --body "Description"
```

### Automated Verification
The release workflow includes automatic version verification:
- Verifies `Cargo.toml` version matches tag
- Verifies `package.json` version matches tag
- Check workflow logs if build produces wrong version
