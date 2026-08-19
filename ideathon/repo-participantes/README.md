# Ideathon Stellar × BAF × CANACINTRA

Aquí vive tu idea y aquí se registra tu participación.
Todo se hace **desde el navegador**: no necesitas instalar nada ni saber programar.

📄 [Programa completo de la jornada](plan-de-trabajo.html) · 📊 [Cómo se evalúa](rubrica.md)

---

## Empieza aquí (en este orden)

| | Paso | Cuándo | Dónde |
|---|---|---|---|
| 1 | **Crea tu cuenta de GitHub y verifica tu correo** | Antes del evento | [github.com/signup](https://github.com/signup) |
| 2 | **Haz clic en Fork** (arriba a la derecha de esta página) | Bloque B2 | Este repo |
| 3 | **Crea tu archivo** `participantes/tu-usuario.md` | Bloque B2 | Tu fork |
| 4 | **Crea la carpeta de tu equipo** y llena los entregables | Bloques B3 a B5 | Tu fork |
| 5 | **Copia la plantilla de demo** y personalízala | Bloque B6 | Tu fork |
| 6 | **Publica tu demo** con GitHub Pages | Bloque B7 | Settings → Pages |
| 7 | **Abre tu Pull Request** | Bloque B7 | Contribute → Open pull request |
| 8 | **Responde el comentario del mentor** con un commit | Bloque B7 | Tu Pull Request |

**¿Nunca has usado GitHub?** La [guía paso a paso](guia-github.md) tiene cada uno de estos pasos con capturas de qué botón apretar.

---

## Qué entrega tu equipo

Una carpeta con seis archivos:

```
ideas/equipo-07-factura-lista/
├── 01-problema.md     Bloque B3 · qué problema resuelven y para quién
├── 02-caso-uso.md     Bloque B4 · qué patrón de Stellar usan y por qué
├── 03-modelo.md       Bloque B5 · quién paga y cuánto
├── 04-pitch.md        Bloque B8 · el guion de sus 3 minutos
├── evidencia.md       Bloque B6 · el diagrama y la liga de su demo
└── demo/
    └── index.html     Bloque B6 · su página, publicada en internet
```

Las plantillas de los seis archivos están en **[`plantillas/`](plantillas/)**.
Un ejemplo completo, ya llenado, está en **[`ideas/equipo-00-ejemplo/`](ideas/equipo-00-ejemplo/)** — ábranlo antes de empezar.

### Su demo va a quedar en esta dirección

```
https://TU-USUARIO.github.io/Ideathon-Stellar-BAF-Canacintra/ideas/equipo-XX/demo/
```

Ejemplo funcionando: [equipo-00-ejemplo](https://marxmad.github.io/Ideathon-Stellar-BAF-Canacintra/ideas/equipo-00-ejemplo/demo/)

---

## Cómo se evalúa

| Componente | Puntos | Quién evalúa |
|---|---|---|
| Calidad de la propuesta | 70 | El jurado |
| Ejecución: quién contribuyó, cuándo y si iteraron | 30 | Automático, desde el historial |

**Categorías:** mejor propuesta general · mejor problema · mejor uso de Stellar · mejor modelo de negocio · mejor demo · mejor ejecución.

Criterios completos en **[rubrica.md](rubrica.md)**.

> **Un archivo = un commit.** No junten todo al final: cada bloque de la jornada cierra con el suyo, y eso se evalúa.

---

## Reglas

1. Trabaja siempre en **tu fork**, nunca en este repo directamente.
2. **No toques la carpeta de otro equipo.** Un Pull Request que modifica archivos ajenos se rechaza.
3. **Nunca subas una clave secreta** (las que empiezan con `S`). Este repo es público y hay una validación automática que lo detecta.
4. Nombra la carpeta de tu equipo en minúsculas y con guiones: `ideas/equipo-07-factura-lista/`.

Detalle en [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Material de consulta

Todo el material técnico y el catálogo de casos de uso está en **[Stellar-Guide](https://github.com/MarxMad/Stellar-Guide)**:

| Necesitas… | Ve a |
|---|---|
| Casos de uso por industria, con contrato de referencia | [`docs/contratos-casos-uso.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/docs/contratos-casos-uso.md) |
| Combinaciones de extremo a extremo | [`docs/playbooks-producto.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/docs/playbooks-producto.md) |
| Qué son las anclas y los estándares SEP | [`docs/sep-estandares-anclas.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/docs/sep-estandares-anclas.md) |
| Tu primer pago en la red de pruebas (nivel N3) | [`exercises/01-pago-simple.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/exercises/01-pago-simple.md) |
| Interfaces conectadas a contratos (nivel N3) | [`docs/frontend-contratos.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/docs/frontend-contratos.md) |
| Cómo dibujar el diagrama de la solución | [`docs/flujos-mermaid.md`](https://github.com/MarxMad/Stellar-Guide/blob/main/docs/flujos-mermaid.md) |

---

## Preguntas frecuentes

**No me aparece el botón Fork.**
No has verificado tu correo. Revisa tu bandeja de entrada y la carpeta de spam.

**Dice que no tengo permiso para guardar.**
Estás editando este repo en vez de tu fork. Revisa que la dirección diga tu usuario. GitHub te va a ofrecer crear el fork: acepta y sigue.

**Mi demo da error 404.**
Puede ser que todavía se esté publicando (espera 2 minutos), que el archivo no se llame exactamente `index.html`, o que la carpeta tenga mayúsculas o acentos.

**Mi compañero no puede editar nuestra carpeta.**
Falta que el dueño del fork lo agregue en *Settings → Collaborators*, y que él acepte la invitación.

**Se me borró lo que había escrito.**
Nada se pierde: en la pestaña **Commits** de tu fork está cada versión guardada. Abre la anterior y recupera el texto.

**El robot marcó mi Pull Request en rojo.**
Haz clic en *Details* para ver qué falta. Casi siempre es un archivo que no creaste o un campo `<...>` que quedó sin llenar. Corrígelo en tu fork y el Pull Request se actualiza solo.

---

## Ideas presentadas

<!-- Se completa al cierre del evento -->

| Equipo | Propuesta | Vertical | Demo | Pull Request |
|---|---|---|---|---|
| | | | | |
