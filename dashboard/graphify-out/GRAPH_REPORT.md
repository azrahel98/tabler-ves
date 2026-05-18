# Graph Report - dashboard  (2026-05-17)

## Corpus Check
- 98 files · ~72,810 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 540 nodes · 541 edges · 74 communities (64 shown, 10 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `801d40e5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 36|Community 36]]

## God Nodes (most connected - your core abstractions)
1. `Personal `/personal` 🔒` - 31 edges
2. `dependencies` - 17 edges
3. `compilerOptions` - 17 edges
4. `Dashboard `/dash` 🔒` - 15 edges
5. `devDependencies` - 9 edges
6. `compilerOptions` - 9 edges
7. `Fileserver `/fileserver`` - 9 edges
8. `Integración de rutas nuevas de `API.md` en el dashboard` - 9 edges
9. `Task 5: Verificación global (grep + build + visual)` - 8 edges
10. `API Endpoints` - 7 edges

## Surprising Connections (you probably didn't know these)
- `click()` --calls--> `buscar()`  [INFERRED]
  src/components/dashboard/mapa.vue → src/components/sindicato/BuscadorPersonal.vue
- `middleware()` --calls--> `useAutenticacionStore`  [EXTRACTED]
  src/router/index.ts → src/stores/auth.ts
- `middleware()` --calls--> `decodificar()`  [EXTRACTED]
  src/router/index.ts → src/stores/auth.ts

## Communities (74 total, 10 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.07
Nodes (40): campo, close(), contactoSchema, form, isSubmitting, limpiarErrores(), { perfilActual }, resultado (+32 more)

### Community 1 - "Community 1"
Cohesion: 0.04
Nodes (48): code:json ([{ "nombre": "Nombre Apellido", "dni": "12345678", "estado":), code:json ({), code:json ("Rows affected: 1"), code:json ([), code:json ({ "dni": "12345678", "estado": "inactivo" }), code:json ("Vínculo eliminado correctamente"), code:json ({ "id": 1, "numero_cuenta": "123456789", "tipo_cuenta": "AHO), code:json ([{ "id": 1, "profesion": "ABOGADO", "universidad": "UNIV.", ) (+40 more)

### Community 2 - "Community 2"
Cohesion: 0.05
Nodes (36): dependencies, axios, chart.js, date-fns, date-fns-tz, jwt-decode, leaflet, lucide-vue-next (+28 more)

### Community 3 - "Community 3"
Cohesion: 0.06
Nodes (33): code:bash (grep -n "font-size: clamp" src/style.css), code:bash (grep -n "\-\-text-theme-" src/style.css), code:bash (grep -nE "^\s*--text-(base|md|lg|2xl|3xl):" src/style.css), code:bash (git add src/style.css), code:bash (grep -nE "text-theme-(xs|sm)" src/style.css), code:bash (grep -nE "text-theme-(xs|sm)" src/style.css), code:bash (git add src/style.css), code:bash (grep -n "text-theme-" src/components/Header/Header.vue) (+25 more)

### Community 4 - "Community 4"
Cohesion: 0.07
Nodes (29): API Endpoints, code:json ({ "code": 400 | 401 | 404 | 500, "error": "Mensaje descripti), code:json ({ "token": "eyJhbGciOi..." }), code:json ("Contraseña cambiada con éxito"), code:json ([{ "id": 1, "original_name": "mi_doc.pdf", "file_hash": "uui), code:json ({), code:json ({), code:json ([{ "id": 1, "original_name": "mi_doc.pdf", "file_hash": "uui) (+21 more)

### Community 5 - "Community 5"
Cohesion: 0.07
Nodes (12): cargandoDocumentos, esSunat, form, isSubmitting, payload, { esAdmin }, isCambioAreaOpen, isEliminarOpen (+4 more)

### Community 6 - "Community 6"
Cohesion: 0.07
Nodes (28): code:json ([{ "operacion": "editar", "detalle": "...", "fecha": "2024-0), code:json ([{ "id": 1, "area": "Gerencia Municipal", "jefe": "Apellido ), code:json ([{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", ), code:json ([{ "id": 1, "nombre": "Resolución", "sigla": "RA" }]), code:json ([{ "distrito": "PIURA", "cantidad": 45 }, { "distrito": "SIN), code:json ([{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", ), code:json ([), code:json ({) (+20 more)

### Community 7 - "Community 7"
Cohesion: 0.1
Nodes (16): middleware(), router, api, authStore, valido, app, decodificar(), JwtPayload (+8 more)

### Community 8 - "Community 8"
Cohesion: 0.1
Nodes (20): abrirModal(), abrirModalUrl(), abrirVistaPrevia(), archivo, buscarDocumentos(), cargando, cargandoLista, cargarDocumentos() (+12 more)

### Community 9 - "Community 9"
Cohesion: 0.11
Nodes (18): compilerOptions, allowImportingTsExtensions, lib, module, moduleDetection, moduleResolution, noEmit, noFallthroughCasesInSwitch (+10 more)

### Community 10 - "Community 10"
Cohesion: 0.12
Nodes (15): 1. Cambio definitivo de área (`cambio_area`), 2. Drill-down por distrito (`activos_por_distrito` versión lista), 3. Registrar PDF externo (`registrar_url`), 4. Eliminar grado académico (`eliminar_gradoa`), Aceptación, code:ts (// Documento de respaldo. Si ya existe un tipo equivalente e), code:ts ({), Contexto (+7 more)

### Community 11 - "Community 11"
Cohesion: 0.14
Nodes (8): click(), d, pLeaflet, buscar(), busquedaRealizada, consulta, { resultados, cargando, trabajadoresAgregados }, store

### Community 12 - "Community 12"
Cohesion: 0.15
Nodes (11): campo, close(), DISTRITOS, form, isSubmitting, limpiarErrores(), payload, resultado (+3 more)

### Community 13 - "Community 13"
Cohesion: 0.14
Nodes (10): accionesAbiertas, { esAdmin }, EventoVinculoModal, isEliminarModalOpen, isEventoModalOpen, isRenunciaModalOpen, store, tieneRenuncia (+2 more)

### Community 14 - "Community 14"
Cohesion: 0.15
Nodes (12): 1. Root HTML (escalado por breakpoints), 2. Escala unificada de cuerpo, 3. Títulos con `clamp` controlado, 4. Migración, 5. Verificación visual, code:css (html {), Contexto y problema, Criterios de aceptación (+4 more)

### Community 15 - "Community 15"
Cohesion: 0.2
Nodes (9): badgeRelacion(), { contactoEmergencia, perfilActual }, copiado, eliminando, { esAdmin }, getRelacionConfig(), getRelacionIcon(), isEditModalOpen (+1 more)

### Community 16 - "Community 16"
Cohesion: 0.17
Nodes (11): compilerOptions, lib, noFallthroughCasesInSwitch, noUncheckedSideEffectImports, noUnusedLocals, noUnusedParameters, strict, tsBuildInfoFile (+3 more)

### Community 17 - "Community 17"
Cohesion: 0.2
Nodes (9): col, columnasDeBusqueda, filasFiltradas, filasPagina, finPagina, i, inicioPagina, totalFilasFiltradas (+1 more)

### Community 18 - "Community 18"
Cohesion: 0.25
Nodes (7): computedHash, skillPath, source, sourceType, skills, frontend-design, version

### Community 19 - "Community 19"
Cohesion: 0.29
Nodes (3): chartData, colorPorDefecto, totalSexo

### Community 20 - "Community 20"
Cohesion: 0.33
Nodes (5): date, dateCorrected, { esAdmin }, isEditModalOpen, { perfilActual }

### Community 21 - "Community 21"
Cohesion: 0.33
Nodes (5): number, opciones, q, regimenes, string

### Community 22 - "Community 22"
Cohesion: 0.4
Nodes (4): cantidadNoAfiliados, datosAfiliacion, leyenda, listaSindicatos

## Knowledge Gaps
- **293 isolated node(s):** `name`, `private`, `version`, `type`, `dev` (+288 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **10 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `API Endpoints` connect `Community 4` to `Community 1`, `Community 6`?**
  _High betweenness centrality (0.026) - this node is a cross-community bridge._
- **Why does `Personal `/personal` 🔒` connect `Community 1` to `Community 4`?**
  _High betweenness centrality (0.026) - this node is a cross-community bridge._
- **Why does `Dashboard `/dash` 🔒` connect `Community 6` to `Community 4`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _293 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.07 - nodes in this community are weakly interconnected._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.04 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._