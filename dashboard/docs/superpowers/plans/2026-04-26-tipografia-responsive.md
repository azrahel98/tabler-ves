# Tipografía Responsive — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reemplazar el sistema tipográfico actual (root con `vw`, tokens duplicados `text-theme-*`, escala incompleta) por una escala unificada con root por breakpoints discretos y títulos con `clamp` controlado.

**Architecture:** Edición localizada de `src/style.css` (variables del bloque `@theme` y selector `html`) más migración de las únicas 2 ocurrencias de `text-theme-*` fuera del CSS (en `Header.vue`). El sistema actual ya define variables CSS consumidas por Tailwind v4 vía `--text-*`, por lo que cambiar valores no requiere tocar los componentes que ya usan `text-xs`, `text-sm`, `text-title-*`, etc.

**Tech Stack:** Vue 3 + Vite, Tailwind CSS v4 (sintaxis `@theme`), TypeScript (sin runner de tests configurado — verificación vía `npm run build` y `grep`).

**Spec de referencia:** `docs/superpowers/specs/2026-04-26-tipografia-responsive-design.md`

---

## File Structure

**Archivos a modificar:**
- `src/style.css` — único archivo con tokens tipográficos. Cambian: variables `--text-*` y `--text-theme-*` (líneas 67-82), referencias internas a `text-theme-*` (líneas 232, 264, 418, 498, 502), y selector `html` (línea 328).
- `src/components/Header/Header.vue` — únicas 2 ocurrencias de `text-theme-sm` en componentes (líneas 79 y 105).

**Archivos NO modificados** (sus valores cambian automáticamente al actualizar las variables CSS): los ~20 archivos que usan `text-xs`, `text-sm`, `text-xl`, `text-2xs`, `text-3xs`, `text-title-*`.

---

### Task 1: Cambiar el root HTML a breakpoints discretos

**Files:**
- Modify: `src/style.css:327-332`

- [ ] **Step 1: Verificar el estado actual del selector `html`**

Run:
```bash
grep -n "font-size: clamp" src/style.css
```
Expected: una línea coincidente — `328:    font-size: clamp(11px, 0.5vw + 7px, 13px);`

- [ ] **Step 2: Reemplazar el selector `html`**

Editar `src/style.css`. Buscar el bloque (líneas ~327-332):

```css
  html {
    font-size: clamp(11px, 0.5vw + 7px, 13px);
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-rendering: optimizeLegibility;
  }
```

Reemplazar por:

```css
  html {
    font-size: 12px;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-rendering: optimizeLegibility;
  }

  @media (min-width: 640px) {
    html { font-size: 13px; }
  }
  @media (min-width: 1280px) {
    html { font-size: 13px; }
  }
  @media (min-width: 2000px) {
    html { font-size: 14px; }
  }
```

- [ ] **Step 3: Verificar que la sustitución es correcta**

Run:
```bash
grep -n "font-size: clamp" src/style.css
```
Expected: sin coincidencias (output vacío).

Run:
```bash
grep -nE "font-size: 1[234]px" src/style.css
```
Expected: cuatro líneas coincidentes (12px, 13px, 13px, 14px).

- [ ] **Step 4: Commit**

```bash
git add src/style.css
git commit -m "feat(style): root HTML por breakpoints discretos"
```

---

### Task 2: Reemplazar el bloque de variables `--text-*` y `--text-theme-*`

**Files:**
- Modify: `src/style.css:67-82`

- [ ] **Step 1: Verificar las variables actuales**

Run:
```bash
grep -nE "^\s*--text-(title-|theme-|2xs|3xs|xs|sm|xl)" src/style.css
```
Expected: 11 líneas coincidentes (las variables existentes en el bloque `@theme`).

- [ ] **Step 2: Reemplazar el bloque de tokens tipográficos**

Editar `src/style.css`. Buscar el bloque actual (líneas ~67-82, después de la sección de `--breakpoint-*`):

