# HBV-EK-003 - Desplazamiento De Observancia

Campaign: 39.7

Estado: 🟢 Implementable

Código: NO

## Pregunta

```text
¿Cuál es la transformación normativa que debe aplicarse cuando un candidato
resulta Viddhā?
```

Esta regla no define Viddhā. Consume el resultado de HBV-EK-002 y transforma el
día de observancia.

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

Nota: las traducciones incluidas aquí son traducciones técnicas de trabajo. La
numeración oficial del proyecto sigue la traducción íntegra de Bhṛgumuni Dāsa
basada en la edición Haridāsa Śāstrī.

## Conocimiento Ya Confirmado

- HBV-EK-001 genera candidatos cuando Ekādaśī está presente en el amanecer
  local.
- HBV-EK-002 invalida un candidato como Viddhā cuando Daśamī está presente
  durante aruṇodaya.
- El desplazamiento no forma parte de la definición de Viddhā. Es una
  transformación posterior.

## Evidencia

### Evidencia 1 - Rechazo Del Candidato Viddhā

Referencia:

```text
HBV 12.317-318
```

Texto clave:

```text
daśamyaikādaśī viddhā vaiṣṇavena viśeṣataḥ
```

Digdarśinī:

```text
daśamyā viddhaikādaśī parityājyā sarvair eva
```

Traducción técnica:

La Ekādaśī contaminada por Daśamī debe abandonarse, especialmente por el
vaiṣṇava.

Tipo de evidencia:

```text
Invalidación previa
```

Relevancia algorítmica:

Esta evidencia conecta HBV-EK-002 con HBV-EK-003: el candidato invalidado no se
observa en su propio día.

### Evidencia 2 - Transformación A Dvādaśī

Referencia:

```text
HBV 12.319
```

Texto clave:

```text
aruṇodaya-velāyāṁ daśamī-saṁyutā yadi |
atropoṣyā dvādaśī syāt
```

Traducción técnica:

Si Daśamī está unida a la ventana de aruṇodaya, entonces Dvādaśī debe ser el
día de ayuno.

Tipo de evidencia:

```text
Transformación normativa
```

Relevancia algorítmica:

La salida de HBV-EK-003 no es un nuevo estado de pureza. Es una reasignación
del día de observancia desde el candidato Viddhā hacia Dvādaśī.

### Evidencia 3 - Confirmación De Dvādaśī Como Día De Ayuno

Referencia:

```text
HBV 12.320
```

Texto clave:

```text
aruṇodaya-velāyāṁ daśamī yadi saṅgatā |
... dvādaśīm upavāsayet
```

Digdarśinī:

```text
aruṇodaye tu daśamī gandha-mātraṁ bhaved yadi
```

Traducción técnica:

Si Daśamī se encuentra en aruṇodaya, aunque sea mínimamente, se debe ayunar en
Dvādaśī.

Tipo de evidencia:

```text
Confirmación normativa
```

Relevancia algorítmica:

La transformación no depende de la cantidad de Daśamī presente durante
aruṇodaya. Cualquier presencia de Daśamī activa la regla de desplazamiento.

### Evidencia 4 - Repetición De La Salida Dvādaśī

Referencia:

```text
HBV 12.324
```

Texto clave:

```text
dvādaśyām upavāsayet
```

Traducción técnica:

El bloque de reglas sobre Ekādaśī contaminada por Daśamī concluye reiterando
que el ayuno debe hacerse en Dvādaśī.

Tipo de evidencia:

```text
Confirmación de salida
```

Relevancia algorítmica:

Confirma que la transformación de HBV-EK-003 produce observancia en Dvādaśī.
La frase sobre Parāṇa en Trayodaśī aparece en el mismo entorno textual, pero se
reserva para HBV-EK-005.

## Regla Normativa Cerrada

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

Esta regla no calcula cuál Dvādaśī es Mahādvādaśī ni determina Parāṇa. Solo
transforma el día de observancia.

## Entradas Y Salida

Entradas:

- día candidato producido por HBV-EK-001;
- resultado de HBV-EK-002 (`Valid` o `InvalidViddha`);
- presencia calendárica de Dvādaśī en el día siguiente, cuando la implementación
  necesite materializar el día de observancia.

Salida:

```text
ObserveOnCandidate / ObserveOnDvadasi
```

No consume:

- nakṣatra;
- clasificación Mahādvādaśī;
- Hari-vāsara para Parāṇa;
- Parāṇa;
- festivales;
- procedimiento ritual de observancia;
- GCal;
- fuentes secundarias para definir la regla.

## Casos De Prueba Normativos

### Caso A - Candidato Válido

```text
HBV-EK-001: Candidate
HBV-EK-002: Valid
```

Resultado:

```text
ObserveOnCandidate
```

### Caso B - Candidato Viddhā

```text
HBV-EK-001: Candidate
HBV-EK-002: InvalidViddha
```

Resultado:

```text
ObserveOnDvadasi
```

### Caso C - Candidato Viddhā Con Parāṇa Mencionado En La Fuente

```text
HBV-EK-002: InvalidViddha
Fuente menciona Trayodaśī para Parāṇa.
```

Resultado en HBV-EK-003:

```text
ObserveOnDvadasi
```

La ventana de Parāṇa no se calcula aquí. Pertenece a HBV-EK-005.

## Alcance Cerrado

HBV-EK-003 no resuelve:

- definición de Viddhā;
- Mahādvādaśī;
- nakṣatra;
- Hari-vāsara para Parāṇa;
- Parāṇa;
- festivales;
- ritual de observancia.

## Estado De Evidencia

```text
E1 Fuente primaria verificada: ✅
E2 Comentario tradicional confirmado: ✅
E3 Interpretación técnica cerrada: ✅
E4 Casos de prueba definidos: ✅
E5 Lista para implementación: ✅
```

## Cierre

```text
Campaign 39.7

PASS

Versos primarios:
Verificados

Digdarśinī:
Revisada

Regla normativa:
Cerrada

Entradas/salida:
Definidas

Casos de prueba:
Definidos

HBV-EK-003:
🟢 Implementable

Código:
NO
```
