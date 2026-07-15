# Catálogo de Algoritmos Astronómicos

Este directorio contiene las fichas técnicas de algoritmos astronómicos usados
por Pancanga Engine.

A partir de M5, cada algoritmo deberá tener una ficha propia antes o junto con
su implementación.

Ningún algoritmo se considera científicamente validado únicamente porque
compila, tiene tests internos o pasa el CI. La validación científica requiere
referencias externas, tolerancias explícitas y rango de validez documentado.

## Formato de Ficha

```text
Algorithm ID
Nombre
Fuente
Entrada
Salida
Precisión
Rango de validez
Tolerancia aceptada
Datos de referencia
Dependencias
Usado por
Estado
Benchmarks
```

## Pipeline Obligatorio

```text
Algorithm
    ↓
Mathematical derivation
    ↓
Scientific references
    ↓
Rust implementation
    ↓
Reference validation
    ↓
Benchmark
```

## Mapas

- [Solar Pipeline](Solar-Pipeline.md)
- [Lunar Pipeline](Lunar-Pipeline.md)
- [Calendar Pipeline](Calendar-Pipeline.md)
- [Vaishnava Rules](Vaishnava-Rules.md)
- [Vaiṣṇava Normative Authority](Vaishnava-Authority.md)
- [Ekādaśī Rules](Ekadasi-Rules.md)
- [Hari-bhakti-vilāsa Source Extraction](Hari-bhakti-vilasa-Source-Extraction.md)
- [ELP2000 Import Pipeline](ELP2000-Import-Pipeline.md)
- [ELP2000 Dataset Audit](ELP2000-Dataset-Audit.md)

## Estados

- ⬜ No implementado
- 🟡 En desarrollo
- 🟢 Implementado, validación científica pendiente
- ✅ Validado contra referencia externa
- ✅ Científicamente validado

## Algoritmos

| ID | Nombre | Estado |
|---|---|---|
| [AST-0001](AST-0001.md) | Julian Centuries | 🟢 Implementado, validación científica pendiente |
| [AST-0002](AST-0002.md) | Longitud Heliocéntrica de la Tierra (VSOP87D) | ✅ Validado contra referencia externa |
| [AST-0003](AST-0003.md) | Longitud Geocéntrica del Sol | ✅ Validado por identidad geométrica |
| [AST-0004](AST-0004.md) | Radio Vector de la Tierra | ✅ Validado contra referencia externa |
| [AST-0005](AST-0005.md) | Aberración Solar | ✅ Validado por fórmula de Meeus |
| [AST-0006](AST-0006.md) | Nutación en Longitud | ✅ Validado por serie truncada de Meeus |
| [AST-0007](AST-0007.md) | Longitud Aparente del Sol | ✅ Validado por integración del pipeline solar |
| [ELP2000 Import Pipeline](ELP2000-Import-Pipeline.md) | Importación reproducible de coeficientes lunares | ✅ Ejecutado contra IMCCE |
| [AST-L001](AST-L001.md) | Fundamental Lunar Arguments | ✅ Validado por polinomios de Meeus |
| [AST-L002](AST-L002.md) | ELP2000 Periodic Longitude Correction | ✅ Cerrado |
| AST-L003 | Lunar Mean Longitude W1 | ✅ Implementado |
| AST-L004 | Geocentric Lunar Longitude | ✅ Implementado |
| AST-L005 | Apparent Lunar Longitude | ✅ Implementado |
| [AST-L006](AST-L006-ELP2000-Frame-Transformation.md) | ELP2000 Frame Transformation | ✅ Validado contra Swiss Ephemeris |
| Lunar Engine API | `astronomy::moon::apparent_longitude()` | ✅ Implementado |
| Campaign 27 | Lunar-Solar Elongation | ✅ Implementado |
| Campaign 28 | Astronomical Tithi Index | ✅ Implementado |
| Campaign 29 | Astronomical Tithi | ✅ Implementado |
| Campaign 31 | Calendar Engine Boundary | 🔒 Congelado |
| Campaign 32 | Sunrise Engine | ✅ Implementado |
| Campaign 33 | Tithi at Sunrise | ✅ Implementado |
| Campaign 34 | Tithi Transition Within the Civil Day | ✅ Implementado |
| Campaign 35 | Tithi Presence in Civil Day | ✅ Implementado |
| [Campaign 36](Vaishnava-Rules.md) | Vaiṣṇava Rules Specification | ✅ Cerrada como frontera normativa |
| [Campaign 37](Vaishnava-Authority.md) | Vaiṣṇava Normative Authority | ✅ Cerrada |
| [Campaign 38](Ekadasi-Rules.md) | Ekādaśī Rules Specification | ✅ Cerrada para auditoría de Knowledge Base |
| [Campaign 39](Hari-bhakti-vilasa-Source-Extraction.md) | Hari-bhakti-vilāsa Source Extraction | ✅ Cerrada |
