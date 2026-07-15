# AST-L001
# Fundamental Lunar Arguments

| Campo | Valor |
|---|---|
| ID | AST-L001 |
| Estado | ✅ Cerrado: implementado y validado contra polinomios de Meeus |
| Autor Principal | Krishna Das |
| Fuente científica | Jean Meeus, *Astronomical Algorithms*, 2nd Edition, Chapter 47 |
| Pipeline | Lunar Pipeline |
| Dependencias físicas | Julian Day, siglos julianos desde J2000.0 |
| Entrada | `JulianDate` |
| Salida | `FundamentalLunarArguments` |
| Implementación Rust | `fundamental_lunar_arguments` |
| Benchmark | Incluido |
| Validación científica | Validado contra valores calculados con los polinomios de Meeus |

---

## 1. Objetivo

Calcular los cuatro argumentos fundamentales usados por la teoría lunar de
Meeus:

- `D`: elongación media de la Luna;
- `M`: anomalía media del Sol;
- `M'`: anomalía media de la Luna;
- `F`: argumento de latitud de la Luna.

Este algoritmo no evalúa términos periódicos y no calcula longitud lunar.

---

## 2. Entrada

```text
JulianDate
```

Internamente se usa:

```text
T = (JD - 2451545.0) / 36525.0
```

---

## 3. Salida

```rust
FundamentalLunarArguments
```

Todos los ángulos están expresados en `Degrees` y normalizados al intervalo:

```text
[0°, 360°)
```

---

## 4. Fórmulas Implementadas

```text
D = 297.8501921
  + 445267.1114034 T
  - 0.0018819 T²
  + T³ / 545868
  - T⁴ / 113065000

M = 357.5291092
  + 35999.0502909 T
  - 0.0001536 T²
  + T³ / 24490000

M' = 134.9633964
   + 477198.8675055 T
   + 0.0087414 T²
   + T³ / 69699
   - T⁴ / 14712000

F = 93.2720950
  + 483202.0175233 T
  - 0.0036539 T²
  - T³ / 3526000
  + T⁴ / 863310000
```

---

## 5. API Pública

```rust
pub struct FundamentalLunarArguments;

pub fn fundamental_lunar_arguments(
    jd: JulianDate,
) -> FundamentalLunarArguments;
```

---

## 6. Tests

Casos cubiertos:

- creación explícita del tipo `FundamentalLunarArguments`;
- argumentos en J2000.0;
- ejemplo del capítulo 47 de Meeus (`JD = 2448724.5`);
- normalización de todos los argumentos.

---

## 7. Numerical Notes

- Se usa `f64`.
- Se normaliza cada argumento después de evaluar el polinomio completo.
- No se evalúan términos periódicos.
- No se calcula tithi.
- No se calcula ninguna regla calendárica o Vaiṣṇava.

---

## 8. Scientific Validation

Fuente oficial:

```text
Jean Meeus, Astronomical Algorithms, 2nd Edition, Chapter 47.
```

Caso de referencia:

```text
JD = 2448724.5

D  = 113.8423037166358°
M  =  97.64351360962246°
M' =   5.150832522958808°
F  = 219.8897207278569°
```

Tolerancia:

```text
1.0e-9 grados
```

Estado:

```text
Validado contra los polinomios de Meeus.
```

---

## 9. Estado

✅ Cerrado.

AST-L001 queda listo como base para los términos periódicos lunares.
