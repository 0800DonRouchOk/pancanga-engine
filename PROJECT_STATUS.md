# Project Status

Version: v1.0 RC1

## Estado General

Pancanga Engine está en fase de Release Candidate para v1.0.

Estado RC1:

```text
PANCANGA ENGINE:
Release Candidate 1 (RC1)

Development:
🔒 CLOSED

Certification:
🟡 FINAL STAGE

Release:
PENDING

Knowledge Base:
🔒 v1.0 FROZEN

Astronomy Engine:
🔒 CERTIFIED

Calendar Engine:
🔒 STABLE

Vaiṣṇava Engine:
🔒 IMPLEMENTED

Open doctrinal issues:
0

Confirmed ENGINE BUG:
0

Status:
READY FOR FINAL CERTIFICATION
```

El objetivo original de v1.0 está definido como:

```text
Astronomía certificada
Calendar Engine estable
Knowledge Base completa
Motor Vaiṣṇava implementado
Certificación contra oráculos externos
Sin ENGINE BUG confirmados
Release Freeze respetado
```

Release Freeze:

```text
Knowledge Base:
🔒 FROZEN

Astronomy:
🔒 FROZEN

Architecture:
🔒 FROZEN

API:
🔒 FROZEN

Scope:
🔒 FROZEN
```

Hasta v1.0 solo se aceptan cambios en estas categorías:

```text
ENGINE BUG
DOCUMENTATION
CERTIFICATION
UX sin alterar semántica
RELEASE
```

Regla de freeze:

```text
Si una mejora no aumenta la confianza en el motor,
no pertenece a la v1.0.
```

Regla RC:

```text
Una versión RC no incorpora nuevas funcionalidades.

Solo puede:
- corregir bugs;
- mejorar documentación;
- completar certificaciones;
- preparar el release.
```

Documentos raíz de cierre:

- [PROJECT_PRINCIPLES.md](PROJECT_PRINCIPLES.md)
- [WHY_PANCANGA_ENGINE_EXISTS.md](WHY_PANCANGA_ENGINE_EXISTS.md)
- [CERTIFICATION.md](CERTIFICATION.md)
- [Final Certification Stage](00_Project/Final-Certification-Stage.md)
- [Pancanga Engine RC1 Experience](00_Project/RC1-Experience.md)

Trabajo restante antes de v1.0:

- Campaign 46F.2 - auditoría de configuración civil para SCS Math.
- Campaign 46C - informe definitivo de validación externa.
- Campaign 46DETERMINISM - certificación determinista.
- Campaign 46BETA - Pancanga Engine RC1 Experience.
- Campaign 46R - auditoría final de release.
- Campaign 47 - release v1.0.

Hitos de certificación cerrados:

- Campaign 46STRESS - certificación de consistencia interna: 73.414 días
  consecutivos, 1900-01-01 a 2100-12-31, 0 contradicciones lógicas y 0
  ENGINE BUG confirmados.

## Arquitectura Congelada

- [PES-0001](01_Specification/PES/PES-0001.md): Project Foundation. Estado:
  🔒 Congelado.
- [ARCH-0001](01_Specification/ARCH/ARCH-0001.md): Arquitectura General.
  Estado: 🔒 Congelado.

## Especificaciones en Revisión

- [PES-0002](01_Specification/PES/PES-0002.md): Sistema Temporal. Estado:
  🟡 En revisión.
- [ARCH-0002](01_Specification/ARCH/ARCH-0002.md): Arquitectura del Subsistema
  Astronómico. Estado: 🟡 En revisión final.

## Decisiones Arquitectónicas

- [ADR-0002](05_Documentation/ADR/ADR-0002.md): Operaciones con
  Ángulos Mediante Tipos Fuertes.
- [ADR-0003](05_Documentation/ADR/ADR-0003.md): Modelo Astronómico Basado en
  Meeus y Ayanāṁśa Lahiri. Estado: 🟡 En revisión.
- [ADR-0004](05_Documentation/ADR/ADR-0004.md): Adoptar VSOP87D para la Cadena
  Solar. Estado: Aceptado.
- [ADR-0005](05_Documentation/ADR/ADR-0005.md): Modelo de Aberración Solar.
  Estado: Aceptado.
- [ADR-0006](05_Documentation/ADR/ADR-0006.md): Modelo de Nutación en
  Longitud. Estado: Aceptado.
- [ADR-0007](05_Documentation/ADR/ADR-0007.md): Referencia Externa de
  Validación Científica. Estado: Aceptado.

## RFC

- [RFC-0001](05_Documentation/RFC/RFC-0001.md): Selección de la Variante
  VSOP87. Estado: ✅ Cerrada.

## Milestones

- M2 - Core Math: implementado y verificado localmente.
- M3 - Astronomical Time: implementado y verificado localmente.
- M4 - Julian Day: implementado y pendiente de revisión técnica.
- AST-0001 - Julian Centuries: implementado; validación científica pendiente.
- AST-0002 - Longitud heliocéntrica de la Tierra (VSOP87D): implementado y
  validado contra `vsop87.chk`.
- AST-0003 - Longitud geocéntrica del Sol: implementado y validado por
  identidad geométrica.
