# Lunar Pipeline v1.0

Mapa operativo de la cadena lunar de Pancanga Engine.

Estado: 🟡 En desarrollo

---

## Principio de Frontera

```text
Astronomía calcula magnitudes.
Calendario calcula presencia en ventanas civiles.
Vaiṣṇava aplica reglas tradicionales.
```

El tithi instantáneo pertenece al motor astronómico porque se define por una
diferencia angular:

```text
Tithi = (λLuna - λSol) / 12°
```

El motor calendárico decidirá qué tithi está presente al amanecer. El motor
Vaiṣṇava decidirá si un día debe observarse como Ekādaśī.

---

## Objetivo

Construir la longitud aparente de la Luna en un instante dado:

```rust
astronomy::moon::apparent_longitude(jd) -> Degrees
```

---

## Cadena Inicial

```text
ELP2000 Import Pipeline
        ↓
AST-L001  Fundamental Lunar Arguments
        ↓
AST-L002  Periodic Longitude Correction
        ↓
AST-L003  Lunar Mean Longitude W1
        ↓
AST-L004  Geocentric Lunar Longitude
        ↓
AST-L006  ELP2000 Frame Transformation
        ↓
AST-0006  Nutation in Longitude
        ↓
AST-L005  Apparent Lunar Longitude
```

---

## Algoritmos

| ID | Nombre | Entrada | Salida | Estado |
|---|---|---|---|---|
| Infra | ELP2000 Import Pipeline | `ELP1`...`ELP36` | Rust generado | Ejecutado contra IMCCE |
| AST-L001 | Fundamental Lunar Arguments | `JulianDate` | `D`, `M`, `M'`, `F` | Implementado |
| AST-L002 | Periodic Longitude Correction | `JulianDate` + tablas de longitud ELP2000 | Corrección periódica en arcseconds | Cerrado |
| AST-L003 | Lunar Mean Longitude W1 | `JulianDate` | Longitud media lunar | Implementado |
| AST-L004 | Geocentric Lunar Longitude | `W1` + corrección periódica | Longitud geocéntrica lunar | Implementado |
| AST-L006 | ELP2000 Frame Transformation | Longitud lunar ELP2000 + `JulianDate` | Longitud media/de fecha | Validado contra Swiss |
| AST-L005 | Apparent Lunar Longitude | Longitud lunar media/de fecha + `Δψ` | `λLuna` aparente/de fecha | Implementado |

---

## Regla de Implementación

Cada algoritmo lunar deberá responder una sola pregunta astronómica.

Ningún algoritmo lunar debe conocer:

- amanecer;
- zona horaria;
- Ekādaśī;
- Mahādvādaśī;
- parāṇa;
- festivales.

---

## Regla de Datos

Los coeficientes ELP2000 no se escribirán a mano dentro del motor.

La ruta aceptada será:

```text
archivos oficiales IMCCE
        ↓
elp2000-importer
        ↓
Rust generado
        ↓
evaluador lunar
```

Esto replica la separación que ya funcionó para VSOP87: los datos pertenecen a
la teoría; el evaluador solo sabe evaluar una serie.

---

## Estado

Lunar Pipeline v1.0 queda iniciado con `AST-L001`.

`AST-L002` queda especificado como corrección periódica de longitud, sin sumar
`W1`. La semántica de `ELP1` y de las constantes ajustables ELP2000-82B/85 ya
quedó validada para la primera familia; las demás familias de longitud deberán
cerrarse con el mismo criterio antes de completar `AST-L002`.

La siguiente familia natural para `AST-L002` es `ELP4`, siempre dentro del orden
oficial de sustitución de `elp82b_2` para longitud. Esta conclusión queda
registrada en:

```text
02_Research/ELP2000/notas/AST-L002-Family-Dependencies.md
```

`AST-L002` usará un mapa interno:

```text
Family -> Semantic Evaluator -> Contribution
```

El primer evaluador validado es `MainProblemEvaluator` mediante `ELP1`. El
siguiente objetivo es validar `ShortPeriodicEvaluator` mediante `ELP4`.
Los órdenes temporales se declaran como parte de cada familia (`t^0`, `t^1`,
`t^2`) y se modelan en código mediante `ElpTemporalOrder`, no mediante ramas
especiales por archivo.
