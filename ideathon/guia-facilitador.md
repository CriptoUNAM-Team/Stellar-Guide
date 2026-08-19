# Guía del facilitador

Cómo correr el día sin que se caiga la métrica. El [temario](temario.md) dice *qué* se enseña; esto dice *cómo* se opera.

---

## Equipo humano mínimo

| Rol | Cuántos | Qué hace |
|---|---|---|
| Facilitador principal | 1 | Da los bloques de contenido y lleva el reloj |
| Facilitador de GitHub | 2 | Check-in de cuentas (B0) y soporte en el bloque B2 |
| Mentores | 1 por cada 2 equipos | Rondas de mesa, review de PRs en B7 |
| Jurado | 3 | Solo B8 (CANACINTRA + técnico + negocio) |
| Operación | 1 | Wifi, comida, cronómetro, proyector |

---

## El bloque que decide el evento: B2

Si a las 10:15 no todos tienen su commit, la métrica del día ya no se recupera — el resto de los bloques asume que la gente ya sabe commitear.

**Cómo se corre:**

1. Proyecta tu propia pantalla y hazlo **en vivo, paso a paso**, a la velocidad del más lento.
2. Después de cada paso, pide señal visible: *"levanten la mano los que ya ven el botón Fork"*. No avances hasta que sea mayoría clara.
3. Los dos facilitadores de GitHub circulan resolviendo rezagados **sin tomar el teclado del participante**: se dicta, no se hace por ellos. Si le resuelves el commit tú, el commit sale con tu nombre y pierdes la métrica.
4. Al terminar, corre `scripts/metricas.sh <repo> --leaderboard` y proyéctalo. Ver su usuario en pantalla es el momento en que el evento hace clic.

**Los cuatro errores que vas a ver, en orden de frecuencia:**

| Error | Se ve como | Solución en 10 segundos |
|---|---|---|
| Correo sin verificar | No aparece el botón Fork | Revisar bandeja y spam; reenviar desde Settings → Emails |
| Editar el repo original | "You don't have permission" | GitHub ofrece forkear; aceptar y seguir |
| Nombre de archivo con mayúsculas, espacios o acentos | El archivo no coincide con lo esperado | Renombrar: minúsculas, guiones, `.md` |
| Crear el archivo fuera de `participantes/` | El validador no lo cuenta | Recrear el archivo escribiendo `participantes/` antes del nombre |

---

## Tiempos internos por bloque

El [temario](temario.md) es el documento del cliente y describe qué se enseña. Esto es cómo se reparte el tiempo dentro de cada bloque.

| Bloque | Reparto |
|---|---|
| **B1** (30) | 15 contexto y reto · 5 verticales · 10 formación de equipos |
| **B2** (45) | 5 conceptos · 25 primer commit guiado · 15 organización del fork del equipo |
| **B3** (60) | 25 clase · 15 llenado de `01-problema.md` · 10 validación cruzada · 10 corrección |
| **B4** (60) | 10 Stellar en lenguaje de negocio · 10 marco de 4 criterios · 15 catálogo · 25 taller |
| **B5** (45) | 20 clase · 25 taller, con paso de mentor por mesa |
| **B6** (120) | 15 demostración en vivo · 75 trabajo · 30 cierre de evidencia |
| **B7** (45) | 10 publicar en Pages · 10 abrir PR · 15 revisión de mentores · 10 iteración |
| **B8** (60) | 3 min de pitch + 2 de preguntas por equipo, con cronómetro proyectado |
| **B9** (20) | 8 resultados · 7 merges en vivo · 5 premiación |

**Reglas que se anuncian en la apertura y se sostienen todo el día:**

- El entregable vive en el repositorio: lo que no está commiteado no cuenta.
- El jurado lee el Pull Request antes del pitch.
- A las 16:20, equipo sin PR es equipo fuera de concurso.
- Commits pequeños y frecuentes durante la jornada, no una carga masiva al final. Es parte de la evaluación y conviene decirlo explícitamente.

**En B5, el paso de mentor por mesa** es una sola pregunta incómoda por equipo, del banco de preguntas de [rubrica.md](rubrica.md). No es una revisión completa: se pregunta una cosa y se sigue.

---

## El bloque de la demo (B6): cómo no perder dos horas

Son 120 minutos y es donde más fácil se va el tiempo en nada. Tres reglas:

