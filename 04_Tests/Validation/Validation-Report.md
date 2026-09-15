# Full External Validation

Campaign: 46C

Status: 🟡 OPEN

## Objetivo

Ejecutar la certificación externa completa de Pancanga Engine usando
exclusivamente:

```text
Swiss Ephemeris
PureBhakti fixtures
SCS Math fixtures
GCal fixtures
```

Campaign 46C no modifica:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Knowledge Base
```

## Resultado Actual

Campaign 46C permanece abierta.

Motivo:

```text
Swiss Oracle:
PASS

GCal Oracle:
EXTERNAL_ORACLE_PENDING

PureBhakti Oracle:
PASS / EDITORIAL DIFFERENCES DOCUMENTED

SCS Math Oracle:
OPEN / DIFFERENCES ANALYZED
```

Se ejecutó comparación externa real contra Swiss Ephemeris y cerró como
`PASS`. Se incorporó el primer HTML real de PureBhakti y se ejecutó una primera
comparación. PureBhakti cerró como `PASS` para observancias, con diferencias
editoriales de Parāṇa documentadas. SCS Math fue incorporado como segundo
oráculo Vaiṣṇava real desde el PDF anual de Sri Chaitanya Saraswat Math; su
primera comparación queda abierta con diferencias analizadas y sin bugs
confirmados. La certificación completa no puede cerrarse porque:

1. `04_Tests/SCSMath/SCSMath-Difference-Report.md` recomienda auditar la
   configuración civil Nabadwip/IST antes de cerrar SCS Math como `PASS`.
   Vyañjulī ya no queda como bloqueo doctrinal después de ISSUE-VAI-001.
2. `04_Tests/Vaishnava/GCal/gcal-fixtures.csv` contiene únicamente plantillas
   `EXTERNAL_ORACLE_PENDING`.
3. El importador GCal está listo, pero no existe todavía una fuente local real
   en `02_Research/GCal`.
4. GCal sigue pendiente de datos reales.

No se inventaron datos ni resultados.

## Insumos Requeridos

| Oráculo | Archivo | Estado |
| ------- | ------- | ------ |
| Swiss Ephemeris | `04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv` | PASS |
| PureBhakti | `04_Tests/PureBhakti/purebhakti-validation.csv` | PASS; 16/16 observance date PASS; Parāṇa editorial differences documented |
| SCS Math | `04_Tests/SCSMath/scsmath-validation.csv` | OPEN; 26 real fixtures; 23/26 observance date PASS (rerun 2026-09-14 post-ORDEN-6, was 11/26); 3 remaining differences; 0 confirmed ENGINE BUG |
| GCal | `04_Tests/Vaishnava/GCal/gcal-fixtures.csv` | Importador listo; fuente real pendiente |

## Rerun post-ORDEN-6 (2026-09-14)

`ORDEN-6` corrigió un defecto de day-carry en `calendar::sunrise()` (commit
`2a6d5fd`, goldens corregidos en `ed9450a`). Este campaign 46C Final reejecutó
los dos validadores de instrumentación (`scsmath_validation.rs`,
`purebhakti_validation.rs`) contra el engine post-ORDEN-6, sin modificar
código de motor:

```text
SCS Math observance PASS: 11 / 26 -> 23 / 26
PureBhakti observance PASS: 16 / 16 -> 16 / 16 (0 filas cambiadas, salida
byte-idéntica)
Swiss Ephemeris: sin rerun — swiss_validation.rs no invoca sunrise() en
ninguna ruta (confirmado leyendo el binario), por lo que queda fuera de
alcance de ORDEN-6.
```

Las 12 filas SCS Math que pasaron de `DIFFERENCE` a `PASS` tienen una
clasificación estática `DOC-001` registrada en
`04_Tests/Vaishnava/normative-observance-certification.csv` que ahora queda
contradicha por la medición. Ese archivo no fue modificado por este campaign;
la reconciliación es una decisión del owner. Ver el reporte de campaign para
la lista completa.

Este rerun no cierra ningún ítem del Release Gate en `CERTIFICATION.md`; esa
decisión pertenece al owner.

## Clasificación De Diferencias

Toda diferencia futura debe clasificarse únicamente como:

```text
PASS
ASTRONOMY
CALENDAR
NORMATIVE
ORACLE DIFFERENCE
ENGINE BUG
```

Nunca se debe modificar el código antes de clasificar la diferencia.

## Reporte CSV

Archivo:

```text
04_Tests/Validation/validation-results.csv
```

El CSV actual registra que Swiss y PureBhakti cerraron como `PASS`, SCS Math
permanece abierto con diferencias analizadas, y GCal permanece pendiente de
datos externos reales.

## Criterio De Cierre

Campaign 46C solo puede cerrarse como `PASS` si:

```text
Swiss PASS
PureBhakti PASS
SCS Math PASS or documented non-critical oracle/calendar differences
GCal PASS
Sin ENGINE BUG críticos
```

Si aparece cualquier diferencia, Campaign 46C permanece:

```text
OPEN
```

y debe contener la lista completa de diferencias clasificadas.

## Estado Técnico

```text
Código del motor:
AST-L006 IMPLEMENTADO

GCal Importer:
LISTO

PureBhakti Extractor:
LISTO

PureBhakti Validator:
LISTO

SCS Math Extractor:
LISTO

SCS Math Validator:
LISTO

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO

Release:
ESPERANDO CERTIFICACIÓN
```
