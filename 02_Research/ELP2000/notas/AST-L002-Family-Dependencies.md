# AST-L002 Family Dependencies

Estado: ✅ Investigación puntual registrada

Objetivo: verificar si las familias de longitud ELP2000-82B deben
implementarse en un orden específico antes de abrir `ELP4`.

---

## Fuentes

```text
02_Research/ELP2000/sources/README
02_Research/ELP2000/sources/elp82b_2
06_Data/Reference/ELP2000/raw/ELP4
```

---

## Organización Oficial

La documentación IMCCE indica que los 36 archivos contienen series para las
tres coordenadas esféricas:

```text
longitude
latitude
distance
```

Los archivos se organizan por grupos de tres. Para cada grupo:

```text
archivo 1 del grupo -> longitude
archivo 2 del grupo -> latitude
archivo 3 del grupo -> distance
```

Por tanto, para `AST-L002`, que calcula solo la corrección periódica de
longitud, las familias relevantes son:

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

---

## Evidencia en `elp82b_2`

La subrutina oficial usa:

```fortran
itab = (ific + 2) / 3
iv   = mod(ific - 1, 3) + 1
```

donde:

```text
iv = 1 -> longitude
iv = 2 -> latitude
iv = 3 -> distance
```

Luego, en la sustitución temporal, recorre:

```fortran
do iv = 1, 3
   r(iv) = 0
   do itab = 1, 12
      ...
      r(iv) = r(iv) + x * sin(y)
   enddo
enddo
```

Para longitud (`iv = 1`), esto define el orden natural:

| `itab` | Familia | Función física |
|---:|---|---|
| 1 | `ELP1` | Problema principal lunar |
| 2 | `ELP4` | Figura terrestre |
| 3 | `ELP7` | Figura terrestre, términos proporcionales a `t` |
| 4 | `ELP10` | Perturbaciones planetarias, tabla 1 |
| 5 | `ELP13` | Perturbaciones planetarias, tabla 1, términos proporcionales a `t` |
| 6 | `ELP16` | Perturbaciones planetarias, tabla 2 |
| 7 | `ELP19` | Perturbaciones planetarias, tabla 2, términos proporcionales a `t` |
| 8 | `ELP22` | Efectos de marea |
| 9 | `ELP25` | Efectos de marea, términos proporcionales a `t` |
| 10 | `ELP28` | Figura lunar |
| 11 | `ELP31` | Perturbaciones relativistas |
| 12 | `ELP34` | Excentricidad solar, términos proporcionales a `t²` |

---

## Dependencias Matemáticas

Las familias de longitud se suman de forma acumulativa en `r(1)`.

No se observa que `ELP4` dependa de la salida numérica de `ELP1`.

Todas las familias comparten infraestructura matemática:

- tiempo `t`;
- argumentos lunares `del`;
- argumento `zeta` para familias short-periodic;
- argumentos planetarios para familias planetary.

Por tanto, la dependencia real no es:

```text
ELP4 depende de ELP1
```

sino:

```text
ELP4 depende de los argumentos ELP comunes y de su propia semántica short-periodic.
```

---

## Semántica de ELP4

El encabezado oficial de `ELP4` es:

```text
EARTH FIGURE PERTURBATIONS. LONGITUDE
```

El formato leído por `elp82b_2` para `ELP4` es el de familias short-periodic:

```fortran
read (...) iz, ilu, pha, xx
```

La rutina construye el argumento como:

```text
y(t) = pha + iz*zeta(t) + Σ ilu_i*del_i(t)
```

usando solo términos constante y lineal:

```text
y(t) = y0 + y1*t
```

La contribución es:

```text
xx * sin(y)
```

Para `ELP4` no se aplican:

- correcciones de amplitud del problema principal;
- factor externo `t`;
- factor externo `t²`;
- suma de `W1`;
- nutación;
- transformación de marco.

---

## Conclusión

`ELP4` es la siguiente familia natural si el objetivo es completar `AST-L002`
por el orden oficial de sustitución de las familias de longitud.

Esta decisión no se basa en una intuición de numeración, sino en la organización
de `elp82b_2`:

```text
iv = 1, itab = 2 -> ELP4
```

La implementación de `ELP4` no debe modificar la semántica de `ELP1`.

Debe agregar únicamente el conocimiento científico propio de las familias
short-periodic.