- AST-0004 - Radio vector de la Tierra (VSOP87D): implementado y validado
  contra `vsop87.chk`.
- AST-0005 - Aberración solar: implementado y validado por fórmula de Meeus.
- AST-0006 - Nutación en longitud: implementado y validado por serie truncada
  de Meeus.
- AST-0007 - Longitud aparente del Sol: implementado y validado por integración
  del pipeline solar.
- [Milestone 01 - Solar Engine v1.0](00_Project/Milestone-01-Solar-Engine.md):
  cerrado.
- [Milestone 03 - Astronomical Engine v1.0](00_Project/Milestone-03-Astronomical-Engine-v1.0.md):
  cerrado.
- [Solar Engine API Review](00_Project/Solar-Engine-API-Review.md): aprobada.
- Solar Pipeline v1.0: cerrado y congelado.
- Solar Engine v1.0 API: estable.
- Validación Solar Engine v1.0 contra Swiss Ephemeris: pendiente de ejecución.
- Lunar Pipeline v1.0: iniciado.
- AST-L001 - Fundamental Lunar Arguments: implementado y validado contra
  polinomios de Meeus.
- Infraestructura de importación ELP2000: ejecutada contra los 36 archivos
  oficiales de IMCCE. Generó 37.872 términos, manifiesto determinístico,
  dataset fingerprint `0x3aa7ebfdd62d91dc` y huella del Rust generado
  `0x338c91efeeaee125`.
- Auditoría estructural del dataset ELP2000: completada.
- AST-L002 - ELP2000 Periodic Longitude Correction: ✅ cerrado. Integra las 12
  familias de longitud ELP2000 y devuelve `Δλ_periodic` en `ArcSeconds`.
  Validación Campaign 22 (`JD = 2488070.0`, `t = 1.0`):
  `-6423.827265758044` arcsec.
- AST-L003 - Lunar Mean Longitude W1: ✅ implementado. Devuelve `W1(t)` como
  `Degrees` normalizado, sin sumar `Δλ_periodic`, nutación ni longitud
  aparente.
- AST-L004 - Geocentric Lunar Longitude: ✅ implementado. Integra `W1` y
  `Δλ_periodic` como `λ_geo = normalize_360(W1 + Δλ/3600)`.
- AST-L006 - ELP2000 Frame Transformation: ✅ implementado y validado contra
  Swiss Ephemeris. Aplica la precesión escalar oficial `5029.0966″ × t` para
  llevar la longitud lunar ELP2000 al marco medio/de fecha antes de nutación.
- AST-L005 - Apparent Lunar Longitude: ✅ implementado. Aplica la nutación en
  longitud existente (`AST-0006`) sobre la longitud lunar media/de fecha, sin
  duplicar el algoritmo de nutación.
- Lunar Engine API: ✅ implementada. Expone
  `astronomy::moon::apparent_longitude(jd)` como fachada pública estable sobre
  `W1`, `Δλ_periodic`, `λ_geo`, AST-L006, `Δψ` y `λ_apparent`.
- Campaign 27 - Elongación Sol-Luna: ✅ implementada. Expone
  `astronomy::lunar_solar_elongation(moon_longitude, sun_longitude)` como
  magnitud astronómica pura, sin lógica de tithi, calendario ni reglas
  vaiṣṇavas.
- Campaign 28 - Índice Astronómico de Tithi: ✅ implementado. Expone
  `astronomy::tithi_index(elongation)` como índice interno `0..29`, sin nombres
  tradicionales, pakṣa, calendario ni reglas vaiṣṇavas.
- Campaign 29 - Tithi Astronómico: ✅ implementado. Agrega
  `AstronomicalTithi` y `Paksha`, derivados únicamente del índice astronómico,
  sin fecha civil, amanecer, calendario ni reglas vaiṣṇavas.
- Astronomical Engine v1.0: ✅ cerrado para cálculo de tithi instantáneo.
  Capacidades cerradas: longitud aparente solar, longitud aparente lunar,
  elongación Sol-Luna, índice astronómico de tithi y `AstronomicalTithi`.
- Campaign 31 - Calendar Engine Boundary: 🔒 congelado. El motor calendárico no
  implementa algoritmos astronómicos; consume únicamente la API pública del
  motor astronómico. Pipeline registrado en
  [Calendar Pipeline v1.0](05_Documentation/Algorithms/Calendar-Pipeline.md).
- Campaign 32 - Sunrise Engine: ✅ implementado. Expone
  `calendar::sunrise(date, location)` para calcular el `JulianDate` del
  amanecer local, sin calcular tithi, aruṇodaya, ventanas calendáricas ni
  reglas vaiṣṇavas.
- Campaign 33 - Tithi at Sunrise: ✅ implementado. Expone
  `calendar::tithi_at_sunrise(sunrise_jd)` y consume únicamente la API pública
  del motor astronómico para determinar el `AstronomicalTithi` presente en el
  amanecer.
- Campaign 34 - Tithi Transition Within the Civil Day: ✅ implementado. Expone
  `calendar::tithi_transition_between_sunrises(sunrise_today, sunrise_tomorrow)`
  para detectar el primer cambio astronómico de tithi dentro del intervalo
  `[amanecer, siguiente amanecer)`, sin decidir presencia calendárica ni aplicar
  reglas vaiṣṇavas.
