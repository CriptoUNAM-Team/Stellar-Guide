# Rúbrica de evaluación — Ideathon Stellar × CANACINTRA

Dos evaluaciones independientes que se suman:
**(A) Calidad de la idea**, que juzga el jurado, y **(B) Disciplina de ejecución**, que juzga el historial de Git sin intervención humana.

---

## A · Calidad de la idea — 70 puntos (jurado)

| Criterio | Pts | Qué se busca | Señal de alerta |
|---|---|---|---|
| **Problema y cliente** | 20 | Problema en una frase con costo cuantificado. Cliente específico. 3 fuentes de evidencia. Nombra la alternativa actual. | "Las empresas necesitan mejores pagos." Cliente = "todos". |
| **Ajuste a Stellar** | 20 | Test de 4 preguntas con ≥ 2 puntos bien justificados. Distingue qué va on-chain y qué off-chain. Patrón del catálogo bien elegido. | Blockchain forzada donde bastaría una base de datos. |
| **Modelo de negocio** | 15 | Quién paga, cuánto, contra qué alternativa. Unit economics que reconoce el costo del on/off ramp. Mercado bottom-up. | Ingresos sin costos. Mercado top-down ("el 1 % de un mercado de X mil millones"). |
| **Demo y evidencia** | 10 | La página publicada cuenta la idea sola, sin que nadie la explique. Bonus por simulador propio (N2) o Testnet (N3). | La plantilla sin personalizar, con los textos de ejemplo todavía puestos. |
| **Pitch** | 5 | Cabe en 3 minutos, sigue la estructura, responde bien las preguntas. | Se pasa del tiempo, presenta 4 personas sin coordinación. |

**Bonus del jurado (hasta +5):** nombrar con honestidad el riesgo mayor y cómo lo mitigarían. Reconocer que la idea *no* justifica blockchain y pivotar a tiempo también suma: se premia el criterio.

---

## B · Disciplina de ejecución — 30 puntos (automática, desde Git)

Se calcula con [`scripts/metricas.sh`](scripts/metricas.sh). No la juzga nadie: sale del historial.

| Criterio | Pts | Cómo se calcula |
|---|---|---|
| Todos los integrantes tienen al menos 1 commit propio | 10 | Autores únicos del equipo ÷ integrantes registrados |
| ≥ 4 commits del equipo, repartidos durante el día | 8 | Commits del PR + dispersión horaria (no todos en la última hora) |
| PR abierto con la plantilla completa y validación en verde | 6 | Estado del GitHub Action |
| Al menos 1 commit posterior al review del mentor | 6 | Timestamp del commit > timestamp del comentario |

> La dispersión horaria se evalúa a propósito: premia al equipo que commiteó en cada bloque sobre el que volcó todo a las 16:25, aunque ambos terminen con el mismo número de archivos.

---

## Premios

| Premio | Se decide con |
|---|---|
| 🥇 **Mejor idea general** | Mayor puntaje total (A + B) |
| 🎯 **Mejor problema** | Criterio "Problema y cliente" |
| ⭐ **Mejor uso de Stellar** | Criterio "Ajuste a Stellar" + evidencia técnica (N3) |
| 🖥️ **Mejor demo** | La página que mejor comunica la idea sin ayuda de nadie |
| 💼 **Mejor modelo de negocio** | Criterio "Modelo de negocio" — premio con voto de CANACINTRA |
| 🔁 **Equipo más constante** | **Solo bloque B**: el mejor historial de Git del evento |

El premio al *Equipo más constante* es central en el diseño: hace visible que la forma de trabajar cuenta, y le da un reconocimiento alcanzable al equipo que quizá no tuvo la idea más brillante pero sí ejecutó con orden.

---

## Guion para mentores en el review de PRs (bloque B7)

Deja **una** pregunta concreta por PR, no un "va bien". Banco de preguntas:

- ¿Por qué el cliente abandonaría su proceso actual? ¿Qué le duele lo suficiente?
- ¿Quién opera el ancla / la conversión a pesos? ¿Ya hablaron con alguna?
- ¿Qué pasa si la contraparte no cumple? ¿Quién resuelve la disputa?
- Si mañana lo hicieran con una base de datos y transferencias SPEI, ¿qué se pierde exactamente?
- ¿Cuánto tiene que costar la operación para que su margen exista?
- ¿Quién dentro de la empresa firma este contrato: compras, finanzas o dirección?
- ¿Qué dato *no* puede estar on-chain por confidencialidad?

- ¿Su demo se entiende sin que ustedes la expliquen? Ábranla en el celular de alguien ajeno al equipo.

Cierra siempre con: *"Contesten con un commit, no en el chat."*
