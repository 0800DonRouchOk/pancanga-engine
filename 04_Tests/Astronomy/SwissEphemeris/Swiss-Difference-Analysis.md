# Swiss Difference Analysis

Campaign: 46A.2

Status: 🟢 Cause identified / engine unchanged

## Objetivo

Explicar el origen de las diferencias entre Pancanga Engine y Swiss Ephemeris
sin modificar el motor.

## Resultado

La diferencia lunar principal queda clasificada como:

```text
MODEL DIFFERENCE / REFERENCE FRAME
```

No se modificó código del motor, Knowledge Base ni arquitectura.

## Evidencia Cuantitativa

La comparación inicial contra Swiss en longitudes aparentes de fecha produjo:

| Métrica | Resultado |
| ------- | --------- |
| Casos | 1000 |
| PASS automático | 7 |
| ASTRONOMY DIFFERENCE | 993 |
| Error solar máximo | 0.000160884533 grados |
| Error lunar máximo | 1.410719337432 grados |
| Error de elongación máximo | 1.410783648046 grados |
| Tithi mismatches | 56 |

La diferencia no es aleatoria. Crece casi linealmente desde J2000:

| Caso | Años desde J2000 | Diferencia lunar contra Swiss de fecha |
| ---- | ---------------- | -------------------------------------- |
| SWISS-0001 | -99.999 | +1.396984681823 grados |
| SWISS-0250 | -49.900 | +0.697201089595 grados |
| SWISS-0500 | +0.399 | -0.005382686828 grados |
| SWISS-0501 | +0.600 | -0.008159241279 grados |
| SWISS-0750 | +50.699 | -0.707934232678 grados |
| SWISS-1000 | +100.998 | -1.410526337432 grados |

Ese patrón es compatible con una diferencia de marco de referencia/equinoccio,
no con error numérico local.

## Prueba De Banderas Swiss

Se compararon las mismas fechas con Swiss Ephemeris usando efemérides locales
reales y dos configuraciones:

```text
swiss_date_ephe:
swetest -edir<ephe> -eswe

swiss_j2000_ephe:
swetest -edir<ephe> -eswe -j2000
```

Resultados sobre 1000 casos:

| Variante Swiss | Error solar máximo | Error lunar máximo | Error elongación máximo | Tithi mismatches |
| --------------- | ------------------ | ------------------ | ----------------------- | ---------------- |
| Longitudes de fecha | 0.000167784533° | 1.410526337432° | 1.410589848045° | 56 |
| Longitudes J2000 | 1.413638910613° | 0.006258866269° | 1.410265648045° | 56 |

Interpretación:

- El Sol de Pancanga coincide con Swiss en longitudes aparentes/de fecha.
- La Luna de Pancanga coincide mucho mejor con Swiss cuando Swiss se fuerza a
  `-j2000`.
- La elongación sigue fallando porque Sol y Luna quedan en marcos distintos si
  se compara una magnitud contra fecha y la otra contra J2000.

## Evidencia De La Fuente ELP2000

Las rutinas oficiales ELP2000 locales indican que la salida rectangular usa:

```text
reference frame : mean dynamical ecliptic and inertial
equinox of J2000 (JD 2451545.0)
```

También declaran una sección explícita:

```text
Precession matrix
```

con los coeficientes `p1...p5` y `q1...q5`.

Esto confirma que ELP2000 contiene una etapa de marco/precesión que debe
tratarse cuidadosamente antes de comparar con longitudes aparentes de fecha.

## Observación Sobre La Infraestructura Swiss

El primer CSV de 1000 casos fue generado por `swetest` sin una ruta explícita
`-edir`. En ese modo, Swiss avisó que no encontraba archivos `.se1` y usó
Moshier como fallback.

Ese fallback no explica la diferencia lunar principal: al repetir la prueba con
`-edir` apuntando a los archivos `.se1`, el patrón lunar permaneció.

Antes de certificación final, el runner debe configurarse para usar una ruta de
efemérides explícita.

## Clasificación Por Pregunta

| Pregunta | Resultado |
| -------- | --------- |
| Misma época | No. La comparación mezcla longitud solar de fecha con longitud lunar cercana a J2000. |
| Modelo Swiss | Swiss de fecha incluye el marco/equinoccio de fecha; `-j2000` alinea mejor con la Luna de Pancanga. |
| Flags actuales | `-bj`, `-p0/-p1`, `-fPl`, `-g,`, `-head`, `-eswe`; falta `-edir` explícito. |
| Escala temporal | No aparece como causa principal; el patrón es secular y cambia de signo en J2000. |
| Delta T | No explica una diferencia secular de ±1.4° por siglo. |
| Nutación | No explica la magnitud; sus efectos son de orden mucho menor. |
| Aberración/deflexión | No explica la magnitud. |
| Precesión/equinoccio | Causa principal probable. |
| Bug de implementación | No se declara todavía. La implementación debe revisarse contra la etapa de precesión de ELP2000 antes de cambiar código. |

## Conclusión

Campaign 46A.2 no autoriza a ajustar constantes ni a modificar el motor para
"hacer coincidir" Swiss.

La evidencia indica que `astronomy::moon::apparent_longitude(jd)` está
devolviendo una longitud lunar cercana al marco J2000 de ELP2000, mientras que
`astronomy::solar::apparent_longitude(jd)` coincide con Swiss en el marco de
fecha. Para validar tithi contra Swiss, Sol y Luna deben estar en el mismo
marco de referencia.

## Próximo Paso Recomendado

Campaign 46A.3 especificó la transformación requerida:

```text
05_Documentation/Algorithms/AST-L006-ELP2000-Frame-Transformation.md
```

Resultado:

```text
Frame Transformation:
SPECIFIED
```

Campaign 46A.4 implementó AST-L006 y la comparación Swiss posterior cerró con
1000 / 1000 casos `PASS` y 0 diferencias de tithi.
