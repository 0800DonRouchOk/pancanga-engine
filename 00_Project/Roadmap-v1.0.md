# Roadmap Oficial v1.0

## Release Freeze

```text
PANCANGA ENGINE v1.0

RELEASE FREEZE

Knowledge Base:
FROZEN

Astronomy:
CERTIFIED

Architecture:
FROZEN

Vaiṣṇava Engine:
IMPLEMENTED

API:
FROZEN

Scope:
FROZEN
```

Cambios permitidos hasta v1.0:

```text
ENGINE BUG
DOCUMENTATION
CERTIFICATION
UX sin alterar semántica
RELEASE
```

Cambios prohibidos hasta v1.0:

```text
Nuevas reglas
Nuevas APIs
Nueva arquitectura
Nuevos algoritmos
Nuevas funcionalidades
Expansión de alcance
```

Regla:

```text
Si una mejora no aumenta la confianza en el motor,
no pertenece a la v1.0.
```

## Fase 0 - Fundación

Estado: completada técnicamente, pendiente de revisión formal en algunos
documentos.

- 🔒 PES-0001
- 🔒 ARCH-0001
- 🟡 ARCH-0002 - En revisión final
- ✅ Core Types
- ✅ Core Math
- ✅ Core Time
- ✅ Julian Day

## Fase 1 - Astronomy Engine

- 🟡 ADR-0003 - En revisión
- ✅ ADR-0004 - VSOP87D para la cadena solar
- ✅ ADR-0005 - Modelo de aberración solar
- ✅ ADR-0006 - Modelo de nutación en longitud
- ✅ ADR-0007 - Referencia externa de validación científica
- 🟢 AST-0001 - Julian Centuries (validación científica pendiente)
- ✅ AST-0002 - Longitud Heliocéntrica de la Tierra (VSOP87D)
- ✅ AST-0003 - Longitud Geocéntrica del Sol
- ✅ AST-0004 - Radio Vector de la Tierra
- ✅ AST-0005 - Aberración Solar
- ✅ AST-0006 - Nutación en Longitud
- ✅ AST-0007 - Longitud Aparente del Sol
- 🔒 Solar Pipeline v1.0 - Cerrado y congelado
- ✅ Milestone 01 - Solar Engine v1.0
- 🔒 Solar Engine v1.0 API - Estable
- 🟡 Validación Solar Engine v1.0 vs Swiss Ephemeris - Pendiente
- 🟡 Lunar Pipeline v1.0 - En desarrollo
- ✅ AST-L001 - Fundamental Lunar Arguments
- ✅ ELP2000 Import Pipeline - 36 archivos oficiales importados
- ✅ ELP2000 Dataset Audit - auditoría estructural completada
- ✅ AST-L002 - Periodic Longitude Correction
- ✅ AST-L003 - Lunar Mean Longitude W1
- ✅ AST-L004 - Geocentric Lunar Longitude
- ✅ AST-L006 - ELP2000 Frame Transformation
- ✅ AST-L005 - Apparent Lunar Longitude
- 🔒 Lunar Engine API - `astronomy::moon::apparent_longitude(jd)`
- ✅ Lunar-Solar Elongation
- ✅ Astronomical Tithi Index
- ✅ Astronomical Tithi
- ✅ Milestone 03 - Astronomical Engine v1.0
- ⬜ Ayanāṁśa

## Fase 2 - Calendario

- 🔒 Calendar Pipeline v1.0 - Frontera Astronomía/Calendario congelada
- ✅ Calendar Engine Base v1.0
- ✅ Sunrise
- ⬜ Sunset
- ✅ Tithi at Sunrise
- ✅ Tithi Changes
- ✅ Tithi Presence
- ⬜ Nakshatra
- ⬜ Yoga
- ⬜ Karana
- ⬜ Masa
- ⬜ Gaurabda

## Fase 3 - Gaudiya Engine

```text
PRIMARY SOURCE AVAILABLE
Translation and verse numbering available
Extraction complete
Knowledge Base v1.0 READY
Hari-bhakti-vilāsa
```

