# ELP2000-82B Dataset Audit

Auditoría del dataset lunar oficial importado para Pancanga Engine.

Estado: ✅ Dataset importado y verificado estructuralmente.

---

## Objetivo

Auditar los artefactos generados a partir de los 36 archivos oficiales
`ELP1` a `ELP36`, sin implementar todavía `AST-L002`.

Esta auditoría cubre estructura, fidelidad, semántica e integridad de datos. No
evalúa fórmulas lunares ni calcula posiciones de la Luna.

---

## Artefactos Auditados

Entrada oficial:

```text
06_Data/Reference/ELP2000/raw/ELP1 ... ELP36
```

Salida generada:

```text
06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs
06_Data/Reference/ELP2000/generated/elp2000_manifest.json
```

---

## Resultado Global

```text
Theory: ELP2000-82B
Import version: 1
Importer version: 1.0.0
Files: 36
Terms: 37872
Dataset fingerprint: 0x3aa7ebfdd62d91dc
Generated Rust fingerprint: 0x338c91efeeaee125
```

SHA-256:

```text
elp2000_coefficients.rs
8ebdd4a28ba3443e21eb44c7607aa7d60eb34adefcf0b2c661305b5fc9ab4e7b

elp2000_manifest.json
e5a3e65f4f56fcd2e1e1cf74e59a8b78352e133b0a5225538e94d65cafbd177b
```

---

## Interpretación de Conteos

El `README` oficial de IMCCE publica `Records` incluyendo la línea de
encabezado.

Pancanga Engine registra como términos reales:

```text
terms = IMCCE Records - 1 header
```

El encabezado se conserva como identidad semántica del archivo, pero no se
importa como término científico.

---

## Formatos Fortran

| Grupo | Archivos | Formato |
|---|---|---|
| Main problem | `ELP1`-`ELP3` | `4i3,2x,f13.5,6(2x,f10.2)` |
| Short periodic | `ELP4`-`ELP9`, `ELP22`-`ELP36` | `5i3,1x,f9.5,1x,f9.5` |
| Planetary | `ELP10`-`ELP21` | `11i3,1x,f9.5,1x,f9.5` |

El parser usa campos de ancho fijo. No depende de separación por espacios.

---

## Unidades

| Coordenada | Unidad oficial |
|---|---|
| Longitude | arcseconds |
| Latitude | arcseconds |
| Distance | kilometers |

No se aplica conversión de unidades durante la importación.

---

## Transformaciones Realizadas

La importación realiza únicamente transformaciones estructurales:

- lee la línea de encabezado y la excluye de los términos;
- parsea campos de ancho fijo según el formato Fortran correspondiente;
- preserva la escritura decimal visible en la fuente;
- conserva el orden original de los términos;
- conserva la identidad de archivo `ELP1`...`ELP36`;
- genera constantes Rust agrupadas por archivo;
- registra huellas reproducibles en el manifiesto.

No realiza:

- evaluación trigonométrica;
- escalado de unidades;
- normalización angular;
- filtrado por precisión;
- reordenamiento de términos;
- truncamiento de coeficientes.

---

## Auditoría Por Archivo

