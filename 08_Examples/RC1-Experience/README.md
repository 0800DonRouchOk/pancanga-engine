# Pancanga Engine RC1 Experience

Primera aplicación privada mínima para usar Pancanga Engine como producto diario.

## Ejecutar

Desde `03_Source/rust`:

```bash
cargo run -p pancanga-engine --example rc1_experience
```

Luego abrir:

```text
http://127.0.0.1:7878
```

Si el puerto quedó ocupado por una ejecución anterior:

```bash
RC1_EXPERIENCE_ADDRESS=127.0.0.1:7879 cargo run -p pancanga-engine --example rc1_experience
```

Y abrir:

```text
http://127.0.0.1:7879
```

## Alcance

Esta beta no modifica el motor, la Knowledge Base ni la arquitectura.

Muestra únicamente:

- qué observar hoy;
- Parāṇa recomendado, cuando existe una presentación práctica calculable;
- ventana normativa de Parāṇa;
- fundamento;
- astronomía;
- certificación.

## Testing Release

La pantalla muestra explícitamente:

```text
Pancanga Engine RC1 · Testing Release.
No usar como único calendario para observancias importantes hasta v1.0.
```

También incluye un botón:

```text
Reportar un problema
```

Ese botón genera un resumen reproducible con ciudad, fecha, resultado visible,
Parāṇa y datos técnicos para pegar en un correo, GitHub Issue, Google Form o
canal privado de testers.

## Zero Cost Deploy para testers

Guía:

[RC1 Experience Deployment](DEPLOYMENT.md)

Recomendación:

```text
Frontend:
Cloudflare Pages Free

Backend:
Render Free ejecutando rc1_experience
```

La página puede apuntar a un backend remoto editando:

```text
rc1-config.js
```

Ejemplo:

```js
window.RC1_API_BASE = "https://pancanga-engine-rc1-api.onrender.com";
```

La pantalla muestra metadatos de build para que las capturas de testers sean
auditables:

```text
Engine: v1.0 RC1
Knowledge Base: HBV v1.0
Astronomy: Swiss Certified
Build: 2026.07 RC1
```

## Parāṇa

La RC1 Experience separa dos capas:

- **Recomendado**: una ventana práctica de presentación. En el caso PureBhakti estudiado, el límite coincide con el primer tercio del día civil.
- **Límite normativo**: el cierre de la ventana calculada por `HBV-EK-005`, desde el final de Hari-vāsara hasta el final de Dvādaśī.

Esta distinción no modifica el motor ni la Knowledge Base.

## Limitación RC1

La pantalla RC1 consume reglas del Motor Vaiṣṇava, pero todavía no deriva
automáticamente todos los hechos de Mahādvādaśī basados en tithi. Si aparece un
caso como Triṣpṛśā, la beta debe tratarlo como siguiente mejora de experiencia,
no como bug del motor.
