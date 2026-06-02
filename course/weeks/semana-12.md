# Semana 12 · Producción (mainnet hardening) + proyecto final

**Módulo 5 · Producción** · Teoría breve + **defensa de proyectos.** **Cierre: Quiz 5.**

## Objetivos de aprendizaje
- Aplicar un checklist de hardening antes de ir a mainnet.
- Considerar observabilidad, manejo de claves y operación.
- Presentar y defender un proyecto integrador.

## Lecturas previas
- [../../docs/checklist-pre-mainnet.md](../../docs/checklist-pre-mainnet.md)
- [../../docs/troubleshooting-integraciones.md](../../docs/troubleshooting-integraciones.md)

## Guion de teoría (≈30 min)
1. Diferencias operativas Testnet → Mainnet (passphrase, fondos reales, sin reset). (5 min)
2. Manejo de claves y secretos; nunca en código; firma segura. (10 min)
3. Observabilidad: monitoreo de tx, reintentos idempotentes, alertas. (10 min)
4. Checklist de seguridad de contratos e integraciones. (5 min)

> **Callout "Vienes de EVM":** mainnet de Stellar **no se resetea** y la finalidad es inmediata: un error
> de fees o de auth es definitivo. La disciplina de Testnet → revisión → mainnet es obligatoria.

## Actividad principal — Defensa del proyecto final (≈90 min)
Cada estudiante/equipo presenta (8-10 min) + Q&A:
- Demo funcional (contratos en Testnet + integración + UI o CLI).
- Explicación de decisiones de diseño, auth y storage.
- Evidencia de tests y checklist de hardening.

Evaluación con [../evaluacion/proyecto-final.md](../evaluacion/proyecto-final.md) y rúbrica.

## Quiz 5 (cierre de Módulo 5)
Temas: producción, hardening, operación, troubleshooting.
Banco: [../evaluacion/quizzes.md](../evaluacion/quizzes.md#quiz-5).

## Entregable final
- Repo del proyecto con README, contratos, tests, instrucciones de despliegue y checklist completado.
- Slides de la presentación.

## Cierre del curso
- Retroalimentación final y siguientes pasos (ecosistema, grants, comunidad SDF).

## Recursos
- https://developers.stellar.org/docs/learn/fundamentals/networks
- https://developers.stellar.org/docs/build/guides/conventions