- ✅ Campaign 36 - Vaiṣṇava Rules Specification
- ✅ Campaign 37 - Vaiṣṇava Normative Authority
- 🟡 Campaign 38 - Ekādaśī Rules Specification
- 🟡 Campaign 39 - Hari-bhakti-vilāsa Source Extraction
- ✅ Campaign 39.5 - HBV-EK-001 Candidato de Ekādaśī
- ✅ Campaign 39.6 - HBV-EK-002 Ekādaśī viddhā
- ✅ Campaign 39.7 - HBV-EK-003 Desplazamiento de observancia
- ✅ Campaign 39.8 - HBV-EK-004 Ocho Mahādvādaśī
- ✅ Campaign 39.9 - HBV-EK-005 Parāṇa / Hari-vāsara
- ✅ Campaign 39.10 - Knowledge Base Audit
- ✅ Campaign 40 - HBV-EK-001 Implementation
- ✅ Campaign 41 - HBV-EK-002 Implementation
- ✅ Campaign 42 - HBV-EK-003 Implementation
- ✅ Campaign 43A - Nakṣatra Engine
- ✅ Campaign 43B - HBV-EK-004 Implementation
- ✅ Campaign 44 - HBV-EK-005 Implementation
- ✅ Campaign 45 - Vaiṣṇava Engine Integration
- 🟡 Campaign 46 - Vaiṣṇava Engine Validation (certificación final en curso)
- ✅ Campaign 46A - Swiss Ephemeris Infrastructure
- ✅ Campaign 46A.2 - Swiss Difference Analysis
- ✅ Campaign 46A.3 - Frame Transformation Specification
- ✅ Campaign 46A.4 - Implement AST-L006
- 🟡 Campaign 46B - GCal Fixtures (fixtures externos pendientes)
- ✅ Campaign 46B.1 - GCal Real Oracle Importer
- ✅ Campaign 46D - PureBhakti HTML Extractor
- ✅ Campaign 46D.1 - PureBhakti Real Fixture Validation
- ✅ Campaign 46D.2 - Parāṇa Difference Analysis (PureBhakti)
- 🟡 Campaign 46F - SCS Math Certification (fixtures reales; diferencias clasificadas)
- ✅ Campaign 46F.1 - SCS Math Difference Analysis
- 🔴 Campaign 46F.2 - Civil Configuration Audit
- 🔴 Campaign 46E - GCal Certification
- 🔴 Campaign 46C Final - Full External Certification
- ✅ Campaign 46STRESS - Internal Consistency Certification
- 🔴 Campaign 46DETERMINISM - Deterministic Certification
- 🔴 Campaign 46BETA - Pancanga Engine RC1 Experience
- 🟡 Campaign 46BETA.7 - Deploy RC1 Experience para testers cerrados
- ✅ Campaign 46BETA.8 - Public RC1 Testing infrastructure prepared
- 🟡 Campaign 46BETA.10 - Zero Cost Deploy (Cloudflare Pages Free + Render Free)
- ✅ Campaign 49.0 - Observance resolution movida al Vaiṣṇava Engine;
  la RC1 consume `Observance` del motor y conserva sólo un adaptador temporal
  de māsa para el contenido beta
- 🟡 Campaign 50.2 - Vaiṣṇava Māsa Specification: frontera técnica documentada;
  implementación bloqueada hasta definir la autoridad normativa de māsa
- 🔴 Campaign 46R - Release Audit
- 🔴 Campaign 47 - v1.0 Release
- 🟡 Final Certification Stage - [criterio de salida](Final-Certification-Stage.md)
- 🔒 KB-VAI-002 - Hari-bhakti-vilāsa Ekādaśī Rules
- 🔒 Knowledge Base v1.0 - congelada
- 🔒 Knowledge Base v1.0 Release - 2026-07-14
- 🟡 Vilāsa 12/13 Scope - Vilāsa 12 determina el día; Vilāsa 13 describe
  observancia
- 🟢 Aruṇodaya Definition - respaldada por mūla y Digdarśinī; definición
  operativa cerrada en HBV-EK-002
- 🟡 Citation Normalization - numeración oficial: Bhṛgumuni Dāsa / Haridāsa
  Śāstrī
- 🔒 Web Research Rule - Internet solo localiza o verifica fuentes; no define
  reglas HBV-EK
- 🔒 Source Precedence - HBV mūla > Digdarśinī > Navadvīpa > Tradition Notes >
  Purāṇas citados como contexto
- 🔒 Formalización HBV - extraer, normalizar y especificar; no buscar reglas
  por web
- 🔒 Methodology Freeze - no modificar metodología por comodidad de una regla
- ✅ KB Final Review - Knowledge Base v1.0 READY
- 🟡 Source Priority - HBV 12/13 primero; Digdarśinī para ambigüedades;
  Navadvīpa Pañjikā para aplicación práctica; Purāṇas solo si hace falta
- 🟡 Source Levels - Nivel A: HBV + Digdarśinī; Nivel B: Navadvīpa + notas de
  tradición; Nivel C: Purāṇas citados
- 🟢 Coverage Matrix - HBV-EK-001 a HBV-EK-005 cerradas, con columna
  `Implementable`
- ✅ Campaign 39.5 Scope - solo candidato de Ekādaśī; sin viddhā,
  Mahādvādaśī, Hari-vāsara, parāṇa ni nakṣatra
