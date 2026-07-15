# KB-VAI-002 - Hari-bhakti-vilāsa - Ekādaśī Rules

Estado: 🔒 v1.0 - Knowledge Base congelada

## 1. Alcance

Este documento sintetiza la estructura normativa que Pancanga Engine utilizará
para especificar las reglas de Ekādaśī según Hari-bhakti-vilāsa.

No implementa reglas.

Deja Campaign 40 lista para implementación.

Su función es organizar la información normativa disponible y convertirla en
una especificación técnica verificable.

## 1.1 División De Alcance

```text
Parte A
Determinación del día
Implementa Pancanga Engine
```

Incluye:

- candidato de Ekādaśī;
- pureza / Viddhā;
- desplazamiento de observancia;
- clasificación de Mahādvādaśī cuando afecte el día observado;
- información mínima necesaria para Parāṇa si la v1.0 lo requiere.

```text
Parte B
Procedimiento del vrata
Fuera del alcance del motor
```

Incluye detalles rituales, prácticas devocionales y modo de observancia que no
modifican la clasificación calendárica del día.

## 2. Fuentes

### Fuente primaria

```text
Hari-bhakti-vilāsa
Capítulos 12-13
```

Estado:

```text
Fuente primaria disponible.
Comentario primario disponible.
Traducción íntegra con numeración de versos disponible.
Versos exactos extraídos para HBV-EK-001 a HBV-EK-005.
```

Fuente primaria localizada:

- Gaudiya Grantha Mandira, `Hari Bhakti Vilasa :: Vilasa 12`
  (`ekādaśī-nirṇayaḥ`), con mūla y ṭīkā.
- Gaudiya Grantha Mandira, índice completo de `Hari-bhakti-vilasa`, que lista
  `Vilasa 13` (`viṣṇu-vratotsavaḥ`).
- Traducción íntegra de Bhṛgumuni Dāsa con numeración normalizada según la
  edición Haridāsa Śāstrī.

Nota de normalización:

[HBV Citation Normalization](HBV-Citation-Normalization.md)

### División Vilāsa 12 / Vilāsa 13

```text
Vilāsa 12
        ↓
Qué día corresponde observar

Vilāsa 13
        ↓
Cómo debe observarse
```

Para Pancanga Engine v1.0, Vilāsa 12 es la fuente principal de la
determinación calendárica del día. Vilāsa 13 se consulta para Parāṇa y para el
procedimiento de observancia cuando afecte una salida mínima del motor.

### Fuentes citadas por Hari-bhakti-vilāsa

Los Purāṇas citados por Hari-bhakti-vilāsa pueden respaldar la autoridad de las
reglas, pero no serán usados directamente como fuente del algoritmo mientras
Pancanga Engine haya elegido Hari-bhakti-vilāsa como autoridad normativa
principal.

Uso:

- respaldo tradicional;
- contexto;
- resolución de dudas cuando Hari-bhakti-vilāsa cite explícitamente una regla.

No uso:

- introducir reglas no presentes en la especificación HBV;
- reemplazar la cita exacta de Hari-bhakti-vilāsa.

### Fuentes secundarias

La fuente secundaria referida `Ramanuja Das / masterhindu.com` puede orientar la
estructura práctica de las reglas.

Estado:

```text
Secundaria.
Provisional.
No normativa.
```

### Oráculo de validación

```text
Gaurābda Calendar / GCal
```

GCal será usado como oráculo calendárico de regresión, no como fuente
normativa.

## 3. Reglas Normativas

Las siguientes reglas quedan organizadas para especificación técnica. Su estado
no es "congelado" hasta verificar los versos exactos.

| ID | Regla | Estado |
|---|---|---|
| HBV-EK-001 | Candidato de Ekādaśī | 🟢 Congelada / Implementable |
| HBV-EK-002 | Ekādaśī viddhā | 🟢 Congelada / Implementable |
| HBV-EK-003 | Desplazamiento de observancia | 🟢 Congelada / Implementable |
| HBV-EK-004 | Mahādvādaśī | 🟢 Congelada / Implementable |
| HBV-EK-005 | Parāṇa / Hari-vāsara | 🟢 Congelada / Implementable |

## 4. Definiciones Calendáricas Confirmadas

