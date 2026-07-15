# Vaiṣṇava Normative Authority

Estado: 🔒 Congelado para v1.0

Campaign 37

## Objetivo

Elegir la autoridad normativa primaria para implementar el Motor Vaiṣṇava de
Pancanga Engine v1.0.

Este documento no implementa reglas. Define qué fuentes gobiernan la
implementación futura.

## Decisión

Pancanga Engine v1.0 implementará las reglas de Ekādaśī siguiendo como
autoridad normativa primaria:

```text
Hari-bhakti-vilāsa
```

La implementación v1.0 será explícitamente una implementación Gauḍīya
Vaiṣṇava basada en Hari-bhakti-vilāsa. No intentará ser un motor universal de
todas las tradiciones hindúes o vaiṣṇavas.

## Fuentes Secundarias

Las fuentes secundarias se usarán para interpretar, contextualizar o validar la
fuente primaria, pero no para contradecirla.

Fuentes secundarias aceptadas:

- Caitanya-caritāmṛta, especialmente la instrucción sobre evitar Ekādaśī
  viddhā y observar Ekādaśī puro;
- comentarios autorizados de la tradición Gauḍīya Vaiṣṇava;
- documentación específica de la tradición adoptada por el proyecto, si se
  incorpora formalmente;
- Gaurābda Calendar / GCal como referencia de validación calendárica, no como
  autoridad normativa primaria.

Wikipedia y otros resúmenes enciclopédicos no son fuentes normativas para
Pancanga Engine. Pueden orientar investigación histórica, pero no definen
reglas de implementación.

## Oráculo Vaiṣṇava

El oráculo vaiṣṇava de v1.0 tendrá dos niveles:

1. Casos manuales derivados directamente de las reglas normativas congeladas.
2. Comparación contra Gaurābda Calendar / GCal como referencia calendárica de
   regresión.

Si GCal difiere de una regla derivada de Hari-bhakti-vilāsa, la discrepancia
no se resolverá modificando el código inmediatamente. Se abrirá una revisión
normativa para determinar si:

- la regla fue mal interpretada;
- el caso requiere una excepción tradicional;
- el oráculo calendárico usa una convención distinta;
- el motor astronómico o calendárico produjo datos incorrectos.

## Resolución De Discrepancias

Orden de autoridad para v1.0:

1. Hari-bhakti-vilāsa.
2. Comentarios o tradición autorizada que expliquen Hari-bhakti-vilāsa.
3. Caitanya-caritāmṛta como principio doctrinal general.
4. GCal y pañcāṅgas de referencia como validación externa.

GCal no puede introducir una regla normativa nueva por sí mismo.

## Tradiciones No Implementadas En v1.0

Pancanga Engine v1.0 no implementará variantes normativas múltiples.

Quedan fuera de alcance:

- reglas regionales;
- variantes de otras sampradāyas;
- variantes modernas sin fuente normativa explícita;
- reglas específicas de una tradición que no estén documentadas en el
  repositorio.

Si la Nityānanda Vaṁśa se adopta como estándar futuro, deberá incorporarse
mediante una campaña normativa propia y con fuentes explícitas antes de
modificar el motor.

## Consecuencia Para El Código

No se escribirá código de Ekādaśī hasta que Campaign 38 congele las reglas
operativas mínimas:

- candidato de Ekādaśī;
- Viddhā;
- desplazamiento de observancia;
- ocho Mahādvādaśī;
- forma pública de la clasificación.

La autoridad queda elegida. Las reglas concretas todavía no.

## Referencias Iniciales

- Caitanya-caritāmṛta Madhya 24.342 registra el principio de evitar Ekādaśī
  viddhā y observar Ekādaśī puro.
- Hari-bhakti-vilāsa queda seleccionado como fuente normativa primaria; los
  pasajes exactos deberán fijarse en Campaign 38 antes de implementar código.

## Estado

Campaign 37: ✅ Cerrada.

Autoridad normativa primaria: 🔒 Hari-bhakti-vilāsa.

Siguiente: Campaign 38 - Ekādaśī Rules Specification.
