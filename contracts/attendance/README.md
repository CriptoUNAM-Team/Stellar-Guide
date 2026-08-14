# attendance — Asistencia de clase (campus)

Registro on-chain de sesiones y presencia. El admin (profesor) abre la clase; marca alumnos. Cada address tiene un contador acumulado.

Pensado para docentes: el alumno es una `Address`, no un nombre. Útil para hablar de identidad, `require_auth` y storage persistente.

## Funciones

- `initialize(admin)`
- `open_session(course, topic) -> session_id`
- `mark_present(session_id, student)`
- `is_present(session_id, student)`
- `get_session(session_id)`
- `attendance_count(student)`

## Lab (sesión 2)

Comandos de deploy/invoke: `docs/comandos-basicos.md`.

```bash
cd contracts/attendance
make test
```