### Aruṇodaya

Estado: 🟢 Respaldado por mūla y Digdarśinī.

Referencias principales:

- HBV 12.315
- HBV 12.316
- HBV 12.317-318
- HBV 12.325

Uso:

- necesario para HBV-EK-002;
- no usado por HBV-EK-001;
- pertenece a la evaluación de pureza / Viddhā, no a la generación inicial de
  candidatos.

Nota:

La definición operativa queda cerrada en HBV-EK-002 como la ventana de 96
minutos anterior al amanecer local, sin mezclarla con desplazamiento ni
Mahādvādaśī.

### HBV-EK-001 - Candidato De Ekādaśī

Pregunta normativa:

```text
¿Qué condición convierte un día civil en candidato de Ekādaśī?
```

Ficha normativa:

[HBV-EK-001 - Candidate Ekādaśī](HBV-EK-001-Candidate-Ekadasi.md)

Versos principales:

- HBV 12.1
- HBV 12.2
- HBV 12.40
- HBV 12.199
- HBV 12.315

Interpretación técnica:

- evaluar la presencia de Ekādaśī en el día civil definido por amanecer local;
- distinguir presencia al amanecer de presencia durante el intervalo entre
  amaneceres;
- no consultar todavía pureza, Viddhā, Mahādvādaśī ni Parāṇa.

Datos necesarios:

- `CivilDayTithiPresence`;
- tithi al amanecer;
- `AstronomicalTithi` presente en el amanecer local.

No necesita:

- nakṣatra;
- Mahādvādaśī;
- Hari-vāsara;
- parāṇa;
- reglas de festivales;
- clasificación final de observancia.

Casos de prueba previstos:

- Ekādaśī presente en el amanecer -> candidato.
- Ekādaśī ausente en el amanecer -> no candidato.
- Ekādaśī aparece como candidato pero luego podría descartarse por Viddhā ->
  sigue siendo candidato en HBV-EK-001.

Regla normativa cerrada:

```text
Un día civil entra como candidato inicial a observancia de Ekādaśī cuando el
tithi presente en su amanecer local es Ekādaśī.
```

Estado: 🟢 Implementable; código NO.

### HBV-EK-002 - Ekādaśī Viddhā

Pregunta normativa:

```text
¿Qué presencia de Daśamī contamina Ekādaśī?
```

Ficha normativa:

[HBV-EK-002 - Ekādaśī Viddhā](HBV-EK-002-Viddha.md)

Versos principales:

- HBV 12.199
- HBV 12.315
- HBV 12.316
- HBV 12.317-318
- HBV 12.319-320
- HBV 12.342-343

Interpretación técnica:

- Ekādaśī puede ser `sampūrṇā` o `viddhā`;
- la viddhā relevante para esta regla es la contaminación por Daśamī anterior;
- la condición se evalúa en aruṇodaya, definido como los 96 minutos anteriores
  al amanecer local;
- si Daśamī está presente dentro de esa ventana, el candidato queda inválido
  por Viddhā.

Datos necesarios:

- amanecer local;
- aruṇodaya;
- tithi en aruṇodaya;
- transición Daśamī -> Ekādaśī;
- presencia de Daśamī antes del amanecer.

No necesita:

- nakṣatra;
- desplazamiento de observancia;
- Mahādvādaśī;
- Hari-vāsara para Parāṇa;
- Parāṇa;
- reglas de festivales.

Regla normativa cerrada:

```text
Un candidato de Ekādaśī queda invalidado como Viddhā cuando Daśamī está
presente durante aruṇodaya, es decir, durante los 96 minutos anteriores al
amanecer local del día candidato.
```

Forma normativa:

```text
IF Daśamī está presente durante aruṇodaya
THEN Candidate = InvalidViddha
ELSE Candidate = ValidForNextRule
```

Implementación equivalente posible:

```text
IF ekadasi_start > sunrise - 96 minutes
THEN Candidate = InvalidViddha
ELSE Candidate = ValidForNextRule
```

Estado: 🟢 Implementable; código NO.

### HBV-EK-003 - Desplazamiento De Observancia

Pregunta normativa:

```text
Si Ekādaśī queda viddhā, ¿cuándo se desplaza la observancia?
```

