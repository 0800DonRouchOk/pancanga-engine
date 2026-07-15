# ELP2000-85 Corrections and Evaluator Semantics

Estado: 🟡 Nota de investigación técnica

Objetivo: responder si las correcciones asociadas a ELP2000-85 modifican
solamente amplitudes o si también afectan argumentos y constantes fundamentales.

Esta nota no implementa código.

---

## Fuentes Primarias

Archivos locales usados:

```text
02_Research/ELP2000/sources/README
02_Research/ELP2000/sources/elp82b.ps
02_Research/ELP2000/sources/elp82b_1
02_Research/ELP2000/sources/elp82b_2
```

Fuente pública:

```text
https://ftp.imcce.fr/pub/ephem/moon/elp82b/
```

La rutina `elp82b_2` se toma como referencia operacional principal porque
expresa explícitamente las constantes, argumentos y correcciones usadas para
evaluar los archivos `ELP1`...`ELP36`.

---

## Respuesta Corta

Las revisiones no deben interpretarse como una simple modificación de
amplitudes.

En la distribución oficial:

- los argumentos provienen de ELP2000-85;
- las constantes de la efeméride están ajustadas a DE200/LE200;
- los argumentos de Delaunay se construyen a partir de esas constantes;
- las amplitudes del problema principal se corrigen durante la evaluación;
- las demás series no se corrigen mediante la misma fórmula de amplitud del
  problema principal.

Por tanto, Pancanga Engine no debe incorporar estos ajustes dentro del
importador como si fueran datos ya resueltos.

El importador debe preservar los datos oficiales. El evaluador debe aplicar la
semántica de la teoría.

---

## Evidencia 1: README Oficial

El `README` oficial distingue tres elementos:

```text
The theory of the lunar motion consists of the series of the
semi-analytical solution ELP2000-82.

The constants of the subsequent lunar ephemeris are fitted to
DE200/LE200.

The arguments come from the semi-analytical theory ELP 2000-85.
```

Consecuencia:

```text
ELP2000-82 = series
DE200/LE200 = ajuste de constantes
ELP2000-85 = argumentos
```

No es una sola capa de coeficientes.

---

## Evidencia 2: Constantes Fundamentales

La rutina oficial declara primero constantes y argumentos lunares:

```fortran
am = 0.074801329518d0
alpha = 0.002571881335d0
dtasm = 2.d0 * alpha / (3.d0 * am)

w(1,0), w(1,1), ..., w(1,4)
w(2,0), w(2,1), ..., w(2,4)
w(3,0), w(3,1), ..., w(3,4)
eart(0), ..., eart(4)
peri(0), ..., peri(4)
```

Estas constantes no son términos periódicos. Definen los polinomios base de:

```text
W1
W2
W3
T
ϖ0
```

Por tanto, afectan los argumentos fundamentales y `W1(t)`.

---

## Evidencia 3: Correcciones de Constantes

La rutina oficial separa explícitamente:

```fortran
Corrections of the constants (fit to DE200/LE200).
```

y define:

```fortran
delnu = +0.55604d0 / rad / w(1,1)
dele  = +0.01789d0 / rad
delg  = -0.08066d0 / rad
delnp = -0.06424d0 / rad / w(1,1)
delep = -0.12879d0 / rad
```

Estas cantidades no son argumentos en sí mismas. Son correcciones ajustadas
que luego se usan para modificar las amplitudes del problema principal.

---

## Evidencia 4: Argumentos de Delaunay

Después de declarar las constantes y sus correcciones, la rutina construye los
argumentos de Delaunay:

```fortran
del(1,i) = w(1,i) - eart(i)
del(4,i) = w(1,i) - w(3,i)
del(3,i) = w(1,i) - w(2,i)
del(2,i) = eart(i) - peri(i)
del(1,0) = del(1,0) + pi
```

En notación astronómica:

```text
D  = W1 - T + 180°
l' = T - ϖ0
l  = W1 - W2
F  = W1 - W3
```