- Campaign 35 - Tithi Presence in Civil Day: ✅ implementado. Agrega
  `CivilDayTithiPresence` y `calendar::tithi_presence_between_sunrises(...)`
  para describir objetivamente el tithi al amanecer, transición opcional y
  tithi posterior, sin interpretar reglas vaiṣṇavas.
- Astronomical Engine v1.0: ✅ milestone cerrado.
- Calendar Engine Base v1.0: ✅ milestone cerrado.
- Campaign 36 - Vaiṣṇava Rules Specification: ✅ cerrada como especificación de
  frontera normativa. Documento:
  [Vaishnava Rules](05_Documentation/Algorithms/Vaishnava-Rules.md). No se
  implementaron reglas.
- Campaign 37 - Vaiṣṇava Normative Authority: ✅ cerrada. Autoridad normativa
  primaria para v1.0: Hari-bhakti-vilāsa. GCal y PureBhakti quedan como
  oráculos calendáricos de validación, no como autoridad normativa primaria.
  Documento:
  [Vaiṣṇava Normative Authority](05_Documentation/Algorithms/Vaishnava-Authority.md).
- Campaign 38 - Ekādaśī Rules Specification: 🟡 cerrada para auditoría de
  Knowledge Base.
  Documento:
[Ekādaśī Rules](05_Documentation/Algorithms/Ekadasi-Rules.md). No se
  implementaron reglas. La especificación provisional basada en fuente
  secundaria queda superada por KB-VAI-002, que ya extrae HBV-EK-001 a
  HBV-EK-005 desde Hari-bhakti-vilāsa.
- Campaign 39 - Hari-bhakti-vilāsa Source Extraction: ✅ extracción HBV-EK
  completa; auditoría de Knowledge Base pendiente. Documento:
  [Hari-bhakti-vilāsa Source Extraction](05_Documentation/Algorithms/Hari-bhakti-vilasa-Source-Extraction.md).
  No se implementó código. Wikipedia queda excluida como fuente normativa.
- KB-VAI-002 - Hari-bhakti-vilāsa Ekādaśī Rules: 🟡 creado. Documento de
  Knowledge Base con referencia concreta a capítulos 12-13, reglas HBV-EK,
  ocho Mahādvādaśī, diagrama de decisión y nota sobre Hari-vāsara.
  Implementación protegida por protocolo hasta verificar versos o edición
  identificable.
  Documento:
  [KB-VAI-002](05_Documentation/Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md).
- 07_Sources: ✅ capa documental creada para registrar autoridad de fuentes.
  Incluye fuentes astronómicas (`Meeus`, `ELP2000`, `Swiss`) y fuentes
  Vaiṣṇavas (`Hari-bhakti-vilāsa`, `Caitanya-caritāmṛta`, interpretaciones).
- 07_Sources/Vaishnava: ✅ prioridad de consulta registrada. La extracción
  comienza por `Hari-bhakti-vilāsa` Vilāsa 12 para HBV-EK-001 a HBV-EK-004 y
  continúa con Vilāsa 13 para HBV-EK-005. `Digdarśinī-ṭīkā` se reserva para
  ambigüedades; `Navadvīpa Pañjikā` para aplicación Gauḍīya práctica; Purāṇas
  citados por HBV solo para confirmación cuando haga falta.
- Jerarquía de fuentes Vaiṣṇavas: ✅ registrada en tres niveles. Nivel A:
  `Hari-bhakti-vilāsa` y `Digdarśinī-ṭīkā`; Nivel B: `Navadvīpa Pañjikā` y
  `Tradition-Notes`; Nivel C: Purāṇas citados por HBV.
- 07_Sources/Vaishnava/Tradition-Notes: ✅ carpeta creada para notas
  interpretativas de la Nityānanda Vaṁśa. No reemplaza la autoridad normativa
  primaria.
- Component maturity: 🟡 registrado. Astronomy Engine v1.0 Stable, Calendar
  Engine v1.0 Stable, Sources Layer v1.0 Stable, Knowledge Base v1.0 Stable,
  Vaiṣṇava Engine v0.1 Planned. Documento:
  [Component Maturity](00_Project/Component-Maturity.md).
- HBV Extraction Protocol: 🔒 activo. Las reglas HBV-EK fueron extraídas una
  por campaña (`39.5` a `39.9`) con métrica Evidence Level E1-E5 y auditadas en
  Campaign 39.10. Documento:
  [HBV Extraction Protocol](05_Documentation/Knowledge-Base/HBV-Extraction-Protocol.md).
- Matriz de cobertura HBV-EK: 🟡 creada. Registra cobertura por regla para
  `HBV`, `Digdarśinī`, `Navadvīpa` e `Implementable`; HBV-EK-001 quedó
  congelada y las reglas restantes permanecen en extracción.
- Citation Normalization: 🟡 registrada. La numeración oficial de reglas HBV-EK
  se normaliza según la traducción íntegra de Bhṛgumuni Dāsa y la edición
  Haridāsa Śāstrī; referencias secundarias previas deben mapearse antes de
  congelar reglas.
