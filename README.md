# Comandos
## add <title> [desc] [due]
Este comando añade una nueva tarea, el titulo es obligatorio, sin embargo la descripcion (*desc*) y la fecha de vencimiento (*due*) son opcionales.
```cargo run -- add "Ir a la conferencia" "de inteligencia artificial" 2025-11-24```

## list [all|pending|doing|done]
Este comando lista todas las tareas. Si no pones ningun modo, muestra pendiente(*pending*) por defecto.
```cargo run -- list all```

## update <title> <field> 
Este comando actualiza un campo de la tarea buscándola por título; *field* puede ser el titulo (*title*), la descripcion (*desc*) o la fecha de vencimiento (*due*).
```cargo run -- update "Ir a la conferencia" desc "de inteligencia emocional"```

## mark <title> <pending|doing|done>
Este comando lo que hace es cambiar el estado de la tarea (por título).
```cargo run -- mark "Ir a la conferencia" doing```

## delete <title>
Este comando elimina la tarea cuyo título coincide exactamente (y elimina todas las entradas que no coincidan).
```cargo run -- delete "Ir a la conferencia"```

