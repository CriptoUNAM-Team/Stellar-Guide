# Programa pedagógico para docentes — Stellar & Soroban

**Para:** profesores de Tecnología, Computación, Informática, Ingeniería de Software, Sistemas e Ingeniería en general.  
**Objetivo:** incorporar blockchain / pagos / contratos inteligentes en el aula **sin reinventar el syllabus**, usando módulos listos del repositorio [Stellar Guide](../README.md).  
**Idioma:** español.

Este documento es la **capa docente**. El detalle semana a semana para un curso completo está en [syllabus.md](syllabus.md) y [weeks/](weeks/). Aquí se responde: *¿qué enseño?, ¿cuántas horas?, ¿en qué materia?, ¿cómo evalúo?, ¿qué preparo yo como profesor?*

---

## 1. Propósito educativo

Al integrar este material, el estudiante no solo “usa una crypto”, sino que desarrolla competencias de:

| Competencia | Qué demuestra en clase |
|---|---|
| **Sistemas distribuidos** | Explica consenso, fallas bizantinas y por qué SCP no es PoW/PoS |
| **Ingeniería de software** | Diseña, prueba y despliega lógica on-chain (Soroban/Rust) |
| **Arquitectura de red** | Distingue ledger, transacciones, operaciones, Horizon y RPC |
| **Finanzas digitales / fintech** | Emite activos, entiende trustlines, pagos y anclas (SEPs) |
| **Producto y seguridad** | Aplica checklist pre-mainnet, auth y manejo de errores |
| **Casos de industria** | Conecta contratos a problemas reales (nómina, ahorro, crédito, trazabilidad alimentaria) |

---

## 2. A quién va dirigido (perfil del grupo)

| Perfil del aula | Nivel sugerido | Enfoque recomendado |
|---|---|---|
| Preparatoria / Bachillerato técnico (computación) | Introductorio | Solo Módulo A (pagos + cuentas). Sin Rust |
| Técnico superior / universidad 1.º–2.º año | Básico–intermedio | Módulos A + B (red clásica) |
| Ingeniería / Informática 3.º–5.º año | Intermedio–avanzado | A + B + C (Soroban) |
| Posgrado / diplomado fintech o blockchain | Avanzado | Curso completo 12 semanas + proyecto |
| Profesoría (formación de docentes TIC) | Variable | Empezar por este documento + taller de 8–16 h |

**Prerrequisitos mínimos del estudiante:** terminal básica y lógica de programación.  
**Deseable (según módulo):** Git, JS o Python, nociones de redes; Rust solo a partir del Módulo C.

---

## 3. Tres formas de incorporarlo en tu clase

No tienes que impartir las 12 semanas. Elige un **formato de adopción**:

### Formato 1 — Cápsula (1–2 sesiones, 2–4 h)

Útil para: “semana de innovación”, clase abierta, laboratorio puntual.

| Sesión | Contenido | Material |
|---|---|---|
| 1 | Qué es Stellar vs otras cadenas + cuentas Testnet | [teoria/01](teoria/01-stellar-vs-otras-cadenas.md), [docs/introduccion](../docs/introduccion.md) |
| 2 | Pago simple en Testnet | [exercises/01-pago-simple.md](../exercises/01-pago-simple.md) |

**Entregable del alumno:** captura de cuenta fondeada + hash de un pago + 5 líneas de reflexión.

### Formato 2 — Unidad didáctica (4–8 sesiones, 8–16 h)

Útil para: insertar dentro de *Redes*, *Sistemas Distribuidos*, *Fintech*, *Desarrollo Web* o *Proyectos*.

| Bloque | Horas | Resultado |
|---|---|---|
| Fundamentos + consenso (visión) | 2–3 h | Explica SCP a alto nivel |
| Pagos y activos | 3–4 h | Emite/usa un activo de prueba o pago path |
| Un contrato Soroban guiado | 3–6 h | Compila, despliega e invoca un contrato del repo |
| Demo / poster | 1–2 h | Presenta el flujo Mermaid + evidencia on-chain |

**Entregable:** mini-proyecto en Testnet (pago + 1 contrato) con README de 1 página.

### Formato 3 — Curso / asignatura completa (12 semanas)

Útil para: optativa, diplomado, taller intensivo o materia electiva.

Usar el [syllabus oficial](syllabus.md) y los planes [weeks/semana-01.md](weeks/semana-01.md) … [semana-12.md](weeks/semana-12.md).

---

## 4. Temario modular (para armar tu propio calendario)

Cada módulo es **independiente hacia arriba**: puedes detenerte en A, en A+B, o seguir hasta E.

### Módulo A — Fundamentos de Stellar (obligatorio de arranque)

