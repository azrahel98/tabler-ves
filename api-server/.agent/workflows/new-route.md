---
description: Pasos a seguir al crear una nueva ruta en el API
---

## Al crear una nueva ruta

1. Crear o modificar el handler en `src/handlers/`
2. Registrar la ruta en `src/routes/`
3. **Actualizar `API.md`** en la raíz del proyecto con la nueva ruta, siguiendo el formato existente:
   - Método HTTP y path
   - Tabla de parámetros del body (campo, tipo, requerido)
   - Ejemplo de respuesta JSON
   - Indicar 🔒 si requiere JWT
4. Si se crean nuevos structs, agregarlos en `src/models/`
5. Usar `ApiError` de `middleware/error.rs` para los errores (estructura uniforme `{ code, error }`)
