# Solar Pipeline v1.0

Mapa oficial de la cadena solar de Pancanga Engine.

Estado: 🔒 Cerrado y congelado

Este documento no es un ADR ni una RFC. Es el mapa operativo de dependencias
para construir la longitud aparente del Sol algoritmo por algoritmo.

Cualquier cambio en esta cadena deberá pasar por un ADR, salvo correcciones
tipográficas o errores científicos objetivos.

Los algoritmos no deben depender conceptualmente del número de otro AST. Deben
declarar dependencias físicas: longitud geocéntrica, radio vector, nutación,
aberración, etc. Los identificadores AST solo ordenan y documentan la entrega.

## Objetivo

Definir el orden científico y matemático de los algoritmos solares.

La cadena solar debe mantenerse separada en transformaciones pequeñas,
auditables y con significado físico explícito.

## Principio

No se implementará una fórmula monolítica para la longitud aparente del Sol.

Cada etapa deberá responder una pregunta astronómica concreta:

- ¿Dónde está la Tierra respecto del Sol?
- ¿Dónde se ve el Sol desde la Tierra?
- ¿Cuál es la distancia Tierra-Sol?
- ¿Cuál es la corrección por aberración?
- ¿Cuál es la corrección por nutación?
- ¿Cuál es la longitud aparente final?

## Grafo de Dependencias

```text
AST-0002
Earth Heliocentric Longitude (L)
        │
        ▼
AST-0003
Geocentric Solar Longitude (Lg)

AST-0004
Earth Radius Vector (R)
        │
        ▼
AST-0005
Solar Aberration
        │
        ▼
AST-0007
Apparent Solar Longitude (λ☉)

AST-0006
Nutation in Longitude (Δψ)
        │
        └──────────────► AST-0007
```

Vista compacta:

```text
          AST-0002 (L)
                │
                ▼
          AST-0003 (Lg)

AST-0004 (R) ───────┐
                    ▼
             AST-0005 (Aberration)
                    │
AST-0006 (Δψ) ──────┘
                    ▼
          AST-0007 (Apparent λ☉)
```

## Algoritmos

| ID | Nombre | Entrada | Salida | Estado |
|---|---|---|---|---|
| AST-0002 | Earth Heliocentric Longitude | `JulianDate` | `L` | Validado contra IMCCE |
| AST-0003 | Geocentric Solar Longitude | `L` | `Lg` | Implementado |
| AST-0004 | Earth Radius Vector | `JulianDate` | `R` | Implementado |
| AST-0005 | Solar Aberration | `Lg`, `R` | Longitud corregida por aberración | Implementado |
| AST-0006 | Nutation in Longitude | `JulianDate` | `Δψ` | Implementado |
| AST-0007 | Apparent Solar Longitude | `Lg`, aberración, `Δψ` | `λ☉` | Implementado |

## Alcance de Cada Etapa

### AST-0002 - Earth Heliocentric Longitude

Calcula la longitud heliocéntrica de la Tierra usando VSOP87D.

No calcula el Sol aparente.

### AST-0003 - Geocentric Solar Longitude

Invierte el punto de vista:

```text
Lg = normalize_360(L + 180°)
```

No aplica aberración.

No aplica nutación.

### AST-0004 - Earth Radius Vector

Calcula el radio vector Tierra-Sol usando VSOP87D.

Este valor será necesario para la aberración solar.

### AST-0005 - Solar Aberration

Calcula la corrección de aberración solar usando el radio vector `R`.

No calcula nutación.

### AST-0006 - Nutation in Longitude

Calcula `Δψ`, la nutación en longitud.

No depende de la longitud heliocéntrica de la Tierra.

### AST-0007 - Apparent Solar Longitude

Combina las etapas previas:

```text
λ☉ = Lg + aberración + Δψ
```

Esta será la primera longitud solar aparente de Pancanga Engine.

## Estado v1.0

La cadena solar v1.0 queda cerrada con:

- longitud heliocéntrica de la Tierra;
- longitud geocéntrica del Sol;
- radio vector Tierra-Sol;
- aberración solar;
- nutación en longitud;
- longitud aparente del Sol.

Cualquier cambio posterior deberá pasar por un ADR, salvo correcciones
tipográficas o errores científicos objetivos.

## Reglas de Implementación

- Cada AST deberá tener ficha técnica propia.
- Cada AST deberá tener tests unitarios.
- Cada AST deberá declarar sus fuentes científicas.
- Cada AST deberá declarar tolerancia y validación.
- Ningún AST posterior deberá mezclar correcciones físicas no declaradas.
- La cadena solar no debe conocer reglas de Tithi, Ekādaśī ni calendario
  Gauḍīya.

## Scientific Validation

Cada algoritmo solar terminado deberá cerrar con:

```text
Scientific Validation

Fuente oficial:
...

Casos de prueba:
...

Error máximo:
...

Error medio:
...

Tolerancia:
...

Estado:
...
```
