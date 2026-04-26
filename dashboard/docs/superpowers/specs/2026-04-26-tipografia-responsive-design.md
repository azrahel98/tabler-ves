# Tipografía responsive y coherente

**Fecha:** 2026-04-26
**Tema:** Rediseño del sistema tipográfico del dashboard
**Estado:** Aprobado por el usuario, listo para plan de implementación

## Contexto y problema

El sistema tipográfico actual en `src/style.css` presenta tres problemas:

1. **Root inestable.** `html { font-size: clamp(11px, 0.5vw + 7px, 13px) }` usa `vw`, lo que produce escalado impredecible. En la mayoría de monitores (1280px–2560px) se queda fijo en 13px (clampeado), pero el cálculo dependiente de `vw` no aporta valor real y sí complica el razonamiento.

2. **Títulos demasiado grandes en pantallas 1444×770.** Las variables `--text-title-*` usan `clamp(min, Xvw, max)`. En 1444px de ancho, `text-title-xl` evalúa a `1.8vw = ~26px`, lo que el usuario percibe como "muy grande" en una laptop estándar.

3. **Tokens duplicados sin lógica clara.** Conviven dos sistemas paralelos:
   - `text-xs`, `text-sm`, `text-xl`, `text-2xs`, `text-3xs`
   - `text-theme-xs`, `text-theme-sm`

   Con valores casi idénticos (`text-xs: 0.75-0.8rem` vs `text-sm: 0.8-0.85rem`), los desarrolladores no distinguen cuándo usar uno u otro. ~310 ocurrencias repartidas en 20+ vistas.

4. **Faltan tamaños intermedios.** No existen `text-base`, `text-md`, `text-lg`, `text-2xl`, `text-3xl` en la escala normal porque `--font-*: initial` desactiva los defaults de Tailwind.

## Decisiones tomadas (con el usuario)

- **Tamaño objetivo:** mediano-pequeño tipo GitHub / Linear (cuerpo ~13px en desktop, H1 ~22-24px).
- **Estrategia de escalado:** híbrida.
  - **Cuerpo:** root fijo por breakpoints discretos, sin `vw`.
  - **Títulos:** `clamp` con rangos estrechos para variación suave entre desktop y 4K.
- **Tokens:** unificación en una sola escala completa, eliminando `text-theme-*`.

## Diseño

### 1. Root HTML (escalado por breakpoints)

```css
html {
  font-size: 12px;
}
@media (min-width: 640px)  { html { font-size: 13px; } }
@media (min-width: 1280px) { html { font-size: 13px; } }
@media (min-width: 2000px) { html { font-size: 14px; } }
```

Razón: predecible y revisable. Sin dependencia de `vw`, los `rem` de los tokens producen px estables.

### 2. Escala unificada de cuerpo

Definida con `rem` puro, dentro del bloque `@theme` de Tailwind v4.

| Token       | Valor      | Px en desktop (root 13px) | Uso                            |
|-------------|------------|---------------------------|--------------------------------|
| `text-3xs`  | 0.625rem   | ~8px                      | Badges micro, etiquetas duras  |
| `text-2xs`  | 0.6875rem  | ~9px                      | Badges, tags                   |
| `text-xs`   | 0.75rem    | ~10px                     | Captions, helper text          |
| `text-sm`   | 0.8125rem  | ~10.5px                   | Texto secundario, labels       |
| `text-base` | 0.875rem   | ~11.5px                   | Cuerpo por defecto             |
| `text-md`   | 1rem       | 13px                      | Cuerpo destacado               |
| `text-lg`   | 1.125rem   | ~14.5px                   | Subtítulos pequeños            |
| `text-xl`   | 1.25rem    | ~16px                     | Sección/heading menor          |
| `text-2xl`  | 1.5rem     | ~19.5px                   | Heading mediano                |
| `text-3xl`  | 1.875rem   | ~24px                     | Heading grande                 |

### 3. Títulos con `clamp` controlado