```css
  --text-title-xl: clamp(1.5rem, 1.8vw, 2rem);    
  --text-title-lg: clamp(1.25rem, 1.4vw, 1.6rem);  
  --text-title-md: clamp(1.1rem, 1.1vw, 1.18em);  
  --text-title-sm: clamp(1rem, 1.05vw, 1.15rem);   

  
  --text-xl: clamp(0.95rem, 1vw, 1.1rem);          
  --text-sm: clamp(0.8rem, 0.75vw, 0.85rem);       
  --text-xs: clamp(0.75rem, 0.7vw, 0.8rem);        
  
  
  --text-theme-sm: clamp(0.825rem, 0.8vw, 0.9rem);
  --text-theme-xs: clamp(0.75rem, 0.7vw, 0.8rem);
  --text-2xs: clamp(0.675rem, 0.6vw, 0.75rem);     
  --text-3xs: clamp(0.575rem, 0.55vw, 0.65rem);    
```

Reemplazar por:

```css
  /* Títulos: clamp con rangos estrechos (estables entre laptop y 4K) */
  --text-title-xl: clamp(1.5rem, 1.2rem + 0.45vw, 1.875rem);
  --text-title-lg: clamp(1.25rem, 1rem + 0.35vw, 1.5rem);
  --text-title-md: clamp(1.125rem, 0.95rem + 0.25vw, 1.25rem);
  --text-title-sm: clamp(1rem, 0.85rem + 0.2vw, 1.125rem);

  /* Escala unificada de cuerpo (rem puro, escala con el root) */
  --text-3xs:  0.625rem;
  --text-2xs:  0.6875rem;
  --text-xs:   0.75rem;
  --text-sm:   0.8125rem;
  --text-base: 0.875rem;
  --text-md:   1rem;
  --text-lg:   1.125rem;
  --text-xl:   1.25rem;
  --text-2xl:  1.5rem;
  --text-3xl:  1.875rem;
```

Notar que las variables `--text-theme-*` se eliminan por completo.

- [ ] **Step 3: Verificar que `--text-theme-*` ya no existe**

Run:
```bash
grep -n "\-\-text-theme-" src/style.css
```
Expected: sin coincidencias.

- [ ] **Step 4: Verificar que las nuevas variables están presentes**

Run:
```bash
grep -nE "^\s*--text-(base|md|lg|2xl|3xl):" src/style.css
```
Expected: 5 líneas coincidentes (las nuevas).

- [ ] **Step 5: Commit**

```bash
git add src/style.css
git commit -m "feat(style): escala tipografica unificada y rangos de titulos estrechos"
```

---

### Task 3: Actualizar referencias internas a `text-theme-*` en `style.css`

**Files:**
- Modify: `src/style.css:232,264,418,498,502`

- [ ] **Step 1: Listar las referencias actuales**

Run:
```bash
grep -nE "text-theme-(xs|sm)" src/style.css
```
Expected: 5 líneas coincidentes:
- 232: `@apply text-theme-sm relative ...` (utility `menu-item`)
- 264: `@apply text-theme-sm relative ...` (utility `menu-dropdown-item`)
- 418: `@apply text-theme-xs! text-gray-700! ...` (`.apexcharts-tooltip-text`)
- 498: `@apply text-theme-sm! font-medium! ...` (`.flatpickr-weekday`)
- 502: `@apply text-theme-sm! flex! ...` (`.flatpickr-day`)

- [ ] **Step 2: Reemplazar `text-theme-sm` → `text-sm` en las 4 ocurrencias**

Aplicar Edit con `replace_all` sobre `src/style.css`:
- old_string: `text-theme-sm`
- new_string: `text-sm`

(Hay 4 ocurrencias de `text-theme-sm` y 1 de `text-theme-xs` en este archivo. El paso 2 atiende solo `text-theme-sm`.)

- [ ] **Step 3: Reemplazar `text-theme-xs` → `text-xs`**

Aplicar Edit:
- old_string: `text-theme-xs!`
- new_string: `text-xs!`

(Es una única ocurrencia con `!`; no usar `replace_all` para evitar tocar otras formas.)

- [ ] **Step 4: Verificar que ya no quedan `text-theme-*` en `style.css`**

Run:
```bash
grep -nE "text-theme-(xs|sm)" src/style.css
```
Expected: sin coincidencias.

- [ ] **Step 5: Commit**

```bash
git add src/style.css
git commit -m "refactor(style): migrar text-theme-* internos a la escala unificada"
```

---

### Task 4: Migrar `text-theme-sm` en `Header.vue`

**Files:**
- Modify: `src/components/Header/Header.vue:79,105`

- [ ] **Step 1: Confirmar las ocurrencias**

Run:
```bash
grep -n "text-theme-" src/components/Header/Header.vue
```
Expected: 2 líneas — 79 y 105, ambas con `text-theme-sm`.

