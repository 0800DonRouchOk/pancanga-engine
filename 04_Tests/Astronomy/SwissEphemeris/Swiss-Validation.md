# Swiss Ephemeris Validation

Campaign: 46A

Status: ✅ PASS

## Objetivo

Incorporar la infraestructura de validación astronómica contra Swiss Ephemeris
sin modificar Pancanga Engine.

Esta infraestructura certifica el motor. No redefine el motor.

## Estado Local

```text
swetest:
DISPONIBLE LOCALMENTE

Versión:
2.10.03

Binario:
04_Tests/Astronomy/SwissEphemeris/bin/swetest

Comparador:
EJECUTADO

CSV:
GENERADO CON 1000 CASOS REALES

Código del motor:
AST-L006 IMPLEMENTADO

Knowledge Base:
SIN CAMBIOS

Arquitectura:
NO
```

## Runner

El adaptador local está en:

```text
03_Source/rust/crates/pancanga-engine/examples/swiss_validation.rs
```

Uso esperado desde `03_Source/rust`:

```text
cargo run -p pancanga-engine --example swiss_validation -- \
  --output ../../04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv \
  --samples 1000
```

Si `swetest` no está disponible, el runner no inventa resultados. Genera un CSV
con estado:

```text
EXTERNAL_ORACLE_PENDING
```

Si `swetest` está disponible, el runner compara automáticamente:

```text
Julian Day
↓
Sun Longitude
↓
Moon Longitude
↓
Elongation
↓
Tithi
```

## Clasificación De Diferencias

El comparador solo usa estas clasificaciones:

```text
PASS
ASTRONOMY DIFFERENCE
ENGINE BUG
EXTERNAL TOOL ERROR
```

La clasificación `ENGINE BUG` se reserva para la revisión humana posterior a la
medición. El runner automático puede producir `PASS`, `ASTRONOMY DIFFERENCE` o
`EXTERNAL TOOL ERROR`.

## CSV

Archivo:

```text
04_Tests/Astronomy/SwissEphemeris/swiss-validation.csv
```

Columnas:

```text
case_id
julian_day
pancanga_sun_longitude
swiss_sun_longitude
sun_error_degrees
pancanga_moon_longitude
swiss_moon_longitude
moon_error_degrees
pancanga_elongation
swiss_elongation
elongation_error_degrees
pancanga_tithi
swiss_tithi
difference_classification
status
notes
```

## Instalación Local Ejecutada

Homebrew no pudo utilizarse dentro del sandbox porque requería escritura en
carpetas externas al proyecto. Se usó la fuente oficial indicada por
Astrodienst:

```text
https://github.com/aloistr/swisseph
```

Se compiló `swetest` para macOS arm64 y se copió el binario a:

```text
04_Tests/Astronomy/SwissEphemeris/bin/swetest
```

Verificación:

```text
swetest -h
Version: 2.10.03
```

## Instalación De `swetest`

### macOS

Opción recomendada:

```text
brew install swiss-ephemeris
```

Verificación:

```text
swetest -h
```

Si el ejecutable no queda en `PATH`, configurar:

```text
SWETEST=/ruta/a/swetest
```

### Linux

Opciones habituales:

```text
sudo apt install swisseph
```

o compilar Swiss Ephemeris desde el paquete oficial de Astrodienst.

Verificación:

```text
swetest -h
```

Si el ejecutable no queda en `PATH`, configurar:

```text
SWETEST=/ruta/a/swetest
```

### Windows

Instalar Swiss Ephemeris desde el paquete oficial de Astrodienst y agregar el
directorio de `swetest.exe` al `PATH`.

Alternativamente, ejecutar el runner con:

```text
SWETEST=C:\ruta\a\swetest.exe
```

## Configuración Técnica Inicial

El runner usa `swetest` con ruta de efemérides explícita cuando está
disponible:

```text
swetest -bj<JD> -p<planet> -fPl -g, -head -eswe -edir<ephe>
```

Planetas:

```text
0 = Sun
1 = Moon
```

Archivos locales de efemérides usados para la muestra 1900-2100:

```text
04_Tests/Astronomy/SwissEphemeris/ephe/sepl_18.se1
04_Tests/Astronomy/SwissEphemeris/ephe/semo_18.se1
```

La muestra predeterminada contiene 1000 casos entre:

```text
JD 2415020.5
JD 2488434.5
```

La tolerancia inicial del runner es:

```text
0.01 grados
```

Esta tolerancia es operativa para infraestructura. La tolerancia científica
oficial debe congelarse después de medir el impacto calendárico real.

## Regla Metodológica

```text
La validación externa certifica el motor.

No redefine el motor.

Toda discrepancia debe documentarse antes de modificar cualquier línea de
código.
```

## Resultado De La Primera Comparación

Muestra:

```text
1000 casos
JD 2415020.5 a 2488434.5
```

Resumen:

| Métrica | Resultado |
| ------- | --------- |
| Filas comparadas | 1000 |
| PASS automático | 7 |
| ASTRONOMY DIFFERENCE | 993 |
| EXTERNAL TOOL ERROR | 0 |
| Tithi mismatches | 56 |
| Error solar máximo | 0.000160884533 grados |
| Error lunar máximo | 1.410719337432 grados |
| Error de elongación máximo | 1.410783648046 grados |

La comparación demuestra que la infraestructura funciona y que ya no hay
`External Oracle Pending` para Swiss. Las diferencias astronómicas deben
analizarse antes de modificar cualquier algoritmo.

## Resultado Después De AST-L006

Campaign 46A.4 implementó la transformación de marco ELP2000 especificada en
AST-L006 y repitió los 1000 casos usando ruta de efemérides explícita.

| Métrica | Resultado |
| ------- | --------- |
| Filas comparadas | 1000 |
| PASS automático | 1000 |
| ASTRONOMY DIFFERENCE | 0 |
| EXTERNAL TOOL ERROR | 0 |
| Tithi mismatches | 0 |
| Error solar máximo | 0.000167784533 grados |
| Error lunar máximo | 0.000469445269 grados |
| Error de elongación máximo | 0.000330443528 grados |

## Criterio De Cierre

Campaign 46A queda en:

```text
PASS
```

La certificación astronómica contra Swiss Ephemeris queda aprobada para la
muestra definida:

```text
Swiss:
PASS

Tithi mismatches:
0
```

Campaign 46A.2 analizó la diferencia lunar. Documento:

```text
04_Tests/Astronomy/SwissEphemeris/Swiss-Difference-Analysis.md
```

Conclusión:

```text
MODEL DIFFERENCE / REFERENCE FRAME
```

La Luna de Pancanga coincide mucho mejor con Swiss cuando Swiss se fuerza a
`-j2000`, mientras que el Sol coincide con Swiss en longitudes de fecha. Esto
indica una mezcla de marcos de referencia antes de la comparación de
elongación/tithi.

Los coeficientes ELP2000 y el intérprete no fueron modificados.
