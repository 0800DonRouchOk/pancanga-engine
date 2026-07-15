# ISSUE-VAI-001 - Vyañjulī Divergence

Estado: ✅ Resolved.

No bloquea: HBV-EK-004.

Código requerido: NO.

## Tema

Durante la extracción de HBV-EK-004 se detectaron dos formulaciones circulantes
para Vyañjulī / Vañjulī:

```text
Versión A
Ekādaśī no completa.

Versión B
Ekādaśī pura + Dvādaśī prolongada hacia Trayodaśī.
```

La revisión final de la fuente local prioritaria favorece la Versión B.

## Fuentes Revisadas

Precedencia aplicada:

```text
1. Hari-bhakti-vilāsa mūla
2. Digdarśinī-ṭīkā
3. Navadvīpa Pañjikā / aplicación tradicional
```

Fuente local:

```text
07_Sources/Vaishnava/Hari-bhakti-vilasa/raw/hbv-vilasa-13.txt
```

Pasajes principales:

```text
HBV 13.356-358
HBV 13.359
HBV 13.372
HBV 13.394-399
Bhṛgumuni Dāsa technical summary after HBV 13
```

## Resolución Normativa

Vyañjulī Mahādvādaśī queda especificada así:

```text
1. Ekādaśī es pura.
2. Dvādaśī está presente al amanecer.
3. Dvādaśī continúa hasta el amanecer siguiente, tocando Trayodaśī.
```

La pureza de Ekādaśī sigue dependiendo de HBV-EK-002:

```text
Ekādaśī debe comenzar antes de Aruṇodaya
(dos muhūrtas = 96 minutos antes del amanecer local).
```

La condición de Dvādaśī en Vyañjulī no hereda ese umbral:

```text
Dvādaśī debe estar presente al amanecer.
No necesita comenzar dos muhūrtas antes del amanecer.
```

## Observancia

Para Vyañjulī:

```text
ObservanceDay = Dvādaśī
Mahādvādaśī = Vyañjulī
```

La regla de Parāṇa se mantiene bajo HBV-EK-005. Este issue no redefine
Hari-vāsara ni la ventana de Parāṇa.

## Nota De Traducción

HBV 13.356-358, en la traducción local de Bhṛgumuni Dāsa, contiene una frase
que dice que se debe comer en Ekādaśī y realizar el vrata en Dvādaśī.

Esa frase se marca como inconsistencia de traducción y no se usa como
fundamento normativo porque contradice:

- la definición del propio Vañjulī-vrata;
- el mantra de declaración de HBV 13.359;
- la lógica general de las Mahādvādaśī;
- el resumen técnico de Bhṛgumuni Dāsa;
- la aplicación tradicional revisada.

## Decisión

```text
ISSUE-VAI-001:
RESOLVED

Formulación adoptada:
Versión B

Impacto en Knowledge Base:
HBV-MD-002 queda cerrado sin divergencia abierta.

Impacto en código:
No requiere cambio lógico; la implementación ya modela Vyañjulī como
Ekādaśī pura + Dvādaśī presente al amanecer + continuidad hacia Trayodaśī.
```

## Estado Operativo

```text
HBV-EK-004:
🟢 PASS

Vyañjulī:
🟢 Resuelta

Open doctrinal issues:
0
```
