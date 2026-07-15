# ELP2000 Importer

Herramienta Rust para verificar e importar coeficientes ELP2000-82B.

La herramienta vive dentro del workspace Rust:

```text
03_Source/rust/tools/elp2000-importer
```

## Responsabilidad

El importador no evalúa la Luna.

Solo hace tres cosas:

1. Lee los archivos oficiales `ELP1` a `ELP36`.
2. Verifica formato, conteos e integridad básica.
3. Genera constantes Rust preservando la escritura decimal original.
4. Genera un manifiesto determinístico de identidad de datos.

## Uso

Colocar los archivos oficiales en:

```text
06_Data/Reference/ELP2000/raw/
```

Generación oficial determinística:

```bash
cargo run -p elp2000-importer
```

Este comando genera:

```text
06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs
06_Data/Reference/ELP2000/generated/elp2000_manifest.json
```

Verificar solamente:

```bash
cargo run -p elp2000-importer -- verify ../../06_Data/Reference/ELP2000/raw
```

Generar con rutas explícitas:

```bash
cargo run -p elp2000-importer -- generate-all \
  ../../06_Data/Reference/ELP2000/raw \
  ../../06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs \
  ../../06_Data/Reference/ELP2000/generated/elp2000_manifest.json
```

## Determinismo

El importador no registra timestamps en los archivos generados.

Con los mismos `ELP1` a `ELP36`, el resultado debe ser byte a byte idéntico en
cualquier máquina.

El manifiesto incluye:

```text
Theory: ELP2000-82B
Import version: 1
Importer version: 1.0.0
Generated Rust fingerprint
Dataset fingerprint
```

La importación oficial actual produjo:

```text
Files: 36
Records: 37872
Dataset fingerprint: 0x3aa7ebfdd62d91dc
Generated Rust fingerprint: 0x338c91efeeaee125
```

## Fuente

IMCCE / Observatoire de Paris, `LUNAR SOLUTION ELP2000-82B`.
