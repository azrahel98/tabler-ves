# Guía de Diseño y Tipografía (`tabler-ves`)

Este documento define las convenciones tipográficas y la relación por defecto entre **labels** (etiquetas) y **datos** (valores) para todo el frontend en `dashboard/`, tomando como referencia el sistema implementado en `PerfilView` (`.perfil-scope`) y los tokens de `dashboard/src/styles/main.css`.

---

## 1. Escala Tipográfica (`dashboard/src/styles/main.css`)

El proyecto utiliza tokens semánticos en `@theme` y una escala compacta de alta densidad en `.perfil-scope`:

| Token | Global (`@theme`) | Compacto (`.perfil-scope`) | Uso principal |
| :--- | :--- | :--- | :--- |
| `--text-nano` | `0.625rem` (10px) | `0.5625rem` (9px) | Únicamente contadores en píldoras/badges mínimos o iniciales de avatar pequeño. **No usar para labels de datos.** |
| `--text-micro` | `0.6875rem` (11px) | `0.625rem` (10px) | Labels en mayúsculas de KPI/resumen, cabeceras de tabla (`<th>`), textos secundarios, fechas/metadatos auxiliares y pestañas. |
| `--text-meta` | `0.75rem` (12px) | `0.6875rem` (11px) | **Tamaño base de datos/valores** en filas clave-valor, celdas numéricas/principales, inputs y descripciones. |
| `--text-xs` | `0.8125rem` (13px) | `0.75rem` (12px) | Títulos de tarjetas (`Card`), títulos de secciones y cabeceras de drawers/modales. |
| `--text-sm` | `0.9375rem` (15px) | `0.875rem` (14px) | Subtítulos destacados o encabezados secundarios. |
| `--text-title` | `1rem` (16px) | `0.9375rem` (15px) | Títulos principales de página (`PageHeader`). |
| `--text-kpi` | `1.25rem` (20px) | `1.125rem` (18px) | Cifras KPI principales de alto impacto (e.g. totales de dona/tarjetas resumen). |

---

## 2. Relación por Defecto: Labels vs. Datos

Para mantener consistencia visual en todas las vistas y futuros componentes, **siempre** se debe respetar la siguiente relación entre etiqueta y valor:

### A. Tiras de Resumen / Mini-KPIs (ej. `PerfilVinculoActualCard`, `CardPersonalPorArea`)
El dato siempre está **1 nivel por encima** del label y con mayor peso visual:
- **Label**: `text-micro font-semibold uppercase tracking-wider text-muted-foreground`
- **Dato**: `text-meta font-semibold text-foreground mt-0.5` *(añadir `font-mono tabular-nums` para cifras, montos, códigos o fechas)*

### B. Filas Clave-Valor con Icono (ej. `PerfilVinculoActualCard`, `PerfilDocumentoDrawer`, `PerfilInfoContacto`)
El label y el dato comparten el **mismo tamaño (`text-meta`)**, diferenciándose por color y peso tipográfico:
- **Contenedor de fila**: `grid grid-cols-[7.75rem_1fr] items-start gap-2.5 text-meta` (o `flex items-center gap-3 text-meta`)
- **Label**: `text-muted-foreground flex items-center gap-1.5 shrink-0` con icono `size-3.5 text-muted-foreground shrink-0`
- **Dato**: `font-medium text-foreground` *(o `font-mono font-medium text-foreground tabular-nums` para fechas, DNI, plazas y códigos)*

### C. Tablas de Datos (ej. `PerfilHistorialVinculosCard`, `PerfilBancoCard`)
- **Cabecera (`<thead>` / `<th>`)**: `text-micro font-medium text-muted-foreground`
- **Celda principal / numérica**: `text-meta font-medium text-foreground` (`font-mono tabular-nums` en montos, cuentas y códigos)
- **Celda secundaria compacta**: `text-micro text-foreground` o `text-muted-foreground`

### D. Cabeceras de Tarjetas y Secciones
- **Contenedor**: `flex items-center justify-between gap-2 border-b border-border pb-2`
- **Título**: `text-xs font-semibold text-foreground tracking-tight` acompañado de icono `size-3.5 text-primary shrink-0`

---

## 3. Reglas para Futuros Componentes

1. **Usar siempre los tokens semánticos** (`text-micro`, `text-meta`, `text-xs`, `text-sm`, `text-title`, `text-kpi`). Evitar tamaños arbitrarios (`text-[...]`) o escalas genéricas (`text-base`, `text-lg`, `text-xl`).
2. **En vistas de ficha, detalle o alta densidad de datos**, envolver el contenedor raíz con `.perfil-scope` para heredar la escala compacta (`10px` label / `11px` dato / `12px` título).
3. **Nunca usar `text-nano` para labels de KPI o formularios**; el tamaño mínimo legible para etiquetas es `text-micro`.
4. **Alineación numérica**: todo valor monetario, porcentaje, fecha o código identificador (`DNI`, `AIRHSP`, `CCI`, `ID`) debe llevar `font-mono tabular-nums`.
