---
trigger: model_decision
description: cuando se cree,edite componentes
---

You are an assistant that creates Vue 3 components (Composition API with <script setup>) for the Tabler VES application — an HR management system. Every component you generate must strictly follow the design system documented below.

TECH STACK

Vue 3 + TypeScript, <script setup lang="ts">
Tailwind CSS (no arbitrary classes except dark mode opacity: dark:bg-white/[0.03])
Pinia (storeToRefs for reactivity)
lucide-vue-next (icons)
vue-chartjs + Chart.js (charts)
date-fns + date-fns/locale/es (dates in Spanish)
Vue Router (RouterLink for navigation)

CARDS / PANELS (card base)
Every panel uses exactly this base class:
rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/[0.03] md:p-6

No box-shadow on cards. Border only.
The DataTable table uses dark:bg-gray-900 instead of dark:bg-white/[0.03].

TYPOGRAPHY
RoleClassesPanel titletext-lg font-semibold text-gray-800 dark:text-white/90Subtitle / descriptiontext-sm text-gray-500 dark:text-gray-400Main value (KPI)text-2xl font-bold text-gray-800 dark:text-white/90Field labeltext-sm font-medium text-gray-800 dark:text-white/90Secondary texttext-xs text-gray-500 dark:text-gray-400Tertiary / meta texttext-[10px] text-gray-400Visual separator ·text-gray-300 dark:text-gray-600 with &middot;

BADGES / PILLS
Base pattern:
inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ring-1 ring-inset
Color variants (light / dark):

Blue: bg-blue-50 text-blue-700 ring-blue-600/20 dark:bg-blue-500/10 dark:text-blue-400 dark:ring-blue-500/20
Green: bg-green-50 text-green-700 ring-green-600/20 dark:bg-green-500/10 dark:text-green-400 dark:ring-green-500/20
Amber: bg-amber-50 text-amber-700 ring-amber-600/20 dark:bg-amber-500/10 dark:text-amber-400 dark:ring-amber-500/20
Red: bg-red-50 text-red-700 ring-red-600/20 dark:bg-red-500/10 dark:text-red-400 dark:ring-red-500/20
Purple: bg-purple-50 text-purple-700 ring-purple-600/20 dark:bg-purple-500/10 dark:text-purple-400 dark:ring-purple-500/20
Gray: bg-gray-50 text-gray-700 ring-gray-600/20 dark:bg-gray-500/10 dark:text-gray-400 dark:ring-gray-500/20

Smaller badge (secondary state): replace px-2.5 with px-2 and use text-[10px].

ICONS IN KPI CARDS
Icon container:
flex h-11 w-11 items-center justify-center rounded-xl bg-{color}-50 dark:bg-{color}-500/10
Icon inside:
h-5 w-5 text-{color}-600 dark:text-{color}-400
Semantic colors used: blue (total), green (active), amber (new), red (resignations/alerts), purple (events).

INTERACTIVE ROWS (lists with RouterLink)
flex items-center justify-between rounded-lg px-2 py-2.5 transition-colors hover:bg-gray-50 dark:hover:bg-white/5
For rows with bottom border (table style):
flex items-center justify-between border-b border-gray-100 py-3 dark:border-gray-800 last:border-none transition-colors
Hover on dense tables: hover:bg-gray-50 dark:hover:bg-gray-800/50

PANEL HEADER WITH COUNTER
html<div class="flex items-center justify-between">

  <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Title</h3>
  <span class="inline-flex items-center rounded-full bg-{color}-50 px-2.5 py-0.5 text-xs font-medium text-{color}-700 ring-1 ring-inset ring-{color}-600/20 dark:bg-{color}-500/10 dark:text-{color}-400 dark:ring-{color}-500/20">
    {{ count }}
  </span>
</div>
```

---

## SCROLLABLE LISTS

```
mt-4 min-h-72 max-h-96 space-y-1 overflow-y-auto
```

No `space-y-1` when rows use `border-b`.

---

## DIVIDERS / SEPARATORS

- Between sections: `border-b border-gray-100 dark:border-gray-800`
- Between panels in grid: gap managed by the grid (`gap-4 md:gap-6`)
- Inline table column header: `border-b border-gray-100 pb-2 dark:border-gray-800` with `text-theme-xs text-gray-400`

---

## LAYOUT GRIDS

| Use                  | Classes                                                         |
| -------------------- | --------------------------------------------------------------- |
| KPI cards (4 col)    | `grid grid-cols-2 gap-4 lg:grid-cols-4 md:gap-6`                |
| Donut charts (3 col) | `grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 md:gap-6` |
| Lists (2 col)        | `grid grid-cols-1 gap-4 lg:grid-cols-2 md:gap-6`                |

---

## CHARTS (Chart.js / vue-chartjs)

- Register only necessary elements (tree-shaking).
- Always `responsive: true`, `maintainAspectRatio: false`.
- Chart.js legend: `display: false` — use custom legend with color dots.
- Chart grids: `color rgba(156,163,175,0.15)`, border disabled.
- Ticks: `color: '#9ca3af'`, `font: { size: 11 }`.
- Bar borders: `borderRadius: 6`, `borderSkipped: false`, `borderWidth: 0`.
- Chart container: `relative h-{n} w-full` (explicit fixed height).

---

## INPUTS AND FORMS

Base input:

```
w-full h-12 pl-{n} pr-4 text-sm text-gray-900 bg-white border-[1.5px] border-gray-200 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-brand-400 focus:ring-4 focus:ring-brand-500/10 dark:bg-white/[0.04] dark:border-white/10 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-brand-500 dark:focus:ring-brand-500/15
```

Compact input (in DataTable):

```
rounded-lg border border-gray-200 bg-gray-50 py-2 pl-8 pr-3 text-sm focus:border-brand-300 focus:ring-2 focus:ring-brand-500/20 dark:border-gray-700 dark:bg-gray-800 dark:text-white
```

---

## BUTTONS

Primary (main action):

```
rounded-xl bg-gradient-to-br from-brand-500 to-brand-600 text-white text-sm font-semibold shadow-lg shadow-brand-500/35 transition-all duration-300 hover:shadow-xl hover:shadow-brand-500/45 hover:-translate-y-0.5 active:translate-y-0 disabled:opacity-70 disabled:cursor-not-allowed
```

Secondary / ghost (dark mode toggle, pagination):

```
rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors
Active pagination: bg-brand-500 text-white

BRAND COLOR
The main accent color is brand-500 / brand-600. Used in:

Input focus rings
Primary button
Active pagination
Active sort icons in DataTable
Form labels on focus


GENERAL RULES

Always implement dark mode with the dark: prefix.
No box-shadow on cards, border only.
Truncate long names with truncate and min-w-0 flex-1 on the container.
Use shrink-0 on badges/elements that must not compress.
Dates always in Spanish using date-fns/locale/es.
Icons always from lucide-vue-next.
Navigation to employee profile: { name: 'personal-profile', params: { dni: x.dni } }.
The main store is useTableroStore in ../../stores/dashboard.
No obvious comments. Only where the logic is not self-evident.
Do not create helpers or abstractions for single use.
```
