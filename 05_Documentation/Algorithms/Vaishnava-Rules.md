# Vaishnava Rules Specification

Estado: ✅ Frontera cerrada; reglas formalizadas en Knowledge Base

Campaign 36

## Objetivo

Definir la frontera normativa mínima del Motor Vaiṣṇava antes de implementar
reglas de observancia.

Este documento no implementa Ekādaśī. Su propósito es dejar claro qué reglas
deben quedar decididas antes de traducirlas a código.

## Principio De Frontera

El Motor Vaiṣṇava interpreta hechos calendáricos ya calculados.

Consume:

- `CivilDayTithiPresence`;
- amanecer local;
- cambio de tithi dentro del día civil;
- `AstronomicalTithi`.

Nunca calcula:

- posiciones del Sol;
- posiciones de la Luna;
- elongación Sol-Luna;
- tithi astronómico instantáneo;
- amanecer;
- transiciones astronómicas.

Produce:

- clasificación religiosa del día;
- información mínima de observancia.

## Fuente Normativa

La fuente normativa primaria para Pancanga Engine v1.0 fue elegida en
[Vaiṣṇava Normative Authority](Vaishnava-Authority.md).

Autoridad primaria:

```text
Hari-bhakti-vilāsa
```

Fuentes secundarias:

- Caitanya-caritāmṛta como principio doctrinal general;
- comentarios autorizados de la tradición elegida;
- Gaurābda Calendar / GCal como referencia de validación, no como fuente
  normativa primaria.

Regla de trabajo:

```text
Fuente normativa primaria
        ↓
Regla escrita en Vaishnava-Rules.md
        ↓
Implementación Rust
        ↓
Validación contra referencia calendárica
```

No se implementará ninguna regla que no pueda vincularse a una fuente normativa
o a una decisión explícita del proyecto.

## Alcance De v1.0

Pancanga Engine v1.0 implementará únicamente el núcleo mínimo necesario para
clasificar la observancia de Ekādaśī.

Incluido:

- Ekādaśī;
- Viddhā, en la medida necesaria para decidir si una observancia de Ekādaśī es
  válida o debe desplazarse;
- Mahādvādaśī, solo si las reglas mínimas de Ekādaśī lo requieren.

Fuera de alcance:

- festivales;
- Cāturmāsya;
- vratas adicionales;
- excepciones regionales;
- variantes de otras tradiciones;
- reglas de parāṇa detalladas, salvo la señal mínima necesaria para futuras
  campañas.

## Modelo De Entrada

El Motor Vaiṣṇava no recibirá longitudes astronómicas ni fechas astronómicas
crudas.

La entrada mínima será una descripción calendárica del día:

```text
CivilDayTithiPresence
        ↓
tithi al amanecer
        ↓
cambio de tithi dentro del día, si existe
        ↓
tithi posterior, si existe
```

Para reglas como Viddhā o Mahādvādaśī puede ser necesario consultar días
adyacentes o ventanas especiales como aruṇodaya. Si la fuente normativa
primaria exige esos datos, se agregará primero la capacidad calendárica mínima
correspondiente, sin mezclarla con la capa Vaiṣṇava.

## Reglas Mínimas A Congelar En Campaign 38

Campaign 38 cerró estas reglas en la Knowledge Base:

1. Qué condición convierte un día civil en candidato de Ekādaśī.
2. Qué presencia de Daśamī invalida o contamina un Ekādaśī por Viddhā.
3. Qué condición desplaza la observancia al día siguiente.
4. Qué casos mínimos se clasifican como Mahādvādaśī.
5. Qué salida pública tendrá la clasificación del día.

Los cinco puntos están escritos y vinculados a Hari-bhakti-vilāsa en
[KB-VAI-002](../Knowledge-Base/KB-VAI-002-Hari-bhakti-vilasa-Ekadasi-Rules.md).
El Motor Vaiṣṇava permanece sin implementación hasta completar Campaign 39.10 -
Knowledge Base Audit.

## Hipótesis De Pipeline Normativo

La extracción de Hari-bhakti-vilāsa confirmó que las reglas pueden organizarse
como pipeline normativo:

```text
HBV-EK-001
        ↓
Generar candidatos
        ↓
HBV-EK-002
        ↓
Eliminar candidatos Viddhā
        ↓
HBV-EK-003
        ↓
Mover observancia
        ↓
HBV-EK-004
        ↓
Transformar en Mahādvādaśī
        ↓
HBV-EK-005
        ↓
Calcular Parāṇa
```

El primer paso, HBV-EK-001, solo define el conjunto de candidatos. No invalida
candidatos, no desplaza observancia y no calcula parāṇa.

## Salida Esperada

La salida v1.0 será una clasificación mínima:

```text
Normal
Ekādaśī
Mahādvādaśī
Otro
```

La clasificación debe explicar qué regla produjo el resultado.

## Referencia De Validación

La validación de esta capa usará oráculos calendáricos, no astronómicos.

Ejemplos de oráculo calendárico:

- Gaurābda Calendar / GCal;
- pañcāṅgas de referencia de la tradición adoptada;
- casos manuales derivados de la fuente normativa.

Una diferencia contra un oráculo calendárico no debe corregirse tocando el
motor astronómico ni el motor calendárico, salvo que el error se demuestre en
esas capas.

## Criterio De Cierre

Campaign 36 queda cerrada cuando:

- el alcance mínimo del Motor Vaiṣṇava queda definido;
- las fuentes normativas candidatas quedan registradas;
- queda explícito que GCal es referencia de validación, no autoridad normativa
  primaria;
- queda explícito que la implementación de Ekādaśī está protegida por protocolo
  de extracción hasta congelar las cinco reglas mínimas de Campaign 38.

## Estado

Campaign 36: ✅ Cerrada como especificación de frontera normativa.

Campaign 37: ✅ Autoridad normativa primaria elegida.

Campaign 38 está registrada en [Ekādaśī Rules Specification](Ekadasi-Rules.md).
Las reglas concretas quedaron formalizadas en la Knowledge Base y esperan la
auditoría final de Campaign 39.10.
