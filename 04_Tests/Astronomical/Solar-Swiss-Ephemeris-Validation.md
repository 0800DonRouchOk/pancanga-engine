# Solar Engine vs Swiss Ephemeris

Estado: 🟡 Pendiente de ejecución

---

## Objetivo

Comparar empíricamente la longitud aparente del Sol calculada por Pancanga
Engine contra Swiss Ephemeris.

Esta prueba no decide por intuición si el Solar Engine v1.0 debe cambiar.
Primero mide.

---

## API de Pancanga Engine

La magnitud a validar es:

```rust
pancanga_engine::astronomy::solar::apparent_longitude(jd)
```

Salida:

```text
Degrees, normalizado a [0°, 360°)
```

---

## Oráculo Externo

Referencia:

```text
Swiss Ephemeris
```

Herramienta esperada:

```text
swetest
```

Estado local actual:

```text
swetest no está disponible en el entorno actual.
```

Por esa razón no se registran todavía resultados numéricos.

---

## Rango de Prueba

Rango inicial:

```text
1900-01-01 a 2100-12-31
```

Muestra:

```text
100 fechas astronómicamente sensibles.
```

Distribución de la muestra:

| Grupo | Cantidad | Motivo |
|---|---:|---|
| Fechas aleatorias | 20 | Cobertura general del rango. |
| Cercanas a novilunio | 20 | Zona crítica para fases lunares. |
| Cercanas a plenilunio | 20 | Control de oposición Sol-Luna. |
| Cercanas a cambios de tithi | 20 | Casos de mayor sensibilidad calendárica. |
| Fechas extremas | 20 | Cobertura de borde: 1900, 1950, 2000, 2050, 2100 y alrededores. |

La muestra no será uniforme. El objetivo es medir el error en condiciones donde
un pequeño desplazamiento angular podría tener consecuencias calendáricas.

---

## Magnitud Comparada

```text
Longitud eclíptica tropical aparente del Sol.
```

Antes de ejecutar la prueba se deberán fijar explícitamente:

- escala temporal del Julian Date usado por Pancanga Engine;
- si Swiss Ephemeris recibirá UT o TT;
- flags exactos de Swiss Ephemeris;
- sistema de coordenadas;
- normalización angular;
- formato de salida.

---

## Cálculo del Error

El error angular deberá calcularse como distancia circular mínima:

```text
error = min(|a - b|, 360° - |a - b|)
```

Métricas obligatorias:

- error máximo;
- error medio;
- error RMS;
- fecha de mayor error;
- cantidad de muestras.
- error agrupado por tipo de muestra.

Unidades de reporte:

- grados;
- segundos de arco;
- radianes cuando sea útil.

---

## Tolerancia

Tolerancia oficial:

```text
TBD
```

La tolerancia se definirá después de comparar Pancanga Engine contra Swiss
Ephemeris y evaluar el impacto real sobre el cálculo del tithi.

No se fija todavía `1"` ni ningún otro valor porque la sensibilidad calendárica
del motor completo aún no fue medida.

---

## Criterio de Decisión

Si el Solar Engine v1.0 queda dentro de la tolerancia definida:

```text
No se modifica el modelo solar.
```

Si aparece un error sistemático relevante:

```text
Se documenta el patrón y se abre un ADR antes de rediseñar el modelo.
```

Si el error pudiera afectar el cálculo de tithi cerca de un cambio lunar:

```text
Se prioriza el análisis de impacto calendárico antes de cambiar el motor solar.
```

---

## Resultado

Pendiente.

No se deben registrar resultados hasta ejecutar Swiss Ephemeris localmente con
configuración documentada.
