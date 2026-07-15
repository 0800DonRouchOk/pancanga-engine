# Milestone 03
# Astronomical Engine v1.0

Fecha: 14 de julio de 2026

Estado: ✅ Cerrado

---

## Objetivo Alcanzado

Pancanga Engine cerró el núcleo astronómico necesario para calcular el tithi
instantáneo.

Este hito no incluye calendario civil, amanecer, Ekādaśī, Mahādvādaśī ni reglas
vaiṣṇavas. Cierra únicamente la capa astronómica que produce las magnitudes
base.

---

## Capacidades Cerradas

```text
✅ astronomy::solar::apparent_longitude(jd)

✅ astronomy::moon::apparent_longitude(jd)

✅ astronomy::lunar_solar_elongation(moon_longitude, sun_longitude)

✅ astronomy::tithi_index(elongation)

✅ astronomy::astronomical_tithi(elongation)
```

---

## Cadena Astronómica Cerrada

```text
Julian Date
      ↓
Longitud aparente del Sol
      ↓
Longitud aparente de la Luna
      ↓
Elongación Sol-Luna
      ↓
Índice astronómico de tithi
      ↓
Tithi astronómico instantáneo
```

---

## Límites Del Hito

No implementado en este milestone:

- amanecer;
- ocaso;
- presencia del tithi al amanecer;
- cambios de tithi dentro del día civil;
- Ekādaśī;
- Mahādvādaśī;
- parāṇa;
- festivales.

Estas responsabilidades pertenecen al motor calendárico y al motor vaiṣṇava.

---

## Significado Del Hito

Este milestone marca el cierre de la etapa astronómica necesaria para iniciar
el motor calendárico.

A partir de este punto, el proyecto deja de construir posiciones astronómicas
base para tithi y comienza a usar esas magnitudes en eventos civiles como el
amanecer.
