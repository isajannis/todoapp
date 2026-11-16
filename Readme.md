# Especificaciones del programa

Cada tarea contiene la siguiente información:
- título
- descripción
- estado — {sin iniciar, en proceso, terminado}
- fecha de creación
- fecha de última modificación — solo cambia cuando cambia el estado (al crear una tarea es igual a la de creación)
- duración — tiempo desde la creación hasta ahora, o hasta que se terminó si el estado es terminado

Las tareas no tienen un id propio: se numeran 1, 2, 3… según la lista que se esté mostrando.

## Flujo

El programa empieza con una bienvenida y un “¿Qué quieres hacer hoy?” que muestra el menú principal:
1. Crear nueva tarea
2. Ver tareas
3. Salir

Si se da la opción “1” pide título (obligatorio) y descripción (puede quedar vacía). La tarea se crea como “sin iniciar” y se vuelve al menú de donde se llamó.

Si se da la opción “2” se muestran las tareas, de la más recientemente modificada a la más antigua. De cada una se ve su número, título, estado y fecha de última modificación. Al final aparece un “¿Qué quieres hacer ahora?” con las opciones:
1. Crear nueva tarea
2. Ver detalles de una tarea
3. Ver en formato tablero
4. Volver al menú principal

La opción “3” muestra las mismas tareas separadas por estado (sin iniciar, en proceso, terminado), con el mismo menú, pero la opción 3 pasa a ser “Ver en formato lista”.

Si se da la opción “2” pide el número de la tarea y muestra todos sus datos, seguido de un “¿Qué quieres hacer ahora?” con las opciones:
1. Modificar un campo
2. Eliminar tarea
3. Volver a ver tareas
4. Volver al menú principal

Con “1” se elige el campo a modificar (título, estado o descripción) y al terminar se imprime “El campo {nombre_campo} ha sido modificado exitosamente!”. Cambiar el estado actualiza la fecha de última modificación.

Con “2” se elimina la tarea y se imprime “La tarea "{título}" ha sido eliminada exitosamente”, y se vuelve a la lista.

Si se escribe algo que no es una opción válida (letras, 0, un número fuera de rango) se avisa y se vuelve a pedir.

## Cómo usarlo

```
cargo run
```

Para probar con tareas de ejemplo, en `src/app.rs` poner `PRELOAD_TASKS` en `true`; con `false` el programa empieza vacío. Las tareas solo viven en memoria: al cerrar el programa se pierden.
