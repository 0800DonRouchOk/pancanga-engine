# HBV-EK-004 - Ocho Mahādvādaśī

Campaign: 39.8

Estado: 🟢 PASS.

Código: NO

## Pregunta

```text
¿Cómo clasifica el Hari-bhakti-vilāsa una observancia válida como uno de los
ocho Mahādvādaśī?
```

Esta regla clasifica una observancia. No define candidato, Viddhā ni
desplazamiento, y no calcula Parāṇa.

## Fuente

Fuentes locales usadas:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-12.txt
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.doc
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt
```

Nota:

- `hbv-vilasa-12.txt` conserva mūla y Digdarśinī.
- `hbv-vilasa-13.doc` es la fuente local completa de Bhṛgumuni Dāsa incorporada
  al repositorio.
- `hbv-vilasa-13.txt` es una copia de trabajo generada localmente desde el
  documento Word para extracción textual.

No se usó Internet para definir reglas.

## Conocimiento Ya Confirmado

- HBV-EK-001 genera candidatos.
- HBV-EK-002 invalida candidatos Viddhā.
- HBV-EK-003 desplaza la observancia a Dvādaśī cuando el candidato queda
  Viddhā.
- HBV-EK-004 clasifica observancias válidas como Mahādvādaśī cuando corresponde.

## Evidencia

### Evidencia 1 - Enumeración De Los Ocho Mahādvādaśī

Referencia:

```text
HBV 12.391-392
HBV 13.265-266
```

Texto clave:

```text
unmīlanī vañjulī ca trispṛśā pakṣa-vardhinī
jayā ca vijayā caiva jayantī pāpa-nāśinī
```

Traducción técnica:

Hari-bhakti-vilāsa enumera ocho Mahādvādaśī: cuatro basados en tithi y cuatro
basados en nakṣatra.

Tipo de evidencia:

```text
Definición / clasificación
```

Relevancia algorítmica:

La implementación no debe tratar Mahādvādaśī como una categoría genérica. Debe
clasificar entre ocho casos explícitos.

### Evidencia 2 - División Tithi / Nakṣatra

Referencia:

```text
HBV 12.391-392
HBV 13.265-266
```

Texto clave:

```text
The first four are connected with a particular tithi and the others with a
particular star.
```

Digdarśinī:

```text
tā mahā-dvādaśyaḥ
```

Traducción técnica:

Los cuatro primeros Mahādvādaśī dependen de configuración de tithi. Los cuatro
últimos dependen de unión con nakṣatra.

Tipo de evidencia:

```text
Clasificación estructural
```

Relevancia algorítmica:

El motor deberá consumir dos clases de datos:

- secuencia de tithis alrededor de Ekādaśī/Dvādaśī;
- nakṣatra asociado a Dvādaśī para los cuatro casos finales.

### Evidencia 3 - Mandatoriedad

Referencia:

```text
HBV 13.282-285
```

Digdarśinī:

```text
The quartet of Jayā and so forth is also shown to be mandatory ... They are
indeed mandatory.
```

Traducción técnica:

Los ocho Mahādvādaśī se tratan como observancias obligatorias dentro de la
determinación Vaiṣṇava.

Tipo de evidencia:

```text
Alcance normativo
```

Relevancia algorítmica:

Pancanga Engine v1.0 no debe implementar solo un Mahādvādaśī mínimo si el
alcance elegido es Hari-bhakti-vilāsa.

## Tabla Normativa

| ID | Mahādvādaśī | Grupo | Condición HBV | Datos requeridos | Estado |
|---|---|---|---|---|---|
| HBV-MD-001 | Unmīlanī | Tithi | Ekādaśī pura continúa más allá del amanecer siguiente; el segundo día se observa como Unmīlanī. | tithi al amanecer; inicio/presencia de Ekādaśī; día siguiente | 🟢 Especificada |
| HBV-MD-002 | Vyañjulī / Vañjulī | Tithi | Ekādaśī es pura; Dvādaśī está presente al amanecer y continúa hacia Trayodaśī. | pureza de Ekādaśī; tithi al amanecer; presencia de Dvādaśī; continuidad de Dvādaśī hacia el día siguiente | 🟢 Especificada; ISSUE-VAI-001 resuelto |
| HBV-MD-003 | Tri-spṛśā | Tithi | Ekādaśī, Dvādaśī y Trayodaśī se conectan en la misma secuencia válida; no debe involucrar Daśamī. | secuencia Ekādaśī/Dvādaśī/Trayodaśī; ausencia de Daśamī; día de observancia | 🟢 Especificada |
| HBV-MD-004 | Pakṣa-vardhinī | Tithi | Amāvasyā o Pūrṇimā completas crecen hacia el día siguiente; la Dvādaśī precedente se clasifica como Pakṣa-vardhinī. | identificación de pakṣa final; Amāvasyā/Pūrṇimā; Dvādaśī precedente | 🟢 Especificada |
| HBV-MD-005 | Jayā | Nakṣatra | Dvādaśī de śukla-pakṣa combinada con Punarvasu. | Dvādaśī; śukla-pakṣa; nakṣatra Punarvasu | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-006 | Vijayā | Nakṣatra | Dvādaśī de śukla-pakṣa combinada con Śravaṇa. | Dvādaśī; śukla-pakṣa; nakṣatra Śravaṇa | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-007 | Jayantī | Nakṣatra | Dvādaśī de śukla-pakṣa combinada con Rohiṇī. | Dvādaśī; śukla-pakṣa; nakṣatra Rohiṇī | 🟢 Especificada; requiere Nakṣatra Engine |
| HBV-MD-008 | Pāpanāśinī | Nakṣatra | Dvādaśī de śukla-pakṣa combinada con Puṣya. | Dvādaśī; śukla-pakṣa; nakṣatra Puṣya | 🟢 Especificada; requiere Nakṣatra Engine |

## Detalles Operativos Extraídos

### Reglas Basadas En Tithi

#### Unmīlanī

Referencia:

```text
HBV 13.321-329
```

Condición técnica:

```text
Ekādaśī pura está presente en el primer amanecer y vuelve a estar presente en
el amanecer siguiente.
```

Salida:

```text
Mahādvādaśī = Unmīlanī
ObservanceDay = second day
```

#### Vyañjulī / Vañjulī

Referencia:

```text
HBV 13.356-358
HBV 12.372-376
```

Condición técnica HBV:

```text
Ekādaśī es pura.
Dvādaśī está presente al amanecer.
Dvādaśī continúa hacia Trayodaśī.
```

Normalización:

```text
La pureza de Ekādaśī sí depende de Aruṇodaya según HBV-EK-002.
La presencia de Dvādaśī para Vyañjulī no hereda el umbral de dos muhūrtas.
```

Estado:

```text
ISSUE-VAI-001 queda resuelto a favor de la formulación:
Ekādaśī pura + Dvādaśī presente al amanecer + Dvādaśī prolongada hacia
Trayodaśī.
```

Nota de traducción:

```text
La frase traducida como "eat on Ekādaśī" en HBV 13.356-358 se trata como
inconsistencia de traducción y no se usa para la especificación normativa.
```

#### Tri-spṛśā

Referencias:

```text
HBV 12.377-388
HBV 13.400-446
```

Condición técnica:

```text
Ekādaśī, Dvādaśī y Trayodaśī se tocan en una misma configuración válida.
Daśamī no debe estar involucrada.
```

Nota:

La fuente distingue la Tri-spṛśā válida de una configuración contaminada por
Daśamī.

#### Pakṣa-vardhinī

Referencias:

```text
HBV 12.389-390
HBV 13.467-472
```

Condición técnica:

```text
Amāvasyā o Pūrṇimā está completa y crece hacia el día siguiente; la Dvādaśī
precedente se clasifica como Pakṣa-vardhinī.
```

### Reglas Basadas En Nakṣatra

Referencia común:

```text
HBV 12.392
HBV 13.485-537
```

Condición común:

```text
Dvādaśī de śukla-pakṣa combinada con el nakṣatra correspondiente.
```

Correspondencia:

| Mahādvādaśī | Nakṣatra |
|---|---|
| Jayā | Punarvasu |
| Vijayā | Śravaṇa |
| Jayantī | Rohiṇī |
| Pāpanāśinī | Puṣya |

Nota operativa:

La extracción local de Bhṛgumuni Dāsa agrega condiciones de combinación entre
Dvādaśī y nakṣatra:

- si el nakṣatra entra después del amanecer en Dvādaśī, el ayuno se toma ese
  día;
- si el nakṣatra entra antes del amanecer, debe durar al menos tanto como el
  tithi para activar la clasificación;
- para ser Mahādvādaśī, Dvādaśī debe durar hasta el ocaso, excepto en el caso
  de Śravaṇa / Vijayā.

Estas condiciones quedan registradas como especificación técnica pendiente de
validación cuando exista `Nakṣatra Engine`.

## Regla Normativa Cerrada

```text
Una observancia válida se clasifica como Mahādvādaśī cuando el día observado es
Dvādaśī y satisface una de las ocho condiciones documentadas por
Hari-bhakti-vilāsa: cuatro basadas en tithi y cuatro basadas en nakṣatra.
```

Salida propuesta:

```text
NormalEkadasi
Mahadvadasi(Unmilani)
Mahadvadasi(Vyanjuli)
Mahadvadasi(Trisprsa)
Mahadvadasi(Pakshavardhini)
Mahadvadasi(Jaya)
Mahadvadasi(Vijaya)
Mahadvadasi(Jayanti)
Mahadvadasi(Papanasini)
```

## Entradas Y Salida

Entradas:

- resultado de HBV-EK-001;
- resultado de HBV-EK-002;
- resultado de HBV-EK-003;
- `CivilDayTithiPresence` para días adyacentes;
- secuencia de tithis alrededor de Ekādaśī/Dvādaśī/Trayodaśī;
- identificación de śukla/kṛṣṇa pakṣa;
- nakṣatra para los casos Jayā, Vijayā, Jayantī y Pāpanāśinī.

Salida:

```text
Normal / Mahādvādaśī(type)
```

No consume:

- Parāṇa;
- Hari-vāsara para romper el ayuno;
- alimentos;
- vigilia;
- procedimiento ritual;
- festivales adicionales.

## Casos De Prueba Normativos

### Caso A - Dvādaśī Sin Condición Especial

```text
ObservanceDay = Dvādaśī
No cumple ninguna de las ocho condiciones.
```

Resultado:

```text
Normal
```

### Caso B - Unmīlanī

```text
Ekādaśī pura continúa en el amanecer siguiente.
```

Resultado:

```text
Mahadvadasi(Unmilani)
```

### Caso C - Vyañjulī

```text
Ekādaśī pura.
Dvādaśī presente al amanecer.
Dvādaśī continúa hacia Trayodaśī.
```

Resultado:

```text
Mahadvadasi(Vyanjuli)
```

Estado:

```text
ISSUE-VAI-001 resuelto.
```

### Caso D - Jayā

```text
Dvādaśī en śukla-pakṣa.
Nakṣatra = Punarvasu.
```

Resultado:

```text
Mahadvadasi(Jaya)
```

## Alcance Cerrado

HBV-EK-004 no resuelve:

- Parāṇa;
- Hari-vāsara;
- horarios de ruptura;
- procedimiento ritual;
- alimentos;
- vigilia;
- festivales adicionales;
- implementación de Nakṣatra Engine.

## Estado De Evidencia

```text
E1 Fuente primaria verificada: ✅
E2 Comentario tradicional confirmado: ✅
E3 Interpretación técnica: ✅ Tabla normativa completa; ISSUE-VAI-001 resuelto
E4 Casos de prueba definidos: ✅
E5 Lista para implementación: ✅ Especificación principal cerrada
```

## Cierre

```text
Campaign 39.8

PASS

Tabla normativa:
Completada

Versos primarios:
Verificados

Digdarśinī:
Revisada donde aparece en la fuente local

Divergencias:
ISSUE-VAI-001 resuelto para Vyañjulī; no quedan issues doctrinales abiertos

Dependencias:
Nakṣatra Engine para MD-005 a MD-008

Código:
NO
```
