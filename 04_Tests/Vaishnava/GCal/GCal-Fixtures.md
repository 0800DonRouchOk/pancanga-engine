# GCal Fixtures

Campaign: 46B / 46B.1

Status: 🟡 Importer Ready / External Oracle Pending

## Objetivo

Construir la colección oficial de fixtures para validar el Motor Vaiṣṇava contra
GCal y calendarios Vaiṣṇavas confiables.

GCal es oráculo de validación. No es autoridad normativa.

## Estado Local

```text
Fixtures GCal:
NO DISPONIBLES

CSV:
GENERADO COMO PLANTILLA OFICIAL

Importador:
LISTO

Código del motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO
```

El directorio `02_Research/GCal` contiene únicamente `.gitkeep`; por lo tanto
no se registran fechas, resultados ni ventanas de Parāṇa todavía.

Campaign 46B.1 agregó infraestructura para importar fuentes reales sin crear
fixtures sintéticos. Si no existe una fuente local oficial, el importador deja
el CSV en estado `EXTERNAL_ORACLE_PENDING`.

## Archivo CSV

```text
04_Tests/Vaishnava/GCal/gcal-fixtures.csv
```

Cada fixture debe contener:

```text
case_id
category
civil_date
location
timezone
sunrise
ekadasi
viddha
mahadvadasi
parana
gcal_source
status
notes
```

## Importador

Herramienta:

```text
03_Source/rust/crates/pancanga-engine/examples/gcal_fixture_import.rs
```

Uso esperado:

```text
cargo run -p pancanga-engine --example gcal_fixture_import -- \
  --input ../../02_Research/GCal/<calendario-oficial>.ics \
  --output ../../04_Tests/Vaishnava/GCal/gcal-fixtures.csv
```

Si se omite `--input`, el importador busca automáticamente un `.ics` o `.csv`
en:

```text
02_Research/GCal
```

Formatos aceptados:

| Formato | Uso |
| ------- | --- |
| `.csv` con columnas de `gcal-fixtures.csv` | Fixture estructurado listo para validación |
| `.csv` de eventos (`date`, `summary`, `description`, `location`, `timezone`, `source`) | Fuente oficial semi-estructurada; requiere revisión |
| `.ics` | Exportación de calendario; importa eventos y marca revisión necesaria |

Los eventos importados desde `.ics` o desde CSV de eventos quedan con estado:

```text
IMPORTED_REQUIRES_REVIEW
```

Ese estado significa que el dato proviene de una fuente real, pero todavía debe
ser revisado antes de usarlo como fixture de certificación. El importador nunca
rellena amanecer, Viddhā, Mahādvādaśī o Parāṇa si la fuente no los declara.

## Categorías Obligatorias

| ID | Categoría | Estado |
| -- | --------- | ------ |
| GCAL-001 | Ekādaśī normal | EXTERNAL_ORACLE_PENDING |
| GCAL-002 | Ekādaśī viddhā | EXTERNAL_ORACLE_PENDING |
| GCAL-003 | Observancia desplazada | EXTERNAL_ORACLE_PENDING |
| GCAL-004 | Unmīlanī | EXTERNAL_ORACLE_PENDING |
| GCAL-005 | Vyañjulī | EXTERNAL_ORACLE_PENDING |
| GCAL-006 | Triṣpṛśā | EXTERNAL_ORACLE_PENDING |
| GCAL-007 | Pakṣavardhinī | EXTERNAL_ORACLE_PENDING |
| GCAL-008 | Jayā | EXTERNAL_ORACLE_PENDING |
| GCAL-009 | Vijayā | EXTERNAL_ORACLE_PENDING |
| GCAL-010 | Jayantī | EXTERNAL_ORACLE_PENDING |
| GCAL-011 | Pāpanāśinī | EXTERNAL_ORACLE_PENDING |
| GCAL-012 | Dvādaśī corta | EXTERNAL_ORACLE_PENDING |

## Reglas De Uso

1. No inventar datos GCal.
2. No convertir GCal en autoridad normativa.
3. No modificar el Motor Vaiṣṇava para coincidir con GCal sin clasificar la
   diferencia.
4. Toda diferencia debe registrarse antes de cambiar código.
5. Si un caso de Vyañjulī difiere, enlazar `ISSUE-VAI-001`.

## Clasificación De Diferencias

Al ejecutar Campaign 46C, cada diferencia debe clasificarse como:

```text
ASTRONOMY
CALENDAR
NORMATIVE
TIMEZONE / CIVIL TIME
ORACLE DIFFERENCE
ENGINE BUG
UNRESOLVED
```

## Criterio Para Promover A PASS

Campaign 46B puede pasar a `PASS` cuando:

1. Cada categoría obligatoria tenga al menos un fixture real.
2. Cada fixture registre fuente GCal o calendario Vaiṣṇava confiable.
3. Cada fixture tenga fecha civil, ubicación, zona horaria, amanecer,
   clasificación y Parāṇa cuando corresponda.
4. No haya datos inventados ni inferidos sin fuente.

## Estado De Cierre

```text
Campaign 46B:
External Oracle Pending

Motivo:
No hay fixtures GCal locales.

Campaign 46B.1:
Importer Ready

Motivo:
No hay archivo GCal real en `02_Research/GCal`; el importador conserva el
estado pendiente sin inventar datos.
```
