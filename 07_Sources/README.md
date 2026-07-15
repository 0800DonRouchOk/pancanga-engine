# Sources

Estado: 🟡 En construcción

Este directorio registra las fuentes primarias, secundarias y de validación que
respaldan Pancanga Engine.

Su propósito no es duplicar toda la investigación de `02_Research/`, sino
responder una pregunta simple:

```text
¿De dónde proviene esta decisión?
```

## Principio

Las fuentes se clasifican por autoridad:

- fuente primaria: define la regla, teoría o dato;
- fuente secundaria: ayuda a interpretar la fuente primaria;
- oráculo de validación: permite comparar resultados, pero no define reglas;
- contexto: orienta investigación, pero no se usa para implementar.

## Estructura

```text
07_Sources/
    Astronomy/
        Meeus/
        ELP2000/
        Swiss/
    Vaishnava/
        Hari-bhakti-vilasa/
        Caitanya-caritamrta/
        Interpretations/
```

## Regla De Uso

Ningún algoritmo o regla normativa debe considerarse cerrado si no puede
apuntar a una fuente registrada o a una decisión explícita del proyecto.

## Relación Con Investigación

`02_Research/` contiene notas, análisis, papeles, enlaces y trabajo en curso.

`07_Sources/` contiene la capa de trazabilidad de autoridad:

```text
Fuente
    ↓
Decisión
    ↓
Especificación
    ↓
Implementación
    ↓
Validación
```