Conclusión: si cambian las constantes base, cambian los argumentos. No basta
con pensar en amplitudes.

---

## Evidencia 5: Corrección de Amplitudes en ELP1

Para el problema principal (`ELP1`, `ELP2`, `ELP3`), la rutina lee:

```fortran
read (...) ilu, coef
```

y calcula:

```fortran
tgv = coef(2) + dtasm * coef(6)

xx = coef(1)
   + tgv * (delnp - am * delnu)
   + coef(3) * delg
   + coef(4) * dele
   + coef(5) * delep
```

Para longitud (`ELP1`), ese `xx` es la amplitud efectiva del término:

```text
ELP1 term = xx * sin(argument)
```

Esta es una corrección de amplitud, pero solo para el problema principal.

---

## Evidencia 6: Otras Series

Las tablas no principales se cargan de forma distinta:

```text
Short periodic:
    amplitude + phase + zeta + Delaunay arguments

Planetary:
    amplitude + phase + planetary arguments + selected Delaunay arguments
```

No aparece la fórmula:

```text
coef(1)
+ tgv * (delnp - am * delnu)
+ coef(3) * delg
+ coef(4) * dele
+ coef(5) * delep
```

para esas tablas.

Conclusión: no todas las familias de series comparten la misma semántica de
corrección.

---

## Decisión de Diseño Para Pancanga Engine

El importador debe seguir siendo un importador de datos oficiales:

```text
ELP source files
    ↓
parse
    ↓
preserve fields
    ↓
generated Rust constants
```

No debe preaplicar:

- correcciones de amplitud;
- construcción de argumentos;
- factores de Poisson;
- conversión de unidades;
- suma de `W1`.

El evaluador debe aplicar la teoría:

```text
constants and arguments
    ↓
term family semantics
    ↓
time factor
    ↓
trigonometric evaluation
    ↓
periodic correction
```

Esto conserva trazabilidad: los datos generados siguen correspondiendo a las
tablas oficiales, y la implementación matemática queda visible en código.

---

## Consecuencia Para AST-L002

`AST-L002` debe diseñarse como un evaluador semántico, no como una suma genérica
de términos.

Debe distinguir al menos:

```text
MainProblemLongitudeTerm
ShortPeriodicLongitudeTerm
PlanetaryLongitudeTerm
```

Además debe existir una capa explícita de constantes ELP:

```text
Elp2000Constants
Elp2000Arguments
```

para que:

- `W1(t)` quede disponible para `AST-L003`;
- los argumentos `D`, `l'`, `l`, `F` sean compartidos por longitud, latitud y
  distancia;
- las correcciones DE200/LE200 queden auditables;
- las amplitudes de `ELP1` se corrijan en el evaluador, no en el importador.

---

## Respuesta Directa

Las correcciones no modifican únicamente amplitudes.

En la práctica operacional de ELP2000-82B/85:

```text
Argumentos:
    sí, provienen de ELP2000-85.

Constantes fundamentales:
    sí, están fijadas/ajustadas para la efeméride y se usan para construir W1,
    D, l', l, F y argumentos planetarios.

Amplitudes:
    sí, el problema principal aplica correcciones explícitas a las amplitudes
    efectivas usando delnu, delnp, delg, dele y delep.

Datos tabulares:
    no deben sobrescribirse durante la importación.
```

Por tanto:

```text
Importador = fidelidad de datos.
Evaluador  = semántica de la teoría.
```

Formulación operativa del principio:

```text
Importador = fidelidad de datos.
Evaluador  = fidelidad de la teoría.
```

---

## Estado

Esta lectura desbloquea el diseño conceptual de `AST-L002`, pero no su
implementación inmediata.

Antes de escribir el evaluador conviene definir los tipos fuertes:

- `ArcSeconds`;
- `JulianCenturiesTdb`;
- `Elp2000Constants`;
- `Elp2000Arguments`.
