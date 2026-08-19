# Logística — checklist previo y del día

## T-2 semanas

- [ ] Crear el repo del ideathon (ver [`scripts/crear-repo.sh`](scripts/crear-repo.sh)) y **recorrerlo completo con una cuenta de GitHub nueva y ajena**: fork → commit → copiar la plantilla de demo → activar Pages → abrir PR. **Este ensayo no es opcional**: es la única forma de detectar fricciones que tú ya no ves por tener sesión iniciada y permisos.
- [ ] Confirmar que la demo de ejemplo carga en Pages y se ve bien **en un celular**.
- [ ] Configurar el repo: Settings → General → Pull Requests → dejar habilitado **solo *Allow merge commits*** (desactivar squash y rebase). Ver el porqué en [metricas.md](metricas.md).
- [ ] Definir jurado (CANACINTRA + técnico + negocio) y mentores: **1 mentor por cada 2 equipos**.
- [ ] Reservar sede con mesas de equipo (no auditorio con butacas fijas) y proyector.

## T-72 horas

- [ ] Correo a inscritos con el prerrequisito: **crear cuenta de GitHub y verificar el correo**, más el link a [guia-github-participantes.md](guia-github-participantes.md).
- [ ] Formulario de registro que capture **usuario de GitHub** (campo obligatorio) — es la llave de toda la métrica.
- [ ] Confirmar wifi de la sede: cuántos dispositivos simultáneos aguanta y si bloquea github.com. Un ideathon de 60 personas contra un wifi de oficina es el riesgo operativo número uno.

## El día — mesa de check-in (08:30)

- [ ] 2 facilitadores dedicados **solo** a crear/verificar cuentas.
- [ ] QR impreso tamaño cartel con el link del repo, pegado en cada mesa.
- [ ] Lista de asistencia con columna "usuario GitHub" — se llena al entrar, no al final.
- [ ] Pantalla lateral con la tabla de métricas proyectada todo el día.

## Materiales

| Qué | Cantidad |
|---|---|
| Extensiones y multicontactos | 1 por mesa |
| Post-its y plumones | 1 juego por equipo |
| Cronómetro proyectable para los pitches | 1 |
| Impresión de la estructura del pitch | 1 por equipo |
| Laptops de respaldo | 2–3 |

## Contingencias

| Riesgo | Plan B |
|---|---|
| Wifi caído o saturado | Hotspots móviles de respaldo; equipos redactan en Markdown local o en papel y commitean en bloque cuando vuelva la red. |
| Alguien no logra crear cuenta | Trabaja en el fork de su equipo como colaborador. Se le registra en `participantes/` con un commit del dueño del fork usando `Co-authored-by`. No pierde la actividad; sí la métrica M1 individual. |
| Un equipo se queda sin idea a las 11:00 | Banco de 5 retos precocinados (uno por vertical del bloque B1) listo para entregar impreso. |
| Equipo desbalanceado (nadie técnico) | Nivel N1 es suficiente para competir por todos los premios excepto "Mejor uso de Stellar". Decirlo en voz alta desde la apertura evita deserciones a media mañana. |
| Se acaba el tiempo antes de los PRs | Recortar B8 a 2 min por pitch antes que sacrificar B7: el PR **es** el entregable medible. |
| Un equipo no logra publicar en Pages | Que enseñen la demo abriendo el archivo desde el repo con la vista *Preview*, o desde su laptop. No pierde el nivel N1; pierde la URL. |

## Después del evento (T+3 días)

- [ ] Correr `scripts/metricas.sh` y generar el reporte final para CANACINTRA.
- [ ] Mergear los PRs pendientes que hayan quedado corregidos (con merge commit).
- [ ] Enviar a cada asistente el link a su contribución pública — es material de portafolio y el mejor gancho para la siguiente convocatoria.
- [ ] Publicar el `README.md` del repo con la tabla de ideas resultantes: queda como catálogo de proyectos para las empresas de la cámara.
