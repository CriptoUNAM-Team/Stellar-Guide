# GitHub para el ideathon — guía del participante

**Todo se hace desde el navegador.** No necesitas instalar nada, ni usar la terminal, ni saber programar.

Tiempo estimado: 15 minutos para tu primer commit.

---

## Antes del evento (5 min, hazlo desde casa)

1. Entra a **[github.com/signup](https://github.com/signup)** y crea tu cuenta.
   - Elige un usuario que puedas poner en tu CV. Va a quedar público.
2. **Verifica tu correo.** GitHub te manda un mail con un botón de confirmación. **Sin esto no vas a poder hacer fork el día del evento** — es el error número uno.
3. Si te pide activar 2FA (verificación en dos pasos), hazlo con una app tipo Google Authenticator o Authy. Tienes días de gracia, pero mejor resuélvelo antes.
4. Sube una foto y pon tu nombre real en el perfil. Es tu portafolio.

---

## Vocabulario mínimo

| Término | Qué es, en cristiano |
|---|---|
| **Repositorio (repo)** | El archivero del proyecto: todos los archivos y su historia. |
| **Commit** | Una versión guardada, con tu nombre y la hora. Es tu firma en el proyecto. |
| **Fork** | Tu copia del archivero, para trabajar sin tocar el original. |
| **Pull Request (PR)** | "Propongo que mis cambios entren al proyecto original." Se revisa y se aprueba. |
| **Merge** | Tus cambios quedan dentro del proyecto oficial, con tu nombre, para siempre. |
| **Markdown (.md)** | Formato de texto simple: `# título`, `**negritas**`, `- listas`. Nada más. |

---

## Paso 1 · Forkea el repo del ideathon

1. Abre el repo: `https://github.com/MarxMad/Ideathon-Stellar-BAF-Canacintra`
2. Arriba a la derecha, botón **Fork** → **Create fork**.
3. Ahora estás en `github.com/<tu-usuario>/Ideathon-Stellar-BAF-Canacintra`. **Todo tu trabajo pasa aquí.**

> Si te equivocas y editas el repo original, no pasa nada: GitHub te dirá que no tienes permiso y te ofrecerá crear el fork automáticamente. Acepta y sigue.

---

## Paso 2 · Tu primer commit (individual, todos lo hacen)

1. En **tu fork**, botón **Add file** → **Create new file**.
2. En el nombre del archivo escribe exactamente:
   ```
   participantes/tu-usuario-github.md
   ```
   Al escribir la diagonal `/`, GitHub crea la carpeta solo. Usa **minúsculas y sin espacios**.
3. Pega esta plantilla y llénala:

   ```markdown
   # <Tu nombre>

   - **Usuario GitHub:** @<tu-usuario>
   - **Universidad / carrera:** <...>
   - **Qué me trae al ideathon:** <una línea>
   - **Un dolor de la industria que conozco de cerca:** <una o dos líneas>
   ```
4. Abajo, en **Commit changes**:
   - Mensaje: `feat: me sumo al ideathon`
   - Deja seleccionado *Commit directly to the main branch*.
   - Botón verde **Commit changes**.

🎉 **Listo: ese es tu primer commit.** Ya apareces en el historial del proyecto con tu nombre.

---

## Paso 3 · La carpeta de tu equipo

El equipo elige **un solo fork** (el de un integrante) donde vivirá el entregable. Ese integrante es el "dueño del fork".

**Para que todos puedan commitear ahí**, el dueño del fork hace:
> Settings → Collaborators → **Add people** → agrega a los usuarios de sus compañeros.
> Cada compañero acepta la invitación desde su correo o desde github.com/notifications.

Toma 2 minutos y es lo que permite que **cada integrante tenga commits propios**, que es parte de lo que se mide.

Dentro de ese fork, creen la carpeta del equipo con los archivos del día:

```
ideas/equipo-XX-nombre-de-la-idea/
├── 01-problema.md
├── 02-caso-uso.md
├── 03-modelo.md
├── 04-pitch.md
├── evidencia.md
└── demo/
    └── index.html   ← su página, la que van a publicar
```

Las plantillas de cada archivo están en la carpeta `plantillas/` del repo. Cópienlas y llénenlas.

**Un archivo = un commit.** No junten todo al final: cada bloque del día cierra con su commit.

---

## Paso 4 · Cómo dar crédito a todo el equipo en un commit

Si un integrante escribe el archivo pero lo pensaron entre todos, agreguen coautores. En el cuadro de **descripción** del commit (el campo grande, debajo del mensaje), dejen una línea en blanco y luego:

```
Co-authored-by: Ana López <ana@users.noreply.github.com>
Co-authored-by: Luis Pérez <luis@users.noreply.github.com>
```

El correo `<usuario>@users.noreply.github.com` funciona para cualquier cuenta y no expone el correo real.

---

## Paso 5 · Publicar su demo (GitHub Pages)

Su página no sirve de mucho guardada en el repo: hay que ponerla en internet. GitHub lo hace gratis y en dos minutos.

1. En **su fork**, entren a **Settings** (la pestaña de hasta la derecha).
2. En el menú de la izquierda, **Pages**.
3. En *Build and deployment* → *Source*, elijan **Deploy from a branch**.
4. En *Branch*, seleccionen **`main`** y la carpeta **`/ (root)`**. **Save**.
5. Esperen 1–2 minutos y recarguen esa misma página: aparece la liga arriba.

Su demo queda en:

```
https://<tu-usuario>.github.io/Ideathon-Stellar-BAF-Canacintra/ideas/equipo-XX/demo/
```

**Ábranla en el celular.** Esa liga va en `evidencia.md` y en el pitch.

> Si sale error 404, casi siempre es una de tres: todavía no termina de publicar (esperen otro minuto), el archivo no se llama exactamente `index.html`, o la carpeta tiene mayúsculas o acentos.

---

## Paso 6 · Abrir el Pull Request (el entregable oficial)

Cuando su carpeta esté completa:

1. En su fork, pestaña **Pull requests** → **New pull request**
   (o el botón **Contribute** → **Open pull request** que aparece arriba).
2. Verifiquen la dirección: **base:** `MarxMad/Ideathon-Stellar-BAF-Canacintra : main` ← **head:** `<tu-usuario>/Ideathon-Stellar-BAF-Canacintra : main`.
3. Título: `Equipo 07 — Factoraje digital para autopartes`
4. La descripción trae un checklist automático. Márquenlo.
5. **Create pull request.**

En menos de un minuto aparece abajo un check automático: ✅ si está completo, ❌ si falta algo. **Si sale ❌, haz clic en *Details* para ver qué falta** — casi siempre es un archivo faltante o un campo `<...>` sin llenar. Corrígelo en tu fork y el PR se actualiza solo.

---

## Paso 7 · Responder al review (esto vale puntos)

Un mentor va a dejar una pregunta en su PR. **No la ignoren.**

1. Lean el comentario.
2. Editen el archivo correspondiente en su fork (botón del lápiz ✏️ arriba del archivo).
3. **Commit changes** con un mensaje como `docs: aclara quién opera el ancla`.
4. El commit aparece solo dentro del mismo PR. Contesten el comentario diciendo qué cambiaron.

Esto se mide (métrica M6) y es, honestamente, la habilidad más útil que se llevan del día: trabajar con retroalimentación en público.

---

## Problemas comunes

| Síntoma | Solución |
|---|---|
| "You don't have permission to push" | Estás en el repo original, no en tu fork. Revisa que la URL diga tu usuario. |
| No aparece el botón **Fork** | No has verificado tu correo. Revisa tu bandeja (y spam). |
| No encuentro mi archivo | Revisa mayúsculas y acentos en el nombre. Usa solo minúsculas, guiones y `.md`. |
| Mi compañero no puede editar | Falta que acepte la invitación de colaborador, o le llegó al correo equivocado. |
| El PR muestra cientos de archivos cambiados | Forkeaste desde una rama distinta o tocaste archivos que no eran tuyos. Llama a un mentor. |
| Se me borró lo que escribí | Todo commit queda en el historial: pestaña **Commits** de tu fork → abre el commit anterior. Nada se pierde. |
