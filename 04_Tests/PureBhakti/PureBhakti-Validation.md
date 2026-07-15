# PureBhakti Validation

Campaign: 46D.1

Status: ✅ PASS / Editorial Differences Documented

## Objetivo

Comparar el Motor Vaiṣṇava contra el primer fixture real extraído del calendario
PureBhakti 2026 para Buenos Aires, sin modificar el motor ni la Knowledge Base.

PureBhakti es oráculo externo de validación. No es autoridad normativa.

## Insumos

```text
HTML fuente:
02_Research/PureBhakti/Calendario-Vaishnava-2026.html

Fixture estructurado:
04_Tests/PureBhakti/purebhakti-fixtures.csv

Comparación:
04_Tests/PureBhakti/purebhakti-validation.csv
```

## Herramientas

```text
Extractor:
03_Source/rust/crates/pancanga-engine/examples/purebhakti_html_extract.rs

Validador:
03_Source/rust/crates/pancanga-engine/examples/purebhakti_validation.rs
```

## Resultado De Certificación

```text
Fixtures reales:
16

Observance date:
16 / 16 PASS

Engine Bugs:
0

Parāṇa:
1 / 16 PASS
15 / 16 EDITORIAL / ROUNDING DIFFERENCE DOCUMENTED

Mahādvādaśī:
1 fixture externo marca Triṣpṛśā.
```

Campaign 46D.2 agregó el análisis específico de Parāṇa:

```text
04_Tests/PureBhakti/PureBhakti-Parana-Difference-Analysis.md
04_Tests/PureBhakti/purebhakti-parana-difference-analysis.csv
```

Resumen:

```text
Editorial Policy:
13

Rounding Difference:
2

ENGINE BUG:
0
```

## Interpretación

La fecha de observancia publicada por PureBhakti coincide con el motor en los
16 casos. El validador acepta dos rutas:

```text
Ekādaśī válida en el día actual
→ observancia en Ekādaśī

Ekādaśī Viddhā el día anterior
→ observancia desplazada a Dvādaśī
```

Las diferencias registradas están concentradas en la ventana de Parāṇa. El
motor calcula el límite final operativo como fin de Dvādaśī según HBV-EK-005,
mientras PureBhakti publica una ventana práctica más corta en 15 casos.

No se modificó el motor para acomodar estas diferencias. Quedan documentadas
como diferencias externas de publicación:

```text
Editorial Policy:
13

Rounding Difference:
2
```

Estas diferencias no bloquean la certificación PureBhakti porque:

```text
Fechas de observancia:
16 / 16 PASS

ENGINE BUG:
0
```

PureBhakti queda certificado como oráculo de observancia para este fixture. La
ventana de Parāṇa permanece documentada como diferencia editorial/redondeo, no
como error del motor.

## Mahādvādaśī

El fixture `PB-0009` marca:

```text
Triṣpṛśā
```

El Motor Vaiṣṇava ya implementa HBV-EK-004, pero este validador no deriva
automáticamente hechos Mahādvādaśī desde una fecha civil. Esa derivación queda
fuera del alcance de Campaign 46D.1 y no se resuelve por conveniencia.

## Código Y Knowledge Base

```text
Código del motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO
```

## Estado De Cierre

```text
Campaign 46D.1:
PASS

PureBhakti Certification:
PASS

Observance:
16 / 16 PASS

Engine Bugs:
0

Parāṇa:
Editorial / rounding differences documented
```