- Web research rule: 🔒 registrada. Desde Campaign 39.6 en adelante, Internet
  solo puede usarse para localizar, descargar o verificar fuentes; ninguna
  regla normativa HBV-EK puede fundamentarse en búsquedas web.
- Source precedence rule: 🔒 registrada. Precedencia: HBV mūla,
  Digdarśinī-ṭīkā, Navadvīpa Pañjikā, Tradition Notes y Purāṇas citados solo
  como contexto. Si una discrepancia no puede resolverse, no se modifica la
  regla y la implementación permanece protegida por protocolo.
- Etapa actual: Formalización del Hari-bhakti-vilāsa. El trabajo pendiente es
  extraer, normalizar y especificar; no investigar reglas mediante web.
- Knowledge Base v1.0: 🔒 cerrada. Fuente primaria disponible,
  Digdarśinī-ṭīkā disponible, reglas HBV-EK extraídas y auditoría aprobada.
- Knowledge Base v1.0 Release: 🔒 frozen. Release Date: 2026-07-14. Regla de
  congelamiento: no modificar Knowledge Base para facilitar implementación;
  solo cambiarla si una fuente primaria obliga al cambio. Documento:
  [Knowledge Base v1.0 Release](05_Documentation/Knowledge-Base/Knowledge-Base-v1.0-Release.md).
- División Vilāsa 12 / Vilāsa 13: 🟡 registrada. Vilāsa 12 responde qué día
  corresponde observar; Vilāsa 13 responde cómo debe observarse.
- Aruṇodaya: 🟢 definición respaldada por mūla y Digdarśinī; quedó cerrada
  operativamente dentro de HBV-EK-002 sin mezclar desplazamiento ni
  Mahādvādaśī.
- Methodology freeze: 🔒 registrada. No se modifica la metodología por
  comodidad de una regla; solo cambia si Hari-bhakti-vilāsa lo exige
  explícitamente.
- Knowledge Base final review: 🔒 registrada. Tras HBV-EK-005, no se pasa
  directamente a código; primero se revisa si un desarrollador puede
  implementar el Motor Vaiṣṇava usando únicamente la Knowledge Base.
- Campaign 39.10 - Knowledge Base Audit: ✅ cerrada. Resultado: Knowledge Base
  v1.0 READY. No se detectaron reglas normativas implícitas fuera de la
  Knowledge Base. Documento:
  [Campaign 39.10](05_Documentation/Knowledge-Base/Campaign-39.10-Knowledge-Base-Audit.md).
- Campaign 39.5 - HBV-EK-001 Candidate Ekādaśī: ✅ cerrada. Regla normativa:
  un día civil entra como candidato inicial cuando el tithi presente en su
  amanecer local es Ekādaśī. No resuelve viddhā, Mahādvādaśī, Hari-vāsara,
  parāṇa ni nakṣatra. Documento:
  [HBV-EK-001](05_Documentation/Knowledge-Base/HBV-EK-001-Candidate-Ekadasi.md).
- Campaign 39.6 - HBV-EK-002 Ekādaśī Viddhā: ✅ cerrada. Regla normativa: un
  candidato de Ekādaśī queda invalidado como Viddhā cuando Daśamī está presente
  durante aruṇodaya, definido operativamente como los 96 minutos anteriores al
  amanecer local del día candidato. No resuelve desplazamiento, Mahādvādaśī,
  Hari-vāsara, parāṇa ni nakṣatra. Documento:
  [HBV-EK-002](05_Documentation/Knowledge-Base/HBV-EK-002-Viddha.md).
- Campaign 39.7 - HBV-EK-003 Desplazamiento de observancia: ✅ cerrada. Regla
  normativa: cuando un candidato de Ekādaśī queda invalidado como Viddhā por
  HBV-EK-002, la observancia se desplaza a Dvādaśī. No resuelve Mahādvādaśī,
  Hari-vāsara, parāṇa ni nakṣatra. Documento:
  [HBV-EK-003](05_Documentation/Knowledge-Base/HBV-EK-003-Observance-Displacement.md).
- Campaign 39.8 - HBV-EK-004 Ocho Mahādvādaśī: ✅ cerrada. Tabla normativa
  extraída para los ocho Mahādvādaśī. ISSUE-VAI-001 queda resuelto para
  Vyañjulī: Ekādaśī pura, Dvādaśī presente al amanecer y Dvādaśī prolongada
  hacia Trayodaśī; Jayā, Vijayā, Jayantī y Pāpanāśinī requieren Nakṣatra
  Engine. Documento:
  [HBV-EK-004](05_Documentation/Knowledge-Base/HBV-EK-004-Mahadvadasi.md).
- Campaign 39.9 - HBV-EK-005 Parāṇa / Hari-vāsara: ✅ cerrada. Regla normativa:
  la ventana válida de Parāṇa comienza cuando Hari-vāsara ha terminado y debe
  cerrarse antes de que termine Dvādaśī. Si Dvādaśī es corta, se registran las
  contingencias de adelantar obligaciones y realizar Parāṇa mínimo con agua.
  Documento:
  [HBV-EK-005](05_Documentation/Knowledge-Base/HBV-EK-005-Parana-Harivasara.md).
