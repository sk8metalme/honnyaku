# Project Structure

## Organization Philosophy

Tauri v2の標準構成に従い、フロントエンド（`src/`）とバックエンド（`src-tauri/`）を明確に分離。フロントエンドはfeature-firstアプローチで、機能ごとにコンポーネント・hooks・libを整理。

## Directory Patterns

### Frontend Source
**Location**: `/src/`
**Purpose**: React/TypeScriptフロントエンドコード
**Example**: `src/components/TranslationPopup.tsx`

### Frontend Components
**Location**: `/src/components/`
**Purpose**: 再利用可能なReactコンポーネント
**Example**: `TranslationPopup.tsx`, `SettingsPanel.tsx`

### Frontend Hooks
**Location**: `/src/hooks/`
**Purpose**: カスタムReact Hooks（ビジネスロジック）
**Example**: `useTranslation.ts`, `useClipboard.ts`

### Frontend Libraries
**Location**: `/src/lib/`
**Purpose**: APIクライアント、ユーティリティ関数
**Example**: `ollama.ts`, `claude.ts`, `language-detect.ts`

### Backend Source
**Location**: `/src-tauri/src/`
**Purpose**: Rustバックエンドコード
**Example**: `src-tauri/src/commands/translate.rs`

### Backend Commands
**Location**: `/src-tauri/src/commands/`
**Purpose**: Tauri IPCコマンド（フロントエンドから呼び出し可能）
**Example**: `clipboard.rs`, `translate.rs`

### Backend LLM Clients
**Location**: `/src-tauri/src/llm/`
**Purpose**: LLM API クライアント実装
**Example**: `ollama.rs`, `claude.rs`

### Tauri Configuration
**Location**: `/src-tauri/`
**Purpose**: Tauri設定ファイル
**Example**: `tauri.conf.json`, `capabilities/default.json`

## Naming Conventions

- **Files (Frontend)**: PascalCase（コンポーネント）, camelCase（hooks, lib）
- **Files (Backend)**: snake_case
- **Components**: PascalCase（`TranslationPopup`）
- **Functions**: camelCase（Frontend）, snake_case（Backend）
- **Types/Interfaces**: PascalCase

## Import Organization

```typescript
// 1. External packages
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

// 2. Internal absolute imports
import { TranslationPopup } from '@/components/TranslationPopup';
import { useTranslation } from '@/hooks/useTranslation';

// 3. Relative imports
import { formatResult } from './utils';
```

**Path Aliases**:
- `@/`: Maps to `/src/`

## Code Organization Principles

1. **Single Responsibility**: 各ファイルは1つの機能・コンポーネントに責任を持つ
2. **Separation of Concerns**: UI（components）、ロジック（hooks）、外部通信（lib）を分離
3. **Tauri IPC境界**: フロントエンドはシステム操作をRust経由でのみ実行
4. **依存方向**: `components` → `hooks` → `lib` → Tauri IPC

---
_Document patterns, not file trees. New files following patterns shouldn't require updates_
