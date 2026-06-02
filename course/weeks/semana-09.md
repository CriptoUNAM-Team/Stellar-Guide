# Semana 9 · Seguridad, upgrades y optimización de recursos

**Módulo 3 · Contratos Soroban** · Teoría + lab. **Cierre de módulo: Quiz 3 + checkpoint proyecto.**

## Objetivos de aprendizaje
- Identificar vulnerabilidades comunes en contratos Soroban.
- Aplicar el patrón de upgrade de WASM.
- Optimizar uso de recursos/fees y manejo de TTL.

## Lecturas previas
- `contracts/loan/src/lib.rs` o `contracts/nft-membership/src/lib.rs`.
- [../../docs/checklist-pre-mainnet.md](../../docs/checklist-pre-mainnet.md)

## Guion de teoría (≈50 min)
1. Superficie de ataque: auth faltante, overflow, reentrada, estado expirado, valores no validados. (15 min)
2. Patrón de upgrade: `update_current_contract_wasm` y control de quién puede actualizar. (10 min)
3. TTL/rent: estrategias para extender storage y evitar archivado inesperado. (10 min)
4. Optimización: minimizar lecturas/escrituras, tamaño de tipos, batching. (10 min)
5. Auditoría: checklist y herramientas. (5 min)

> **Callout "Vienes de Solidity":** muchos bugs son análogos (auth, overflow) pero añade dos propios de
> Soroban: **estado que expira** y **autorización explícita** mal propagada en llamadas cross-contract.

## Demo en vivo
- Introducir un bug de auth en una copia de `loan`, mostrar el exploit en test, luego corregirlo.
- Ejecutar `/security-review` (o revisión manual) sobre el diff.

## Lab calificado 9 — "Auditoría y hardening"
**Entregable:**
1. Tomar `loan` o `nft-membership` y escribir un reporte de auditoría (mín. 4 hallazgos potenciales).
2. Corregir al menos 2 con tests que prueben la corrección.
3. Implementar y probar una función de upgrade protegida por auth de admin.
4. Medir antes/después el fee de una invocación tras optimizar.

## Quiz 3 (cierre de Módulo 3)
Temas: Soroban (ejecución, storage/TTL, auth, eventos, testing, SAC, seguridad).
Banco: [../evaluacion/quizzes.md](../evaluacion/quizzes.md#quiz-3).

## Checkpoint del proyecto final
Entregar: idea, contratos/SEPs a usar, arquitectura (diagrama), plan de 3 semanas.
Especificación: [../evaluacion/proyecto-final.md](../evaluacion/proyecto-final.md).

## Recursos
- https://developers.stellar.org/docs/build/guides/conventions
- https://developers.stellar.org/docs/build/smart-contracts/example-contracts/upgrading-wasm-bytecode
