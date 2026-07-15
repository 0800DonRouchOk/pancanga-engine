# AST-L002 — ELP2000 Periodic Longitude Correction

Primer algoritmo de corrección periódica de longitud lunar ELP2000.

| Campo | Valor |
|---|---|
| ID | AST-L002 |
| Estado | ✅ Cerrado |
| Fuente científica | ELP2000-82B / ELP2000-85 |
| Entrada pública propuesta | `JulianDate` |
| Entrada interna propuesta | `t: JulianCenturiesTdb` |
| Salida propuesta | `ArcSeconds` firmado |
| Datos | 12 tablas de longitud ELP2000 |
| Nota semántica ELP1 | `02_Research/ELP2000/notas/ELP1-Semantics.md` |
| Nota semántica ELP4 | `02_Research/ELP2000/notas/ELP4-Semantics.md` |
| Nota de correcciones | `02_Research/ELP2000/notas/ELP2000-85-Corrections.md` |
| Nota de orden de familias | `02_Research/ELP2000/notas/AST-L002-Family-Dependencies.md` |
| Implementación Rust | `elp2000_evaluator.rs` |
| Evaluador lunar | `MainProblem`, `ShortPeriodic` y `Planetary` validados |

---

## Objetivo

Evaluar exclusivamente las contribuciones periódicas de longitud lunar de
ELP2000-82B.

La salida de `AST-L002` no es la longitud lunar completa.

Este algoritmo produce solamente una corrección periódica firmada en segundos
de arco.

Nombre conceptual de la salida:

```text
Periodic Longitude Correction
```

---

## Principio de Diseño

`AST-L002` no debe entenderse como una suma genérica de términos.

Debe entenderse como un intérprete de la teoría ELP2000:

```text
Importador = fidelidad de datos.
Evaluador  = fidelidad de la teoría.
```

El importador conserva los archivos oficiales de IMCCE sin aplicar semántica
astronómica. El evaluador construye argumentos, aplica constantes, interpreta
familias de términos, aplica factores temporales y produce `Δλ`.

Por eso el módulo interno no debe llamarse de forma genérica
`evaluate_terms.rs`.

Nombre interno propuesto:

```text
elp2000_evaluator.rs
```

Ese nombre refleja que el módulo no evalúa solamente trigonometría, sino la
semántica operacional de ELP2000-82B/85.

---

## ELP2000 Interpreter

El componente interno responsable de `AST-L002` se denominará conceptualmente:

```text
ELP2000 Interpreter
```

No interpreta un lenguaje de programación. Interpreta una teoría científica:
ELP2000-82B con constantes, argumentos y revisiones ELP2000-85 aplicadas de
forma explícita.

### Responsabilidades

El `ELP2000 Interpreter` es responsable de:

- construir los argumentos fundamentales requeridos por las tablas de longitud;
- aplicar las constantes y correcciones de ELP2000-82B/85 donde corresponda;
- aplicar los factores temporales `1`, `t` y `t²`;
- evaluar cada familia de términos según su semántica propia;
- preservar la separación entre datos importados y reglas de evaluación;
- devolver exclusivamente `Δλ`, la corrección periódica de longitud lunar.

### No Responsabilidades

El `ELP2000 Interpreter` no es responsable de:

- calcular `W1`;
- calcular la longitud lunar completa;
- calcular longitud aparente;
- aplicar nutación;
- transformar marcos de referencia;
- calcular latitud lunar;
- calcular distancia Tierra-Luna;
- calcular elongación, tithi, nakṣatra, yoga o karaṇa;
- conocer reglas calendáricas;
- conocer reglas vaiṣṇavas.

### Flujo Conceptual

```text
ELP2000 Data
        │
        ▼
ELP2000 Interpreter
        │
        ▼
Δλ
        │
        ▼
W1
        │
        ▼
Longitud lunar
        │
        ▼
Correcciones
        │
        ▼
Longitud aparente
```

---

## Fuera de Alcance

`AST-L002` no debe:

- sumar la longitud media lunar `W1`;
- calcular la longitud geocéntrica final;
- aplicar nutación;
- transformar marcos de referencia;
- calcular latitud lunar;
- calcular distancia Tierra-Luna;
- calcular tithi, nakṣatra, yoga o karaṇa.

---

## Tablas de Longitud

Los archivos ELP2000-82B se organizan en grupos de tres:

```text
longitud — latitud — distancia
```

Por lo tanto, las tablas de longitud son:

```text
ELP1
ELP4
ELP7
ELP10
ELP13
ELP16
ELP19
ELP22
ELP25
ELP28
ELP31
ELP34
```

Nota: los archivos oficiales usan nombres sin cero inicial (`ELP1`, no
`ELP01`).

---

## Tabla de Contribuciones