| Tema | Objetivos de aprendizaje | Recursos | Evaluación sugerida |
|---|---|---|---|
| Stellar vs otras cadenas | Contrastar cuentas, activos y operaciones nativas vs EVM | [teoria/01](teoria/01-stellar-vs-otras-cadenas.md) | Quiz corto / debate |
| Setup y primera cuenta | Instalar CLI, crear y fondear cuenta Testnet | [docs/instalacion](../docs/instalacion.md), [semana-01](weeks/semana-01.md) | Lab 1 |
| Ledger, tx, Horizon/RPC | Describir recorrido de una transacción | [teoria/03](teoria/03-arquitectura-red-y-ledger.md), [semana-03](weeks/semana-03.md) | Lab pago |

**Horas sugeridas:** 4–6 h de clase + 2 h de tarea.

### Módulo B — Consenso y red clásica

| Tema | Objetivos | Recursos | Evaluación |
|---|---|---|---|
| SCP / FBA | Explicar quórums y safety vs liveness | [teoria/02](teoria/02-consenso-scp.md), [semana-02](weeks/semana-02.md) | Reporte / Quiz 1 |
| Activos, trustlines, DEX | Emitir activo y crear oferta | [semana-04](weeks/semana-04.md) | Lab token |
| Multisig y control de cuentas | Configurar 2-de-3 | [semana-05](weeks/semana-05.md) | Lab multisig |

**Horas sugeridas:** 6–10 h.  
**Enlace curricular típico:** Sistemas Distribuidos, Seguridad, Redes.

### Módulo C — Contratos inteligentes Soroban

| Tema | Objetivos | Recursos | Evaluación |
|---|---|---|---|
| Intro Soroban + Rust | Compilar, desplegar, invocar | [semana-06](weeks/semana-06.md), `contracts/counter` | Lab counter |
| Auth, eventos, testing | Extender contrato con tests | [semana-07](weeks/semana-07.md), `contracts/payroll` | Lab + tests |
| Tokens SAC y composabilidad | Integrar vault/savings | [semana-08](weeks/semana-08.md), `yield` / `savings` | Lab integrado |
| Seguridad y fees | Revisar riesgos y recursos | [semana-09](weeks/semana-09.md), `loan` / `nft-membership` | Checklist seguridad |

**Horas sugeridas:** 10–16 h.  
**Enlace curricular típico:** Ingeniería de Software, Programación Avanzada, Arquitectura de Software.

**Caso industrial opcional (nuevo en el repo):** trazabilidad alimentaria con `contracts/food-trace` — útil en materias de IoT, logística, sistemas de información o proyectos interdisciplinarios.

### Módulo D — SEPs, anclas e integraciones

| Tema | Objetivos | Recursos | Evaluación |
|---|---|---|---|
| SEPs y web auth | Implementar autenticación SEP-10 | [semana-10](weeks/semana-10.md), [docs/sep](../docs/sep-estandares-anclas.md) | Lab auth |
| Adapters + frontend | Conectar protocolo externo / wallet | [semana-11](weeks/semana-11.md), `examples/integrations` | Mini UI |

**Horas sugeridas:** 6–10 h.  
**Enlace curricular típico:** Desarrollo Web, APIs, Fintech.

### Módulo E — Producción y proyecto

| Tema | Objetivos | Recursos | Evaluación |
|---|---|---|---|
| Hardening pre-mainnet | Aplicar checklist | [semana-12](weeks/semana-12.md), [checklist](../docs/checklist-pre-mainnet.md) | Rúbrica proyecto |
| Defensa / demo | Presentar evidencia on-chain | [proyecto-final](evaluacion/proyecto-final.md) | Defensa oral |

**Horas sugeridas:** 4–8 h + trabajo autónomo.

---

## 5. Mapa: materia escolar ↔ módulo del programa

| Materia / asignatura típica | Módulos que encajan | Entregable tipo |
|---|---|---|
| Introducción a la computación | A (cápsula) | Cuenta + pago Testnet |
| Redes de computadoras | A + B (consenso + Horizon) | Diagrama + captura de ledger |
| Sistemas distribuidos | B (SCP) + C intro | Comparativa PoW/PoS/SCP |
| Programación / algoritmos | C (contracts) | Tests de un contrato |
| Ingeniería de software | C + E | Proyecto con rúbrica |
| Seguridad informática | B multisig + C auth/seguridad | Threat model de 1 página |
| Bases de datos / SI | food-trace o payroll | Modelo de datos on-chain vs off-chain |
| Desarrollo web / móvil | D | UI + Freighter + invoke |
| Emprendimiento / fintech | A + B + D | Pitch + flujo SEP |
| Proyecto de titulación | Curso completo | Producto E2E en Testnet |

