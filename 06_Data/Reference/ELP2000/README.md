# ELP2000 Reference Data

Área reservada para datos oficiales de ELP2000.

```text
raw/
    ELP1 ... ELP36

generated/
    elp2000_coefficients.rs
    elp2000_manifest.json
```

Los archivos en `raw/` deben provenir de la distribución oficial de IMCCE.

La generación de Rust debe hacerse exclusivamente mediante:

```text
elp2000-importer
```

No editar manualmente los coeficientes generados.

El manifiesto registra la identidad del dataset importado: versión de
importación, versión del importador, teoría científica, conteos, huellas
reproducibles de fuente y huella del archivo Rust generado.

Importación oficial actual:

```text
Files: 36
Records: 37872
Dataset fingerprint: 0x3aa7ebfdd62d91dc
Generated Rust fingerprint: 0x338c91efeeaee125
```
