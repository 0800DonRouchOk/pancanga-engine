# ELP2000 Short Periodic Oracle

Estado: Campaign 05 cerrado

Este documento versiona el oraculo independiente para la semantica
`ShortPeriodic` del `ELP2000 Interpreter`, cubriendo la familia completa
`ELP4`.

El oraculo no llama codigo Rust de Pancanga Engine. Se construye directamente
desde:

- datos oficiales IMCCE (`ELP4`);
- constantes y argumentos descritos por ELP2000-82B/85;
- procedimiento operacional documentado por `elp82b_2`.

---

## Oracle ID

```text
ELP2000-ELP4-J2000-Contributions
```

## Alcance

Validar los 347 terminos oficiales de `ELP4`.

No valida:

- `AST-L002`;
- longitud lunar;
- `W1`;
- nutacion;
- tithi;
- calendario;
- reglas vaisnavas.

---

## Fuente

```text
Source file:
06_Data/Reference/ELP2000/raw/ELP4

Source theory:
ELP2000-82B

Operational reference:
02_Research/ELP2000/sources/elp82b_2

Source SHA-256:
f27ea439bf8f4fd35bed31c0a42de5414db07f9fcc3587237f6908891d43d773
```

Primer registro cientifico:

```text
0 0 0 0 1 270.00000 0.00003
```

Interpretacion:

```text
iz    = 0
ilu   = [0, 0, 0, 1]
phase = 270.00000 degrees
xx    = 0.00003 arcsec
```

---

## Entrada

```text
JD = 2451545.0
t  = 0.0 Julian centuries TDB from J2000.0
```

---

## Algoritmo Independiente

Para una familia short-periodic, `elp82b_2` prepara:

```text
y0 = phase * deg + iz*zeta(0) + sum_i ilu_i*del_i(0)
y1 =               iz*zeta(1) + sum_i ilu_i*del_i(1)
```

y luego sustituye:

```text
y(t) = y0 + y1*t
Delta_lambda_i(t) = xx * sin(y(t))
```

Para el primer termino de `ELP4`:

```text
y(t) = 270 degrees + F(t)
```

porque:

```text
iz = 0
ilu = [0, 0, 0, 1]
```

En J2000:

```text
F0 = W1(0) - W3(0)
```

con:

```text
W1(0) = 218 deg 18 min 59.95571 sec
W3(0) = 125 deg 02 min 40.39816 sec
```

Valores independientes:

```text
F0       = 1.627905233371468086 rad
phase   = 4.712388980384689674 rad
argument= 6.340294213756157760 rad
xx      = 0.000030000000000000 arcsec
sin(arg)= 0.057077868881385586
```

Contribucion del primer termino:

```text
Delta_lambda_1 = 0.000001712336066442 arcsec
```

Suma parcial de los primeros 10 terminos:

```text
Delta_lambda_1..10 = 0.004759349973172258 arcsec
```

Suma parcial de los primeros 100 terminos:

```text
Delta_lambda_1..100 = 0.027406824891217551 arcsec
```

Suma total de la familia completa `ELP4`:

```text
Delta_lambda_ELP4 = 5.031975142947603175 arcsec
```

---

## Output

```text
Output file:
04_Tests/Astronomical/ELP2000-ELP4-J2000-Contributions.csv

Output columns:
index,argument_rad,xx_arcsec,contribution_arcsec,partial_sum_arcsec

Rows:
347

Output SHA-256:
92f2d3fe27a252aee3e51dfca2524b7dcd03369829efe1ed5476fd04c1a8614f
```

---

## Validacion Esperada

```text
argument tolerance:
1.0e-15 rad

xx tolerance:
1.0e-18 arcsec

contribution tolerance:
1.0e-18 arcsec

Expected result:
PASS
```

Si el interprete difiere del oraculo:

- no ajustar constantes para forzar coincidencia;
- no modificar la arquitectura congelada;
- documentar la diferencia;
- revisar la interpretacion de la semantica `ShortPeriodic`.
