# RC1 Experience Deployment

Campaign: 46BETA.10

Status: READY FOR ZERO COST PUBLIC RC1 DEPLOYMENT

Current deploy attempt:

[Campaign 46BETA.10 - Public RC1 Deployment Report](Public-RC1-Deployment-Report.md)

## Purpose

Publish Pancanga Engine RC1 Experience for real testers using only free
infrastructure.

This deployment does not change:

```text
Motor
Knowledge Base
Architecture
Astronomy
Vaiṣṇava rules
Certified v1.0 scope
```

## Zero-Cost Architecture

```text
Frontend:
Cloudflare Pages Free

Backend:
Render Free

Fallback:
Railway Free

Last resort:
Fly.io Free, only if no paid resources are required
```

Recommended RC1 shape:

```text
Cloudflare Pages
↓
static RC1 frontend

Render Free
↓
rc1_experience backend
```

The frontend calls the backend through:

```text
window.RC1_API_BASE
```

## Frontend - Cloudflare Pages Free

Recommended public tester URL:

```text
https://pancanga-engine-rc1.pages.dev
```

Publish directory:

```text
08_Examples/RC1-Experience
```

Build command:

```text
none
```

To point the page at the backend, edit:

```text
08_Examples/RC1-Experience/rc1-config.js
```

Example:

```js
window.RC1_API_BASE = "https://pancanga-engine-rc1-api.onrender.com";
```

Deploy command:

```bash
wrangler pages deploy 08_Examples/RC1-Experience \
  --project-name pancanga-engine-rc1
```

## Backend - Render Free

Configuration files:

```text
Deploy/RC1/Dockerfile
Deploy/RC1/render.yaml
```

Render setup:

```text
Service type:
Web Service

Runtime:
Docker

Plan:
Free

Dockerfile path:
Deploy/RC1/Dockerfile
```

Required environment variables:

```text
RC1_EXPERIENCE_ALLOWED_ORIGIN=https://pancanga-engine-rc1.pages.dev
```

Expected backend URL:

```text
https://pancanga-engine-rc1-api.onrender.com
```

Render injects `PORT`; the Rust backend uses it automatically when
`RC1_EXPERIENCE_ADDRESS` is not set.

## Fallbacks

If Render Free is not sufficient for the RC1 test:

```text
1. Railway Free, if available for the account.
2. Fly.io Free, only if it can be used without paid resources.
```

Fly.io fallback files remain available:

```text
Deploy/RC1/fly.toml
```

Do not use paid resources for RC1.

## Local Testing

From `03_Source/rust`:

```bash
RC1_EXPERIENCE_ADDRESS=127.0.0.1:7878 \
cargo run --release -p pancanga-engine --example rc1_experience
```

Then open:

```text
http://127.0.0.1:7878
```

## Deployment Checklist

```text
□ Render Free web service created
□ Render backend deployed
□ Backend HTTPS URL opens
□ CORS allows Cloudflare Pages origin
□ Cloudflare Pages project created
□ Publish directory set to 08_Examples/RC1-Experience
□ rc1-config.js points to Render backend
□ Frontend HTTPS URL opens on desktop
□ Frontend HTTPS URL opens on mobile
□ Calculation succeeds from deployed frontend
□ Valencia smoke test PASS
□ Buenos Aires smoke test PASS
□ Reportar un problema produces a reproducible report
□ RC1 testing warning remains visible
□ Build metadata footer visible in screenshots
□ Cost remains zero
```

## Tester Warning

The RC1 screen includes:

```text
Pancanga Engine RC1 · Testing Release.
No usar como único calendario para observancias importantes hasta v1.0.
```

This warning must remain visible until v1.0.

## Build Metadata

The RC1 footer must remain visible:

```text
Pancanga Engine RC1 · Testing Release
Engine: v1.0 RC1
Knowledge Base: HBV v1.0
Astronomy: Swiss Certified
Build: 2026.07 RC1
```

The values are configured in:

```text
rc1-config.js
```

## Issue Reporting

The RC1 frontend includes a `Reportar un problema` button.

It generates a reproducible report containing:

```text
Ciudad
Fecha seleccionada
Estado visible
Resultado visible
Parāṇa recomendado
Límite normativo
Datos técnicos
```

For testing, testers can paste this report into:

```text
GitHub Issue
Google Form
Email
private tester channel
```

No third-party issue system is hardcoded into the beta.

## Release-Freeze Rule

RC1 deployment may improve:

```text
UX clarity
tester feedback
deployment reproducibility
certification evidence
```

It may not add:

```text
new rules
new APIs
new algorithms
new calendar scope
new certified behavior
paid infrastructure dependency
```

If a tester report suggests a new feature, document it for v2.0 unless it is a
confirmed v1.0 ENGINE BUG.