---

## 6. Secuencia pedagógica sugerida (por sesión)

Usa la [plantilla de clase](recursos/plantilla-clase.md). Ritmo canónico:

```text
5 min   Apertura / recap
45-60   Teoría (máx. 3 puntos clave + 1 contraste con EVM o Web2)
15-20   Demo en vivo (CLI + explorador)
60-90   Lab guiado (entregable concreto)
5       Cierre + tarea
```

**Principios didácticos de este material:**

1. **Ver siempre on-chain** — proyecta Stellar Lab / Expert; el hash ancla el concepto.  
2. **Solo Testnet en clase** — evita riesgo y costos; Friendbot fondea.  
3. **Aprender por contraste** — “en Web2 / en EVM esto se hace así; aquí así”.  
4. **Entregables reproducibles** — el alumno debe poder re-ejecutar desde su README.  
5. **IA permitida para aprender, no para sustituir la defensa** — el estudiante explica su código.

---

## 7. Evaluación lista para el LMS

Puedes adoptar la tabla del [syllabus](syllabus.md) o una versión **reducida** para unidad didáctica:

### Opción curso completo (12 semanas)

| Componente | Peso |
|---|---|
| Quizzes (5) | 20% |
| Labs | 35% |
| Proyecto final | 35% |
| Participación / portafolio | 10% |

Detalle: [evaluacion/rubricas.md](evaluacion/rubricas.md), [quizzes.md](evaluacion/quizzes.md), [proyecto-final.md](evaluacion/proyecto-final.md).

### Opción unidad corta (8–16 h)

| Componente | Peso |
|---|---|
| Lab práctico | 50% |
| Quiz o reporte teórico | 20% |
| Mini-demo / poster | 20% |
| Bitácora / README | 10% |

**Aprobación sugerida:** ≥ 70% y al menos un lab con evidencia on-chain (hash o ID de contrato).

---

## 8. Checklist del profesor (antes del primer día)

### Ambiente técnico
- [ ] Node.js LTS, Rust + Cargo, Stellar CLI, Git instalados en tu máquina (y, si aplica, en el lab).  
- [ ] Guía: [docs/instalacion.md](../docs/instalacion.md).  
- [ ] Identidad Testnet de demo (`stellar keys generate` + Friendbot).  
- [ ] Freighter instalado si vas a mostrar wallet.  
- [ ] Probar en la sala: `stellar contract build` y un pago de prueba.

### Diseño de clase
- [ ] Elegir formato (cápsula / unidad / curso 12 semanas).  
- [ ] Publicar syllabus o esta guía + criterios de evaluación el día 1.  
- [ ] Definir canal de entrega (repo GitHub del alumno, LMS, Drive).  
- [ ] Preparar 1 “plan B” offline (slides + diagrama) por si cae Testnet/RPC.

### Pedagogía
- [ ] Leer [plantilla-clase.md](recursos/plantilla-clase.md).  
- [ ] Tener a mano el [glosario](recursos/glosario.md) para proyectar términos.  
- [ ] Anticipar 3 errores frecuentes (path del WASM, workspace Cargo, contrato no inicializado).

---

## 9. Errores frecuentes en el aula (y cómo anticiparlos)

| Síntoma | Causa típica | Qué decir / hacer |
|---|---|---|
| `manifest path does not exist` | Ejecutan el comando desde `src/` u otra carpeta | “La ruta es relativa al directorio actual; vuelve a la raíz del repo” |
| Workspace member missing / multiple roots | Carpetas vacías o workspaces anidados en `contracts/` | Revisar `Cargo.toml` raíz; excluir workspaces anidados |
| `Error(Contract, #1)` en invoke | Contrato no inicializado | Primero `initialize`, luego la función de negocio |
| Amount “raro” en XLM | Confunden XLM con stroops | 1 XLM = 10 000 000 stroops |
| Deploy falla | Identidad sin fondos o red incorrecta | Friendbot + `--network testnet` |

---

## 10. Proyectos sugeridos por nivel (para evaluación o feria)

| Nivel | Proyecto | Contratos / docs |
|---|---|---|
| Intro | Remesa educativa: 2 cuentas + pago + explicación de fees | `exercises/01` |
| Intermedio | Token de campus + oferta en DEX | Semana 4 |
| Intermedio–avanzado | Nómina o ahorro on-chain | `payroll`, `savings` |
| Avanzado | Vault de yield con XLM/SAC | `yield` |
| Industria / interdisciplinario | Trazabilidad de un alimento (lote → transporte → recall) | `food-trace` |
| Full stack | UI + Freighter + adapter | `examples/integrations`, semana 11 |