- [ ] **Step 2: Reemplazar todas las ocurrencias**

Aplicar Edit con `replace_all` sobre `src/components/Header/Header.vue`:
- old_string: `text-theme-sm`
- new_string: `text-sm`

- [ ] **Step 3: Verificar el resultado**

Run:
```bash
grep -n "text-theme-" src/components/Header/Header.vue
```
Expected: sin coincidencias.

- [ ] **Step 4: Commit**

```bash
git add src/components/Header/Header.vue
git commit -m "refactor(header): migrar text-theme-sm a text-sm"
```

---

### Task 5: Verificación global (grep + build + visual)

**Files:** ninguno modificado en esta tarea.

- [ ] **Step 1: Verificar que no queda `text-theme-*` en todo el proyecto**

Run:
```bash
grep -rnE "text-theme-(xs|sm)" src/
```
Expected: sin coincidencias en ningún archivo.

- [ ] **Step 2: Verificar que no queda `--text-theme-*`**

Run:
```bash
grep -rn "\-\-text-theme-" src/
```
Expected: sin coincidencias.

- [ ] **Step 3: Verificar que el root usa media queries discretas**

Run:
```bash
grep -nE "html \{ font-size:" src/style.css
```
Expected: 3 líneas (las dentro de `@media`). El selector inicial sin media query también debe existir; verificarlo:

Run:
```bash
grep -nE "font-size: 12px;" src/style.css
```
Expected: al menos una línea coincidente (root móvil).

- [ ] **Step 4: Compilar el proyecto (type-check + build)**

Run:
```bash
npm run build
```
Expected: build exitoso, sin errores de TypeScript ni de Tailwind. Si Tailwind reporta clases desconocidas relacionadas con tipografía, revisar que las variables `--text-*` correspondientes estén en el bloque `@theme`.

- [ ] **Step 5: Sanity check visual**

Levantar el dev server:
```bash
npm run dev
```

Abrir en el navegador a tamaño 1444×770 (DevTools → Responsive) y verificar:
- `DashboardView`: el `<h2>Dashboard</h2>` con `text-title-xl` se ve ~22px (antes ~26px). Las tarjetas de métrica con números grandes (`text-title-lg`) se ven ~18px.
- `Sidebar`: ítems del menú (`menu-item` → `text-sm`) legibles, no demasiado pequeños.
- `Header`: nombre del usuario (`text-sm`) y dropdown items legibles.
- `PersonalProfile`: títulos de pestañas y contenido proporcionados, sin desbordes.

Probar también:
- Anchos 375px (móvil), 1024px (tablet/laptop chica), 2560px (4K si está disponible).
- En móvil el cuerpo debe verse compacto (root 12px) pero legible.
- En 4K los títulos deben subir un par de px sin desbordar layout.

- [ ] **Step 6: Commit final si hubo ajustes visuales**

Si los pasos visuales requirieron retoques (tamaño concreto fuera de rango aceptable), aplicarlos y commitear. Si no, esta tarea no produce commit propio.

```bash
git status
```

Expected: working tree clean (asumiendo que no hubo retoques).

---

## Self-review

**Spec coverage:**
- Sección 1 (root) → Task 1 ✓
- Sección 2 (escala unificada de cuerpo) → Task 2 ✓
- Sección 3 (títulos con clamp) → Task 2 (mismas variables) ✓
- Sección 4 (migración) → Tasks 3 y 4 ✓
- Sección 5 (verificación visual) → Task 5 ✓
- Criterios de aceptación → Task 5 (pasos 1-5) ✓

**Placeholder scan:** sin TBD / TODO / "implementar después". Cada paso tiene comando concreto o código completo.

**Type / referencia consistency:** los nombres de tokens en Task 2 (`--text-base`, `--text-md`, `--text-lg`, `--text-2xl`, `--text-3xl` y los `--text-title-*`) coinciden con lo escrito en el spec sección 2 y 3, y los reemplazos en Tasks 3-4 (`text-sm`, `text-xs`) usan exactamente los nombres definidos en Task 2.

**Notas de ejecución:**
- Existen muchos archivos modificados en el working tree (estado pre-existente del usuario). Cada tarea hace `git add` solo de los archivos que tocó esa tarea, no `git add -A`.
- No hay runner de tests; la verificación es `grep` + `npm run build` + sanity visual.