| Archivo | Contribución | Formato | Términos | Orden temporal |
|---|---|---|---:|---|
| ELP1 | Problema principal lunar | Main problem | 1023 | `t^0` |
| ELP4 | Figura terrestre | Short periodic | 347 | `t^0` |
| ELP7 | Figura terrestre, términos de Poisson | Short periodic | 14 | `t^1` |
| ELP10 | Perturbaciones planetarias, tabla 1 | Planetary | 14328 | `t^0` |
| ELP13 | Perturbaciones planetarias, tabla 1, términos de Poisson | Planetary | 4384 | `t^1` |
| ELP16 | Perturbaciones planetarias, tabla 2 | Planetary | 170 | `t^0` |
| ELP19 | Perturbaciones planetarias, tabla 2, términos de Poisson | Planetary | 226 | `t^1` |
| ELP22 | Efectos de marea | Short periodic | 3 | `t^0` |
| ELP25 | Efectos de marea, términos de Poisson | Short periodic | 6 | `t^1` |
| ELP28 | Figura lunar | Short periodic | 20 | `t^0` |
| ELP31 | Perturbaciones relativistas | Short periodic | 11 | `t^0` |
| ELP34 | Excentricidad solar, términos de Poisson | Short periodic | 28 | `t^2` |

Total de términos de longitud:

```text
20560
```

El orden de implementación recomendado para completar `AST-L002` sigue el
orden oficial de sustitución de `elp82b_2` para longitud (`iv = 1`), no una
preferencia informal de numeración. La nota
`AST-L002-Family-Dependencies.md` registra esta verificación.

---

## Mapa Interno del Intérprete

`AST-L002` se implementará mediante evaluadores semánticos.

Cada familia declara qué evaluador científico la interpreta:

| Familia | Magnitud | Semántica | Evaluador | Orden temporal |
|---|---|---|---|---|
| ELP1 | Longitud | Main Problem | `MainProblemEvaluator` | `t^0` |
| ELP4 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^0` |
| ELP7 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^1` |
| ELP10 | Longitud | Planetary | `PlanetaryEvaluator` | `t^0` |
| ELP13 | Longitud | Planetary | `PlanetaryEvaluator` | `t^1` |
| ELP16 | Longitud | Planetary | `PlanetaryEvaluator` | `t^0` |
| ELP19 | Longitud | Planetary | `PlanetaryEvaluator` | `t^1` |
| ELP22 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^0` |
| ELP25 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^1` |
| ELP28 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^0` |
| ELP31 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^0` |
| ELP34 | Longitud | Short Periodic | `ShortPeriodicEvaluator` | `t^2` |

En Rust, este orden se modela como `ElpTemporalOrder::{Order0, Order1,
Order2}`. No se guarda como un número suelto ni como una condición dispersa:
forma parte de la definición científica de la familia.

Regla:

```text
Family -> Semantic Evaluator -> Contribution
```

El intérprete no debe crecer como una lista de excepciones por familia. Una
nueva familia debe aportar únicamente el conocimiento científico que le es
propio. Si comparte semántica con una familia ya validada, debe reutilizar el
mismo evaluador.

Regla de extensión:

```text
Una nueva familia debe aportar únicamente conocimiento científico nuevo.
Nunca debe exigir una nueva arquitectura, salvo que la teoría ELP2000 lo
requiera explícitamente.
```

---

## Orden Temporal

Las siguientes familias tienen orden temporal `t^1`:

```text
ELP7
ELP13
ELP19
ELP25
```

La siguiente tabla tiene orden temporal `t^2`:

```text
ELP34
```

Las demás tablas de longitud tienen orden temporal `t^0`.

---

## Unidades

Las series de longitud ELP2000 están publicadas en:

```text
arcseconds
```

`AST-L002` debe preservar la salida como segundos de arco firmados.

La conversión a grados o radianes pertenece a algoritmos posteriores.

---

## API Propuesta

```rust
pub fn longitude_periodic_correction(jd: JulianDate) -> ArcSeconds;
```

Antes de implementar esta API debe existir un tipo fuerte `ArcSeconds` o un
tipo equivalente aprobado dentro de `core/types`.

---

## Relación con W1

ELP2000 exige sumar la longitud media lunar polinómica:

```text
W1(t) = W1(0) + νt + W1(2)t² + W1(3)t³ + W1(4)t⁴
```

`W1` no pertenece a `AST-L002`.

La cadena correcta es:

```text
AST-L002  Periodic Longitude Correction
        ↓
AST-L003  Longitud media lunar W1
        ↓
AST-L004  Longitud geocéntrica lunar
```

---

## Advertencia Crítica

`ELP1` no debe tratarse como una simple tabla genérica de amplitud, fase y
frecuencia.

El problema principal contiene coeficientes y derivadas respecto de constantes
ajustables. Su evaluación debe seguir las fórmulas y constantes adoptadas por
ELP2000-82B/85.

La nota de investigación `ELP1-Semantics.md` registra la primera lectura
operacional de esta semántica a partir de la notice y de las rutinas oficiales.

Esta advertencia bloquea la implementación del evaluador hasta que quede
documentada la semántica exacta de:

- constantes ajustables;
- correcciones aplicadas al problema principal;
- relación entre argumentos lunares y columnas importadas;
- signos y función trigonométrica aplicable.

---

## Investigación Pendiente

Antes de implementar código debe responderse:

