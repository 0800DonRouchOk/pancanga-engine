# ELP1 Semantics and Lunar Longitude Definition

Estado: 🟡 Nota de investigación técnica

Objetivo: responder cómo define ELP2000-82B la longitud eclíptica de la Luna y
qué papel cumple `ELP1` dentro de `AST-L002`.

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

La rutina `elp82b_2` se toma como referencia operacional porque es la versión
oficial rápida con los mismos datos `ELP1`...`ELP36`.

---

## Variable Temporal

La notice define `t` como tiempo baricéntrico TDB en siglos julianos desde
J2000.0.

La rutina oficial implementa:

```fortran
t(1) = (tjj - 2451545.0d0) / 36525.d0
t(2) = t(1) * t(1)
t(3) = t(2) * t(1)
t(4) = t(3) * t(1)
```

Por tanto, para ELP2000-82B:

```text
t = Julian centuries TDB from J2000.0
```

No es el `τ` de VSOP87.

---

## Argumentos Fundamentales en Notación ELP

La notice expresa los argumentos como polinomios:

```text
λ = λ(0) + λ(1)t + λ(2)t² + λ(3)t³ + λ(4)t⁴
```

Los argumentos de Delaunay se derivan de:

```text
W1 = mean longitude of the Moon
W2 = mean longitude of lunar perigee
W3 = mean longitude of lunar ascending node
T  = mean heliocentric mean longitude of the Earth-Moon barycenter
ϖ0 = mean longitude of the perihelion of the Earth-Moon barycenter
```

La notice define:

```text
D  = W1 - T + 180°
l' = T - ϖ0
l  = W1 - W2
F  = W1 - W3
```

La rutina `elp82b_2` codifica esos argumentos como:

```fortran
del(1,i) = w(1,i) - eart(i)
del(4,i) = w(1,i) - w(3,i)
del(3,i) = w(1,i) - w(2,i)
del(2,i) = eart(i) - peri(i)
del(1,0) = del(1,0) + pi
```

Correspondencia:

```text
del(1) = D
del(2) = l'
del(3) = l
del(4) = F
```

---

## Definición de Longitud Lunar

La notice define la longitud media lunar:

```text
W1(t) = W1(0) + νt + W1(2)t² + W1(3)t³ + W1(4)t⁴
```

También indica que `W1` debe añadirse a la suma de las series de longitud
Fourier y Poisson para obtener la longitud de la Luna referida a la eclíptica
dinámica media de la fecha.

Operacionalmente, `elp82b_2` lo implementa así:

```fortran
r(1) = sum of longitude series in arcseconds

r(1) = r(1) / rad
     + w(1,0)
     + w(1,1)t
     + w(1,2)t²
     + w(1,3)t³
     + w(1,4)t⁴
```

En notación de Pancanga Engine:

```text
Δλ_periodic(t) = sum of ELP longitude Fourier and Poisson series

λ_moon,date(t) = W1(t) + Δλ_periodic(t) / rad
```

donde:

```text
rad = 648000 / π arcseconds per radian
```

Conclusión:

```text
AST-L002 = Δλ_periodic(t)
AST-L003 = W1(t)
AST-L004 = W1(t) + Δλ_periodic(t) / rad
```

---

## Qué Representa ELP1

`ELP1` es la parte de longitud del problema principal lunar.

El `README` oficial lo describe como:

```text
Main problem. Longitude periodic terms (sine)
```

Pero no es una lista genérica `A sin(argument)` sin más. Sus columnas incluyen
coeficientes del problema principal y derivadas respecto de constantes
ajustables.

La rutina oficial lee cada término de `ELP1` con:

```fortran
read (...) ilu, coef
```

donde:

```text
ilu[1..4] = multiplicadores de D, l', l, F
coef[1..6] = amplitud base y derivadas/correcciones del problema principal
```

Para el problema principal, el argumento se forma como:

```text
y(t) = Σ_i ilu_i * del_i(t)
```

