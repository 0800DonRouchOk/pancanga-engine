# Campaign 01 - ELP4 Semantic Investigation

Estado: completo para revision cientifica

Objetivo: determinar la semantica matematica exacta de `ELP4` antes de
implementar `ShortPeriodicEvaluator`.

Restriccion de campana:

```text
No implementar nada que no pueda justificarse con ELP2000-82B/85.
Si la arquitectura congelada contradice la teoria, detener la implementacion y
documentar la contradiccion antes de escribir codigo.
```

---

## Fuentes

Fuentes oficiales locales:

```text
02_Research/ELP2000/sources/README
02_Research/ELP2000/sources/elp82b.ps
02_Research/ELP2000/sources/elp82b_1
02_Research/ELP2000/sources/elp82b_2
06_Data/Reference/ELP2000/raw/ELP4
06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs
```

Referencias concretas:

- `README`, lineas 11-17: identifica las referencias Chapront-Touze &
  Chapront 1983 y 1988.
- `README`, lineas 25-29: describe ELP2000-82 como series, con constantes
  ajustadas a DE200/LE200 y argumentos de ELP2000-85.
- `README`, lineas 43-50: confirma series trigonometricas y de Poisson, y
  unidades de longitud/latitud en segundos de arco.
- `README`, lineas 86-95: identifica `ELP4` como perturbaciones por figura
  terrestre en longitud.
- `elp82b_2`, lineas 51-54: declara `del`, `zeta`, `nterm`, `nrang` y zonas
  internas del evaluador.
- `elp82b_2`, lineas 145-153: construye argumentos `del` y `zeta`.
- `elp82b_2`, lineas 225-252: lee y prepara las familias short-periodic.
- `elp82b_2`, lineas 331-342 y 372-376: sustituye el tiempo y suma longitud.
- `elp82b_1`, lineas 217-245: muestra la forma operacional directa equivalente
  para figuras, mareas, relatividad y excentricidad solar.
- `raw/ELP4`, linea 1: encabezado oficial de la familia.
- `raw/ELP4`, linea 2: primer termino oficial.
- `elp2000_coefficients.rs`, lineas 15-20: estructura generada
  `ElpShortPeriodicTerm`.
- `elp2000_coefficients.rs`, lineas 31784-31792: metadata y primer termino
  generado de `ELP4`.

---

## Resumen Ejecutivo

`ELP4` es la familia de longitud de las perturbaciones por figura terrestre.
Pertenece a la semantica short-periodic de ELP2000.

Para cada termino:

```text
y(t) = phase + iz * zeta(t) + sum_i ilu_i * del_i(t)
```

con solo parte constante y lineal:

```text
y(t) = y0 + y1 * t
```

La contribucion individual de `ELP4` a la correccion periodica de longitud es:

```text
Delta_lambda_i(t) = xx_i * sin(y_i(t))
```

`ELP4` tiene orden temporal `t^0`. No multiplica la amplitud por `t` ni por
`t^2`.

---

## Respuestas Directas

### 1. Que representa fisicamente `ELP4`?

`ELP4` representa perturbaciones de longitud lunar debidas a la figura
terrestre.

Evidencia:

```text
raw/ELP4 line 1:
EARTH FIGURE PERTURBATIONS. LONGITUDE
```

La tabla oficial del `README` identifica `ELP4` como:

```text
Earth figure perturbations. Longitude
```

`elp82b_1` agrupa esta familia en el bloque operacional:

```text
Figures - Tides - Relativity - Solar eccentricity
```

### 2. Que magnitud contribuye?

Contribuye a la longitud lunar, no a latitud ni distancia.

Evidencia:

```text
ELP4 -> iv = mod(4 - 1, 3) + 1 = 1
```

En `elp82b_2`, `iv = 1` corresponde a longitud durante la sustitucion de tiempo.
La contribucion de longitud se acumula en `r(1)`.

Unidades: segundos de arco, porque el `README` indica que las unidades de
longitud y latitud son arcseconds.

### 3. Cual es la ecuacion matematica exacta que evalua cada termino?

