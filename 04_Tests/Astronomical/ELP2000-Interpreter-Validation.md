# ELP2000 Interpreter Validation

Estado: ✅ ELP1 validado contra oráculo independiente

Objetivo: registrar evaluaciones independientes de términos oficiales de `ELP1`
en J2000.0 para validar el mecanismo del `ELP2000 Interpreter`.

Este archivo no implementa código.

---

## Fuente

Archivo oficial:

```text
06_Data/Reference/ELP2000/raw/ELP1
```

Familia generada:

```text
ELP1
Términos: 1023
Fingerprint: 0xa460f0ac886bf342
Referencia: ELP2000-82B
```

---

## Entrada

```text
JD = 2451545.0
t  = 0.0 Julian centuries TDB from J2000.0
```

---

## Primer Término

Primer registro científico:

```text
0  0  0  2     -411.60287      168.48   -18433.81     -121.62        0.40       -0.18        0.00
```

Multiplicadores:

```text
[0, 0, 0, 2]
```

Por tanto, el argumento del término es:

```text
2F
```

Argumentos ELP en J2000:

```text
D  =  5.198466741027443 rad
l' = -0.043125180208125 rad
l  =  2.355555898265799 rad
F  =  1.627905233371468 rad
```

Argumento del primer término:

```text
2F = 3.255810466742936 rad
```

Amplitud base:

```text
-411.602870000000000 arcsec
```

Amplitud corregida:

```text
-411.595672253951705 arcsec
```

Contribución individual:

```text
Δλ_1 = 46.909407726369665 arcsec
```

Este caso valida:

- lectura del registro fuente;
- construcción del argumento `2F`;
- aplicación de correcciones de amplitud del problema principal;
- contribución individual en segundos de arco.

---

## Primeros 10 Términos

La siguiente tabla fue calculada independientemente desde el archivo crudo
`ELP1`, usando la misma semántica operacional documentada desde `elp82b_2`.

```text
N   Multiplicadores   Argumento (rad)       Amplitud corregida (")    Contribución (")        Suma parcial (")
1   [0, 0, 0, 2]      3.255810466742936     -411.595672253951705       46.909407726369665      46.909407726369665
2   [0, 0, 0, 4]      6.511620933485872        0.420325326402302        0.095184379669358      47.004592106039020
3   [0, 0, 0, 6]      9.767431400228809       -0.000589968715943        0.000198222088956      47.004790328127974
4   [0, 0, 1, -6]    -7.411875501963009        0.000159992178986       -0.000144609342556      47.004645718785419
5   [0, 0, 1, -4]    -4.156065035220073       -0.080187303291296       -0.068095222465122      46.936550496320294
6   [0, 0, 1, -2]    -0.900254568477137       39.533294846642107      -30.973748515740343      15.962801980579950
7   [0, 0, 1, 0]      2.355555898265799    22639.585779999782972    16018.824329116887384   16034.787131097467864
8   [0, 0, 1, 2]      5.611366365008736      -45.099602226720890       28.070474326497681   16062.857605423965651
9   [0, 0, 1, 4]      8.867176831751671        0.090916966899834        0.048108914488250   16062.905714338454345
10  [0, 0, 1, 6]     12.122987298494607       -0.000189992178986        0.000081506280391   16062.905795844735621
```

Suma parcial de los primeros 10 términos:

```text
16062.905795844735621 arcsec
```

Esta tabla valida que el intérprete no usa lógica especial para el primer
término: recorre una familia de datos y aplica la misma semántica a cada
registro.

---

## Subconjunto de 100 Términos

El mismo mecanismo del intérprete fue validado sobre los primeros 100 términos
de `ELP1`.

Puntos de control independientes:

```text
Términos evaluados    Suma parcial (arcsec)
1                      46.909407726369665
10                  16062.905795844735621
25                  15314.322215712032630
50                  15304.996677063070820
75                  15360.039551360734549
100                 15362.154930280750705
```

La validación de 100 términos no introduce una semántica nueva. Confirma que el
intérprete puede recorrer un subconjunto más amplio de `ELP1` aplicando el mismo
evaluador de término principal.

---

## Familia Completa ELP1

El oráculo independiente completo queda registrado como fixture CSV:

```text
04_Tests/Astronomical/ELP2000-ELP1-J2000-Contributions.csv
```

Su identidad científica queda versionada en:

```text
04_Tests/Astronomical/ELP2000-Oracle-Manifest.md
```

Cada fila contiene:

```text
index,contribution_arcsec,partial_sum_arcsec
```

Puntos de control independientes:

```text
Términos evaluados    Contribución (arcsec)      Suma parcial (arcsec)
1                         46.909407726369665       46.909407726369665
10                         0.000081506280391    16062.905795844735621
25                        -0.000035833769806    15314.322215712032630
50                        -0.024225162241017    15304.996677063070820
75                        -0.000024726081058    15360.039551360734549
100                        0.000333608706669    15362.154930280750705
250                       -0.007864080591296    15443.273988445484065
500                        0.000617856865812    18006.745323595670925
750                        0.001177655293477    17991.107686061011918
1000                      -0.000944456020868    17990.968817877332185
1023                       0.000019780211897    17990.968840315887064
```

Suma total de `ELP1` en J2000:

```text
17990.968840315887064 arcsec
```

---

## ELP1 Summary

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

Estas métricas comparan, término por término, las contribuciones producidas por
el intérprete Rust contra el oráculo independiente generado desde el archivo
crudo oficial `ELP1`.

---

## Campaign 02/03/04/05 - ELP4 ShortPeriodic Family

La primera validación de la semántica `ShortPeriodic` empezó con el primer
término oficial de `ELP4` y luego se extendió al subconjunto de los primeros 10
términos oficiales. Campaign 04 reutilizó exactamente el mismo mecanismo para
validar los primeros 100 términos oficiales. Campaign 05 reutilizó el mismo
mecanismo para validar la familia completa.

Fuente:

```text
06_Data/Reference/ELP2000/raw/ELP4
```

Primer registro científico:

```text
0 0 0 0 1 270.00000 0.00003
```

Evaluación J2000:

```text
primer argumento       = 6.340294213756157760 rad
primer xx              = 0.000030000000000000 arcsec
primera contribución   = 0.000001712336066442 arcsec
suma parcial 10 terms  = 0.004759349973172258 arcsec
suma parcial 100 terms = 0.027406824891217551 arcsec
suma total ELP4        = 5.031975142947603175 arcsec
```

Oráculo independiente:

```text
04_Tests/Astronomical/ELP2000-Short-Periodic-Oracle.md
04_Tests/Astronomical/ELP2000-ELP4-J2000-Contributions.csv
```

Estado:

```text
Familia completa ELP4:
PASS

ShortPeriodic:
validado con ELP4
```

---

## Estado

`ELP1` queda cerrado como familia del problema principal de longitud.

`ShortPeriodic` queda validado sobre los 347 términos oficiales de `ELP4`.

Esto no cierra `AST-L002`: todavía falta interpretar las demás familias de
longitud ELP2000. El siguiente paso natural es `ELP7`, que reutiliza
`ShortPeriodic` con orden temporal `t^1`.