con `del_i(t)` evaluado como polinomio hasta cuarto grado.

La amplitud efectiva no es solamente `coef(1)`.

La rutina calcula:

```fortran
tgv = coef(2) + dtasm * coef(6)

x = coef(1)
  + tgv * (delnp - am * delnu)
  + coef(3) * delg
  + coef(4) * dele
  + coef(5) * delep
```

Luego suma:

```fortran
r(1) = r(1) + x * sin(y)
```

para longitud.

Por tanto:

```text
ELP1 term = corrected_amplitude * sin(D/l'/l/F combination)
```

No:

```text
raw_amplitude * sin(raw_argument)
```

---

## Constantes Ajustables Usadas por ELP1

La rutina oficial declara:

```fortran
am    = 0.074801329518
alpha = 0.002571881335
dtasm = 2 * alpha / (3 * am)
```

y las correcciones ajustadas a DE200/LE200:

```fortran
delnu = +0.55604 / rad / w(1,1)
dele  = +0.01789 / rad
delg  = -0.08066 / rad
delnp = -0.06424 / rad / w(1,1)
delep = -0.12879 / rad
```

Estas constantes pertenecen a la semántica del problema principal. Deben
aplicarse antes de evaluar `ELP1`.

---

## Semántica de las Otras Tablas de Longitud

Las tablas de longitud no principales se evalúan con una forma más directa:

```text
x * sin(y)
```

pero el argumento `y` depende del tipo de tabla.

### Tablas Short Periodic

Archivos:

```text
ELP4, ELP7, ELP22, ELP25, ELP28, ELP31, ELP34
```

La rutina usa:

```text
y(t) = phase + iz*zeta(t) + Σ_i ilu_i * del_i(t)
```

con solo parte constante y lineal del argumento.

### Tablas Planetary

Archivos:

```text
ELP10, ELP13, ELP16, ELP19
```

Para `ELP10` y `ELP13`, el argumento combina longitudes planetarias con
`D`, `l` y `F`.

Para `ELP16` y `ELP19`, el argumento combina longitudes planetarias con los
cuatro argumentos `D`, `l'`, `l`, `F`.

---

## Factores de Poisson

La rutina confirma:

```text
ELP7, ELP13, ELP19, ELP25  -> multiplicar amplitud por t
ELP34                     -> multiplicar amplitud por t²
```

Esto coincide con la interpretación documental de las familias `/t` y `/t2`.

---

## Respuesta Directa a la Pregunta

ELP2000-82B define la longitud eclíptica lunar como:

```text
λ_moon,date(t) = W1(t) + Δλ_periodic(t) / rad
```

donde:

```text
W1(t) = W1(0) + νt + W1(2)t² + W1(3)t³ + W1(4)t⁴
```

y:

```text
Δλ_periodic(t)
```

es la suma de las series de longitud Fourier y Poisson de ELP2000-82B,
incluyendo `ELP1` evaluado con sus correcciones de constantes.

La salida correcta de `AST-L002` es:

```text
Δλ_periodic(t)
```

en segundos de arco, firmada.

`AST-L002` no debe sumar `W1`.

---

## Consecuencia Para Pancanga Engine

Antes de implementar `AST-L002`, el evaluador debe soportar tres familias:

```text
MainProblemLongitudeTerm
ShortPeriodicLongitudeTerm
PlanetaryLongitudeTerm
```

`ELP1` requiere una ruta especial para calcular la amplitud corregida.

Las otras tablas no deben forzar a `ELP1` dentro de un evaluador genérico
demasiado simple.

---

## Estado

La semántica básica de `ELP1` queda identificada a partir de la notice y de las
rutinas oficiales.

Pendiente antes de código:

- decidir cómo representar `ArcSeconds`;
- decidir cómo nombrar el tipo de tiempo `JulianCenturiesTdb`;
- diseñar el evaluador sin mezclar longitud, latitud y distancia;
- elegir casos de validación contra la rutina oficial o Swiss Ephemeris.
