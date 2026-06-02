# Semana 2 · El consenso de Stellar (SCP / Federated Byzantine Agreement)

**Módulo 1 · Fundamentos** · Sesión teórica fuerte + lab de análisis.

## Objetivos de aprendizaje
- Definir quorum slice, quorum, intersección de quórums y conjunto v-blocking.
- Explicar el voto federado (vote → accept → confirm) y las fases nominación/balotaje.
- Justificar por qué SCP prioriza safety y ofrece finalidad determinista.

## Lecturas previas
- [../teoria/02-consenso-scp.md](../teoria/02-consenso-scp.md) (lectura completa, es densa).

## Guion de teoría (≈70 min)
1. Repaso: las 4 familias de consenso y el trade-off safety/liveness. (10 min)
2. La pregunta central: BFT sin lista fija → FBA. (5 min)
3. Quorum slice vs quorum (con la analogía de las fuentes de confianza). (15 min)
4. Intersección de quórums = safety; el fork como falla topológica. (10 min)
5. Conjunto v-blocking y propagación de la verdad. (10 min)
6. Voto federado y protocolo de balotas (prepare → commit → externalize). (15 min)
7. Validadores reales, organizaciones y Tier 1; sin minería ni stake. (5 min)

> **Callout "Vienes de PoW/PoS":** aquí la seguridad **no se compra** con hash ni stake; se **configura**
> con confianza solapada. El riesgo no es un 51%, es una mala topología de quórums.

## Demo en vivo
- Mostrar la configuración de un `quorum set` (umbral + inner sets) y exploradores de validadores
  (p. ej. quórum/validadores en stellar.expert o herramientas de la SDF).
- Diagrama en pizarra: dos quórums que **no** se intersectan → fork.

## Lab calificado 2 — "Anatomía de un quórum"
**Entregable:** reporte en el repo personal:
1. Encontrar y describir el quorum set de 2 organizaciones validadoras reales.
2. Diagramar (Mermaid) las relaciones de confianza y señalar la intersección.
3. Explicar con tus palabras: ¿qué pasaría si esas orgs dejaran de incluirse mutuamente?
4. Comparar finalidad Stellar vs tu cadena previa (1 párrafo).

## Tarea
- Responder las 6 preguntas de comprensión del doc de teoría.
- Preparar dudas para la sesión de la semana 3.

## Recursos
- Whitepaper SCP: https://www.stellar.org/papers/stellar-consensus-protocol
- https://developers.stellar.org/docs/learn/fundamentals/stellar-consensus-protocol