| Archivo | Formato | IMCCE Records | Headers | Términos esperados | Términos generados | Coordenada | Unidad | Función física | Transformación |
|---|---|---:|---:|---:|---:|---|---|---|---|
| ELP1 | Main problem | 1024 | 1 | 1023 | 1023 | Longitude | arcseconds | Main problem longitude periodic terms, sine series | Header excluido; índices y coeficientes preservados; orden original |
| ELP2 | Main problem | 919 | 1 | 918 | 918 | Latitude | arcseconds | Main problem latitude periodic terms, sine series | Header excluido; índices y coeficientes preservados; orden original |
| ELP3 | Main problem | 705 | 1 | 704 | 704 | Distance | kilometers | Main problem distance periodic terms, cosine series | Header excluido; índices y coeficientes preservados; orden original |
| ELP4 | Short periodic | 348 | 1 | 347 | 347 | Longitude | arcseconds | Earth figure perturbations, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP5 | Short periodic | 317 | 1 | 316 | 316 | Latitude | arcseconds | Earth figure perturbations, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP6 | Short periodic | 238 | 1 | 237 | 237 | Distance | kilometers | Earth figure perturbations, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP7 | Short periodic | 15 | 1 | 14 | 14 | Longitude | arcseconds | Earth figure perturbations, longitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP8 | Short periodic | 12 | 1 | 11 | 11 | Latitude | arcseconds | Earth figure perturbations, latitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP9 | Short periodic | 9 | 1 | 8 | 8 | Distance | kilometers | Earth figure perturbations, distance/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP10 | Planetary | 14329 | 1 | 14328 | 14328 | Longitude | arcseconds | Planetary perturbations, table 1, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP11 | Planetary | 5234 | 1 | 5233 | 5233 | Latitude | arcseconds | Planetary perturbations, table 1, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP12 | Planetary | 6632 | 1 | 6631 | 6631 | Distance | kilometers | Planetary perturbations, table 1, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP13 | Planetary | 4385 | 1 | 4384 | 4384 | Longitude | arcseconds | Planetary perturbations, table 1, longitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP14 | Planetary | 834 | 1 | 833 | 833 | Latitude | arcseconds | Planetary perturbations, table 1, latitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP15 | Planetary | 1716 | 1 | 1715 | 1715 | Distance | kilometers | Planetary perturbations, table 1, distance/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP16 | Planetary | 171 | 1 | 170 | 170 | Longitude | arcseconds | Planetary perturbations, table 2, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP17 | Planetary | 151 | 1 | 150 | 150 | Latitude | arcseconds | Planetary perturbations, table 2, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP18 | Planetary | 115 | 1 | 114 | 114 | Distance | kilometers | Planetary perturbations, table 2, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP19 | Planetary | 227 | 1 | 226 | 226 | Longitude | arcseconds | Planetary perturbations, table 2, longitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP20 | Planetary | 189 | 1 | 188 | 188 | Latitude | arcseconds | Planetary perturbations, table 2, latitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP21 | Planetary | 170 | 1 | 169 | 169 | Distance | kilometers | Planetary perturbations, table 2, distance/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP22 | Short periodic | 4 | 1 | 3 | 3 | Longitude | arcseconds | Tidal effects, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP23 | Short periodic | 3 | 1 | 2 | 2 | Latitude | arcseconds | Tidal effects, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP24 | Short periodic | 3 | 1 | 2 | 2 | Distance | kilometers | Tidal effects, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP25 | Short periodic | 7 | 1 | 6 | 6 | Longitude | arcseconds | Tidal effects, longitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP26 | Short periodic | 5 | 1 | 4 | 4 | Latitude | arcseconds | Tidal effects, latitude/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP27 | Short periodic | 6 | 1 | 5 | 5 | Distance | kilometers | Tidal effects, distance/t | Header excluido; término marcado como proporcional a `t`; orden original |
| ELP28 | Short periodic | 21 | 1 | 20 | 20 | Longitude | arcseconds | Moon figure perturbations, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP29 | Short periodic | 13 | 1 | 12 | 12 | Latitude | arcseconds | Moon figure perturbations, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP30 | Short periodic | 15 | 1 | 14 | 14 | Distance | kilometers | Moon figure perturbations, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP31 | Short periodic | 12 | 1 | 11 | 11 | Longitude | arcseconds | Relativistic perturbations, longitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP32 | Short periodic | 5 | 1 | 4 | 4 | Latitude | arcseconds | Relativistic perturbations, latitude | Header excluido; índices y coeficientes preservados; orden original |
| ELP33 | Short periodic | 11 | 1 | 10 | 10 | Distance | kilometers | Relativistic perturbations, distance | Header excluido; índices y coeficientes preservados; orden original |
| ELP34 | Short periodic | 29 | 1 | 28 | 28 | Longitude | arcseconds | Planetary perturbations, solar eccentricity, longitude/t2 | Header excluido; término marcado como proporcional a `t²`; orden original |
| ELP35 | Short periodic | 14 | 1 | 13 | 13 | Latitude | arcseconds | Planetary perturbations, solar eccentricity, latitude/t2 | Header excluido; término marcado como proporcional a `t²`; orden original |
| ELP36 | Short periodic | 20 | 1 | 19 | 19 | Distance | kilometers | Planetary perturbations, solar eccentricity, distance/t2 | Header excluido; término marcado como proporcional a `t²`; orden original |

---

## Tests Estructurales Agregados

El importador incluye tests que verifican:

- presencia de los 36 archivos oficiales;
- conteos por archivo;
- total de `37_872` términos;
- regeneración byte a byte de `elp2000_coefficients.rs`;
- regeneración byte a byte de `elp2000_manifest.json`;
- fingerprint del dataset;
- fingerprint del Rust generado.

---

## Veredicto

La capa de datos ELP2000-82B queda auditada estructuralmente.

No se detectaron:

- archivos faltantes;
- términos faltantes;
- reordenamiento;
- encabezados importados como términos;
- pérdida estructural de identidad `ELP1`...`ELP36`.

`AST-L002` queda desbloqueado desde el punto de vista de datos. Antes de
implementar el evaluador lunar, debe definirse exactamente qué tablas de
longitud intervienen en la corrección periódica de longitud.
