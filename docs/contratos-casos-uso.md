# Contratos Soroban por Casos de Uso (Starter Pack)

Guía orientada a nuevos desarrolladores para pasar de "idea" a "MVP ejecutable" en Stellar.

## Enfoque recomendado (actual)

- **Contratos**: Soroban (Rust + WASM).
- **Lectura/escritura on-chain**: prioriza **Stellar RPC** para apps nuevas; usa Horizon cuando trabajes con flujos clásicos de cuentas/pagos.
- **Assets**: usa Stellar Asset Contract (SAC) para activos fungibles y coleccionables tokenizados.
- **Pruebas**: combina unit tests del contrato + pruebas end-to-end en Testnet.
- **Seguridad**: valida auth explícitamente, define límites y maneja errores de forma determinística.

---

## 1) Préstamos colateralizados (Loan Vault)

### Problema que resuelve
Permite prestar un activo (p. ej. USDC testnet) usando otro activo como colateral.

### Modelo mínimo
- `create_position(borrower, collateral_amount)` transfiere colateral al contrato
- `borrow(position_id, debt_amount)` / `repay(position_id, amount)` con transfers SAC
- `liquidate(liquidator, position_id)` si el ratio cae bajo umbral
- `set_min_collateral_bps` (admin) para demo de liquidación sin oráculo

### Datos clave on-chain
- Colateral depositado por usuario.
- Deuda total + tasa.
- Umbral de liquidación (LTV máximo).
- Estado de la posición (`Active`, `Liquidatable`, `Closed`).

### Eventos útiles
- `position_opened`
- `debt_borrowed`
- `repaid`
- `liquidated`

### Riesgos y controles
- **Oráculos**: nunca asumas precio hardcoded en producción.
- **Liquidación parcial**: evita que una sola liquidación destruya toda la posición.
- **Límites**: tope por usuario y por pool.

---

## 2) Dispersión de Nómina (Payroll Disburser)

### Problema que resuelve
Paga a múltiples empleados/proveedores en una sola corrida.

### Modelo mínimo
- `create_payroll(period_id, asset, total_budget)`
- `add_recipient(period_id, recipient, amount)`
- `approve_period(period_id)` (firma admin/multisig)
- `execute_period(period_id)` (dispersa fondos)

### Datos clave on-chain
- `period_id` (ej. `2026-04`).
- Lista de destinatarios y montos.
- Estado (`Draft`, `Approved`, `Executed`).
- Hash de evidencia off-chain (CSV o nómina firmada).

### Eventos útiles
- `period_approved`
- `payment_sent`
- `period_executed`

### Riesgos y controles
- **Idempotencia**: `execute_period` no debe pagar dos veces.
- **Multisig**: requerir aprobación de más de un firmante para montos altos.
- **Trazabilidad**: registra hash del archivo fuente.

---

## 3) Ahorro programado (Goal-based Savings)

### Problema que resuelve
Permite depósitos periódicos y bloqueo voluntario hasta fecha objetivo.

### Modelo mínimo
- `create_goal(user, target_amount, unlock_time)`
- `deposit(goal_id, amount)`
- `withdraw(goal_id)` solo cuando `now >= unlock_time` (o con penalización).
- `cancel_goal(goal_id)` si se permite cierre anticipado.

### Datos clave on-chain
- Meta y progreso actual.
- Fecha de desbloqueo.
- Configuración de penalización/costo por retiro anticipado.

### Eventos útiles
- `goal_created`
- `goal_funded`
- `goal_unlocked`
- `goal_closed`

### Riesgos y controles
- **Tiempos**: define reglas claras de `unlock_time`.
- **UX**: separa "saldo total" vs "saldo disponible".
- **Penalizaciones**: explícitas y auditables.

---

## 4) Yield básico (Vault de rendimiento)

### Problema que resuelve
Agrupa depósitos en una bóveda y distribuye rendimiento proporcional por share.

### Modelo mínimo
- `deposit(user, amount) -> shares`
- `withdraw(user, shares) -> amount`
- `harvest(yield_amount)` (actualiza índice de rendimiento)

### Datos clave on-chain
- Total de activos en vault.
- Total de shares emitidas.
- Índice acumulado de rendimiento por share.

### Eventos útiles
- `vault_deposit`
- `vault_withdraw`
- `vault_harvest`

### Riesgos y controles
- **Cálculo de shares**: cuidado con redondeos.
- **Front-running lógico**: define ventanas o snapshots para `harvest`.
- **Límites de retiro**: opcionales para estabilidad.

---

## 5) NFTs / Membresías / Certificados