| Token            | Valor                                          | Px en 1444×770 | Px en 4K |
|------------------|-----------------------------------------------|----------------|----------|
| `text-title-sm`  | `clamp(1rem, 0.85rem + 0.2vw, 1.125rem)`       | ~14px          | ~15px    |
| `text-title-md`  | `clamp(1.125rem, 0.95rem + 0.25vw, 1.25rem)`   | ~16px          | ~17px    |
| `text-title-lg`  | `clamp(1.25rem, 1rem + 0.35vw, 1.5rem)`        | ~18px          | ~20px    |
| `text-title-xl`  | `clamp(1.5rem, 1.2rem + 0.45vw, 1.875rem)`     | ~22px          | ~25px    |

Comparación con el sistema actual en 1444×770:
- `text-title-xl`: **~26px → ~22px**
- `text-title-lg`: **~20px → ~18px**

Razón del formato `0.85rem + 0.2vw`: la base `rem` da estabilidad (depende del root, no del viewport), y el `vw` aporta solo un delta pequeño. Variación máxima ~3px entre laptop y 4K.

### 4. Migración

**`src/style.css`:**
1. Reemplazar `font-size` del selector `html` por las cuatro media queries de la sección 1.
2. Eliminar las variables actuales: `--text-title-xl`, `--text-title-lg`, `--text-title-md`, `--text-title-sm`, `--text-xl`, `--text-sm`, `--text-xs`, `--text-theme-sm`, `--text-theme-xs`, `--text-2xs`, `--text-3xs`.
3. Insertar el bloque completo de tokens nuevos (secciones 2 y 3) dentro de `@theme`.
4. Actualizar referencias a `text-theme-*` dentro del mismo `style.css`:
   - `.apexcharts-tooltip-text`, `.apexcharts-tooltip-title`, etc.
   - `.flatpickr-weekday`, `.flatpickr-day`
   - utilidades `menu-item`, `menu-dropdown-item`, `menu-dropdown-badge`
   Reemplazar `text-theme-xs` → `text-xs` y `text-theme-sm` → `text-sm`.

**Componentes (~20 archivos en `src/`):**
- Search & replace global:
  - `text-theme-xs` → `text-xs`
  - `text-theme-sm` → `text-sm`
- Los `text-xs`, `text-sm`, `text-xl`, `text-2xs`, `text-3xs`, `text-title-*` existentes **no se renombran** — sus valores cambian al actualizar las variables.
- Los archivos modificados según `git status` ya tienen cambios pendientes; la migración se aplica encima de ellos.

### 5. Verificación visual

Tras aplicar los cambios, en una laptop 1444×770:
- Títulos de página un ~15-20% más pequeños.
- Tarjetas de métrica con números grandes pasan de ~26px a ~22px.
- Cuerpo y badges casi idénticos (variación de 0.5-1px).
- En móvil todo un poco más compacto (root baja a 12px).
- En 4K los títulos suben un par de px y el root pasa a 14px.

## Fuera de alcance

- Familias de fuente (sigue Inter Variable).
- Pesos (`font-medium`, `font-bold`, etc.).
- `line-height` y `tracking`. Si tras el cambio se ven raros, abrir tarea aparte.
- Colores, spacing, layout, breakpoints.

## Criterios de aceptación

- `style.css` no contiene la cadena `text-theme-` en ninguna parte.
- `style.css` no contiene `--text-theme-`.
- Tras `grep` en `src/`, no quedan ocurrencias de `text-theme-xs` ni `text-theme-sm`.
- El root HTML usa media queries discretas, no `clamp` con `vw`.
- Las nuevas clases `text-base`, `text-md`, `text-lg`, `text-2xl`, `text-3xl` están disponibles.
- Visual sanity check en `DashboardView`, `PersonalProfile`, `Sidebar` y `Header` a 1444×770: títulos de tarjetas legibles y proporcionados, no exceden el alto de su contenedor.