- Campaign 40 - HBV-EK-001 Implementation: ✅ implementada. Agrega
  `vaishnava::ekadasi_candidate_at_sunrise(...)`,
  `vaishnava::ekadasi_candidate_for_civil_day(...)` y
  `EkadasiCandidate`. La regla solo evalúa si Ekādaśī está presente al
  amanecer; no resuelve Viddhā, Mahādvādaśī, Hari-vāsara, Parāṇa, Nakṣatra ni
  festivales.
- Campaign 41 - HBV-EK-002 Implementation: ✅ implementada. Agrega
  `vaishnava::invalidate_viddha_candidate(...)`,
  `vaishnava::arunodaya_start(...)`, `ViddhaCandidateStatus` y la constante
  `ARUNODAYA_BEFORE_SUNRISE_MINUTES`. La regla solo invalida un candidato como
  Viddhā cuando Daśamī está presente en Aruṇodaya; no desplaza la observancia,
  no resuelve Mahādvādaśī, no calcula Nakṣatra y no calcula Parāṇa.
- Campaign 42 - HBV-EK-003 Implementation: ✅ implementada. Agrega
  `vaishnava::observance_displacement(...)` y
  `EkadasiObservanceDisposition`. La regla solo transforma el resultado de
  HBV-EK-002: `ValidCandidate` se observa en Ekādaśī, `InvalidViddha` se
  desplaza a Dvādaśī y `NotCandidate` no produce candidato de observancia. No
  clasifica Mahādvādaśī, no calcula Nakṣatra, no calcula Hari-vāsara y no
  calcula Parāṇa.
- Campaign 43A - Nakṣatra Engine: ✅ implementada. Agrega
  `astronomy::nakshatra::index(...)` y `NakshatraIndex` como división
  astronómica mínima de la longitud lunar en 27 nakṣatras. Incluye las
  correspondencias requeridas por HBV-EK-004: Punarvasu, Śravaṇa, Rohiṇī y
  Puṣya. No implementa yoga, karaṇa ni otros elementos del pañcāṅga.
- Campaign 43B - HBV-EK-004 Implementation: ✅ implementada. Agrega
  `vaishnava::classify_mahadvadasi(...)`, `MahadvadasiType`,
  `TithiMahadvadasiCondition` y `MahadvadasiClassification`. La regla codifica
  los cuatro Mahādvādaśī basados en tithi y los cuatro basados en nakṣatra
  según la Knowledge Base congelada. `ISSUE-VAI-001` está resuelto. No
  calcula Hari-vāsara, no calcula Parāṇa y no incorpora festivales.
- Campaign 44 - HBV-EK-005 Implementation: ✅ implementada. Agrega
  `vaishnava::hari_vasara_end(...)`, `vaishnava::parana_window(...)`,
  `ParanaWindow`, `ParanaMode` y `ParanaWindowError`. La regla calcula el final
  operativo de Hari-vāsara como el primer cuarto de Dvādaśī, abre la ventana de
  Parāṇa después de amanecer y de Hari-vāsara, la cierra antes del fin de
  Dvādaśī y representa explícitamente la contingencia `ShortDvadasi` cuando no
  queda ventana ordinaria. No formatea hora local, no decide alimentos, no
  ejecuta procedimiento ritual y no modifica Mahādvādaśī.
- Campaign 45 - Vaiṣṇava Engine Integration: ✅ implementada. Agrega
  `vaishnava::classify_vaishnava_day(...)`, `VaishnavaDayInput`,
  `VaishnavaDayClassification`, `MahadvadasiFacts`, `ParanaInput` y
  `VaishnavaEngineError`. La fachada integra HBV-EK-001 a HBV-EK-005 sin
  recalcular astronomía, amanecer, tithi ni nakṣatra. `NotCandidate` corta el
  pipeline sin Parāṇa; una observancia requiere datos de Parāṇa explícitos.
  `ISSUE-VAI-001` está resuelto.
- Campaign 46 - Vaiṣṇava Engine Validation: 🟡 validación externa abierta. No
  se declara PASS completo porque SCS Math quedó incorporado con diferencias
  clasificadas pendientes de análisis y `02_Research/GCal` no contiene fixtures
  de oráculo Vaiṣṇava. Swiss y PureBhakti ya cerraron como PASS; PureBhakti
  conserva diferencias editoriales de Parāṇa documentadas.
  Documento:
  [Vaiṣṇava Engine Validation](04_Tests/Vaishnava/Vaishnava-Engine-Validation.md).
  CSV:
  [Vaiṣṇava Engine Validation CSV](04_Tests/Vaishnava/Vaishnava-Engine-Validation.csv).
  Regresión local: PASS (`cargo fmt --all --check`, `cargo test -p
  pancanga-engine`, `cargo clippy -p pancanga-engine -- -D warnings`,
  `cargo doc --no-deps`). Código, Knowledge Base y arquitectura sin cambios.