---

## 11. Formación mínima del docente (si tú también estás aprendiendo)

Orden recomendado **antes** de dar la primera clase:

1. Leer [docs/introduccion.md](../docs/introduccion.md) y [teoria/01](teoria/01-stellar-vs-otras-cadenas.md) (1–2 h).  
2. Completar setup + [pago simple](../exercises/01-pago-simple.md) (1–2 h).  
3. Leer [teoria/02-consenso-scp.md](teoria/02-consenso-scp.md) a nivel “puedo dibujarlo en pizarra” (2 h).  
4. Compilar e invocar `contracts/counter` o `contracts/yield` (2–3 h).  
5. Hojear una semana de [weeks/](weeks/) y la [plantilla](recursos/plantilla-clase.md) (1 h).

**Total estimado de preparación:** 8–12 h para una unidad corta; 20–30 h si vas a dar el curso completo.

---

## 12. Resultados de aprendizaje por formato (para tu programa oficial)

### Cápsula (2–4 h)
El estudiante **crea** una cuenta en Testnet y **ejecuta** un pago, **identificando** elementos de una transacción Stellar.

### Unidad (8–16 h)
El estudiante **explica** diferencias de Stellar frente a otra arquitectura, **opera** la red de prueba y **despliega** al menos un contrato Soroban, documentando el flujo.

### Curso (12 semanas)
El estudiante **diseña e implementa** un producto sobre Stellar/Soroban (contratos + opcional SEP/UI), **justifica** decisiones de seguridad y **defiende** evidencia on-chain conforme a la rúbrica del proyecto final.

---

## 12.1 Plan de capacitación docente (2 sesiones de 4 horas)

Pensado para formación de profesorado universitario que luego impartirá el programa en sus materias.

### Sesión 1 (4 horas) — enfoque pedagógico y diseño de clase

| Bloque | Duración | Resultado esperado |
|---|---:|---|
| Diagnóstico y objetivos de capacitación | 30 min | Definir contexto de cada profesor (materia, semestre, nivel) |
| Competencias y encaje curricular | 60 min | Mapeo materia ↔ módulos (A–E) listo |
| Fundamentos Stellar para docencia | 60 min | Guion conceptual para explicar en aula |
| Taller de microplaneación | 75 min | 1 clase de 90–120 min diseñada por profesor |
| Revisión entre pares y ajustes | 15 min | Versión final de microplaneación |

**Entregable sesión 1:** microplaneación docente con objetivo, secuencia didáctica, actividad práctica y criterio de evaluación.

### Sesión 2 (4 horas) — implementación técnica y evaluación

| Bloque | Duración | Resultado esperado |
|---|---:|---|
| Demo integral (setup → invoke) | 50 min | Flujo técnico completo entendido |
| Práctica guiada por profesor | 90 min | Flujo reproducido con evidencia on-chain |
| Adaptación de rúbricas al curso | 50 min | Evaluación lista para plataforma institucional |
| Simulación de mini clase docente | 35 min | Ensayo de impartición con feedback |
| Cierre y plan de implementación | 15 min | Compromiso de aplicación en su curso |

**Entregable sesión 2:** kit docente (guion de clase + comandos probados + rúbrica + evidencia on-chain).

### Evidencias mínimas para certificar capacitación

- Microplaneación de al menos 1 sesión universitaria (90–120 min).  
- Evidencia on-chain reproducible (hash de tx o ID de contrato).  
- Instrumento de evaluación listo (rúbrica o checklist).  
- Plan de implementación con semana/materia concreta.

---

## 13. Cómo citar / compartir este material

- Repo: Stellar Guide (talleres en español) — carpeta `course/`.  
- Syllabus intensivo: [syllabus.md](syllabus.md).  
- Esta guía docente: `course/programa-docentes.md`.  
- Licencia / créditos: seguir el README del repositorio; autor del curso: **Gerry Vela**.

---

## 14. Próximo paso práctico

1. Elige formato (cápsula / unidad / 12 semanas).  
2. Copia la tabla de evaluación que corresponda a tu LMS.  
3. Abre la semana de arranque: [weeks/semana-01.md](weeks/semana-01.md).  
4. Si vas a industria alimentaria u otro caso vertical, usa `contracts/food-trace` como lab de cierre del Módulo C.

**Presentación para impartir:** abre en el navegador [`../presentacion/docentes.html`](../presentacion/docentes.html) (16 diapositivas, notas del presentador con tecla **N**).

¿Dudas de encaje curricular? Parte de la sección 5 (mapa materia ↔ módulo) y recorta por horas disponibles; el material está pensado para **recortar sin romper la progresión**.
