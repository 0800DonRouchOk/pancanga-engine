# HBV-EK-005 - Parāṇa / Hari-vāsara

Campaign: 39.9

Estado: 🟢 PASS.

Código: NO.

## Pregunta

```text
¿Cómo determina el Hari-bhakti-vilāsa la ventana válida para realizar el
Parāṇa?
```

Esta regla define la ventana normativa para romper el ayuno. No clasifica
Ekādaśī, no decide Viddhā, no desplaza observancias y no clasifica
Mahādvādaśī.

## Fuente

Fuentes locales usadas:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.doc
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt
```

No se usó Internet para definir reglas.

## Conocimiento Ya Confirmado

- HBV-EK-001 genera candidatos.
- HBV-EK-002 invalida candidatos Viddhā.
- HBV-EK-003 desplaza la observancia cuando corresponde.
- HBV-EK-004 clasifica Mahādvādaśī.
- HBV-EK-005 solo define la ventana de Parāṇa y la restricción de
  Hari-vāsara.

## Evidencia

### Evidencia 1 - Parāṇa Completa El Vrata

Referencia:

```text
HBV 13.229-230
```

Tipo de evidencia:

```text
Regla de cierre del vrata
```

Relevancia algorítmica:

La ruptura del ayuno ocurre después de las obligaciones matutinas de Dvādaśī y
completa el vrata. Esto define Parāṇa como una acción posterior al ayuno, no
como parte de la selección del día de observancia.

### Evidencia 2 - Parāṇa Dentro De Dvādaśī

Referencia:

```text
HBV 13.236
```

Tipo de evidencia:

```text
Límite temporal principal
```

Relevancia algorítmica:

Parāṇa debe realizarse dentro de Dvādaśī, después de completar las obligaciones
necesarias. La referencia al "medio de Dvādaśī" establece preferencia
operativa, pero no autoriza transgredir Dvādaśī.

### Evidencia 3 - Prohibición De Transgredir Dvādaśī

Referencias:

```text
HBV 13.238
HBV 13.239-241
HBV 13.242-245
```

Tipo de evidencia:

```text
Límite superior obligatorio
```

Relevancia algorítmica:

No romper el ayuno dentro de Dvādaśī cuenta como transgredir Dvādaśī. La
ventana válida de Parāṇa no debe extenderse a Trayodaśī cuando Dvādaśī está
disponible.

### Evidencia 4 - Dvādaśī Corta

Referencias:

```text
HBV 13.246-249
HBV 13.250-251
HBV 13.252-256
```

Tipo de evidencia:

```text
Regla de contingencia
```

Relevancia algorítmica:

Cuando Dvādaśī es corta, las obligaciones se adelantan. Si el tiempo no alcanza
para una ruptura ordinaria del ayuno, el texto admite romperlo con agua para
evitar transgredir Dvādaśī.

### Evidencia 5 - Hari-vāsara

Referencias:

```text
HBV 13.257
HBV 13.258-259
```

Tipo de evidencia:

```text
Límite inferior obligatorio
```

Relevancia algorítmica:

Hari-vāsara incluye el primer cuarto de Dvādaśī y se describe también como el
enlace formado por el último cuarto de Ekādaśī y el primer cuarto de Dvādaśī.
No se debe romper el ayuno durante este intervalo.

## Regla Normativa Cerrada

```text
La ventana válida de Parāṇa comienza cuando Hari-vāsara ha terminado y debe
cerrarse antes de que termine Dvādaśī.
```

Regla completa:

```text
Parāṇa debe realizarse en Dvādaśī, después de Hari-vāsara, y antes de que
Dvādaśī sea transgredida. Si Dvādaśī es tan corta que el cumplimiento ordinario
de las obligaciones impediría romper el ayuno dentro de Dvādaśī, las
obligaciones se adelantan y, si es necesario, Parāṇa puede realizarse con agua.
```

## Forma Normativa

```text
IF current_time dentro de Hari-vāsara
THEN Parāṇa = Prohibido

IF current_time después de Hari-vāsara AND antes del fin de Dvādaśī
THEN Parāṇa = Permitido

IF Dvādaśī es corta AND existe riesgo de transgredir Dvādaśī
THEN adelantar obligaciones y permitir Parāṇa mínimo con agua
```

## Entradas

```text
- día de observancia producido por HBV-EK-001 a HBV-EK-004
- amanecer local del día de Parāṇa
- intervalo de Dvādaśī
- intervalo de Ekādaśī para determinar el enlace de Hari-vāsara cuando aplique
- fin de Hari-vāsara
```

## Salida

```text
ParāṇaWindow

- earliest_allowed_time
- latest_allowed_time
- harivasara_end
- dvadasi_end
- short_dvadasi_contingency
```

## No Consume

```text
- clasificación inicial de candidato
- cálculo de Viddhā
- desplazamiento de observancia
- clasificación Mahādvādaśī
- procedimiento ritual detallado
- alimentos permitidos
- vigilia
- festivales
```

## Casos De Prueba Normativos

### Caso A - Dvādaśī Normal

```text
Dvādaśī continúa suficientemente después del amanecer.
Hari-vāsara termina antes de que Dvādaśī termine.

Resultado:
Parāṇa permitido después de Hari-vāsara y antes del fin de Dvādaśī.
```

### Caso B - Intento Durante Hari-vāsara

```text
El ayuno se intenta romper dentro del primer cuarto de Dvādaśī.

Resultado:
Parāṇa prohibido.
```

### Caso C - Dvādaśī Corta

```text
Dvādaśī termina poco después del amanecer.
Existe riesgo de transgredir Dvādaśī.

Resultado:
Las obligaciones se adelantan; si no hay tiempo suficiente, Parāṇa mínimo con
agua queda permitido para evitar transgredir Dvādaśī.
```

### Caso D - Trayodaśī

```text
Dvādaśī estuvo disponible pero el ayuno no se rompió dentro de ella.

Resultado:
Parāṇa en Trayodaśī no es una ventana válida ordinaria.
```

## Evidence Level

```text
E1 Fuente primaria verificada: ✅
E2 Comentario tradicional confirmado: ✅
E3 Interpretación técnica: ✅
E4 Casos de prueba definidos: ✅
E5 Lista para implementación: ✅
```

## Cierre

```text
Campaign 39.9

PASS

Versos primarios:
Verificados

Digdarśinī:
Revisada donde aparece en la fuente local

Regla normativa:
Cerrada

Entradas/salida:
Definidas

Casos de prueba:
Definidos

HBV-EK-005:
🟢 Implementable

Código:
NO
```

