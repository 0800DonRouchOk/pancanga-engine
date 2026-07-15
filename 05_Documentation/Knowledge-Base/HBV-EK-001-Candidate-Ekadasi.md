# HBV-EK-001 - Candidate Ekādaśī

Campaign: 39.5

Estado: 🟢 Implementable

Código: NO

## Pregunta

```text
¿Qué condición hace que un día entre como candidato inicial a observancia de
Ekādaśī?
```

Esta regla no decide pureza, Viddhā, desplazamiento, Mahādvādaśī ni Parāṇa.

## Fuente

Fuente primaria usada:

```text
Hari-bhakti-vilāsa
Vilāsa 12 - ekādaśī-nirṇayaḥ
Gaudiya Grantha Mandira
Mūla y Digdarśinī-ṭīkā
Edición Haridāsa Śāstrī
```

Archivo local de trabajo:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-12.docx
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-12.txt
```

Nota: la traducción íntegra de Bhṛgumuni Dāsa queda como numeración oficial de
referencia del proyecto. El archivo descargado desde Gaudiya Grantha Mandira
contiene mūla y ṭīkā en sánscrito; las traducciones de esta ficha son
traducciones técnicas de trabajo y deben cotejarse con Bhṛgumuni Dāsa si se
incorpora una cita traducida extensa.

## Evidencia

### Evidencia 1 - Ekādaśī Como Tithi

Referencia:

```text
HBV 12.1
```

Texto:

```text
namo bhagavate tasmai yasya priyatamā tithiḥ |
ekādaśī dvādaśī ca sarvābhīṣṭa-pradā nṛṇām ||
```

Digdarśinī:

```text
yasya bhagavataḥ priyatamā parama-vallabhā ekādaśī dvādaśī ca tithir eva ...
```

Traducción técnica:

Ekādaśī y Dvādaśī son tratadas explícitamente como tithis queridas por
Bhagavān.

Tipo de evidencia:

```text
Definición
```

Relevancia algorítmica:

La regla candidata opera sobre tithi, no sobre nakṣatra, festivales ni otra
magnitud.

### Evidencia 2 - Día De Hari / Día De Ayuno

Referencia:

```text
HBV 12.2
```

Texto:

```text
harer dine viśeṣeṇa kuryāt taṁ pakṣayor dvayoḥ ||
```

Digdarśinī:

```text
harer dinam ekādaśī dvādaśī cety upavāsa-dinaṁ lakṣyate ...
```

Traducción técnica:

El "día de Hari" se identifica, en la explicación, con Ekādaśī y Dvādaśī como
días de ayuno.

Tipo de evidencia:

```text
Obligación / aclaración del comentario
```

Relevancia algorítmica:

El candidato inicial se genera desde el día calendárico asociado al tithi
Ekādaśī. La decisión posterior sobre si el ayuno se observa ese mismo día o se
desplaza no pertenece a HBV-EK-001.

### Evidencia 3 - Ambas Quincenas

Referencia:

```text
HBV 12.40
```

Texto:

```text
ekādaśyām upavaset pakṣayor ubhayor api |
dvādaśyām arcayed viṣṇuṁ ...
```

Traducción técnica:

El ayuno de Ekādaśī aplica en ambas quincenas.

Tipo de evidencia:

```text
Definición de alcance
```

Relevancia algorítmica:

El candidato se genera tanto para Śukla Ekādaśī como para Kṛṣṇa Ekādaśī.

### Evidencia 4 - Completa Y Viddhā Son Clasificaciones Posteriores

Referencia:

```text
HBV 12.199
```

Texto:

```text
ekādaśī ca sampūrṇā viddheti dvividhā smṛtā |
```

Digdarśinī:

```text
pūrvayā daśamyā viddhā tyājyā ...
```

Traducción técnica:

Ekādaśī se recuerda como de dos clases: completa y viddhā. La explicación
indica que la viddhā por Daśamī anterior debe rechazarse.

Tipo de evidencia:

```text
Delimitación de responsabilidades
```

Relevancia algorítmica:

HBV-EK-001 puede generar un candidato antes de decidir si es completo o viddhā.
La pureza y el rechazo por Daśamī pertenecen a HBV-EK-002.

### Evidencia 5 - Día Calendárico Y Amanecer

Referencia:

```text
HBV 12.315
```

Texto:

```text
pratipat-prabhṛtayaḥ sarvā udayād udayād raveḥ |
sampūrṇā iti vikhyātā hari-vāsara-varjitāḥ ||
```

Digdarśinī:

```text
raver udayāt ekam udayam ārabhya ā-udayād anyodayāvadhi yadi syus tadā
sampūrṇā ity arthaḥ | harivāsara ekādaśī tad-varjitāḥ ...
```

Traducción técnica:

Los tithis ordinarios se consideran completos de un amanecer al siguiente. La
ṭīkā aclara que Hari-vāsara, es decir Ekādaśī, tiene una consideración
especial para determinar su completitud.

Tipo de evidencia:

```text
Aclaración del comentario
```

Relevancia algorítmica:

El amanecer es el punto calendárico básico para asociar un tithi a un día civil.
La regla especial de completitud de Ekādaśī pertenece a la evaluación de pureza
o Viddhā, no a la generación inicial del candidato.

## Regla Normativa

```text
Un día civil entra como candidato inicial a observancia de Ekādaśī cuando el
tithi presente en su amanecer local es Ekādaśī.
```

Esta regla produce candidatos. No decide si el candidato es puro, viddhā,
desplazado o Mahādvādaśī.

## Interpretación Técnica

HBV-EK-001 es una regla de entrada al conjunto de candidatos:

```text
CivilDayTithiPresence
        ↓
