# Índice de Especificaciones

Este índice registra los documentos normativos y de arquitectura de Pancanga
Engine.

## PES

| Documento | Título | Estado |
|---|---|---|
| [PES-0001](PES/PES-0001.md) | Project Foundation | 🔒 Congelado |
| [PES-0002](PES/PES-0002.md) | Sistema Temporal | 🟡 En revisión |

## Arquitectura

| Documento | Título | Estado |
|---|---|---|
| [ARCH-0001](ARCH/ARCH-0001.md) | Arquitectura General | 🔒 Congelado |
| [ARCH-0002](ARCH/ARCH-0002.md) | Arquitectura del Subsistema Astronómico | 🟡 En revisión final |

## ADR

| Documento | Título | Estado |
|---|---|---|
| [ADR-0002](../05_Documentation/ADR/ADR-0002.md) | Operaciones con Ángulos Mediante Tipos Fuertes | Aceptado |
| [ADR-0003](../05_Documentation/ADR/ADR-0003.md) | Modelo Astronómico Basado en Meeus y Ayanāṁśa Lahiri | 🟡 En revisión |
| [ADR-0004](../05_Documentation/ADR/ADR-0004.md) | Adoptar VSOP87D para la Cadena Solar | Aceptado |
| [ADR-0005](../05_Documentation/ADR/ADR-0005.md) | Modelo de Aberración Solar | Aceptado |
| [ADR-0006](../05_Documentation/ADR/ADR-0006.md) | Modelo de Nutación en Longitud | Aceptado |
| [ADR-0007](../05_Documentation/ADR/ADR-0007.md) | Referencia Externa de Validación Científica | Aceptado |

## RFC

| Documento | Título | Estado |
|---|---|---|
| [RFC-0001](../05_Documentation/RFC/RFC-0001.md) | Selección de la Variante VSOP87 | ✅ Cerrada |

## Algoritmos

| Documento | Título | Estado |
|---|---|---|
| [AST-0001](../05_Documentation/Algorithms/AST-0001.md) | Julian Centuries | 🟢 Implementado, validación científica pendiente |
| [AST-0002](../05_Documentation/Algorithms/AST-0002.md) | Longitud Heliocéntrica de la Tierra (VSOP87D) | ✅ Validado contra IMCCE |
| [AST-0003](../05_Documentation/Algorithms/AST-0003.md) | Longitud Geocéntrica del Sol | ✅ Validado por identidad geométrica |
| [AST-0004](../05_Documentation/Algorithms/AST-0004.md) | Radio Vector de la Tierra | ✅ Validado contra IMCCE |
| [AST-0005](../05_Documentation/Algorithms/AST-0005.md) | Aberración Solar | ✅ Validado por fórmula de Meeus |
| [AST-0006](../05_Documentation/Algorithms/AST-0006.md) | Nutación en Longitud | ✅ Validado por serie truncada de Meeus |
| [AST-0007](../05_Documentation/Algorithms/AST-0007.md) | Longitud Aparente del Sol | ✅ Validado por integración del pipeline solar |
| [Solar Pipeline](../05_Documentation/Algorithms/Solar-Pipeline.md) | Mapa de Dependencias de la Cadena Solar | 🔒 Cerrado v1.0 |
| [Lunar Pipeline](../05_Documentation/Algorithms/Lunar-Pipeline.md) | Mapa de Dependencias de la Cadena Lunar | 🟡 En desarrollo |
| [ELP2000 Import Pipeline](../05_Documentation/Algorithms/ELP2000-Import-Pipeline.md) | Importación reproducible de coeficientes lunares | ✅ Ejecutado contra IMCCE |
| [ELP2000 Dataset Audit](../05_Documentation/Algorithms/ELP2000-Dataset-Audit.md) | Auditoría estructural del dataset lunar | ✅ Completada |
| [AST-L001](../05_Documentation/Algorithms/AST-L001.md) | Fundamental Lunar Arguments | ✅ Validado por polinomios de Meeus |
| [AST-L002](../05_Documentation/Algorithms/AST-L002.md) | ELP2000 Periodic Longitude Correction | ✅ Cerrado |
| AST-L003 | Lunar Mean Longitude W1 | ✅ Implementado |
| AST-L004 | Geocentric Lunar Longitude | ✅ Implementado |
| AST-L005 | Apparent Lunar Longitude | ✅ Implementado |
| Lunar Engine API | `astronomy::moon::apparent_longitude()` | ✅ Implementado |
| Campaign 27 | Lunar-Solar Elongation | ✅ Implementado |
| Campaign 28 | Astronomical Tithi Index | ✅ Implementado |
| Campaign 29 | Astronomical Tithi | ✅ Implementado |

## Milestones Técnicos

| Documento | Título | Estado |
|---|---|---|
| [M2-Core-Math](M2-Core-Math.md) | Core Math | En desarrollo |
| [M3-Astronomical-Time](M3-Astronomical-Time.md) | Astronomical Time | En desarrollo |
| [M4-Julian-Day](M4-Julian-Day.md) | Julian Day | En desarrollo |
