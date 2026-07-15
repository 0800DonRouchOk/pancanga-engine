# HBV Extraction Protocol

Estado: 🔒 Activo

Este protocolo define cómo se extraerán reglas de Hari-bhakti-vilāsa para
Pancanga Engine.

## Principio

No se extraen todas las reglas de una vez.

Cada regla se trabaja como una campaña independiente:

```text
Vilāsa 12/13
        ↓
Regla HBV-EK
        ↓
Versos
        ↓
Interpretación técnica
        ↓
Datos del motor
        ↓
Casos de prueba
        ↓
Cierre
```

## Prioridad De Fuentes

| Prioridad | Tema | Śāstra / fuente | Estado |
|---|---|---|---|
| ⭐⭐⭐⭐⭐ | Ekādaśī, viddhā, Mahādvādaśī, parāṇa | Hari-bhakti-vilāsa, Vilāsas 12-13 | Extraído; auditado |
| ⭐⭐⭐⭐☆ | Casos ambiguos y observancia Gauḍīya | Digdarśinī-ṭīkā de Sanātana Gosvāmī | Consultar cuando haga falta |
| ⭐⭐⭐⭐☆ | Aplicación práctica Gauḍīya moderna | Navadvīpa Pañjikā | Referencia práctica |
| ⭐⭐⭐☆☆ | Confirmación de citas originales | Purāṇas citados por HBV | Solo si hace falta |

## Niveles De Fuente

| Nivel | Fuentes | Función |
|---|---|---|
| A - Autoridad normativa | Hari-bhakti-vilāsa; Digdarśinī-ṭīkā | Definir o aclarar reglas normativas |
| B - Autoridades de aplicación | Navadvīpa Pañjikā; Tradition-Notes | Confirmar cómo se aplica una regla en la tradición |
| C - Confirmación | Purāṇas citados por HBV | Verificar contexto original solo si hace falta |

## Precedencia De Fuentes

Cuando dos fuentes discrepen, Pancanga Engine usará este orden de precedencia:

1. Hari-bhakti-vilāsa, mūla.
2. Digdarśinī-ṭīkā.
3. Navadvīpa Pañjikā.
4. Tradition Notes.
5. Purāṇas citados por Hari-bhakti-vilāsa, solo como contexto.

Si una discrepancia no puede resolverse:

```text
No se modifica la regla.
Se documenta la divergencia.
La implementación permanece protegida por protocolo hasta resolverla.
```

## Congelamiento Metodológico

```text
No modificar la metodología por comodidad de una regla.
```

Toda propuesta de cambio metodológico debe responder primero:

```text
¿El Hari-bhakti-vilāsa obliga a cambiarla?
```

Si la respuesta es sí, se modifica y se documenta. Si la respuesta es no, la
metodología permanece congelada.

## Cierre De Formalización

Después de cerrar HBV-EK-005 no se debe pasar inmediatamente a código.

Primero se realizará una revisión completa de la Knowledge Base con una sola
pregunta:

```text
¿Un desarrollador que nunca leyó el Hari-bhakti-vilāsa puede implementar
correctamente el Motor Vaiṣṇava usando únicamente la Knowledge Base?
```

Si la respuesta es sí, la fase de Formalización del Hari-bhakti-vilāsa queda
cerrada. Si la respuesta es no, falta especificación, no código.

## Regla Sobre Internet

Una vez identificada la autoridad primaria, ninguna regla normativa podrá
fundamentarse en búsquedas web.

Uso permitido de Internet:

- localizar una edición confiable;
- conseguir una copia del texto;
- verificar datos bibliográficos;
- comprobar enlaces o procedencia de una fuente.

Uso no permitido de Internet:

- definir una regla HBV-EK;
- resolver una ambigüedad normativa;
- sustituir Hari-bhakti-vilāsa;
- sustituir Digdarśinī-ṭīkā;
- rellenar vacíos con resúmenes, calendarios o fuentes secundarias.

Para Campaigns 39.6 a 39.9, la extracción debe trabajar sobre:

- Hari-bhakti-vilāsa;
- Digdarśinī-ṭīkā cuando haya ambigüedad;
- Navadvīpa Pañjikā para aplicación práctica, no para autoridad primaria;
- Tradition-Notes solo como interpretación secundaria vinculada a una regla
  HBV-EK.

## Orden De Extracción

```text
☐ HBV 12
    ☑ HBV-EK-001
    ☑ HBV-EK-002
    ☑ HBV-EK-003
    ☑ HBV-EK-004

☑ HBV 13
    ☑ HBV-EK-005

☐ Digdarśinī
    ☐ Resolver ambigüedades

☐ Navadvīpa Pañjikā
    ☐ Confirmar aplicación práctica
```

## Matriz De Cobertura

| Regla | HBV | Digdarśinī | Navadvīpa | Implementable | Estado |
|---|:---:|:---:|:---:|:---:|---|
| HBV-EK-001 | ✅ | ✅ | ⬜ | 🟢 | Congelada |
| HBV-EK-002 | ✅ | ✅ | ✅ | 🟢 | Congelada |
| HBV-EK-003 | ✅ | ✅ | ⬜ | 🟢 | Congelada |
| HBV-EK-004 | ✅ | ✅ | ⬜ | 🟢 | Congelada; ISSUE-VAI-001 resuelto |
| HBV-EK-005 | ✅ | ✅ | ⬜ | 🟢 | Congelada |

