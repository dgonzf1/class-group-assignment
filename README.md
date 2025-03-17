# Asignacion de grupos y clases

Programa que toma las clases ofrecidas por un establecimiento y reparte estas en 3 grupos en base a las preferencias de los
estudiantes para inscribir asignaturas.

## Uso

El formato requerido para el archivo de estudiantes es
```
[
  ...
  {
    "name": String,
    "student_id": u64,
    "year": u8,
    "class_priority": {
      "1": class_id (u64),
      ...
      "6": class_id (u64)
    }
  },
  ...
]
```

Para el archivo de clases
```
[
  ...
  {
    "class_id": u64,
    "name": String,
    "professor": Profesor,
    "min_students": u16,
    "max_students": u16
  },
  ...
]
```

Para correr este programa se utiliza
```
cargo run -- estudiantes.json clases.json
```

### Todo

- [ ] Realizar interfaz grafica para el programa.
- [ ] Desarrollar ejecutable
- [ ] Aceptar otros formatos de entrada.
- [ ] Guardar resultados en distintos formatos(csv, google drive, excel, etc..)
- [ ] Agregar prioridad para inscribir ramos a alumnos de mayores años.
- [ ] Refactorizar y parametrizar variables.
- [ ] Validar otros metodos para asignar los grupos a las clases.

### In Progress

### Done ✓
