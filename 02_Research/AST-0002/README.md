# Investigación Asociada a AST-0002

Esta carpeta reúne referencias y notas para la posición heliocéntrica de la
Tierra usando `VSOP87D`.

## Estructura

- `papers/` - documentación técnica, papers y fuentes primarias.
- `links/` - enlaces a recursos VSOP87 y referencias externas.
- `notas/` - notas de revisión, decisiones abiertas y análisis.
- `ejemplos/` - datos de referencia y casos de validación.

## Preguntas de Revisión

- ¿Versión completa o truncada?
- ¿Qué tolerancia requiere Pancanga Engine?
- ¿Qué datos externos se usarán para validación científica?

## Decisión Cerrada

`ADR-0004` adopta `VSOP87D` como variante oficial para `AST-0002`.

## Validación

La primera implementación de `AST-0002` valida la longitud heliocéntrica de la
Tierra contra los casos oficiales de `vsop87.chk`.

- Términos importados: 1080.
- Coincidencia decimal contra `VSOP87D.ear`: exacta.
- Error máximo observado: `3.968958495192964619e-11` rad.