Leyenda:

- HBV: pasaje primario verificado.
- Digdarśinī: comentario revisado cuando aplica.
- Navadvīpa: aplicación práctica comparada cuando aplica.
- Implementable: la regla tiene suficiente evidencia y no depende de una regla
  pendiente para ser traducida a código.
- 🟡 Implementable: existe base textual suficiente para extracción técnica,
  pero la regla aún no está congelada para implementación.
- Congelada: regla lista para implementación.

Una regla puede tener fuentes verificadas y aun así no ser implementable si
depende de otra regla todavía abierta.

## Campaign 39.5 - Alcance Estricto

Pregunta única:

```text
¿Qué define que un día sea candidato a Ekādaśī según Hari-bhakti-vilāsa?
```

Entregable:

```text
HBV-EK-001

Versos:
Definición textual:
Interpretación técnica:
Datos que necesita Pancanga Engine:
No necesita:
Casos de prueba:
Estado:
```

HBV-EK-001 no debe resolver:

- Ekādaśī viddhā;
- Mahādvādaśī;
- Hari-vāsara;
- parāṇa;
- nakṣatra;
- reglas de festivales.

### Cuatro Niveles De Respuesta

Campaign 39.5 debe responder la pregunta en cuatro niveles, sin mezclar reglas
posteriores.

#### 1. Definición Textual

Extraer el pasaje de Hari-bhakti-vilāsa que define el candidato de Ekādaśī.

```text
Versos:
Texto:
```

Todavía no se interpreta.

#### 2. Definición Normativa

Traducir el pasaje a una regla única, limitada al candidato.

Ejemplo de forma esperada:

```text
Un día es candidato a Ekādaśī cuando ...
```

No debe decidir si el candidato queda invalidado por Viddhā.

#### 3. Datos Que Consume Pancanga Engine

La extracción debe separar explícitamente datos necesarios y no necesarios.

```text
Necesita:
✓ Tithi
✓ Amanecer

No necesita:
✗ Nakṣatra
✗ Hari-vāsara
✗ Mahādvādaśī
✗ Parāṇa
```

#### 4. Casos De Prueba

No más de tres casos iniciales:

- Ekādaśī presente en el amanecer -> candidato.
- Ekādaśī ausente en el amanecer -> no candidato.
- Ekādaśī aparece como candidato pero luego podría descartarse por Viddhā ->
  sigue siendo candidato en HBV-EK-001, porque la invalidación pertenece a
  HBV-EK-002.

### Hipótesis De Composición

La estructura de reglas Vaiṣṇavas se tratará como una hipótesis de trabajo
hasta confirmarla verso por verso:

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

Esta hipótesis no desbloquea implementación. Solo organiza la extracción.

## Consultas No Prioritarias

No se invierte tiempo ahora en:

- Padma Purāṇa completo;
- Brahma-vaivarta Purāṇa completo;
- Nārada Purāṇa completo;
- historias devocionales completas de Ekādaśī;
- māhātmyas que no definan reglas operativas.

## Campañas

| Campaign | Regla | E1 | E2 | E3 | E4 | E5 | Estado |
|---|---|---|---|---|---|---|---|
| 39.5 | HBV-EK-001 - Candidato de Ekādaśī | ✅ | ✅ | ✅ | ✅ | ✅ | Cerrada |
| 39.6 | HBV-EK-002 - Ekādaśī viddhā | ✅ | ✅ | ✅ | ✅ | ✅ | Cerrada |
| 39.7 | HBV-EK-003 - Desplazamiento de observancia | ✅ | ✅ | ✅ | ✅ | ✅ | Cerrada |
| 39.8 | HBV-EK-004 - Ocho Mahādvādaśī | ✅ | ✅ | ✅ | ✅ | ✅ | Cerrada |
| 39.9 | HBV-EK-005 - Parāṇa / Hari-vāsara | ✅ | ✅ | ✅ | ✅ | ✅ | Cerrada |
| 39.10 | Knowledge Base Audit | N/A | N/A | N/A | N/A | N/A | Cerrada |

## Evidence Level

Cada regla HBV-EK se mide con cinco niveles de evidencia:

| Nivel | Significado |
|---|---|
| E1 | Fuente primaria verificada |
| E2 | Comentario tradicional confirmado |
| E3 | Interpretación técnica cerrada |
| E4 | Caso de prueba definido |
| E5 | Lista para implementación |

Ejemplo:

```text
HBV-EK-001

E1 ✅
E2 ✅
E3 🟡
E4 ⬜
E5 ⬜
```

## Formato De Cierre

Cada campaña debe producir:

```text
Regla:
Fuente:
Versos:
Texto:
Interpretación técnica:
Datos que consume:
Casos de prueba:
Estado:
```

## Criterio De PASS

Una regla pasa solo cuando:

- cita una fuente primaria identificable;
- extrae versos o referencias precisas;
- define una interpretación técnica mínima;
- enumera los datos que consume Pancanga Engine;
- propone casos de prueba;
- no introduce código.

## Protección De Implementación

Campaign 40 queda desbloqueada por Knowledge Base v1.0.

La protección cambia de forma:

```text
No modificar Knowledge Base para facilitar implementación.

Solo modificarla si una fuente primaria obliga al cambio.
```