- Campaign 46A - Swiss Ephemeris Infrastructure: ✅ PASS. `swetest` 2.10.03 quedó
  instalado localmente en
  `04_Tests/Astronomy/SwissEphemeris/bin/swetest`. Se generaron 1000 casos
  reales en `swiss-validation.csv` después de AST-L006: 1000 PASS automáticos,
  0 `ASTRONOMY DIFFERENCE`, 0 diferencias de índice de tithi, 0 errores de
  herramienta externa. Documento:
  [Swiss Validation](04_Tests/Astronomy/SwissEphemeris/Swiss-Validation.md).
  CSV:
  [swiss-validation.csv](04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv).
  Knowledge Base y arquitectura sin cambios.
- Campaign 46A.2 - Swiss Difference Analysis: ✅ causa identificada sin tocar
  el motor. La diferencia lunar principal queda clasificada como
  `MODEL DIFFERENCE / REFERENCE FRAME`: Pancanga Moon coincide mucho mejor con
  Swiss usando `-j2000`, mientras Pancanga Sun coincide con Swiss en longitudes
  de fecha. La comparación de elongación/tithi mezcla marcos y requiere una
  campaña separada para especificar la transformación ELP2000 de J2000 a marco
  de fecha antes de modificar código. Documento:
  [Swiss Difference Analysis](04_Tests/Astronomy/SwissEphemeris/Swiss-Difference-Analysis.md).
  CSV:
  [swiss-difference-analysis.csv](04_Tests/Astronomy/SwissEphemeris/swiss-difference-analysis.csv).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46A.3 - Frame Transformation Specification: ✅ especificada sin
  código. AST-L006 define que la longitud lunar escalar ELP2000 debe pasar por
  la precesión oficial `5029.0966″ × t` antes de exponerse como longitud lunar
  aparente/de fecha compatible con el Solar Engine. La prueba documental muestra
  que esta corrección reduce el error lunar máximo contra Swiss `-edir -eswe` a
  `0.000469445269°`, el error de elongación máximo a `0.000330443529°` y los
  tithi mismatches a `0` en 1000 casos. Documento:
  [AST-L006](05_Documentation/Algorithms/AST-L006-ELP2000-Frame-Transformation.md).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46A.4 - Implement AST-L006: ✅ implementada. Se agregó AST-L006 al
  Astronomy Engine y se integró en `astronomy::moon::apparent_longitude(jd)`
  entre `λ_geo` y nutación. La comparación Swiss con ruta `-edir` explícita
  cerró con 1000 / 1000 PASS, error lunar máximo `0.000469445269°`, error de
  elongación máximo `0.000330443528°` y 0 diferencias de tithi. El intérprete
  ELP2000, el importador, la Knowledge Base y la arquitectura normativa no se
  modificaron.
- Campaign 46B - GCal Fixtures: 🟡 External Oracle Pending. Estructura oficial
  creada para fixtures GCal sin inventar datos. `02_Research/GCal` contiene
  solo `.gitkeep`, por lo que todas las categorías obligatorias quedan
  pendientes de oráculo externo. Documento:
  [GCal Fixtures](04_Tests/Vaishnava/GCal/GCal-Fixtures.md). CSV:
  [gcal-fixtures.csv](04_Tests/Vaishnava/GCal/gcal-fixtures.csv). Código del
  motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46B.1 - GCal Real Oracle Importer: ✅ infraestructura lista. Se
  agregó un importador local para convertir calendarios reales `.ics` o `.csv`
  en `gcal-fixtures.csv` sin generar datos sintéticos. Si no existe una fuente
  local real en `02_Research/GCal`, el importador conserva
  `EXTERNAL_ORACLE_PENDING`. Código del motor, Knowledge Base y arquitectura sin
  cambios.
