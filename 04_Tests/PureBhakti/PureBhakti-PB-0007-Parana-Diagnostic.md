# PureBhakti PB-0007 Parāṇa Diagnostic

Campaign: 46BETA.2

Status: ✅ CLOSED / Editorial Policy

## Caso

```text
Fixture:
PB-0007

Observancia:
Kāmikā Ekādaśī

Lugar:
Buenos Aires, Argentina

Fecha de ayuno:
2026-08-08

Fecha de Parāṇa:
2026-08-09
```

## Comparación

```text
PureBhakti:
07:51–11:13

Pancanga Engine:
07:51–23:32
```

La fecha de observancia coincide.

```text
Observancia:
PASS

ENGINE BUG:
0
```

## Diagnóstico Del Motor

```text
sunrise:
07:40

sunset:
18:18

first_third_of_day_end:
11:13

hari_vasara_end:
07:51

dvadasi_start:
2026-08-09 02:37

dvadasi_end:
2026-08-09 23:32

dvadasi_duration_hours:
20.930370

parana_start:
07:51

parana_end:
23:32

parana_mode:
Standard
```

## Interpretación

El inicio de Parāṇa del motor coincide con PureBhakti.

```text
Motor:
max(sunrise, hari_vasara_end)
= 07:51

PureBhakti:
07:51
```

El límite final del motor corresponde exactamente al fin de Dvādaśī.

```text
parana_end = dvadasi_end = 23:32
```

Esto confirma que Pancanga Engine está aplicando literalmente HBV-EK-005:

```text
Parāṇa comienza después de amanecer y Hari-vāsara.
Parāṇa termina antes de que termine Dvādaśī.
```

PureBhakti publica un límite final más corto:

```text
11:13
```

Ese límite coincide exactamente con el final del primer tercio del día civil:

```text
sunrise:
07:40

sunset:
18:18

primer tercio del día:
07:40–11:13
```

El archivo HTML de PureBhakti no declara en el propio calendario cuál es la
fuente o política editorial usada para ese límite superior. Por lo tanto, esta
coincidencia se documenta como origen práctico probable, no como regla normativa
HBV. No hay evidencia para modificar el motor ni la Knowledge Base.

## Clasificación

```text
ENGINE BUG:
NO

CONFIGURATION:
NO confirmado

EDITORIAL POLICY:
YES

Regla adicional documentable:
Primer tercio del día civil como límite práctico probable.
Pendiente de fuente explícita.
```

## Decisión

No modificar:

```text
Astronomy Engine
Calendar Engine
Vaiṣṇava Engine
Knowledge Base
```

Este caso queda cerrado como diferencia editorial en el límite superior de la
ventana publicada por PureBhakti.
