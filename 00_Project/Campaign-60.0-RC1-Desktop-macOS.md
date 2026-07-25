# Campaign 60.0: RC1 Desktop macOS

Status: PACKAGING RC1

## Objective

Package Pancanga Engine RC1 as a macOS desktop application without porting the
engine or changing any calendrical logic.

## Scope

The desktop package must preserve the existing certified core:

```text
Astronomy Engine
Calendar Engine
Observance Resolver
Knowledge Base normativa
Observance JSON content
Schemas
Parāṇa logic
```

## Packaging Strategy

Campaign 60.0 uses the current RC1 Rust backend and RC1 web interface as-is.

```text
Pancanga Engine.app
│
├── stay-open macOS launcher
├── embedded rc1_experience backend
├── embedded Observance Library JSON
├── local HTTP server
└── browser-launched RC1 interface
```

The user does not need Terminal or a manually typed localhost URL.

## Deliverables

```text
Deploy/macOS/build_rc1_macos_app.sh
Deploy/macOS/README.md
dist/macOS/Pancanga Engine.app
dist/macOS/Pancanga-Engine-RC1.dmg when hdiutil is available
```

## Current RC1 Behavior

```text
Double click:
PASS

Backend auto-start:
PASS

Offline engine/content:
PASS

Terminal required:
NO

Manual 127.0.0.1 entry:
NO

Backend shutdown on app exit:
PASS
```

## Verification

Local packaged app smoke test:

```text
App bundle:
PASS

Info.plist:
PASS

Embedded backend:
PASS

Embedded content JSON:
33 files

Backend auto-start:
PASS

Calculation API:
PASS

Content route:
PASS

Backend shutdown on macOS quit:
PASS

DMG:
PASS
```

## Known RC1 Limitation

This package opens the RC1 interface in the default browser. A later Tauri phase
should host the same interface inside a native WebKit window so the user never
sees the local URL at all.

## Product Naming

The project now distinguishes:

```text
Pancanga Engine
core/backend/calculation library

Pancanga for macOS
user-facing desktop product
```

## Next Campaigns

```text
60.1
Tauri shell with embedded backend and WebKit window

60.2
Desktop UX: icon, menu, about panel, preferences, theme

60.3
Signing, notarization, distributable DMG
```