Ficha normativa:

[HBV-EK-003 - Desplazamiento De Observancia](HBV-EK-003-Observance-Displacement.md)

Versos principales:

- HBV 12.317-318
- HBV 12.319
- HBV 12.320
- HBV 12.324

Interpretación técnica:

- HBV-EK-003 no define Viddhā; consume el resultado de HBV-EK-002;
- si el candidato queda invalidado por Viddhā, la observancia se desplaza a
  Dvādaśī;
- si el candidato no queda invalidado, la observancia permanece en el día
  candidato;
- la referencia textual a Parāṇa en Trayodaśī se reserva para HBV-EK-005.

Datos necesarios:

- día candidato producido por HBV-EK-001;
- resultado de HBV-EK-002.

No necesita:

- nakṣatra;
- Mahādvādaśī;
- Hari-vāsara para Parāṇa;
- Parāṇa;
- festivales;
- procedimiento ritual de observancia.

Regla normativa cerrada:

```text
Cuando un candidato de Ekādaśī queda invalidado como Viddhā por HBV-EK-002, la
observancia se desplaza a Dvādaśī.
```

Forma lógica:

```text
IF Candidate == InvalidViddha
THEN ObservanceDay = Dvādaśī
ELSE ObservanceDay = CandidateDay
```

Estado: 🟢 Implementable; código NO.

### HBV-EK-004 - Mahādvādaśī

Ficha normativa:

[HBV-EK-004 - Ocho Mahādvādaśī](HBV-EK-004-Mahadvadasi.md)

Decisión de alcance:

No se implementará un "Mahādvādaśī mínimo" genérico. La especificación v1.0
enumera los ocho Mahādvādaśī individualmente.

Ocho Mahādvādaśī a especificar:

| ID | Nombre | Estado |
|---|---|---|
| HBV-MD-001 | Unmīlanī Mahādvādaśī | 🟢 Especificada |
| HBV-MD-002 | Vyañjulī Mahādvādaśī | 🟢 Especificada; ISSUE-VAI-001 resuelto |
| HBV-MD-003 | Tri-spṛśā Mahādvādaśī | 🟢 Especificada |
| HBV-MD-004 | Pakṣa-vardhinī Mahādvādaśī | 🟢 Especificada |
| HBV-MD-005 | Jaya Mahādvādaśī | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-006 | Vijaya Mahādvādaśī | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-007 | Jayantī Mahādvādaśī | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-008 | Pāpanāśinī Mahādvādaśī | 🟢 Especificada; requiere Nakṣatra Engine |

Datos necesarios:

- presencia de Ekādaśī y Dvādaśī;
- secuencia de tithis en días consecutivos;
- relación con nakṣatra para Jayā, Vijayā, Jayantī y Pāpanāśinī;
- reglas específicas de cada tipo.

Estado: 🟢 PASS. La tabla normativa queda cerrada. La divergencia de Vyañjulī
quedó resuelta en [ISSUE-VAI-001](ISSUE-VAI-001-Vyanjuli-Divergence.md) a
favor de la formulación: Ekādaśī pura, Dvādaśī presente al amanecer y Dvādaśī
prolongada hacia Trayodaśī. Jayā, Vijayā, Jayantī y Pāpanāśinī conservan
dependencia técnica de Nakṣatra Engine.

### HBV-EK-005 - Parāṇa / Hari-vāsara

Pregunta normativa:

```text
¿Cómo se define la ventana válida para romper el ayuno?
```

[HBV-EK-005 - Parāṇa / Hari-vāsara](HBV-EK-005-Parana-Harivasara.md)

Versos principales:

- HBV 13.229-230
- HBV 13.236
- HBV 13.238-245
- HBV 13.246-256
- HBV 13.257-259

Interpretación técnica:

- Parāṇa completa el vrata después de las obligaciones matutinas.
- Parāṇa debe realizarse dentro de Dvādaśī.
- No debe realizarse durante Hari-vāsara.
- Si Dvādaśī es corta, se adelantan obligaciones y, si es necesario, se permite
  Parāṇa mínimo con agua para evitar transgredir Dvādaśī.

Regla normativa cerrada:

