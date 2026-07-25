# Pancanga Engine RC1 Desktop macOS

Campaign: 60.0

Status: RC1 PACKAGING

## Objective

Package Pancanga Engine RC1 as a macOS application bundle without modifying the
astronomy engine, calendar engine, observance resolver, Knowledge Base, JSON
content, schemas, or Parāṇa logic.

## Current Packaging Strategy

This RC1 package embeds the existing Rust RC1 backend and the existing RC1
frontend/content. It does not port the engine.

```text
Pancanga Engine.app
│
├── Contents/MacOS/PancangaEngine
├── Contents/Resources/rc1_experience
└── Contents/Resources/content/ekadasi/*.json
```

When opened by double click, the app:

```text
starts the local backend
opens the RC1 interface automatically
keeps the server hidden from Terminal
stops the backend when the app quits
```

## Build

From the project root:

```bash
./Deploy/macOS/build_rc1_macos_app.sh
```

Output:

```text
dist/macOS/Pancanga Engine.app
dist/macOS/Pancanga-Engine-RC1.dmg   (when hdiutil is available)
```

## Scope Boundary

No changes are made to:

```text
Astronomy Engine
Calendar Engine
Observance Resolver
Knowledge Base normativa
Observance JSON content
Schema
Parāṇa logic
```

## Tauri Roadmap

This script creates a native macOS `.app` bundle for RC1 testing using a small
AppKit launcher that starts and stops the embedded backend. The next desktop
phase can replace the browser handoff with a Tauri/WebKit window while keeping
the same embedded backend and content layout.

```text
60.1
Tauri shell with embedded backend

60.2
Desktop UX: icon, menu, preferences, native window polish

60.3
Signing, notarization, distributable DMG
```
