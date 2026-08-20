# Frontend — apps de clase

Tres pantallas de flujo real (lista, urna, boleta) y exploradores de funciones para el resto de contratos.

La **lectura** no necesita wallet. La **escritura** la firma Freighter. La página nunca pide una clave `S…`.

## Cómo abrirlo

```bash
cd frontend
python3 -m http.server 8080
```

Abre [http://localhost:8080/](http://localhost:8080/) o [attendance.html](http://localhost:8080/attendance.html).

Sirve por `http://localhost`. Freighter no firma en `file://`.

## Demo de lista

1. El grupo abre la lista: ya se ve el tablero (sesión y roster).
2. El admin conecta Freighter (cuenta `profe`, importada *dentro* de la extensión) y abre sesión si hace falta.
3. Cada alumno conecta su cuenta y toca **Estoy presente**. Freighter pide firmar.

Sin extensión, la CLI firma igual:

```bash
stellar contract invoke --id CBQ2RV6RGJMGOJGJRMV6JFCYIVMBQNPDGUNW57I2YJCG2RQU5T7MD2BO \
  --source alumno --network testnet -- \
  mark_present --session_id 1 --student alumno
```

El contrato vive en Testnet:

`CBQ2RV6RGJMGOJGJRMV6JFCYIVMBQNPDGUNW57I2YJCG2RQU5T7MD2BO`

Quien firma `mark_present` es el alumno, no el admin.
