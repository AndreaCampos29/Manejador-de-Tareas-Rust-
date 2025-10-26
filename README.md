# Task Tracker
Es un manejador de tareas por consola que permite añadir, listar, actualizar, marcar y eliminar tareas. Cada tarea contiene un id, title, desc (opcional), status, created y due (opcional).

## Comandos

### 1) add <title> [desc] [due]
Este comando crea una nueva tarea con estado inicial pending (pendiente) y la guarda en el archivo json

Ej:
`cargo run -- add "Salir de compras" "comprar pollo" 2025-11-03`

### 2) list [all|pending|doing|done]
Este comando lista tareas. Si no se indica el modo, muestra pending (pendiente) por defecto.

Ej:
`cargo run -- list all`

### 3) update <title> <field> <value>
Este comando actualiza el field (title, desc, due) de la primera tarea que se encuentre donde el título coincida exactamente

Ej:
`cargo run -- update "Salir de compras" desc "comprar pan"`

### 4) mark <title> <pending|doing|done>
Este comando cambia el estado de un tarea por medio del titulo

Ej:
`cargo run -- mark "Salir de compras" doing`

### 5) delete <title>
Este comando elimina la/s tarea/s cuyo título coincide exactamente con el que se coloque, ya que si hay duplicados, se eliminaran todas las coincidencias.

Ej:
`cargo run -- delete "Salir de compras"`

## Pruebas Unitarias (test)

### 1) cargo test
Este comando se utilizada para ejecutar todas las pruebas

### 2) cargo test -- --nocapture
Este comando nos permite ver las salidas `println!` durante las pruebas

### 3) cargo test --test del_test
Este nos permite ejecutar un archivo de tests en concreto

### 4) cargo test update_title
Y este nos permite ejecutar un test mediante el nombre 
