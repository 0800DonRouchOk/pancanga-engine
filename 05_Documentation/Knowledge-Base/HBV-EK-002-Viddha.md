# HBV-EK-002 - Ekādaśī Viddhā

Campaign: 39.6

Estado: 🟢 Implementable

Código: NO

## Pregunta

```text
¿Cuándo un candidato deja de ser válido por ser Viddhā?
```

Esta regla solo invalida candidatos contaminados por Daśamī. No desplaza la
observancia, no clasifica Mahādvādaśī y no calcula Parāṇa.

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

## Evidencia

### Evidencia 1 - Clasificación Sampūrṇā / Viddhā

Referencia:

```text
HBV 12.199
```

Texto clave:

```text
ekādaśī ca sampūrṇā viddheti dvividhā smṛtā
```

Digdarśinī:

```text
pūrvayā daśamyā viddhā tyājyā
```

Traducción técnica:

Ekādaśī se clasifica en dos estados: completa (`sampūrṇā`) y contaminada
(`viddhā`). La Digdarśinī aclara que la contaminación por la Daśamī anterior
debe rechazarse.

Tipo de evidencia:

```text
Definición / aclaración del comentario
```

Relevancia algorítmica:

HBV-EK-002 consume un candidato de HBV-EK-001 y lo clasifica como válido o
inválido por contaminación de Daśamī.

### Evidencia 2 - Criterio De Completitud Especial Para Ekādaśī

Referencia:

```text
HBV 12.315
```

Texto clave:

```text
sampūrṇā iti vikhyātā hari-vāsara-varjitāḥ
```

Digdarśinī:

```text
udayāt pūrvaṁ muhūrta-dvayaṁ yady asau bhavati,
tadaiva sampūrṇaḥ syāt
```

Traducción técnica:

Los tithis ordinarios se consideran completos de amanecer a amanecer, pero
Hari-vāsara, identificado aquí con Ekādaśī, queda exceptuado. Para que Ekādaśī
sea completa debe estar presente dos muhūrtas antes del amanecer.

Tipo de evidencia:

```text
Aclaración del comentario
```

Relevancia algorítmica:

El criterio de validez no es solamente el tithi presente al amanecer. La regla
debe evaluar el intervalo anterior al amanecer.

### Evidencia 3 - Dos Muhūrtas Antes Del Amanecer

Referencia:

```text
HBV 12.316
```

Texto clave:

```text
udayāt prāk ... muhūrta-dvaya-saṁyutā
```

Traducción técnica:

Para el gṛhastha, se observa la Ekādaśī que está unida a los dos muhūrtas
anteriores al amanecer; esa Ekādaśī recibe el nombre de completa.

Tipo de evidencia:

```text
Definición operativa
```

Relevancia algorítmica:

El motor debe calcular:

```text
aruṇodaya = sunrise - 2 muhūrta
```

Con la convención ya aceptada por el proyecto:

```text
2 muhūrta = 96 minutos
```

### Evidencia 4 - Ekādaśī No Completa Es Viddhā

Referencia:

```text
HBV 12.317-318
```

Texto clave:

```text
ekādaśī tu sampūrṇā viddhānyā parikīrtitā
```

Digdarśinī:

```text
udayāt prāk muhūrta-dvayāntar-daśamī-prāptyā ...
daśamyā viddhaikādaśī parityājyā
```

Traducción técnica:

Ekādaśī es completa si está presente durante los dos muhūrtas anteriores al
amanecer; la otra es llamada viddhā. La Digdarśinī identifica la causa como
presencia de Daśamī dentro de ese intervalo y declara que debe abandonarse.

Tipo de evidencia:

```text
Definición / invalidación
```

Relevancia algorítmica:

La regla lógica se expresa como presencia de Daśamī en la ventana de
aruṇodaya, no como una condición posterior de observancia.

### Evidencia 5 - Daśamī En Aruṇodaya

Referencia:

```text
HBV 12.319-320
```

Texto clave:

```text
aruṇodaya-velāyāṁ daśamī-saṁyutā
```

