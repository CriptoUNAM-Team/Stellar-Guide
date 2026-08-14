# grades — Calificaciones (campus)

El profesor (admin) registra `score / max_score` por alumno y actividad. El alumno consulta su nota; no puede alterarla.

Sirve para enseñar: roles, escritura restringida, lectura pública, validación (`score <= max_score`).

## Funciones

- `initialize(admin)`
- `record_grade(student, assignment, score, max_score)`
- `get_grade(student, assignment)`

## Lab (sesión 2)

Comandos de deploy/invoke: `docs/comandos-basicos.md`.

```bash
cd contracts/grades
make test
```
