# Public RC1 Deployment Report

Campaign: 46BETA.10

Status: BLOCKED BY FREE ACCOUNT CREDENTIALS

## Objective

Publish Pancanga Engine RC1 Experience on the public internet for real testers
using only free infrastructure.

Target architecture:

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

This campaign does not modify:

```text
Motor
Knowledge Base
Architecture
Astronomy
Vaiṣṇava rules
Certified v1.0 scope
```

This campaign forbids:

```text
paid services
paid domains
paid compute
scope expansion
```

## Current Deployment Status

```text
Frontend:
NOT ONLINE

Backend:
NOT ONLINE

HTTPS:
PENDING

API:
PENDING

CORS:
CONFIGURED, NOT DEPLOYED

Desktop:
PENDING

Mobile:
PENDING

Cost:
€0 TARGET

RC1 Experience:
READY TO DEPLOY
```

## Files Verified

```text
Deploy/RC1/Dockerfile
Deploy/RC1/render.yaml
Deploy/RC1/fly.toml
Deploy/RC1/README.md
08_Examples/RC1-Experience/DEPLOYMENT.md
08_Examples/RC1-Experience/README.md
08_Examples/RC1-Experience/index.html
08_Examples/RC1-Experience/rc1-config.js
08_Examples/RC1-Experience/_headers
```

## Tool Availability

Detected locally:

```text
npm:
AVAILABLE

npx:
AVAILABLE

brew:
AVAILABLE
```

Not detected locally:

```text
wrangler:
MISSING
```

## Credentials Required

To complete deployment, the user must authenticate:

```text
Cloudflare account:
REQUIRED

Render account:
REQUIRED
```

No deploy was attempted because the environment does not contain authenticated
Cloudflare or Render credentials.

## Required User Commands

### 1. Install Cloudflare Wrangler

```bash
npm install -g wrangler
```

or use `npx` without global install:

```bash
npx wrangler --version
```

Login:

```bash
wrangler login
```

### 2. Create Render Free Backend

Create a Render Web Service from this repository.

Use:

```text
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

Render injects `PORT`; the backend uses it automatically when
`RC1_EXPERIENCE_ADDRESS` is not set.

Expected backend URL:

```text
https://pancanga-engine-rc1-api.onrender.com
```

## Frontend Deploy Steps

Edit:

```text
08_Examples/RC1-Experience/rc1-config.js
```

Set:

```js
window.RC1_API_BASE = "https://pancanga-engine-rc1-api.onrender.com";
```

Deploy to Cloudflare Pages:

```bash
wrangler pages deploy 08_Examples/RC1-Experience \
  --project-name pancanga-engine-rc1
```

Expected frontend URL:

```text
https://pancanga-engine-rc1.pages.dev
```

## Fallbacks

If Render Free is not sufficient:

```text
1. Railway Free, if available.
2. Fly.io Free, only if no paid resources are required.
```

Do not use a paid service for RC1.

## Post-Deploy Verification Checklist

```text
□ Frontend URL opens over HTTPS
□ Backend URL opens over HTTPS
□ Frontend calls /api/calculate successfully
□ CORS allows frontend origin
□ Desktop calculation works
□ Mobile calculation works
□ Valencia smoke test PASS
□ Buenos Aires smoke test PASS
□ Observance is displayed
□ Parāṇa recomendado is displayed
□ Límite normativo is displayed
□ Fundamento panel works
□ Astronomía panel works
□ Certificación panel works
□ Reportar un problema generates a reproducible report
□ RC1 Testing Release warning is visible
□ Build metadata appears in footer and screenshots
□ Cost remains €0
```

## Expected Final Report Format

When deployed, update this document with:

```text
Frontend URL:
https://pancanga-engine-rc1.pages.dev

Backend URL:
https://pancanga-engine-rc1-api.onrender.com

Deploy date:
YYYY-MM-DD

Engine:
v1.0 RC1

Knowledge Base:
HBV v1.0

Astronomy:
Swiss Certified

Build:
2026.07 RC1

Cost:
€0

Status:
ONLINE
```

## Current Campaign Result

```text
Campaign 46BETA.10

Frontend:
PENDING

Backend:
PENDING

HTTPS:
PENDING

API:
PENDING

Smoke Test:
PENDING

Desktop:
PENDING

Mobile:
PENDING

RC1 Experience:
READY TO DEPLOY

Cost:
€0 TARGET

Deploy:
BLOCKED BY FREE ACCOUNT CREDENTIALS
```
