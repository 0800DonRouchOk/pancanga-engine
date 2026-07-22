# Observance Content Binding Certification

Campaign: 52.3

Mode: DIAGNOSTIC ONLY

Status: PASS

## Scope

This diagnostic verifies where the correspondence between the observance
resolved by Pancanga Engine and the historical content rendered by the RC1
Experience is established.

No code, rules, Knowledge Base, catalog data, UI, or architecture were modified.

## Test Case

```text
Location:
Buenos Aires, Argentina

Date:
2026-08-08

Local RC1 instance:
http://127.0.0.1:7892/
```

## Observance Resolver Output

The RC1 API returned the following observance identity:

```text
id:
EK-010

slug:
kamika

display_name:
Kāmikā Ekādaśī

observance_type:
ekadasi

masa:
Śrāvaṇa

paksha:
Kṛṣṇa

source:
masa_paksha

fasting_for:
None
```

The complete date flow also remained stable:

```text
input:
2026-08-08

processed:
2026-08-08

rendered:
8 de agosto de 2026
```

## Content Requested By RC1

The current RC1 frontend binds historical content through:

```text
data.observance_content.slug
```

For this case, the requested content path is:

```text
/content/ekadasi/kamika.json
```

The served content file returned:

```text
History ID:
kamika

History slug:
kamika

History title:
Kāmikā Ekādaśī
```

## Binding Mechanism

The current binding chain is:

```text
Pancanga Engine
↓
Observance Resolver
↓
observance_content.slug = kamika
↓
RC1 loadFestivalLibrary("kamika")
↓
/content/ekadasi/kamika.json
↓
History title: Kāmikā Ekādaśī
```

The RC1 does not bind this history by date, index, display name, or chronological
sequence in the current implementation. It uses the resolved `slug` returned by
the calculation API.

## Comparison

```text
Observance Resolver
↓
EK-010
kamika
Kāmikā Ekādaśī
Śrāvaṇa
Kṛṣṇa
source: masa_paksha

RC1 content request
↓
kamika.json

Content repository
↓
kamika
Kāmikā Ekādaśī
```

Result:

```text
PASS
```

## Stale Runtime Finding

During the diagnostic, older local RC1 instances were still listening on:

```text
http://127.0.0.1:7878/
http://127.0.0.1:7879/
```

These instances did not represent the current repository state.

Observed stale behavior:

```text
7878:
API response did not include observance_content.

7879:
HTML still contained a fixed content request:
fetch("./content/ekadasi/pavitropana.json")

7879:
/content/ekadasi/kamika.json returned 404.

7879:
/content/ekadasi/pavitropana.json returned 200.
```

This explains why an old browser session could show Putradā / Pavitrāropaṇī
content even when the current repository resolves the 2026-08-08 observance as
Kāmikā Ekādaśī.

Classification:

```text
No OBS-BIND-001 in current RC1.
No OBS-BIND-002 in current RC1.
No OBS-BIND-003 in current RC1.
No OBS-BIND-004 in current RC1.

Runtime issue:
STALE_LOCAL_INSTANCE
```

## Conclusion

For the current RC1 implementation served on port 7892, the content binding is
correct.

```text
Resolver:
PASS

Content request:
PASS

Content repository:
PASS

UI binding:
PASS

First point of inconsistency:
None in current RC1.

Observed mismatch source:
Stale local server instance.
```

## Certification Result

```text
Campaign 52.3

Observance Content Binding Certification:
PASS

Case:
Buenos Aires
2026-08-08

Resolved observance:
EK-010
Kāmikā Ekādaśī

Requested history:
kamika

Rendered content source:
kamika.json

Motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Catalog:
SIN CAMBIOS

UI:
SIN CAMBIOS
```
