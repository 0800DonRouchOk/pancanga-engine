# PureBhakti Fixtures

Campaign: 46D.1

Status: 🟡 Real Fixture Imported / Review Pending

## Objetivo

Convertir un calendario HTML Vaiṣṇava real de PureBhakti en fixtures
estructurados para certificación externa.

PureBhakti es oráculo de validación. No es autoridad normativa.

## Estado Local

```text
HTML PureBhakti:
DISPONIBLE

Extractor:
LISTO

CSV:
GENERADO CON DATOS REALES

Código del motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO
```

Fuente local:

```text
02_Research/PureBhakti/Calendario-Vaishnava-2026.html
```

El extractor produjo 16 fixtures reales de Ekādaśī para Buenos Aires,
Argentina. Las filas conservan estado `IMPORTED_REQUIRES_REVIEW` hasta revisión
manual del calendario externo.

## Archivo CSV

```text
04_Tests/PureBhakti/purebhakti-fixtures.csv
```

Cada fixture contiene:

```text
case_id
date
location
latitude
longitude
timezone
name
is_mahadvadasi
mahadvadasi_type
parana_start
parana_end
masa
paksha
source_file
status
```

## Extractor

Herramienta:

```text
03_Source/rust/crates/pancanga-engine/examples/purebhakti_html_extract.rs
```

Uso esperado:

```text
cargo run -p pancanga-engine --example purebhakti_html_extract -- \
  --input ../../02_Research/PureBhakti/Calendario-Vaishnava-2026.html \
  --output ../../04_Tests/PureBhakti/purebhakti-fixtures.csv
```

Si se omite `--input`, el extractor usa por defecto:

```text
../../02_Research/PureBhakti/Calendario-Vaishnava-2026.html
```

## Estados

| Estado | Significado |
| ------ | ----------- |
| `EXTERNAL_ORACLE_PENDING` | No existe HTML local real |
| `IMPORTED_REQUIRES_REVIEW` | El dato fue extraído de una fuente real y debe revisarse antes de certificación |

El extractor nunca inventa:

- ubicación;
- latitud;
- longitud;
- zona horaria;
- māsa;
- pakṣa;
- Mahādvādaśī;
- Parāṇa.

Si esos datos no están declarados en el HTML, quedan vacíos y la fila permanece
en revisión.

## Regla De Uso

El HTML de PureBhakti se convierte en fixture reproducible. El fixture no se
edita para hacer coincidir el motor.

Si el motor difiere del fixture, la diferencia debe clasificarse antes de tocar
código:

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

Campaign 46D puede pasar a `PASS` cuando:

1. las 16 filas importadas sean revisadas contra el HTML fuente;
2. las fechas, nombres, Mahādvādaśī y Parāṇa requeridos estén confirmados;
3. no haya datos inventados ni rellenados sin fuente;
4. la comparación con el motor tenga diferencias clasificadas.

## Resultado De Extracción

```text
Fixtures reales:
16

Ubicación:
Buenos Aires, Argentina

Coordenadas:
-34.616667, -58.383333

Zona horaria:
UTC −3 (America/Argentina/Buenos_Aires)

Estado:
IMPORTED_REQUIRES_REVIEW
```

Campos poblados en las 16 filas:

```text
fecha
nombre
Mahādvādaśī
tipo de Mahādvādaśī
Parāṇa inicio
Parāṇa fin
māsa
pakṣa
ubicación
zona horaria
```

## Primera Comparación

Reporte:

```text
04_Tests/PureBhakti/purebhakti-validation.csv
```

Resumen:

```text
Observance date:
16 / 16 PASS

Parāṇa:
1 / 16 PASS
15 / 16 CONFIGURATION DIFFERENCE

Mahādvādaśī:
1 fixture marca Triṣpṛśā; la derivación automática de hechos Mahādvādaśī no
forma parte de este validador.
```

Las diferencias de Parāṇa quedan clasificadas antes de modificar código. No se
modificó el motor para coincidir con PureBhakti.

## Estado De Cierre

```text
Campaign 46D.1:
Fixture Imported

PureBhakti Oracle:
Real fixture available

Motivo:
Pendiente revisión manual de fixtures y análisis de diferencias de Parāṇa.
```