Forma operacional en `elp82b_2`:

```text
zone(1) = xx
zone(2) = phase * deg + iz * zeta(0) + sum_i ilu_i * del_i(0)
zone(3) =              iz * zeta(1) + sum_i ilu_i * del_i(1)

y(t) = zone(2) + zone(3) * t
Delta_lambda_i(t) = xx * sin(y(t))
```

Forma equivalente:

```text
Delta_lambda_i(t) =
    xx_i * sin(
        phase_i
        + iz_i * (zeta_0 + zeta_1 * t)
        + sum_{j=1..4} ilu_{i,j} * (del_{j,0} + del_{j,1} * t)
    )
```

Para `ELP4`, el orden temporal externo es `t^0`, por tanto no hay factor
adicional sobre `xx`.

### 4. Que significan cada uno de sus campos?

`ELP4` usa el formato short-periodic:

```fortran
read (...) iz, ilu, pha, xx
```

En el archivo raw, cada registro contiene:

```text
iz ilu1 ilu2 ilu3 ilu4 phase xx
```

En Pancanga Engine, el importador preserva esos campos como:

```rust
ElpShortPeriodicTerm {
    zeta_multiplier,
    multipliers,
    phase_degrees,
    amplitude,
}
```

Correspondencia:

| Campo oficial | Campo Rust | Significado |
|---|---|---|
| `iz` | `zeta_multiplier` | multiplicador entero de `zeta` |
| `ilu(1..4)` | `multipliers` | multiplicadores de los argumentos `del(1..4)` |
| `pha` | `phase_degrees` | fase constante del termino, en grados |
| `xx` / `x` | `amplitude` | amplitud del termino, en segundos de arco para longitud |

Primer termino oficial (`raw/ELP4`, linea 2):

```text
iz = 0
ilu = [0, 0, 0, 1]
phase = 270.00000 deg
xx = 0.00003 arcsec
```

Primer termino generado:

```text
zeta_multiplier = 0
multipliers = [0, 0, 0, 1]
phase_degrees = 270.00000
amplitude = 0.00003
```

### 5. Que es `zeta`?

`zeta` es un argumento auxiliar construido a partir de la longitud media lunar
`W1` y la constante de precesion.

En `elp82b_2`:

```text
zeta(0) = w(1,0)
zeta(1) = w(1,1) + precess
```

Por tanto:

```text
zeta(t) = zeta(0) + zeta(1) * t
```

Solo se usan los terminos constante y lineal para las familias
short-periodic.

### 6. Que representan los argumentos `del`?

`del(1..4)` son los argumentos lunares comunes derivados de `w`, `eart` y
`peri`.

En `elp82b_2`:

```text
del(1,i) = w(1,i) - eart(i)
del(4,i) = w(1,i) - w(3,i)
del(3,i) = w(1,i) - w(2,i)
del(2,i) = eart(i) - peri(i)
del(1,0) = del(1,0) + pi
```

En notacion astronomica ya usada en el proyecto:

| `del` | Interpretacion |
|---|---|
| `del(1)` | elongacion media lunar `D` |
| `del(2)` | anomalia media solar `l'` |
| `del(3)` | anomalia media lunar `l` |
| `del(4)` | argumento de latitud lunar `F` |

Para `ELP4` se usan solamente `del(_,0)` y `del(_,1)`.

### 7. Como interviene `iz`?

`iz` multiplica a `zeta` dentro del argumento trigonometrico.

En `elp82b_2`:

```text
y = y + iz * zeta(k)
```

Por tanto:

```text
iz * zeta(t) = iz * zeta(0) + iz * zeta(1) * t
```

Si `iz = 0`, como en el primer termino de `ELP4`, `zeta` no contribuye a ese
termino.

### 8. Cual es el significado fisico de `phase`?

`phase` es la fase constante del termino periodico, expresada en grados en los
datos oficiales.

En `elp82b_2`, durante la preparacion de las familias short-periodic:

```text
if k = 0: y = pha * deg
if k != 0: y = 0
```

Por tanto, `phase` solo contribuye a la parte constante del argumento:

