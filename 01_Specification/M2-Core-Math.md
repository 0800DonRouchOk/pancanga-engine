# M2 - Core Math

Version: 1.0.0-draft

Estado: En desarrollo

## Objetivo

Construir el núcleo matemático sobre el cual descansará toda la astronomía de
Pancanga Engine.

Este módulo contendrá exclusivamente operaciones matemáticas genéricas y
trigonométricas.

No conocerá conceptos astronómicos ni reglas del calendario.

## Alcance

- Constantes matemáticas.
- Normalización de ángulos.
- Normalización de radianes.
- Normalización de horas.
- Conversión entre grados y radianes.
- Funciones trigonométricas usando tipos fuertes.
- Comparación segura de valores de punto flotante.
- Benchmarks para rutas matemáticas de alta frecuencia.

## Arquitectura

M2 vive en `core/math` y puede depender únicamente de:

- `core/types::Degrees`
- `core/types::Radians`
- funciones numéricas de la biblioteca estándar de Rust

No debe depender de módulos de astronomía, calendario lunisolar, reglas
Gauḍīya, engine, API pública, datos ni calendarios.

## Estructura

```text
03_Source/rust/crates/pancanga-engine/src/core/math/
  mod.rs
  constants.rs
  normalize.rs
  trigonometry.rs
  compare.rs
```

## API Pública

### constants.rs

```rust
pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = std::f64::consts::TAU;
pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;
pub const DEG_TO_RAD: f64 = PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / PI;
pub const EPSILON: f64 = 1.0e-12;
```

### normalize.rs

```rust
pub fn normalize_360(angle: Degrees) -> Degrees;
pub fn normalize_180(angle: Degrees) -> Degrees;
pub fn normalize_radians(angle: Radians) -> Radians;
pub fn normalize_hours(hours: f64) -> f64;
```

### trigonometry.rs

```rust
pub fn deg_to_rad(d: Degrees) -> Radians;
pub fn rad_to_deg(r: Radians) -> Degrees;
pub fn sin_deg(angle: Degrees) -> f64;
pub fn cos_deg(angle: Degrees) -> f64;
pub fn tan_deg(angle: Degrees) -> f64;
pub fn asin_deg(x: f64) -> Degrees;
pub fn acos_deg(x: f64) -> Degrees;
pub fn atan_deg(x: f64) -> Degrees;
pub fn atan2_deg(y: f64, x: f64) -> Degrees;
```

### compare.rs

```rust
pub fn approximately_equal(a: f64, b: f64) -> bool;
pub fn nearly_zero(x: f64) -> bool;
pub fn clamp_unit(x: f64) -> f64;
```

## Tests

Las pruebas unitarias deben cubrir:

- `normalize_360(370 deg) -> 10 deg`
- `normalize_360(-10 deg) -> 350 deg`
- `normalize_360(720 deg) -> 0 deg`
- `normalize_180(270 deg) -> -90 deg`
- `sin_deg(90 deg) -> 1`
- `cos_deg(180 deg) -> -1`
- `tan_deg(45 deg) -> 1`
- `deg_to_rad(180 deg) -> PI`
- `rad_to_deg(PI) -> 180 deg`
- comportamiento de borde de funciones trigonométricas inversas
- helpers de comparación y clamping

## Benchmarks

El benchmark `core_math` mide:

- `normalize_360`
- `sin_deg`
- `cos_deg`
- `atan2_deg`

Comando:

```text
cargo bench --bench core_math
```

## Criterio de Aceptación

M2 se considerará terminado cuando:

- Todas las funciones sean puras.
- No existan asignaciones mutables innecesarias.
- Todos los tests pasen.
- La documentación `rustdoc` esté completa.
- No exista ninguna dependencia de astronomía.
- No exista ninguna dependencia de calendarios.
