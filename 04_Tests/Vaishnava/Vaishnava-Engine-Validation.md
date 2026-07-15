# Vaiṣṇava Engine Validation

Campaign: 46

Status: 🟡 External validation OPEN / local regression PASS

## Resultado

Campaign 46 no se declara `PASS` completo todavía.

La regresión normativa local del Motor Vaiṣṇava está cubierta por las pruebas
de `HBV-EK-001` a `HBV-EK-005` y por la fachada integrada
`classify_vaishnava_day(...)`. La batería local completa pasó en esta campaña.

La validación externa de punta a punta queda abierta. Swiss Ephemeris ya cerró
como `PASS`, PureBhakti ya aporta fixtures reales con diferencias clasificadas,
y GCal sigue pendiente de fuente local real:

```text
Oráculo astronómico/calendárico:
PASS

Motivo:
Swiss Ephemeris ejecutado con 1000 casos.

Oráculo Vaiṣṇava:
ABIERTO

Motivo:
PureBhakti importado y comparado; GCal pendiente.
```

No se modificó código, arquitectura ni Knowledge Base.

## Alcance De Campaign 46

Validar de punta a punta:

```text
Astronomy Engine
↓
Calendar Engine
↓
Vaiṣṇava Engine
↓
Clasificación final
↓
Parāṇa
```

Campaign 46 no agrega funcionalidades nuevas. Su responsabilidad es comparar
el comportamiento ya implementado contra oráculos externos separados.

## Inventario De Oráculos

| Oráculo | Estado | Uso |
| ------- | ------ | --- |
| Swiss Ephemeris / `swetest` | PASS | Validar astronomía y eventos calendáricos |
| PureBhakti | OPEN; fixtures reales importados | Validar fecha de observancia y Parāṇa |
| GCal / calendario Vaiṣṇava de referencia | Plantilla creada; fixtures pendientes | Validar clasificación Vaiṣṇava y Parāṇa |
| Pruebas Rust locales | Disponible | Regresión normativa interna |

GCal permanece como oráculo de validación. No es autoridad normativa.

## Clasificación De Diferencias

Toda diferencia futura debe clasificarse antes de modificar código:

```text
ASTRONOMY
CALENDAR
NORMATIVE
TIMEZONE / CIVIL TIME
ORACLE DIFFERENCE
ENGINE BUG
UNRESOLVED
```

No se debe corregir el motor hasta identificar la capa responsable.

## Campos Requeridos Por Caso

Cada caso de validación externa debe registrar:

```text
Ubicación
Fecha civil
Zona horaria
Amanecer
Aruṇodaya
Tithi en amanecer
Tithi en aruṇodaya
Transiciones relevantes
Nakṣatra
Candidato
Viddhā
Desplazamiento
Mahādvādaśī
Ventana de Parāṇa
Resultado del oráculo
Resultado del motor
PASS / DIFFERENCE
Clasificación de diferencia
```

## Matriz De Casos Requeridos