```text
y0 = phase * deg + ...
```

No modifica la amplitud.

### 9. Como se construye exactamente el argumento trigonometrico?

Preparacion en memoria:

```text
y0 = phase * deg + iz*zeta(0) + sum_i ilu_i*del_i(0)
y1 =               iz*zeta(1) + sum_i ilu_i*del_i(1)
```

Sustitucion temporal:

```text
y(t) = y0 + y1*t
```

Evaluacion:

```text
sin(y(t))
```

`elp82b_1` muestra la forma directa equivalente:

```text
y = phase*deg
for k = 0..1:
    y += iz*zeta(k)*t^k
    y += sum_i ilu_i*del_i(k)*t^k
```

### 10. Que significa `xx`?

`xx` es la amplitud del termino.

Para `ELP4`, como es una familia de longitud, `xx` esta en segundos de arco.
La contribucion individual queda en segundos de arco:

```text
xx * sin(y)
```

A diferencia de `ELP1`, `xx` no pasa por correcciones `delnu`, `delnp`,
`delg`, `dele` ni `delep`.

### 11. Como aparece el orden temporal?

El orden temporal externo no esta embebido en el registro individual. Pertenece
a la familia.

En `elp82b_2`, la sustitucion temporal multiplica la amplitud por `t` para
ciertas tablas y por `t^2` para otra:

```text
itab = 3, 5, 7, 9  -> x = x * t
itab = 12          -> x = x * t^2
```

`ELP4` corresponde a:

```text
iv = 1
itab = 2
```

Por tanto, su orden temporal es:

```text
ElpTemporalOrder::Order0
```

No hay multiplicador externo:

```text
x_effective = xx
```

### 12. Existen correcciones especiales equivalentes a las del problema principal?

No para `ELP4`.

Las correcciones especiales de amplitud aparecen en la ruta del problema
principal (`ELP1` a `ELP3`):

```text
tgv = coef(2) + dtasm*coef(6)
xx = coef(1)
   + tgv*(delnp - am*delnu)
   + coef(3)*delg
   + coef(4)*dele
   + coef(5)*delep
```

La ruta short-periodic (`ELP4`, `ELP7`, `ELP22`, `ELP25`, `ELP28`, `ELP31`,
`ELP34`, y sus pares de latitud/distancia) no aplica esa correccion. Para
`ELP4`, `xx` es la amplitud efectiva de la tabla, salvo por el posible orden
temporal familiar. Como `ELP4` es `Order0`, no se modifica.

### 13. Que partes pertenecen al evaluador y cuales ya vienen resueltas en los datos?

Vienen resueltas en los datos/importador:

- `iz`;
- `ilu(1..4)`;
- `phase`;
- `xx`;
- conteo de terminos (`347`);
- fingerprint de fuente;
- identidad de archivo (`ELP4`);
- preservacion decimal de los registros oficiales.

Pertenecen al evaluador:

- construir `t = (JD_TDB - J2000) / 36525`;
- construir `zeta(0..1)`;
- construir `del(1..4, 0..1)`;
- convertir `phase` de grados a radianes;
- formar `y0` y `y1`;
- aplicar el orden temporal de la familia (`Order0` para `ELP4`);
- evaluar `xx * sin(y0 + y1*t)`;
- devolver contribuciones en `ArcSeconds`;
- exponer una estructura de auditoria con argumento, amplitud y contribucion.

No pertenecen al evaluador short-periodic:

- sumar `W1`;
- aplicar correcciones de amplitud del problema principal;
- evaluar argumentos planetarios;
- aplicar nutacion;
- transformar marcos;
- calcular tithi o cualquier regla calendaria.

---

## Formula de Implementacion Para `ShortPeriodicEvaluator`

Para un termino `term` y una definicion de familia `definition`:

```text
t = (JD_TDB - 2451545.0) / 36525

zeta0 = w1_0
zeta1 = w1_1 + precess

del_j0, del_j1 = argumentos lunares comunes

y0 =
    radians(term.phase_degrees)
    + term.zeta_multiplier * zeta0
    + sum_j term.multipliers[j] * del_j0

y1 =
    term.zeta_multiplier * zeta1
    + sum_j term.multipliers[j] * del_j1

argument = y0 + y1 * t

amplitude =
    term.amplitude * t^definition.temporal_order.exponent()

contribution =
    amplitude * sin(argument)
```