1. **Demostración de 15 minutos, en vivo, sin teoría.** Abres la plantilla, cambias el nombre de la idea, el color y una cifra, guardas, recargas. Que vean el ciclo completo *editar → guardar → ver* antes de tocar nada.
2. **A las 14:45, ronda de una sola pregunta:** *¿ya está el texto del problema en la página?* El equipo que a esa hora sigue eligiendo colores va tarde, y hay que decírselo.
3. **Publicar todos juntos a las 15:55**, en B7. Es el momento con más energía del día: proyecta la primera demo que salga en línea y ábrela en tu celular frente a la sala.

**Lo que sí o sí debe funcionar antes del evento:** que tú hayas hecho el recorrido completo desde una cuenta ajena. Si Pages no está bien configurado en el repo o la plantilla tiene una ruta rota, lo descubres con 40 personas esperando.

**Errores que vas a ver en B6:**

| Error | Solución |
|---|---|
| Copiaron la plantilla pero se ve texto plano | Guardaron el archivo con otro nombre o sin `.html`. Debe ser `demo/index.html` exacto. |
| Pages da 404 | Todavía no termina de publicar (1–2 min), o la carpeta lleva mayúsculas o acentos. |
| Rompieron el simulador | Casi siempre es una coma o una comilla faltante en `PASOS`. Que deshagan con Ctrl+Z hasta que vuelva a funcionar y avancen de a poco. |
| Quieren meter imágenes | Se suben a la misma carpeta `demo/` y se referencian por nombre. Nada de ligas a Google Drive: no cargan. |

---

## Ritmo del día

- **Solo hay una pausa formal** (10 min, a media tarde) además de la comida. Los bloques de la mañana son de trabajo en mesa: quien necesite salir, sale. El corte colectivo cuesta diez minutos de recuperar a la sala, y en ocho horas eso se paga varias veces.
- **Anuncia el cierre de cada bloque con su commit.** Literal: *"Bloque cerrado. Quiero ver el commit de `01-problema.md` antes del break."* La disciplina de commitear por bloque no ocurre sola.
- **Ronda de mesas obligatoria** en B3 y B5: cada mentor visita a sus equipos y hace **una** pregunta incómoda (banco de preguntas en [rubrica.md](rubrica.md)). No revisa todo: pregunta una cosa y se va.
- **Alerta de las 15:00.** A esa hora anuncia cuánto falta para el cierre de PRs. El error clásico es que los equipos se enamoren del diseño y no abran el PR.
- **A las 16:20, PR o nada.** Un equipo sin PR no compite. Dilo desde la apertura y sostenlo.

---

## Qué hacer con el equipo que saca 1/4 en el test de blockchain

Va a pasar, y es el mejor momento pedagógico del día. **No los rescates forzando la tecnología.** Dales dos salidas:

1. **Cambiar el problema** conservando la industria: casi siempre hay un dolor vecino que sí cruza fronteras o involucra a varias partes.
2. **Sostener la idea sin blockchain** y defender esa decisión en el pitch. Si lo argumentan bien, el bonus del jurado (+5) lo cubre.

Lo que se premia es el criterio para distinguir cuándo la herramienta aplica. Un equipo que llega a esa conclusión y lo dice en público enseña más al resto de la sala que uno que forzó el encaje.

---

## Guion corto para el review de PRs (B7)

Con 20 minutos y 2–3 PRs por mentor, hay ~7 minutos por PR:

1. **2 min** — leer `01-problema.md` y `03-modelo.md`. Ignora la redacción; busca el número.
2. **1 min** — verificar que el check automático esté en verde (pestaña *Checks* del PR).
3. **3 min** — escribir **una** pregunta concreta como comentario del PR. Que sea contestable con un commit.
4. **1 min** — cerrar con: *"Contesten con un commit, no en el chat."*

No apruebes el PR en ese momento: el merge es ceremonial y se hace en B9, en público.

---

## Después del pitch: el cierre que convierte el evento en pipeline

En B9, además del leaderboard y los premios:

- Mergea los PRs en vivo (**merge commit**, nunca squash).
- Muestra el historial del repo con los nombres de todos: *"esto es público, es verificable, y ya es parte de su portafolio"*.
- Enseña la ruta que sigue: el [curso de 12 semanas](../course/README.md) de este repo para quien quiera profundizar, y la vinculación con empresas de la cámara para los equipos con piloto identificado.
- Pide a cada equipo que deje su idea en la tabla del README del repo. Ese README termina siendo el catálogo de proyectos que CANACINTRA puede circular entre sus socios.