- Campaign 46D - PureBhakti HTML Extractor: ✅ infraestructura lista. Se agregó
  un extractor local para convertir `Calendario-Vaishnava-2026.html` en
  `purebhakti-fixtures.csv` sin inventar datos.
  Documento: [PureBhakti Fixtures](04_Tests/PureBhakti/PureBhakti-Fixtures.md).
  CSV: [purebhakti-fixtures.csv](04_Tests/PureBhakti/purebhakti-fixtures.csv).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46D.1 - PureBhakti Real Fixture Validation: ✅ PASS. Se leyó
  `02_Research/PureBhakti/Calendario-Vaishnava-2026.html`, se generaron 16
  fixtures reales para Buenos Aires y se ejecutó la primera comparación externa.
  Resultado: 16 / 16 fechas de observancia PASS; 1 / 16 ventanas de Parāṇa PASS;
  15 / 16 diferencias de Parāṇa documentadas como diferencias editoriales o de
  redondeo; 0 `ENGINE BUG`.
  Documento:
  [PureBhakti Validation](04_Tests/PureBhakti/PureBhakti-Validation.md). CSV:
  [purebhakti-validation.csv](04_Tests/PureBhakti/purebhakti-validation.csv).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46D.2 - Parāṇa Difference Analysis: ✅ cerrada. Se analizaron las 15
  diferencias de Parāṇa contra PureBhakti sin modificar el motor. Resultado:
  13 casos clasificados como `Editorial Policy`, 2 como `Rounding Difference`,
  0 `ENGINE BUG`. La fecha de observancia permanece 16 / 16 PASS. Documento:
  [PureBhakti Parāṇa Difference Analysis](04_Tests/PureBhakti/PureBhakti-Parana-Difference-Analysis.md).
  CSV:
  [purebhakti-parana-difference-analysis.csv](04_Tests/PureBhakti/purebhakti-parana-difference-analysis.csv).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46F - SCS Math Certification: 🟡 OPEN. Se incorporó el PDF real
  `02_Research/SCSMath/GaudiyaCalendar_2026-2027.pdf`, se creó el extractor
  `04_Tests/SCSMath/scsmath_pdf_extract.py` y se generaron 26 fixtures reales.
  Primera comparación: 11 / 26 fechas de observancia PASS; 15 diferencias
  calendáricas clasificadas; 11 diferencias de configuración de Parāṇa entre
  los casos comparables; 0 `ENGINE BUG` confirmado. Documentos:
  [SCS Math Fixtures](04_Tests/SCSMath/SCSMath-Fixtures.md) y
  [SCS Math Validation](04_Tests/SCSMath/SCSMath-Validation.md). CSV:
  [scsmath-fixtures.csv](04_Tests/SCSMath/scsmath-fixtures.csv) y
  [scsmath-validation.csv](04_Tests/SCSMath/scsmath-validation.csv). Código del
  motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46F.1 - SCS Math Difference Analysis: ✅ cerrada. Se analizaron las
  15 diferencias de observancia contra SCS Math sin modificar el motor.
  Resultado: 14 `CONFIGURATION_DIFFERENCE`, 1 `TRADITION_DIFFERENCE`
  relacionada con Vyañjulī, 0 `ENGINE_BUG`. ISSUE-VAI-001 quedó resuelto
  posteriormente a favor de la formulación HBV/DDT. Las 11
  diferencias de Parāṇa comparables quedan como política editorial/configuración,
  coherentes con el patrón ya visto en PureBhakti. Documento:
  [SCS Math Difference Report](04_Tests/SCSMath/SCSMath-Difference-Report.md).
  Código del motor, Knowledge Base y arquitectura sin cambios.
- Campaign 46C - Full External Validation: 🟡 OPEN. Reporte unificado creado
  para certificar el motor contra Swiss Ephemeris, PureBhakti, SCS Math y GCal.
  Swiss ya cerró como PASS; PureBhakti cerró como PASS con diferencias
  editoriales de Parāṇa documentadas; SCS Math queda abierto con diferencias
  analizadas; GCal sigue en `EXTERNAL_ORACLE_PENDING`.
  Documento:
  [Validation Report](04_Tests/Validation/Validation-Report.md). CSV:
  [validation-results.csv](04_Tests/Validation/validation-results.csv). Código
  del motor, Knowledge Base y arquitectura sin cambios.
- Certification Summary: 🟡 Release Candidate. Swiss Ephemeris cerró 1000 /
  1000 PASS; PureBhakti cerró 16 / 16 observancias PASS con 0 `ENGINE BUG`;
  SCS Math fue incorporado con 26 fixtures reales y diferencias analizadas;
  GCal permanece como oráculo externo pendiente. Documento:
  [CERTIFICATION](CERTIFICATION.md).
- Campaign 39.5 extraction shape: ✅ cerrada en cuatro niveles: definición
  textual, definición normativa, datos consumidos y tres casos de prueba
  máximos. El pipeline Vaiṣṇava queda como hipótesis de composición hasta
  confirmarse verso por verso.
- Semántica de `ELP1`: primera nota técnica creada desde la notice y rutinas
  oficiales de IMCCE.
- Correcciones ELP2000-85: nota técnica creada; el importador preserva datos
  crudos y el evaluador aplicará la semántica de la teoría.
- Principio lunar ELP2000: `Importador = fidelidad de datos; Evaluador =
  fidelidad de la teoría`. El módulo interno propuesto para AST-L002 es
  `elp2000_evaluator.rs`.
- ELP2000 Interpreter: responsabilidades y no-responsabilidades documentadas
  antes de crear el módulo Rust.
- [Milestone 02 - Lunar Engine First Term](00_Project/Milestone-02-Lunar-Engine-First-Term.md):
  cerrado. El primer término oficial de `ELP1` fue evaluado y validado contra
  un oráculo independiente.
- ELP1 primeros 10 términos: evaluados mediante el mismo bucle de auditoría y
  validados contra oráculo independiente. Suma parcial J2000:
  `16062.905795844735621` arcsec.
- ELP1 subconjunto de 100 términos: validado contra oráculo independiente con
  puntos de control 1, 10, 25, 50, 75 y 100. Suma parcial J2000:
  `15362.154930280750705` arcsec.
- ELP1 familia completa: 1023 / 1023 términos validados contra oráculo
  independiente. Suma total J2000: `17990.968840315887064` arcsec. Métricas:
  máximo, media y RMS `<= 1.0e-11` arcsec. Fingerprint:
  `0xa460f0ac886bf342`.
- Revisión técnica posterior a ELP1: semántica residual removida del
  importador (`time_power` y `ElpCoordinate`), oráculo ELP1 J2000 versionado y
  API general `audit_family(...)` creada para auditoría de familias.