Traducción técnica:

Si Daśamī se une a la ventana de aruṇodaya, el ayuno no se toma en ese
candidato de Ekādaśī. La consecuencia de observar Dvādaśī pertenece a
HBV-EK-003, no a esta regla.

Tipo de evidencia:

```text
Invalidación
```

Relevancia algorítmica:

HBV-EK-002 solo produce `Valid` o `InvalidViddha`. La acción posterior queda
fuera de alcance.

### Evidencia 6 - Definición De Aruṇodaya

Referencia:

```text
HBV 12.342-343
```

Texto clave:

```text
udayāt prāk catasraś ca ghaṭikā aruṇodayaḥ
```

Digdarśinī:

```text
prātar uṣasi yāś catasro ghaṭikās tāḥ
```

Traducción técnica:

Aruṇodaya se define como las cuatro ghaṭikās anteriores al amanecer.

Tipo de evidencia:

```text
Definición calendárica
```

Relevancia algorítmica:

Cuatro ghaṭikās equivalen a dos muhūrtas. Esta definición confirma la ventana
operativa usada por HBV-EK-002:

```text
aruṇodaya = sunrise - 96 minutes
```

## Regla Normativa Cerrada

```text
Un candidato de Ekādaśī queda invalidado como Viddhā cuando Daśamī está
presente durante aruṇodaya, es decir, durante los 96 minutos anteriores al
amanecer local del día candidato.
```

Forma normativa en términos de tithi:

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

Implementación por muestreo puntual cuando el motor usa el inicio de la ventana:

```text
IF tithi_at(sunrise - 96 minutes) == Daśamī
THEN Candidate = InvalidViddha
```

La regla se aplica solamente a Ekādaśī. No se extrapola a otros tithis o
festividades.

## Entradas Y Salida

Entradas:

- `CivilDayTithiPresence` del día candidato;
- `AstronomicalTithi` en aruṇodaya;
- amanecer local del día candidato;
- estado o intervalo de tithi durante aruṇodaya;
- instante de inicio de Ekādaśī, solo si el motor lo expone como dato derivado;
- duración fija de aruṇodaya para esta regla: 96 minutos antes del amanecer.

Salida:

```text
Valid / InvalidViddha
```

No consume:

- nakṣatra;
- Hari-vāsara para Parāṇa;
- clasificación Mahādvādaśī;
- desplazamiento a Dvādaśī;
- reglas de festivales;
- GCal;
- fuentes secundarias para definir la regla.

## Casos De Prueba Normativos

### Caso A - Ekādaśī Presente En Aruṇodaya

```text
Ekādaśī presente al amanecer.
Ekādaśī presente en sunrise - 96 minutos.
```

Resultado:

```text
Valid
```

### Caso B - Daśamī Presente En Aruṇodaya

```text
Ekādaśī presente al amanecer.
Daśamī presente en sunrise - 96 minutos.
```

Resultado:

```text
InvalidViddha
```

### Caso C - Ekādaśī Comienza Exactamente En Aruṇodaya

```text
ekadasi_start == sunrise - 96 minutos
```

Resultado:

```text
Valid
```

Motivo técnico:

La regla normativa invalida solamente cuando Daśamī está presente durante
aruṇodaya. En una implementación equivalente basada en instantes, si Ekādaśī ya
está presente desde el comienzo exacto de la ventana, no hay Daśamī dentro de
aruṇodaya.

### Caso D - Candidato Viddhā No Decide Desplazamiento

```text
Ekādaśī presente al amanecer.
Daśamī presente en aruṇodaya.
```

Resultado en HBV-EK-002:

```text
InvalidViddha
```

La decisión de observar Dvādaśī pertenece a HBV-EK-003.

## Alcance Cerrado

HBV-EK-002 no resuelve:

- desplazamiento de observancia;
- Mahādvādaśī;
- Hari-vāsara para Parāṇa;
- Parāṇa;
- nakṣatra;
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
Campaign 39.6

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

HBV-EK-002:
🟢 Implementable

Código:
NO
```