| ID | Caso | Estado | Nota |
| -- | ---- | ------ | ---- |
| VAI-001 | Ekādaśī normal śukla | Pendiente de oráculo externo | Requiere fixture fechado |
| VAI-002 | Ekādaśī normal kṛṣṇa | Pendiente de oráculo externo | Requiere fixture fechado |
| VAI-003 | Ekādaśī viddhā | Pendiente de oráculo externo | Debe registrar Daśamī en aruṇodaya |
| VAI-004 | Observancia desplazada | Pendiente de oráculo externo | Debe pasar por `ObserveOnDvadasi` |
| VAI-MD-001 | Unmīlanī | Pendiente de oráculo externo | Mahādvādaśī basado en tithi |
| VAI-MD-002 | Vyañjulī | Pendiente de oráculo externo | Debe enlazar `ISSUE-VAI-001` si difiere |
| VAI-MD-003 | Triṣpṛśā | Pendiente de oráculo externo | Mahādvādaśī basado en tithi |
| VAI-MD-004 | Pakṣavardhinī | Pendiente de oráculo externo | Mahādvādaśī basado en tithi |
| VAI-MD-005 | Jayā | Pendiente de oráculo externo | Punarvasu + śukla Dvādaśī |
| VAI-MD-006 | Vijayā | Pendiente de oráculo externo | Śravaṇa + śukla Dvādaśī |
| VAI-MD-007 | Jayantī | Pendiente de oráculo externo | Rohiṇī + śukla Dvādaśī |
| VAI-MD-008 | Pāpanāśinī | Pendiente de oráculo externo | Puṣya + śukla Dvādaśī |
| VAI-PAR-001 | Dvādaśī corta | Pendiente de oráculo externo | Debe activar `ShortDvadasi` |
| VAI-PAR-002 | Hari-vāsara termina antes del amanecer | Pendiente de oráculo externo | Parāṇa comienza en amanecer |
| VAI-PAR-003 | Hari-vāsara termina después del amanecer | Pendiente de oráculo externo | Parāṇa comienza al fin de Hari-vāsara |
| VAI-EDGE-001 | Caso cercano a transición exacta | Pendiente de oráculo externo | Requiere tolerancia documentada |
| VAI-GEO-001 | Latitud media norte | Pendiente de oráculo externo | Validación geográfica |
| VAI-GEO-002 | Latitud media sur | Pendiente de oráculo externo | Validación geográfica |
| VAI-GEO-003 | Ecuador | Pendiente de oráculo externo | Validación geográfica |
| VAI-TIME-001 | Fecha histórica | Pendiente de oráculo externo | Dentro del rango soportado |
| VAI-TIME-002 | Fecha futura | Pendiente de oráculo externo | Dentro del rango soportado |

La versión tabular editable está en
`04_Tests/Vaishnava/Vaishnava-Engine-Validation.csv`.

## ISSUE-VAI-001

`ISSUE-VAI-001` queda resuelto después de la revisión final de HBV
13.356-399 y la fuente local de Bhṛgumuni Dāsa / Digdarśinī.

Campaign 46 debe seguir incluyendo al menos un caso de Vyañjulī. Si el motor y
un oráculo externo difieren, no se modifica la implementación antes de
clasificar la diferencia como:

```text
CONFIGURATION DIFFERENCE
TRADITION DIFFERENCE
ORACLE DIFFERENCE
```

según corresponda, enlazando `ISSUE-VAI-001`.

## Decisión De Cierre

Campaign 46 queda en estado:

```text
🟡 External validation OPEN
```

Condiciones pendientes para declarar `PASS`:

1. Revisar las diferencias de Parāṇa detectadas contra PureBhakti.
2. Incorporar fixtures GCal o calendarios Vaiṣṇavas confiables en
   `04_Tests/Vaishnava/GCal/gcal-fixtures.csv`.
3. Poblar la matriz con más fechas, ubicaciones y zonas horarias concretas.
4. Comparar resultado del motor contra los oráculos restantes.
5. Clasificar toda diferencia antes de modificar código.

## Estado Técnico

```text
Código:
NO MODIFICADO

Knowledge Base:
NO MODIFICADA

Arquitectura:
NO MODIFICADA

Regresión local:
PASS

Validación externa:
ABIERTA
```

## PureBhakti 2026

Campaign 46D.1 incorporó el primer calendario HTML externo real:

```text
02_Research/PureBhakti/Calendario-Vaishnava-2026.html
```

Resultados:

```text
Fixtures:
16

Fecha de observancia:
16 / 16 PASS

Parāṇa:
1 / 16 PASS
15 / 16 CONFIGURATION DIFFERENCE
```

Reporte:

```text
04_Tests/PureBhakti/PureBhakti-Validation.md
04_Tests/PureBhakti/purebhakti-validation.csv
```

## Verificación Local

```text
cargo fmt --all --check
PASS

cargo test -p pancanga-engine
PASS

cargo clippy -p pancanga-engine -- -D warnings
PASS

cargo doc --no-deps
PASS
```