### Problema que resuelve
Representa pertenencia, acceso o certificación on-chain.

### Modelo mínimo
- `mint(to, token_id, metadata_uri)`
- `transfer(from, to, token_id)` (si es transferible)
- `burn(token_id)` (si aplica)

### Datos clave on-chain
- `token_id`
- Propietario actual
- `metadata_uri` (JSON/IPFS/HTTPS)
- Flags de transferibilidad/revocabilidad

### Eventos útiles
- `nft_minted`
- `nft_transferred`
- `nft_burned`

### Riesgos y controles
- **Metadata mutable/inmutable**: decide desde diseño.
- **Permisos de minteo**: rol admin o lista de emisores.
- **Privacidad**: no guardar PII en metadata pública.

---

## 6) Campus: asistencia, votación, calificaciones

Tres contratos pequeños para talleres con profesores. El alumno es una `Address`; el profesor es `admin`.

### Asistencia (`contracts/attendance`)
- `open_session(course, topic)`
- `mark_present(session_id, student)` (no se puede marcar dos veces)
- `attendance_count(student)`

### Votación (`contracts/voting`)
- `create_proposal(title)`
- `vote(proposal_id, voter, support)` — un voto por address
- `close_proposal` / `get_proposal`

### Calificaciones (`contracts/grades`)
- `record_grade(student, assignment, score, max_score)` — solo admin
- `get_grade` — lectura pública

## 7) AMM de producto constante (`contracts/amm`)

Pool de dos SAC. `add_liquidity` y `swap_a_for_b` con `x * y = k` (sin fee). Sirve para mostrar precio implícito y slippage en el lab.

## 8) Más campus: biblioteca, inscripción, depósito

### Biblioteca (`contracts/library`)
- `add_title(title, copies)`
- `checkout` / `return_copy` (el alumno firma)
- Un préstamo por alumno y título; `available` no baja de 0

### Inscripción (`contracts/enrollment`)
- `create_course(name, capacity)`
- `enroll` / `drop_course`
- Cupo lleno → `CourseFull`

### Depósito condicional (`contracts/escrow`)
- `lock(payer, payee, amount)` transfiere al contrato
- El árbitro `release` al payee o `refund` al payer
- Un trato cerrado no se mueve otra vez

## Estructura base para cualquier contrato

1. **Roles**: admin, operador, usuario final.
2. **Auth**: `require_auth` para toda acción sensible.
3. **Estados válidos**: enum claro + transiciones permitidas.
4. **Límites**: montos máximos, ventanas de tiempo, pausas de emergencia.
5. **Eventos**: cada operación de negocio debe emitir evento.
6. **Errores tipados**: códigos de error estables para cliente.

## Plan de pruebas recomendado

- **Unitarias**: validaciones, auth, transiciones de estado y límites.
- **Propiedad/invariantes**: no crear fondos de la nada, no doble ejecución.
- **Integración**: contrato + SDK cliente + Testnet.
- **Casos de falla**: permisos inválidos, saldo insuficiente, periodo expirado.

## Roadmap para este repositorio

1. Empezar por `Payroll Disburser` (más simple de explicar en talleres).
2. Extraer plantillas de storage + auth + eventos reutilizables.
3. Añadir carpeta `contracts/<caso>/` con README y comandos de deploy.
4. Agregar ejercicios guiados en `exercises/` por cada caso.
5. Cerrar con una sección de seguridad y checklist pre-mainnet.

## Implementaciones incluidas en este repo

- `contracts/payroll`: dispersión de pagos por periodo (`disperse_period`) con protección contra doble ejecución.
- `contracts/savings`: ahorro por metas con `unlock_time` y penalización por retiro anticipado (`penalty_bps`).
- `contracts/loan`: préstamo colateralizado con transfers reales, LTV por `min_collateral_bps` y `liquidate`.
- `contracts/yield`: bóveda de rendimiento por shares con `deposit`, `harvest` y `withdraw`.
- `contracts/amm`: pool `x * y = k` (`add_liquidity`, `swap_a_for_b`).
- `contracts/nft-membership`: emisión y transferencia básica de NFTs de membresía/certificados.
- `contracts/attendance`: sesiones de clase y lista de asistencia por `Address`.
- `contracts/voting`: propuestas sí/no, un voto por address, cierre y resultados.
- `contracts/grades`: calificaciones por alumno/actividad; solo el admin escribe.
- `contracts/library`: préstamo de ejemplares con cupo y devolución.
- `contracts/enrollment`: inscripción a curso con `capacity`.
- `contracts/escrow`: depósito condicional; el árbitro libera o reembolsa.
