# M4 - Julian Day

Version: 1.0.0-draft

Estado: En desarrollo

## Objetivo

Implementar la conversión entre fecha civil gregoriana y Julian Day siguiendo
el algoritmo del capítulo 7 de Jean Meeus.

M4 no introduce lógica astronómica, calendarios lunisolares ni reglas
Gauḍīya.

## Estructura

```text
03_Source/rust/crates/pancanga-engine/src/core/time/julian/
  mod.rs
  algorithms.rs
  conversion.rs
  validation.rs
```

## API Pública

```rust
pub fn gregorian_to_jd(date_time: CivilDateTime) -> JulianDate;
pub fn jd_to_gregorian(julian_date: JulianDate, time_scale: TimeScale) -> Option<CivilDateTime>;
pub fn is_leap_year(year: i32) -> bool;
pub fn weekday(julian_date: JulianDate) -> Weekday;
```

## Referencia Principal

- Jean Meeus, Astronomical Algorithms, 2nd Edition, Chapter 7.

## Criterio de Aceptación

M4 se considerará terminado cuando:

- el algoritmo esté documentado con referencia bibliográfica;
- existan pruebas con ejemplos publicados por Meeus;
- existan pruebas round-trip;
- las funciones sean legibles y pequeñas;
- no exista lógica lunisolar;
- no exista lógica astronómica fuera del cálculo de Julian Day.
