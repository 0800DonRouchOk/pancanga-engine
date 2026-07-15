# Pancanga Engine RC1 Deploy

This folder contains deployment files for Campaign 46BETA.10.

Purpose:

```text
Publish the RC1 Experience using zero-cost infrastructure.
```

It does not modify:

```text
Pancanga Engine motor
Knowledge Base
Architecture
Certified v1.0 scope
```

It must not use:

```text
paid services
paid domains
paid compute
```

## Backend - Recommended

Platform:

```text
Render Free
```

Render service:

```text
Type:
Web Service

Runtime:
Docker

Plan:
Free

Dockerfile:
Deploy/RC1/Dockerfile
```

Environment:

```text
RC1_EXPERIENCE_ALLOWED_ORIGIN=https://pancanga-engine-rc1.pages.dev
```

Render injects `PORT`; the backend uses it automatically when
`RC1_EXPERIENCE_ADDRESS` is not set.

Expected backend URL:

```text
https://pancanga-engine-rc1-api.onrender.com
```

## Backend - Fallback

If Render Free is not sufficient:

```text
1. Railway Free, if available.
2. Fly.io Free, only if no paid resources are required.
```

Fly.io fallback config remains available:

```text
Deploy/RC1/fly.toml
```

## Frontend

Platform:

```text
Cloudflare Pages Free
```

Publish directory:

```text
08_Examples/RC1-Experience
```

Set the backend URL in:

```text
08_Examples/RC1-Experience/rc1-config.js
```

Example:

```js
window.RC1_API_BASE = "https://pancanga-engine-rc1-api.onrender.com";
```