- Dependencias de familias AST-L002: verificado en `elp82b_2` que `ELP4` es la
  siguiente familia natural de longitud (`iv = 1`, `itab = 2`) y que no depende
  numéricamente de `ELP1`; comparte argumentos comunes y aporta semántica
  short-periodic propia.
- Mapa interno AST-L002: agregado `Family -> Semantic Evaluator ->
  Contribution` en el intérprete. Las 12 familias de longitud quedan mapeadas a
  `MainProblem`, `ShortPeriodic` o `Planetary`, con orden temporal explícito
  `ElpTemporalOrder::{Order0, Order1, Order2}`.
- Arquitectura del intérprete ELP2000 congelada: una nueva familia debe aportar
  únicamente conocimiento científico nuevo y solo puede exigir nueva
  arquitectura si la teoría ELP2000 lo requiere explícitamente.
- Campaign 02: `ShortPeriodicEvaluator` implementado para el primer término
  oficial de `ELP4` y validado contra oráculo independiente. Resultado J2000:
  argumento `6.340294213756157760` rad, `xx = 0.000030000000000000` arcsec,
  contribución `0.000001712336066442` arcsec.
- Campaign 03: los primeros 10 términos oficiales de `ELP4` fueron evaluados
  reutilizando el mismo `ShortPeriodicEvaluator` y validados contra el oráculo
  independiente. Suma parcial J2000: `0.004759349973172258` arcsec. La
  semántica `ShortPeriodic` siguió pendiente de 100 términos y familia
  completa.
- Campaign 04: los primeros 100 términos oficiales de `ELP4` fueron evaluados
  reutilizando el mismo `ShortPeriodicEvaluator` y validados contra el oráculo
  independiente. Suma parcial J2000: `0.027406824891217551` arcsec. La
  semántica `ShortPeriodic` completa siguió pendiente de familia completa.
- Campaign 05: la familia completa `ELP4` fue evaluada reutilizando el mismo
  `ShortPeriodicEvaluator` y validada contra el oráculo independiente. Total:
  347 términos. Suma final J2000: `5.031975142947603175` arcsec. Semántica
  `ShortPeriodic` validada con `ELP4`.
- Semánticas validadas: ✅ Main Problem, ✅ Short Periodic, ✅ Planetary.
- Campaign 06: primer término oficial de `ELP7` validado con
  `ShortPeriodicEvaluator` y `ElpTemporalOrder::Order1`. Punto de control:
  `JD = 2488070.0` (`t = 1.0`). Resultado: argumento
  `-15581.584239863033872098` rad, `xx = 0.000030000000000000` arcsec,
  contribución `0.000019675814376061` arcsec.
- Campaign 07: los primeros 10 términos oficiales de `ELP7` fueron evaluados
  reutilizando el mismo `ShortPeriodicEvaluator` con `ElpTemporalOrder::Order1`.
  Punto de control: `JD = 2488070.0` (`t = 1.0`). Suma parcial:
  `0.000621037403414594` arcsec.
- Campaign 08: la familia completa `ELP7` fue evaluada reutilizando el mismo
  `ShortPeriodicEvaluator` con `ElpTemporalOrder::Order1`. Total: 14 términos.
  Punto de control: `JD = 2488070.0` (`t = 1.0`). Suma final:
  `0.000575433851509927` arcsec.
- ShortPeriodic: ✅ `Order0` validado con `ELP4`; ✅ `Order1` validado con
  `ELP7`.
- Campaign 09: primer término oficial de `ELP22` validado con
  `ShortPeriodicEvaluator` y `ElpTemporalOrder::Order0`. Punto de control:
  `JD = 2451545.0`. Resultado: argumento `4.539260219449968048` rad,
  `xx = 0.000040000000000000` arcsec, contribución
  `-0.000039402024505457` arcsec.
- Campaign 10: familia completa `ELP22` validada con `ShortPeriodicEvaluator`
  y `ElpTemporalOrder::Order0`. Total: 3 términos. Suma final J2000:
  `0.000438385678939491` arcsec.
- ShortPeriodic consolidada: `Order0` validado con `ELP4`, `ELP22`, `ELP28` y
  `ELP31`; `Order1` validado con `ELP7` y `ELP25`; `Order2` validado con
  `ELP34`.
- Planetary consolidada: `Order0` validado con `ELP10` y `ELP16`; `Order1`
  validado con `ELP13` y `ELP19`.
- Campaign 22: AST-L002 integrado y validado con las 12 familias de longitud:
  `ELP1`, `ELP4`, `ELP7`, `ELP10`, `ELP13`, `ELP16`, `ELP19`, `ELP22`,
  `ELP25`, `ELP28`, `ELP31` y `ELP34`.

## Próximo Enfoque

Pancanga Engine está en RC1. El desarrollo está cerrado. El foco restante es
exclusivamente certificación final y release.

Ruta final:

```text
Campaign 46F.2
Civil Configuration Audit

↓

Campaign 46C Final
Full External Certification

↓

Campaign 46BETA
Pancanga Engine RC1 Experience

↓

Campaign 46R
Release Audit

↓

Campaign 47
v1.0 Release
```

No se agregan funcionalidades nuevas antes de v1.0.

Documento operativo:

[Final Certification Stage](00_Project/Final-Certification-Stage.md)
