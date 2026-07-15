# PureBhakti Parāṇa Difference Analysis

Campaign: 46D.2

Status: ✅ CLOSED / Editorial Differences Documented

## Objetivo

Analizar las 15 diferencias de Parāṇa detectadas contra el calendario
PureBhakti 2026 sin modificar:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Knowledge Base
```

## Insumos

```text
Fixture:
04_Tests/PureBhakti/purebhakti-fixtures.csv

Comparación:
04_Tests/PureBhakti/purebhakti-validation.csv

Análisis:
04_Tests/PureBhakti/purebhakti-parana-difference-analysis.csv
```

## Resultado General

```text
Fechas de observancia:
16 / 16 PASS

ENGINE BUG:
0

Parāṇa PASS completo:
1 / 16

Parāṇa con diferencia:
15 / 16
```

## Hallazgo Principal

Las diferencias no afectan la determinación del día de Ekādaśī. El corazón del
Motor Vaiṣṇava coincide con PureBhakti para todas las observancias importadas.

Las diferencias están concentradas en el límite final de Parāṇa:

```text
Inicio de Parāṇa:
mayormente igual o ±1 minuto

Fin de Parāṇa:
PureBhakti publica una ventana práctica más corta en 13 casos
```

Esto no tiene el patrón de un error en HBV-EK-001 a HBV-EK-004, ni de un error
astronómico general. El patrón apunta a una diferencia de configuración o
política editorial para el límite superior mostrado al usuario.

## Clasificación

| Clasificación | Casos | Interpretación |
| ------------- | ----- | -------------- |
| `Editorial Policy` | 13 | PureBhakti publica un límite final más corto que el fin astronómico de Dvādaśī usado por HBV-EK-005. |
| `Rounding Difference` | 2 | La diferencia es solo de un minuto. |
| `ENGINE BUG` | 0 | No se detectó bug crítico del motor. |

## Casos De Redondeo

```text
PB-0003
Parama (Kāmadā) Ekādaśī
PureBhakti: 07:58–11:09
Motor:      07:57–11:08

PB-0016
Mokṣadā Ekādaśī
PureBhakti: 05:37–09:09
Motor:      05:37–09:08
```

Estos casos deben revisarse como diferencia de redondeo o de convención de
minuto, no como cambio doctrinal.

## Casos De Ventana Práctica Más Corta

En 13 casos, el inicio coincide o difiere solo por un minuto, pero PureBhakti
cierra la ventana mucho antes que el límite final calculado por el motor.

El motor implementa la Knowledge Base v1.0:

```text
Parāṇa comienza después de amanecer y Hari-vāsara.
Parāṇa termina antes de que termine Dvādaśī.
```

PureBhakti parece publicar una ventana práctica más restringida. La fuente HTML
no declara en el propio archivo cuál es esa regla editorial adicional.

### Caso RC1 Experience: PB-0007

La primera diferencia observada durante el uso privado de la RC1 Experience fue
documentada como ficha específica:

```text
04_Tests/PureBhakti/PureBhakti-PB-0007-Parana-Diagnostic.md
```

Resultado:

```text
Observancia:
PASS

PureBhakti:
07:51–11:13

Pancanga Engine:
07:51–23:32

Diagnóstico:
parana_end = dvadasi_end

Clasificación:
Editorial Policy

ENGINE BUG:
0
```

### Origen Probable Del Límite Práctico

La Campaign 46BETA.3 analizó el límite publicado de PB-0007:

```text
PureBhakti:
07:51–11:13

Pancanga Engine:
07:51–23:32
```

El diagnóstico mostró que `11:13` coincide exactamente con el final del primer
tercio del día civil:

```text
sunrise:
07:40

sunset:
18:18

primer tercio del día:
07:40–11:13
```

Documento:

```text
04_Tests/PureBhakti/PureBhakti-Practical-Parana-Origin.md
```

Esta coincidencia se clasifica como política de presentación/editorial mientras
no exista una fuente normativa explícita que obligue a modificar HBV-EK-005.

## Qué NO Se Concluye

No se concluye que:

```text
HBV-EK-005 esté mal.
PureBhakti esté mal.
El motor deba modificarse.
La Knowledge Base deba cambiar.
```

La diferencia queda documentada hasta identificar si PureBhakti aplica una regla de
presentación práctica adicional, por ejemplo un límite de mañana, un límite por
tercio del día, una convención de calendario externa o una política editorial.

## Decisión De Certificación

Estas diferencias no bloquean la certificación PureBhakti porque la
certificación principal evalúa si el motor toma las mismas decisiones de
observancia que el calendario externo.

```text
Fechas de observancia:
16 / 16 PASS

ENGINE BUG:
0
```

El análisis de Parāṇa queda cerrado como diferencia editorial/redondeo
documentada. No se modifica el motor porque no existe evidencia de que
HBV-EK-005 esté implementado incorrectamente.

## Próximo Paso

Antes de tocar código, se necesita una fuente o especificación externa que
explique cómo PureBhakti determina el límite final publicado de Parāṇa.

Si esa política se confirma como una convención de presentación, debe modelarse
como configuración de oráculo o capa de presentación, no como cambio a
HBV-EK-005.

## Estado De Cierre

```text
Campaign 46D.2:
CLOSED

Parāṇa Difference Analysis:
COMPLETED FOR CURRENT FIXTURE

Certification Impact:
NON-BLOCKING EDITORIAL DIFFERENCE

Motor:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO
```