tithi al amanecer
        ↓
Ekādaśī?
        ↓
Candidate / NotCandidate
```

Si el día es posteriormente rechazado por Daśamī en aruṇodaya, esa decisión se
aplica en HBV-EK-002. Por lo tanto, un día puede ser candidato en HBV-EK-001 y
ser eliminado después.

## Entradas

- `CivilDayTithiPresence`
- `AstronomicalTithi` presente en el amanecer local

## Salida

```text
Candidate
NotCandidate
```

## No Consume

- Aruṇodaya
- Nakṣatra
- Hari-vāsara
- Parāṇa
- clasificación Mahādvādaśī
- GCal
- fuentes secundarias

## Casos De Prueba Normativos

### Caso A

```text
Tithi al amanecer:
Ekādaśī

Resultado:
Candidate
```

### Caso B

```text
Tithi al amanecer:
Otro tithi

Resultado:
NotCandidate
```

### Caso C

```text
Tithi al amanecer:
Ekādaśī

Condición posterior:
El candidato podría resultar viddhā.

Resultado HBV-EK-001:
Candidate

Resultado posterior:
Será evaluado por HBV-EK-002.
```

## Exclusiones

HBV-EK-001 no desarrolla:

- Daśamī-viddhā;
- umbral de 96 minutos;
- aruṇodaya;
- desplazamiento a Dvādaśī;
- Mahādvādaśī;
- Parāṇa;
- festivales.

## Estado De Evidencia

| Nivel | Estado |
|---|---|
| E1 - Fuente primaria verificada | ✅ |
| E2 - Comentario tradicional confirmado | ✅ |
| E3 - Interpretación técnica cerrada | ✅ |
| E4 - Caso de prueba definido | ✅ |
| E5 - Lista para implementación | ✅ |

## Cierre

```text
Campaign 39.5

✅ PASS

Versos primarios:
Verificados

Digdarśinī:
Revisada donde corresponde

Regla normativa:
Cerrada

Entradas/salida:
Definidas

Casos de prueba:
Definidos

HBV-EK-001:
🟢 Implementable

Código:
NO
```
