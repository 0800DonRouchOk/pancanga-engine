# Solar Engine API Review

Fecha: 11 de julio de 2026

Estado: ✅ Aprobada

---

## Objetivo

Revisar si el Solar Engine v1.0 entrega una interfaz pública adecuada para ser
usada por el futuro modelo lunar y, más adelante, por el cálculo del tithi.

Esta revisión no reabre la matemática solar. Solo evalúa la API.

---

## Pregunta Principal

```text
¿El motor solar realmente entrega lo que la teoría lunar va a necesitar?
```

Respuesta:

```text
Sí, con una fachada pública estable.
```

---

## Decisión de API

La API estable del Solar Engine v1.0 será:

```rust
pub fn apparent_longitude(jd: JulianDate) -> Degrees;
```

ubicada en:

```text
pancanga_engine::astronomy::solar
```

Esta función devuelve la longitud eclíptica aparente del Sol normalizada al
intervalo:

```text
[0°, 360°)
```

---

## Motivo

El modelo lunar y el cálculo del tithi no deben depender de los detalles
internos del pipeline solar.

Necesitan una magnitud física:

```text
λ☉ aparente
```

No necesitan saber cómo se compone internamente:

- longitud heliocéntrica de la Tierra;
- longitud geocéntrica del Sol;
- radio vector;
- aberración;
- nutación.

---

## Resultado de la Revisión

| Pregunta | Resultado |
|---|---|
| ¿Las unidades son correctas? | Sí. La salida pública usa `Degrees`. |
| ¿La normalización es clara? | Sí. La salida está en `[0°, 360°)`. |
| ¿La entrada es estable? | Sí. La entrada es `JulianDate`. |
| ¿Los tipos fuertes son suficientes? | Sí para v1.0: `JulianDate`, `Degrees`, `AstronomicalUnits`. |
| ¿El futuro motor lunar puede consumir esta API? | Sí. |
| ¿Debe depender de `astronomy::algorithms`? | No. Debe depender de `astronomy::solar`. |

---

## Límite Público

Los algoritmos catalogados siguen disponibles para auditoría, documentación y
pruebas:

```text
pancanga_engine::astronomy::algorithms
```

Pero los consumidores internos de alto nivel deberán usar:

```text
pancanga_engine::astronomy::solar
```

---

## Estado

Solar Engine v1.0 queda declarado como:

```text
API estable.
```

Si aparece un error, se corrige.

Si se propone rediseñar la interfaz pública, deberá pasar por un ADR.
