# AST-L006 - ELP2000 Frame Transformation

Campaign: 46A.3 / 46A.4

Status: ✅ Implemented / validated against Swiss Ephemeris

## Objetivo

Especificar cómo Pancanga Engine debe transformar la longitud lunar derivada de
ELP2000 antes de compararla con Swiss Ephemeris y antes de usarla en el motor
calendárico.

Campaign 46A.3 specified the transformation. Campaign 46A.4 implemented it.

## Pregunta Respondida

```text
ELP2000 longitude scalar
        ↓
Frame transformation
        ↓
Lunar longitude compatible with Solar apparent longitude
        ↓
Calendar Engine
```

## Evidencia

Campaign 46A.2 identificó que la diferencia lunar contra Swiss no era un error
numérico local sino una diferencia de marco de referencia.

Comparación inicial:

| Configuración Swiss | Error lunar máximo |
| ------------------- | ------------------ |
| Swiss apparent/date | 1.410526337432° |
| Swiss `-j2000` | 0.006258866269° |

Esto demuestra que la longitud lunar actual de Pancanga Engine está mucho más
cerca del marco J2000 que del marco aparente/de fecha que usa Swiss por defecto.

La fuente ELP2000 local también declara que la rutina completa devuelve
coordenadas rectangulares en:

```text
mean dynamical ecliptic and inertial equinox of J2000
```

y contiene una sección explícita de matriz de precesión.

## Qué Entrega Hoy Pancanga Engine

El pipeline lunar actual calcula:

```text
W1(t)
        ↓
Δλ_periodic(t)
        ↓
λ_elp = W1 + Δλ_periodic / 3600
        ↓
λ_current = λ_elp + Δψ
```

Esta salida fue nombrada como longitud lunar aparente, pero la comparación
externa muestra que todavía no está en el mismo marco que la longitud aparente
solar de fecha.

Clasificación técnica:

```text
λ_elp:
longitud lunar escalar ELP2000 antes de corrección de marco de fecha

λ_current:
λ_elp + nutación, todavía sin precesión escalar hacia fecha
```

## Qué Necesita El Calendar Engine

Para calcular elongación y tithi, el motor calendárico necesita que Sol y Luna
estén expresados en el mismo marco.

El Solar Engine ya coincide con Swiss en longitudes aparentes/de fecha:

```text
Error solar máximo contra Swiss:
0.000167784533°
```

Por tanto, la salida lunar pública que consume calendario debe ser:

```text
longitud geocéntrica lunar aparente/de fecha
```

No debe mezclar:

```text
Sol apparent/date
Luna J2000-like
```

## Qué Espera Swiss

El modo oficial de validación contra Swiss debe usar:

```text
swetest -edir<ephe> -eswe
```

sin `-j2000`.

Ese modo representa la referencia externa para longitudes aparentes/de fecha.
La bandera `-j2000` es útil para diagnóstico, pero no debe usarse para validar
tithi final, porque pondría el Sol y la Luna en un marco distinto al del motor
calendárico esperado.

## Transformación Normativa Para v1.0

Para la longitud lunar escalar requerida por tithi, Pancanga Engine debe aplicar
la constante oficial de precesión de ELP2000:

```text
precess = 5029.0966 arcsec / century
```

Con:

```text
t = (JD_TDB - 2451545.0) / 36525
```

La transformación especificada para v1.0 es:

```text
λ_mean_of_date =
normalize_360(
    λ_elp + (5029.0966 / 3600) × t
)
```

Luego se aplica la nutación en longitud compartida con el Solar Engine:

```text
λ_apparent_of_date =
normalize_360(
    λ_mean_of_date + Δψ
)
```

Equivalente operacional para el pipeline actual:

```text
λ_apparent_of_date =
normalize_360(
    λ_current + (5029.0966 / 3600) × t
)
```

La forma preferida para implementación es la primera, porque separa mejor las
responsabilidades:

```text
ELP2000 scalar longitude
        ↓
mean-of-date frame correction
        ↓
nutation
        ↓
apparent-of-date longitude
```

## Validación De La Transformación Escalar

Aplicando únicamente:

```text
λ_corrected = λ_current + (5029.0966 / 3600) × t
```

y comparando contra Swiss con `-edir<ephe> -eswe`, los 1000 casos de Campaign
46A producen:

| Métrica | Resultado |
| ------- | --------- |
| Error lunar máximo | 0.000469445269° |
| Error lunar medio | 0.000216546462° |
| Error lunar RMS | 0.000245166456° |
| Error elongación máximo | 0.000330443529° |
| Error elongación medio | 0.000192686925° |
| Error elongación RMS | 0.000204943127° |
| Tithi mismatches | 0 |

Esto confirma que la corrección de marco requerida para tithi v1.0 es la
precesión escalar de ELP2000, no un ajuste empírico.

## Dónde Debe Vivir

La transformación debe vivir en el Astronomy Engine como algoritmo explícito.

No debe vivir en:

```text
ELP2000 interpreter
Swiss validation adapter
Calendar Engine
Vaiṣṇava Engine
```

Razón:

```text
ELP2000 calcula la teoría lunar.
La transformación de marco prepara la magnitud astronómica pública.
El calendario consume solo APIs astronómicas ya compatibles.
```

Nombre sugerido:

```text
AST-L006
elp2000_frame_precession(...)
```

El nombre exacto puede adaptarse al código, pero la responsabilidad debe seguir
siendo única:

```text
convertir una longitud lunar ELP2000 escalar al marco medio/de fecha
```

## Lo Que No Hace

AST-L006 no debe:

- modificar coeficientes ELP2000;
- modificar el importador;
- modificar el intérprete;
- recalcular Sol;
- calcular tithi;
- aplicar reglas calendáricas;
- aplicar reglas vaiṣṇavas;
- usar Swiss como fuente normativa.

## Nota Sobre La Transformación Rectangular Completa

La rutina oficial ELP2000 completa transforma longitud, latitud y distancia a
coordenadas rectangulares y aplica una matriz de precesión con coeficientes
`p1...p5` y `q1...q5`.

Pancanga Engine v1.0 necesita solo longitud lunar para tithi. La validación
externa muestra que la precesión escalar oficial:

```text
5029.0966 arcsec / century
```

es suficiente para alinear la longitud lunar con Swiss a nivel de tithi en la
muestra de 1000 casos.

Cuando se implementen latitud lunar, distancia lunar o una posición lunar
vectorial completa, debe abrirse una campaña separada para implementar la
transformación rectangular oficial completa.

## Criterio De Cierre

La implementación de AST-L006 se cierra con:

```text
Swiss Ephemeris:
PASS

Tithi mismatches:
0 en la muestra de 1000 casos

ELP2000 interpreter:
SIN CAMBIOS

Knowledge Base:
SIN CAMBIOS
```

## Estado

```text
Campaign 46A.4

✅ PASS

Frame Transformation:
IMPLEMENTED

Código:
YES

Motor:
AST-L006 ADDED

Knowledge Base:
SIN CAMBIOS
```