Para `ELP4`:

```text
definition.semantic_evaluator = ShortPeriodic
definition.temporal_order = Order0
amplitude = term.amplitude
```

---

## Primer Termino de ELP4

Fuente oficial:

```text
0 0 0 0 1 270.00000 0.00003
```

Interpretacion:

```text
iz = 0
ilu = [0, 0, 0, 1]
phase = 270 deg
xx = 0.00003 arcsec
```

Argumento:

```text
y(t) = 270 deg + F(t)
```

porque:

```text
iz = 0
ilu1 = 0
ilu2 = 0
ilu3 = 0
ilu4 = 1
```

Contribucion:

```text
Delta_lambda_1(t) = 0.00003 * sin(270 deg + F(t))
```

En J2000:

```text
t = 0
y(0) = 270 deg + F0
```

Este debe ser el primer caso de oraculo independiente para Campaign 02.

### Resultado de Campaign 02

El primer termino oficial de `ELP4` fue implementado mediante
`ShortPeriodicEvaluator` sin modificar la arquitectura congelada del
interprete.

Validacion en J2000:

```text
argument     = 6.340294213756157760 rad
xx           = 0.000030000000000000 arcsec
contribution = 0.000001712336066442 arcsec
```

Oraculo independiente:

```text
04_Tests/Astronomical/ELP2000-Short-Periodic-Oracle.md
04_Tests/Astronomical/ELP2000-ELP4-J2000-Contributions.csv
```

Estado:

```text
Primer termino ELP4:
PASS

Semantica ShortPeriodic completa:
pendiente de validacion sobre 10 terminos, 100 terminos y familia completa.
```

### Resultado de Campaign 03

Los primeros 10 terminos oficiales de `ELP4` fueron evaluados reutilizando
exactamente el mismo `ShortPeriodicEvaluator`.

Suma parcial en J2000:

```text
Delta_lambda_1..10 = 0.004759349973172258 arcsec
```

Estado:

```text
Primeros 10 terminos ELP4:
PASS

Semantica ShortPeriodic completa:
pendiente de validacion sobre 100 terminos y familia completa.
```

---

### Resultado de Campaign 04

Los primeros 100 terminos oficiales de `ELP4` fueron evaluados reutilizando
exactamente el mismo `ShortPeriodicEvaluator`.

Suma parcial en J2000:

```text
Delta_lambda_1..100 = 0.027406824891217551 arcsec
```

Estado:

```text
Primeros 100 terminos ELP4:
PASS

Semantica ShortPeriodic completa:
pendiente de validacion sobre familia completa.
```

---

### Resultado de Campaign 05

Los 347 terminos oficiales de `ELP4` fueron evaluados reutilizando exactamente
el mismo `ShortPeriodicEvaluator`.

Suma total en J2000:

```text
Delta_lambda_ELP4 = 5.031975142947603175 arcsec
```

Estado:

```text
Familia completa ELP4:
PASS

Semantica ShortPeriodic:
validada con ELP4.
```

---

## Criterio Para Abrir Implementacion

`ShortPeriodicEvaluator` puede implementarse cuando:

- use `ElpFamilyDefinition`;
- use `ElpSemanticEvaluator::ShortPeriodic`;
- use `ElpTemporalOrder`;
- no modifique `MainProblemEvaluator`;
- no modifique el importador;
- exponga una auditoria del termino con:
  - familia;
  - indice;
  - `iz`;
  - multiplicadores `del`;
  - `phase`;
  - `y0`;
  - `y1`;
  - argumento final;
  - amplitud efectiva;
  - contribucion.

---

## Decision

`ELP4` no requiere una nueva arquitectura.

Requiere implementar la segunda semantica del interprete:

```text
ShortPeriodicEvaluator
```

La arquitectura congelada es suficiente para representar `ELP4`.
