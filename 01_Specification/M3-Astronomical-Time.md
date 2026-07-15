# M3 - Astronomical Time

Version: 1.0.0-draft

Estado: En desarrollo

## Objetivo

Crear la estructura de tipos y la API pública para representar tiempo civil y
tiempo astronómico en Pancanga Engine.

M3 no implementa fórmulas astronómicas, conversiones entre escalas temporales
ni reglas de calendario.

## Alcance

- `TimeScale`
- `Instant`
- `CivilDate`
- `CivilTime`
- `CivilDateTime`
- `DeltaT`

## Arquitectura

M3 vive en `core/time`.

Puede depender de:

- `core/types::JulianDate`
- tipos propios de `core/time`
- biblioteca estándar de Rust

No debe depender de módulos de astronomía, calendario lunisolar, reglas
Gauḍīya, engine ni API pública.

## Estructura

```text
03_Source/rust/crates/pancanga-engine/src/core/time/
  mod.rs
  scale.rs
  instant.rs
  civil.rs
  delta_t.rs
```

## API Pública

```rust
pub enum TimeScale;
pub struct Instant;
pub struct CivilDate;
pub struct CivilTime;
pub struct CivilDateTime;
pub struct DeltaT;
```

## Criterio de Aceptación

M3 se considerará terminado cuando:

- Todos los tipos tengan `rustdoc`.
- Todos los tipos tengan constructores explícitos.
- La representación interna esté encapsulada.
- Existan pruebas unitarias básicas.
- No exista dependencia con `Astronomy`.
- No existan fórmulas astronómicas.
