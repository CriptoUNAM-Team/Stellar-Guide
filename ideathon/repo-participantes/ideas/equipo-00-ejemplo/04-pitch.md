# 04 · Guion del pitch (3 minutos)

> ⚠️ Ejemplo de referencia.

| Tiempo | Sección | Guion |
|---|---|---|
| 0:00–0:30 | **Problema** | Un proveedor de autopartes en el Bajío entrega hoy y cobra en 90 días. Para aguantar, paga factoraje al 28 %: hasta 250 mil pesos al año sobre una cartera de 4 millones. |
| 0:30–0:50 | **Cliente** | Proveedores Tier 2 de 30 a 80 empleados. Solo en el corredor del Bajío hay cientos, y CANACINTRA nos da la puerta de entrada. |
| 0:50–1:50 | **Solución y Stellar** | Registramos el hash del CFDI en Stellar: la factura ya no se puede financiar dos veces. La armadora la confirma, un inversionista la fondea, y el pago se libera solo cuando entra el dinero. Test de 4 preguntas: 4 de 4 — tres partes que no se confían, auditoría de terceros, capital extranjero y liquidación automática. |
| 1:50–2:20 | **Evidencia** | Flujo completo diseñado y un adelanto simulado en Testnet con el UUID de la factura en el memo. El hash está en nuestro PR. |
| 2:20–3:00 | **Modelo y siguiente paso** | 1.2 % sobre el monto adelantado; margen de 0.54 % después del costo del ancla. El lunes le llevamos una factura real de 500 mil a una empresa de estampado en Silao para correr el piloto en Testnet. |

## Preguntas que nos van a hacer (y nuestra respuesta)

| Pregunta probable | Respuesta preparada |
|---|---|
| ¿Por qué no una base de datos? | Porque el inversionista no tiene por qué confiar en *nuestra* base de datos. La unicidad de la factura tiene que ser verificable por él, no prometida por nosotros. |
| ¿Quién opera el ancla? | En el piloto, un ancla existente con operación en México; no construimos el on/off ramp nosotros. |
| ¿Qué pasa si la armadora no paga? | El riesgo es del inversionista, igual que en el factoraje tradicional; por eso arrancamos solo con armadoras investment grade y facturas confirmadas. |
| ¿Cuál es su riesgo mayor? | El regulatorio. Por eso el piloto es con inversionistas institucionales y cesión de derechos tradicional. |

## Equipo

| Nombre | Usuario GitHub | Rol en el equipo |
|---|---|---|
| Ejemplo Uno | @ejemplo-uno | Negocio y entrevistas |
| Ejemplo Dos | @ejemplo-dos | Modelo financiero |
| Ejemplo Tres | @ejemplo-tres | Diagramas y Testnet |