```text
La ventana válida de Parāṇa comienza cuando Hari-vāsara ha terminado y debe
cerrarse antes de que termine Dvādaśī.
```

Estado: 🟢 Implementable; código NO.

Punto crítico:

Esta es una regla matemática y calendárica. HBV-EK-005 ya define
normativamente:

- inicio de Dvādaśī;
- final de Dvādaśī;
- duración total de Dvādaśī;
- primer 25% de esa duración;
- relación de la ventana de parāṇa con el amanecer local.

Datos necesarios:

- instante exacto de inicio de Dvādaśī;
- instante exacto de fin de Dvādaśī;
- sunrise del día de parāṇa;
- límite superior antes de transgredir Dvādaśī.

Estado: 🟢 definido en [HBV-EK-005](HBV-EK-005-Parana-Harivasara.md). La
implementación concreta de la ventana pertenece al Motor Calendárico /
Vaiṣṇava, pero la regla normativa ya está cerrada.

## 4. Interpretación Técnica

La capa Vaiṣṇava no calcula astronomía.

Consume hechos calendáricos:

```text
CivilDayTithiPresence
        ↓
tithi al amanecer
        ↓
transición dentro del día
        ↓
presencia de tithi en días adyacentes
```

Si una regla requiere aruṇodaya, Hari-vāsara o parāṇa, esas magnitudes deben
ser provistas por el Motor Calendárico antes de que el Motor Vaiṣṇava aplique
la regla normativa.

## 5. Datos Que Consume Pancanga Engine

Datos ya disponibles:

- amanecer;
- tithi al amanecer;
- transición de tithi entre amaneceres;
- presencia de tithi durante el día civil.

Datos probablemente necesarios:

- aruṇodaya;
- tithi en aruṇodaya;
- consulta de días adyacentes;
- inicio y fin exactos de Dvādaśī;
- cálculo de Hari-vāsara;
- ventana de parāṇa.

## 6. Casos De Prueba

Los casos de prueba deberán construirse después de verificar los versos exactos.

Casos mínimos esperados:

1. Ekādaśī puro.
2. Ekādaśī viddhā por Daśamī.
3. Observancia desplazada a Dvādaśī.
4. Cada uno de los ocho Mahādvādaśī.
5. Parāṇa después de Hari-vāsara.
6. Caso donde GCal difiere de la interpretación normativa y se requiere
   análisis.

Cada caso deberá registrar:

- fecha civil;
- ubicación;
- amanecer;
- aruṇodaya, si aplica;
- tithi al amanecer;
- transiciones de tithi;
- clasificación esperada;
- fuente normativa.

## 7. Diagrama De Decisión

```text
CivilDayTithiPresence
        │
        ▼
¿Ekādaśī presente?
        │
   ┌────┴────┐
   │         │
  NO        SÍ
   │         │
   ▼         ▼
Normal   ¿Viddhā?
             │
      ┌──────┴──────┐
      │             │
     SÍ            NO
      │             │
      ▼             ▼
Desplazar      ¿Mahādvādaśī?
a Dvādaśī          │
              ┌────┴────┐
              │         │
             SÍ        NO
              │         │
              ▼         ▼
      Clasificar   Ekādaśī normal
```

Este diagrama resume la composición normativa extraída. La auditoría final de
Knowledge Base debe verificar que ninguna regla requerida por el Motor
Vaiṣṇava haya quedado implícita fuera de las fichas HBV-EK.

## 8. Pendientes / Cuestiones Abiertas

- Iniciar Campaign 40 - Vaiṣṇava Engine Implementation.
- [ISSUE-VAI-001](ISSUE-VAI-001-Vyanjuli-Divergence.md) resuelto. Vyañjulī no
  mantiene divergencia doctrinal abierta.
- Preparar oráculos manuales derivados de la fuente primaria antes de validar
  la implementación del Motor Vaiṣṇava.

## Estado Final Del Documento

KB-VAI-002 contiene las cinco reglas HBV-EK cerradas para implementación.
Campaign 39.10 confirmó que no quedan reglas normativas implícitas fuera de la
Knowledge Base.

```text
Especificación vaiṣṇava: v1.0 READY
Implementación vaiṣṇava: 0%
Campaign 40: desbloqueada para implementación
```
