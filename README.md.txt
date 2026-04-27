# ✅ Todo Solana — Lista de Tareas en la Blockchain

Proyecto desarrollado en **Rust + Anchor** sobre la red **Solana** como parte de una certificación.

## ¿Qué hace?

Permite a cualquier usuario crear, leer, actualizar y eliminar tareas personales guardadas en la blockchain de Solana. Cada tarea se almacena en una cuenta **PDA** (Program Derived Address) única por usuario.

## Funciones del programa

| Función          | Descripción                              |
|------------------|------------------------------------------|
| `create_task`    | Crea una nueva tarea (C del CRUD)        |
| `update_task`    | Edita título y descripción (U del CRUD)  |
| `toggle_complete`| Marca como completada/pendiente          |
| `delete_task`    | Elimina la tarea (D del CRUD)            |

## ¿Qué es una PDA?

Una **Program Derived Address** es una dirección en Solana generada a partir de semillas fijas (`"task"` + clave del usuario + título). Esto garantiza que cada tarea sea única y solo su dueño pueda modificarla.

## Tecnologías

- [Rust](https://www.rust-lang.org/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Devnet](https://solana.com/)

## Cómo usarlo

1. Clona el repositorio
```bash
git clone https://github.com/TU_USUARIO/todo-solana
cd todo-solana
```

2. Instala dependencias y compila
```bash
anchor build
```

3. Ejecuta las pruebas
```bash
anchor test
```