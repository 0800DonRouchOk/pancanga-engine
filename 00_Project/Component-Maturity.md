# Component Maturity

Estado: 🟡 En seguimiento

Este documento registra la madurez relativa de cada componente principal de
Pancanga Engine.

## Estado Actual

| Componente | Versión | Estado |
|---|---:|---|
| Astronomy Engine | v1.0 | ✅ Stable |
| Calendar Engine | v1.0 | ✅ Stable |
| Sources Layer | v1.0 | ✅ Stable |
| Knowledge Base | v1.0 | 🔒 Stable |
| Vaiṣṇava Engine | v0.1 | 🚧 Planned |

## Lectura

```text
Astronomy Engine
        ↓
Calendar Engine
        ↓
Vaiṣṇava Engine
```

Y, en paralelo:

```text
Fuentes
        ↓
Knowledge Base
        ↓
Especificación
        ↓
Código
```

La segunda cadena es documental, no software. Su función es asegurar que cada
regla implementada pueda responder:

```text
¿Por qué el algoritmo hace esto?
```

## Regla

La implementación del Motor Vaiṣṇava ya cuenta con Knowledge Base v1.0. La fase
documental queda cerrada y el siguiente trabajo es implementación.

Antes de iniciar código Vaiṣṇava, la Knowledge Base debe superar esta revisión:

```text
¿Un desarrollador que nunca leyó el Hari-bhakti-vilāsa puede implementar
correctamente el Motor Vaiṣṇava usando únicamente la Knowledge Base?
```

Resultado:

```text
Knowledge Base v1.0
READY
```
