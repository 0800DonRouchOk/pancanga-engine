# Calendar Pipeline v1.0

Estado: 🔒 Congelado

## Objetivo

Definir la frontera mínima entre el motor astronómico y el motor calendárico.

El motor calendárico no implementa algoritmos astronómicos. Consume únicamente
la API pública del motor astronómico.

## Pipeline

```text
Coordenadas geográficas
        ↓
Amanecer local
        ↓
JulianDate del amanecer
        ↓
astronomy::solar::apparent_longitude()
        ↓
astronomy::moon::apparent_longitude()
        ↓
Lunar-Solar Elongation
        ↓
AstronomicalTithi
        ↓
Cambio de tithi dentro del día
        ↓
Presencia calendárica del tithi
```

## Entrada Mínima

- coordenadas geográficas;
- fecha civil local;
- zona horaria o regla equivalente para convertir entre hora local y tiempo
  astronómico;
- API pública del motor astronómico.

## Salida Mínima

- instante de amanecer local;
- `JulianDate` correspondiente al amanecer;
- tithi astronómico presente en el amanecer;
- cambios de tithi dentro del día civil;
- presencia calendárica del tithi durante el día civil.

## Límites

Queda explícitamente fuera:

- Ekādaśī;
- Mahādvādaśī;
- parāṇa;
- festivales;
- reglas vaiṣṇavas.

## Regla De Frontera

El motor calendárico responde preguntas de presencia temporal civil.

El motor astronómico responde preguntas de posición y magnitudes instantáneas.

El motor vaiṣṇava interpretará más adelante los resultados calendáricos según
reglas tradicionales.
