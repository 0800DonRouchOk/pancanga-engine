# Campaign 46BETA - Pancanga Engine RC1 Experience

Priority: HIGH.

Status: active.

## Objective

Use Pancanga Engine as an end-user experience before release.

The goal is not to prove the doctrine, astronomy, or engine again. Those layers
are already frozen or certified. The goal is to build trust through use.

```text
Would the maintainer trust tomorrow's Ekādaśī observance to this application?
```

## Scope

Allowed:

```text
minimal private application
single-screen guidance result
UX notes
explanation clarity
missing-information notes
comparison display against certification fixtures
release-blocking usability issues
first-screen screenshot
```

Forbidden:

```text
modifying the astronomy engine
modifying the calendar engine
modifying the Vaiṣṇava engine rules
modifying the Knowledge Base
changing doctrinal interpretations
adding new v1.0 scope
```

## First RC1 Experience Surface

A minimal application is required.

It does not need final art direction, production polish, or public release
quality. It must be usable enough that the maintainer can consult it as a real
daily tool.

First screen:

```text
Pancanga Engine RC1 Experience

Today

Tuesday, July 14, 2026

Valencia

✓ Today Ekādaśī is observed

Parāṇa
09:52-13:18

Why?

Astronomy

Certification
```

The first screen must answer the three practical questions in less than ten
seconds:

```text
1. Do I fast today?
2. When is Parāṇa?
3. Why did the engine decide this?
```

Minimum result display:

```text
Today
✓ Observe Ekādaśī

or

Today
✓ Observe Mahādvādaśī

or

Today
No Ekādaśī fast today

Parāṇa
2026-07-15
09:48-13:11

Why?

Certification
```

## Explanation View

The "Why?" view is essential. It is the soul of the product and must expose why
the result was chosen.

Example:

```text
Why is this Ekādaśī?

Ekādaśī was present at sunrise.
↓
HBV 12.1 / HBV-EK-001

Daśamī was not present during Aruṇodaya.
↓
HBV 12.315 / HBV-EK-002

No Mahādvādaśī condition was present.
↓
HBV-EK-004

Therefore the observance remains today.
```

## Astronomy View

The "Astronomy" view may expose the technical facts after the primary answer is
clear.

Minimum content:

```text
Sun
Moon
Elongation
Tithi
Nakṣatra
Aruṇodaya
Sunrise
```

## Certification View

The "Certification" view shows whether Pancanga Engine agrees with available
certification fixtures and why any difference exists.

```text
Pancanga Engine
↓
PureBhakti
↓
PASS
```

or:

```text
Pancanga Engine
↓
SCS Math
↓
DIFFERENCE
↓
CONFIGURATION_DIFFERENCE
```

Minimum certification summary:

```text
Swiss Ephemeris
✓

PureBhakti
✓

SCS Math
Configuration different

ENGINE BUG
0
```

## Experience Log

During the RC1 Experience, record:

```text
date tested
location
question asked
engine result
was the result understandable?
did it answer "Do I fast?"
did it answer "When is Parāṇa?"
did it answer "Why?"
did it feel like guidance rather than a calculator?
was any information missing?
was trust reduced by UX ambiguity?
was a screenshot captured?
action required
```

## Exit Criterion

The RC1 Experience closes when the maintainer can state:

```text
I have used Pancanga Engine for my own Ekādaśī decisions and no longer need to
open another calendar for ordinary use.
```

and all release-blocking UX issues are either:

```text
fixed
or
documented as post-v1.0 work
```

## Relationship To Release

Campaign 46BETA happens after external certification evidence is in place and
before Campaign 46R.

```text
RC1
↓
Pancanga Engine RC1 Experience
↓
Release Audit
↓
v1.0
```

The RC1 Experience may improve presentation, reports, and local tooling. It
must not change the certified engine semantics.

## Ingeniería Con Trazabilidad

Durante el desarrollo de Pancanga Engine, cada diferencia detectada se analiza
hasta identificar su causa exacta.

En una de las últimas campañas de validación de la experiencia privada
(`Campaign 46BETA.6`), una discrepancia visual permitió descubrir un
comportamiento del frontend: cuando el cálculo de Parāṇa no podía completarse,
la interfaz mantenía en pantalla el resultado anterior, generando la impresión
de que el motor había calculado una fecha incorrecta.

La investigación mostró que el motor nunca había producido un resultado
erróneo.

La fecha seleccionada, recibida, procesada y renderizada coincidía:

```text
Fecha seleccionada: 2027-02-17
Fecha recibida:    2027-02-17
Fecha procesada:   2027-02-17
Fecha renderizada: 17 de febrero de 2027
```

La solución consistió en corregir únicamente la experiencia de usuario,
limpiando la pantalla antes de cada nuevo cálculo para evitar que permanecieran
visibles resultados anteriores.

Del mismo modo, se mejoró el comportamiento de la interfaz para los casos de
**Dvādaśī corta**, evitando que el sistema intentara construir artificialmente
una ventana práctica inexistente y mostrando explícitamente que se trata de un
caso de contingencia.

Otro refinamiento surgió al analizar el caso **Triṣpṛśā**. El motor ya disponía
de la lógica doctrinal correspondiente (`HBV-EK-004`), pero la interfaz aún no
derivaba automáticamente todos los hechos relacionados con Mahādvādaśī. En
lugar de presentar una afirmación incorrecta, la experiencia fue ajustada para
reflejar fielmente el estado de la información disponible.

Lo importante es que ninguna de estas mejoras requirió modificar el motor de
cálculo ni la base doctrinal. Fueron correcciones de presentación que refuerzan
uno de los principios fundamentales del proyecto: cada resultado debe ser fiel
tanto a las fuentes como a la información que realmente puede demostrarse.

> **Campaign 46BETA.6**
>
> **Motor:** sin cambios  
> **Knowledge Base:** sin cambios  
> **Arquitectura:** sin cambios
>
> **Validación:** `cargo fmt`, `cargo clippy`, `cargo test`
>
> **Resultado:** PASS

## Historical Screenshot

When the first screen correctly answers a real observance question, save a
screenshot as part of the release record.

Purpose:

```text
Capture the first moment Pancanga Engine became a usable tool:

What should I do today?
```
