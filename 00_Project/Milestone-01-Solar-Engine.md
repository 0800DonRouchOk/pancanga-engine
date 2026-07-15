# Milestone 01
# Solar Engine v1.0

Fecha: 11 de julio de 2026

Estado: ✅ Cerrado

---

## Objetivo Alcanzado

Pancanga Engine cerró su primer subsistema astronómico completo: el motor solar.

El Solar Engine v1.0 permite construir la longitud aparente del Sol a partir de
un Julian Date mediante una cadena de algoritmos pequeños, auditables y
separados por fenómeno físico.

Este hito marca el cierre oficial del Solar Pipeline v1.0.

La revisión posterior de interfaz declara además que el Solar Engine v1.0 tiene
API estable.

---

## Algoritmos Implementados

| ID | Algoritmo | Estado |
|---|---|---|
| AST-0001 | Julian Centuries | Implementado |
| AST-0002 | Earth Heliocentric Longitude | Validado contra IMCCE |
| AST-0003 | Geocentric Solar Longitude | Validado por identidad geométrica |
| AST-0004 | Earth Radius Vector | Validado contra IMCCE |
| AST-0005 | Solar Aberration | Validado por fórmula de Meeus |
| AST-0006 | Nutation in Longitude | Validado por serie truncada de Meeus |
| AST-0007 | Apparent Solar Longitude | Validado por integración del pipeline |

---

## Cadena Cerrada

```text
Julian Date
      ↓
Earth Heliocentric Longitude
      ↓
Geocentric Solar Longitude
      ↓
Earth Radius Vector
      ↓
Solar Aberration
      ↓
Nutation in Longitude
      ↓
Apparent Solar Longitude
```

---

## Decisiones Congeladas

- `ADR-0004`: adoptar `VSOP87D` para la cadena solar.
- `ADR-0005`: usar el modelo de aberración solar de Meeus.
- `ADR-0006`: usar la serie truncada de Meeus para nutación en longitud en
  Pancanga Engine v1.0.
- `Solar Pipeline v1.0`: cerrado y congelado.
- `Solar Engine v1.0 API`: estable.

Cualquier cambio posterior en la cadena solar deberá pasar por un ADR, salvo
correcciones tipográficas o errores científicos objetivos.

---

## Precisión Conseguida

Validaciones registradas:

- `AST-0002`: longitud heliocéntrica de la Tierra validada contra `vsop87.chk`.
- `AST-0004`: radio vector de la Tierra validado contra `vsop87.chk`.
- `AST-0005`: aberración validada contra la fórmula de Meeus.
- `AST-0006`: nutación en longitud validada contra la serie truncada elegida.
- `AST-0007`: integración validada por identidad algebraica.

Errores observados ya documentados:

- `AST-0002`: error máximo observado aproximado de `3.97e-11 rad`.
- `AST-0004`: error máximo observado de `4.745226434010874073e-11 AU`.

La precisión del Solar Engine v1.0 es suficiente para avanzar hacia el cálculo
del tithi, donde la magnitud crítica será la diferencia entre la longitud lunar
y la longitud solar aparente.

---

## Bibliografía Utilizada

- Jean Meeus, *Astronomical Algorithms*, 2nd Edition.
- Bretagnon, P.; Francou, G. "Planetary theories in rectangular and spherical
  variables: VSOP87 solution", *Astronomy and Astrophysics*, 202, 309, 1988.
- IMCCE / Observatoire de Paris, `VSOP87D.ear`.
- IMCCE / Observatoire de Paris, `vsop87.chk`.

---

## Significado del Hito

Este milestone no representa solamente la finalización de una lista de
algoritmos.

Representa el cierre del primer subsistema científico de Pancanga Engine.

A partir de este punto, el proyecto queda listo para iniciar el modelo lunar
sobre una base solar estable, trazable y auditada.