- ¿Qué representa físicamente `ELP1` dentro del problema principal lunar?
- ¿Qué ecuación de Chapront-Touzé & Chapront usa esos términos?
- ¿Cómo se combina el problema principal con `W1`?
- ¿Qué constantes o revisiones de ELP2000-85 modifican la evaluación?
- ¿Cuál es exactamente la salida esperada de `AST-L002` antes de sumar `W1`?

Esta investigación debe quedar documentada antes de escribir el evaluador.

Estado de investigación:

- `ELP1-Semantics.md` confirma que `AST-L002` produce `Δλ_periodic(t)` y no
  suma `W1`.
- `ELP2000-85-Corrections.md` confirma que las revisiones afectan argumentos,
  constantes y amplitudes del problema principal. Por tanto, el importador debe
  preservar datos crudos y el evaluador debe aplicar la semántica de la teoría.
- `AST-L002` se tratará como un intérprete ELP2000, no como un evaluador
  genérico de tablas trigonométricas.
- El contrato de responsabilidades del `ELP2000 Interpreter` queda definido en
  esta especificación antes de crear `elp2000_evaluator.rs`.
- El primer término oficial de `ELP1`, los subconjuntos de 10 y 100 términos, y
  la familia completa de 1023 términos fueron evaluados y validados contra
  oráculos independientes registrados en
  `04_Tests/Astronomical/ELP2000-Interpreter-Validation.md`.

---

## ELP1 Validation Summary

`ELP1` queda validado como familia del problema principal de longitud.

Esta validación demuestra que el intérprete puede recorrer la familia completa
aplicando la semántica de `ELP2000-82B/85` término por término.

```text
Terms:
1023 / 1023

Independent oracle:
PASS

Maximum error:
<= 1.0e-11 arcsec

Mean error:
<= 1.0e-11 arcsec

RMS:
<= 1.0e-11 arcsec

Fingerprint:
0xa460f0ac886bf342

Reference:
ELP2000-82B
```

Suma total de `ELP1` en J2000:

```text
17990.968840315887064 arcsec
```

Fixture independiente:

```text
04_Tests/Astronomical/ELP2000-ELP1-J2000-Contributions.csv
```

Manifiesto del oráculo:

```text
04_Tests/Astronomical/ELP2000-Oracle-Manifest.md
```

Nota: esta validación cierra `ELP1`, pero no cierra `AST-L002`. Aún deben
interpretarse y validarse las demás familias de longitud.

API de auditoría vigente:

```rust
audit_family(ElpFamily::Elp1, jd)
```

Los métodos específicos de `ELP1` quedan como atajos de auditoría, no como el
patrón que debe guiar la incorporación de nuevas familias.

---

## ELP4 ShortPeriodic Subset Validation

Campaign 02 implementó `ShortPeriodicEvaluator` para el primer término oficial
de `ELP4`.

Campaign 03 reutilizó exactamente el mismo mecanismo para evaluar los primeros
10 términos oficiales de `ELP4`.

Campaign 04 reutilizó exactamente el mismo mecanismo para evaluar los primeros
100 términos oficiales de `ELP4`.

Campaign 05 reutilizó exactamente el mismo mecanismo para evaluar la familia
completa `ELP4`.

No implementa:

- `AST-L002`;
- `W1`;
- longitud lunar;
- nutación;
- calendario;
- reglas vaisnavas.

Oráculo independiente:

```text
04_Tests/Astronomical/ELP2000-Short-Periodic-Oracle.md
04_Tests/Astronomical/ELP2000-ELP4-J2000-Contributions.csv
```

Resultado J2000:

```text
primer argumento       = 6.340294213756157760 rad
primer xx              = 0.000030000000000000 arcsec
primera contribución   = 0.000001712336066442 arcsec
suma parcial 10 terms  = 0.004759349973172258 arcsec
suma parcial 100 terms = 0.027406824891217551 arcsec
suma total ELP4        = 5.031975142947603175 arcsec
```

Estado:

```text
Familia completa ELP4:
PASS

Semántica ShortPeriodic:
validada con ELP4.
```

---

## Criterio de Aceptación

`AST-L002` podrá implementarse cuando:

- esté definido el tipo fuerte de salida (`ArcSeconds` o equivalente);
- esté definido el módulo interno `elp2000_evaluator.rs` o un nombre equivalente
  aprobado explícitamente;
- esté documentada la evaluación correcta de `ELP1`;
- esté documentada la evaluación de formatos `MainProblem`, `ShortPeriodic` y
  `Planetary`;
- los factores `1`, `t` y `t²` estén cubiertos por tests;
- se confirme que ninguna tabla de latitud o distancia participa en este
  algoritmo.

---

## Estado

`AST-L002` queda cerrado.

La implementación integra las 12 familias de longitud ELP2000:

```text
ELP1
ELP4
ELP7
ELP10
ELP13
ELP16
ELP19
ELP22
ELP25
ELP28
ELP31
ELP34
```

No suma `W1`, no aplica nutación y no calcula longitud lunar aparente.

Validación final de Campaign 22:

```text
JD = 2488070.0
t = 1.0
Δλ_periodic = -6423.827265758044 arcsec
Oracle = PASS
```
