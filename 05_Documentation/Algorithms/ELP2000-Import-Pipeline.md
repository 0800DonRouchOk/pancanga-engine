# ELP2000 Import Pipeline

Infraestructura reproducible para importar coeficientes lunares ELP2000.

Estado: ✅ Ejecutado contra los 36 archivos oficiales de IMCCE.

---

## Objetivo

Construir una ruta auditable entre los archivos originales de IMCCE y los
coeficientes Rust usados por Pancanga Engine:

```text
ELP2000-82B original
        ↓
Parser de ancho fijo
        ↓
Verificación de integridad
        ↓
Rust auto-generado
        ↓
Evaluador lunar
```

Esta etapa no calcula longitud lunar. Tampoco evalúa la serie. Su única
responsabilidad es garantizar que los datos entren al repositorio de forma
reproducible.

---

## Fuente Científica

Fuente oficial:

```text
IMCCE / Observatoire de Paris
LUNAR SOLUTION ELP2000-82B
```

La distribución oficial contiene 36 archivos de datos:

```text
ELP1 ... ELP36
```

Los archivos contienen las series semianalíticas relacionadas con longitud,
latitud y distancia lunar. Las unidades publicadas son segundos de arco para
longitud y latitud, y kilómetros para distancia.

---

## Verificaciones Obligatorias

El importador debe verificar:

- presencia de los 36 archivos `ELP1` a `ELP36`;
- longitud lógica esperada de cada archivo;
- número exacto de registros de cada tabla;
- formato fijo de cada registro;
- parseo válido de enteros y decimales;
- preservación de la escritura decimal original;
- huella reproducible de cada archivo fuente;
- generación determinista del archivo Rust;
- generación determinista del manifiesto de datos.

Nota: el campo `Records` publicado por el `README` oficial de IMCCE incluye la
línea de encabezado. Pancanga Engine registra como `records` la cantidad de
términos de coeficientes importados, es decir, el total publicado menos ese
encabezado.

---

## Identidad de Datos

Los datos importados tienen identidad propia, separada de la versión del
software.

La versión inicial del formato de importación es:

```text
Import version: 1
Importer version: 1.0.0
Theory: ELP2000-82B
```

Cada importación completa generará:

```text
elp2000_coefficients.rs
elp2000_manifest.json
```

El manifiesto registra:

- teoría científica;
- versión del importador;
- total de archivos;
- total de registros;
- huella de cada archivo fuente;
- huella combinada del dataset.
- huella del archivo `elp2000_coefficients.rs` generado.

No se registra hora de generación dentro del manifiesto, porque un timestamp
rompería la reproducibilidad byte a byte.

Importación oficial ejecutada:

```text
Files: 36
Records: 37872
Dataset fingerprint: 0x3aa7ebfdd62d91dc
Generated Rust fingerprint: 0x338c91efeeaee125
```

---

## Formatos Soportados

El importador implementa los tres formatos Fortran usados por la rutina
original:

```text
ELP1-ELP3
4i3,2x,f13.5,6(2x,f10.2)

ELP4-ELP9 y ELP22-ELP36
5i3,1x,f9.5,1x,f9.5

ELP10-ELP21
11i3,1x,f9.5,1x,f9.5
```

Esto evita depender de separación por espacios, porque los campos enteros de
Fortran pueden aparecer pegados visualmente.

---

## Comandos

Desde `03_Source/rust`:

Generación oficial determinística:

```bash
cargo run -p elp2000-importer
```

Ese comando usa:

```text
Input:
../../06_Data/Reference/ELP2000/raw

Outputs:
../../06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs
../../06_Data/Reference/ELP2000/generated/elp2000_manifest.json
```

Verificación explícita:

```bash
cargo run -p elp2000-importer -- verify ../../06_Data/Reference/ELP2000/raw
```

Generación explícita de coeficientes y manifiesto:

```bash
cargo run -p elp2000-importer -- generate-all \
  ../../06_Data/Reference/ELP2000/raw \
  ../../06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs \
  ../../06_Data/Reference/ELP2000/generated/elp2000_manifest.json
```

---

## Estado de Aceptación

Esta infraestructura se considerará cerrada cuando:

- los 36 archivos oficiales estén disponibles localmente: ✅
- `verify` confirme todos los conteos oficiales: ✅
- la generación produzca siempre el mismo archivo Rust: ✅
- la generación produzca siempre el mismo manifiesto JSON: ✅
- el manifiesto registre la huella del archivo Rust generado: ✅
- quede registrado el total de registros importados y las huellas de fuente: ✅
- el archivo generado sea revisado antes de conectarlo con `AST-L002`: ✅

`AST-L002` queda desbloqueado desde el punto de vista de datos, pero todavía
requiere resolver la semántica de evaluación de `ELP1` antes de escribir el
evaluador lunar.