- ✅ Campaign 39.5 Extraction Shape - definición textual, definición normativa,
  datos consumidos y máximo tres casos de prueba
- ✅ Campaign 39.6 Scope - solo invalidación por Viddhā; sin desplazamiento,
  Mahādvādaśī, Hari-vāsara, parāṇa ni nakṣatra
- ✅ Campaign 39.7 Scope - solo desplazamiento de observancia a Dvādaśī; sin
  Mahādvādaśī, Hari-vāsara, parāṇa ni nakṣatra
- ✅ Campaign 39.8 Scope - tabla normativa de los ocho Mahādvādaśī cerrada;
  ISSUE-VAI-001 resuelve Vyañjulī sin bloquear HBV-EK-004
- ✅ Campaign 39.9 Scope - ventana de Parāṇa / Hari-vāsara cerrada; sin código
- ⬜ Ekadasi
- ⬜ Mahadvadasi
- ⬜ Parana
- ⬜ Festivals

## Post-v1.0 / v2 Backlog

Estas campañas no pertenecen a v1.0. Quedan registradas para evitar deuda
técnica implícita durante la RC1.

### Campaign 50.0 - Native Vaiṣṇava Māsa

Priority: HIGH for v2 planning.

Status:

```text
50.0A Observance identity contract:
PASS

50.0B Native Calendar Engine Vaiṣṇava Māsa:
OPEN

50.1 Native Vaiṣṇava Māsa Engine:
STOP - missing implementable local specification

50.2 Vaiṣṇava Māsa Specification:
STOP - specification boundary documented; normative implementation rule still
missing

50.3 HBV Vaiṣṇava Māsa Research:
STOP - local HBV/Gauḍīya research did not find an implementable māsa algorithm
```

Objective:

```text
Implement native Vaiṣṇava Māsa calculation inside the Calendar Engine so the
Observance Engine can resolve ordinary Ekādaśī identities without any RC1
adapter.
```

Completed in 50.0A:

```text
The Vaiṣṇava Engine now centralizes observance identity resolution.

Ordinary Ekādaśī:
resolved from formal Vaiṣṇava Māsa + Pakṣa.

Mahādvādaśī:
resolved from HBV-EK-004 rule output.

Missing ordinary Ekādaśī māsa:
explicit error, never silent fallback.
```

Current RC1 state:

```text
Astronomy
↓
Calendar Engine
↓
temporary māsa adapter
↓
Observance Engine
↓
Festival Catalog
↓
RC1
```

Target v2 state:

```text
Astronomy
↓
Calendar Engine
↓
Vaiṣṇava Māsa
↓
Pakṣa
↓
Mahādvādaśī Rules
↓
Observance Engine
↓
Festival Catalog
↓
API / RC1 / Web / Mobile
```

Required scope:

```text
Māsa normal
Adhika Māsa
Kṣaya Māsa, if supported by the engine
Compatibility with Ekādaśī observance naming
```

Exit criterion:

```text
The RC1/content-layer māsa adapter is removed.
The Calendar Engine supplies formal Vaiṣṇava Māsa.
The Observance Engine resolves Ekādaśī identity from calendar facts only.
```

v1.0 rule:

```text
Do not implement Campaign 50.0 during the v1.0 release freeze unless a
confirmed ENGINE BUG makes it release-blocking.
```

Campaign 50.1 audit:

```text
00_Project/Campaign-50.1-Native-Vaishnava-Masa-Engine.md
```

Campaign 50.2 specification:

```text
00_Project/Campaign-50.2-Vaishnava-Masa-Specification.md
05_Documentation/Knowledge-Base/KB-CAL-001-Vaishnava-Masa-Specification.md
```

Campaign 50.3 research:

```text
00_Project/Campaign-50.3-HBV-Vaishnava-Masa-Research.md
05_Documentation/Knowledge-Base/KB-CAL-002-HBV-Vaishnava-Masa-Rules.md
```

## Metodología desde M5

Cada algoritmo será un entregable independiente.

Cada entregable deberá incluir:

- ficha técnica del algoritmo;
- implementación Rust;
- pruebas unitarias;
- ejemplos publicados cuando existan;
- benchmark cuando el algoritmo esté en una ruta crítica;
- documentación `rustdoc`;
- vínculo con el capítulo del libro técnico correspondiente.

## Madurez De Componentes

- ✅ Astronomy Engine v1.0 Stable
- ✅ Calendar Engine v1.0 Stable
- ✅ Sources Layer v1.0 Stable
- 🔒 Knowledge Base v1.0 Stable
- 🚧 Vaiṣṇava Engine v0.1 Planned
