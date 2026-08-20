// Motor compartido por las páginas de contrato.
// Cada página define window.CONTRATO = "<nombre>" antes de cargar este archivo.

import {
  contract,
  Networks,
  Keypair,
} from "https://esm.sh/@stellar/stellar-sdk@16";

const RPC = "https://soroban-testnet.stellar.org";
const RED = Networks.TESTNET;

const nombre = window.CONTRATO;
const spec = window.SPECS[nombre];
const info = window.CONTRATOS[nombre];

const $ = (sel, raiz = document) => raiz.querySelector(sel);
const crear = (tag, props = {}) => Object.assign(document.createElement(tag), props);

const erroresContrato = {};
for (const e of spec) {
  if (e.type === "enum") for (const c of e.cases) erroresContrato[c.value] = `${e.name}::${c.name}`;
}

function desenvolver(valor) {
  if (valor && typeof valor === "object" && "value" in valor && Object.keys(valor).length === 1) {
    return valor.value;
  }
  return valor;
}

function bonito(valor) {
  return JSON.stringify(desenvolver(valor), (_k, v) => (typeof v === "bigint" ? v.toString() : v), 2);
}

function explicarError(texto) {
  const m = /Error\(Contract,\s*#(\d+)\)/.exec(texto);
  if (m) {
    const cod = Number(m[1]);
    const nom = erroresContrato[cod];
    return nom
      ? `Error del contrato #${cod} — ${nom}\n\n${texto}`
      : `Error del contrato #${cod}\n\n${texto}`;
  }
  if (/MissingValue/.test(texto)) {
    return `El contrato no encontró un dato que esperaba.\n¿Corriste initialize?\n\n${texto}`;
  }
  return texto;
}

const LECTURA_EXTRA = new Set(["attendance_count", "get_reserves"]);
function esLectura(n) {
  return /^(get|is|has|list)_/.test(n) || LECTURA_EXTRA.has(n);
}

function enumDelSpec(nombreTipo) {
  return spec.find((e) => e.type === "enum" && e.name === nombreTipo);
}

function convertir(valorCrudo, tipo) {
  const t = tipo.type;
  if (t === "bool") return Boolean(valorCrudo);
  if (["u32", "i32"].includes(t)) {
    if (valorCrudo === "") throw new Error("falta un número");
    return Number(valorCrudo);
  }
  if (["u64", "i64", "u128", "i128", "u256", "i256", "timepoint", "duration"].includes(t)) {
    if (valorCrudo === "") throw new Error("falta un número");
    return BigInt(valorCrudo);
  }
  if (t === "custom") {
    if (valorCrudo === "") throw new Error("elige una opción");
    const en = enumDelSpec(tipo.name);
    if (en) return valorCrudo;
    return valorCrudo;
  }
  if (valorCrudo === "") throw new Error("falta un valor");
  return valorCrudo;
}

function campoPara(entrada) {
  const tipo = entrada.value;
  const t = tipo.type;
  const env = crear("div", { className: "campo" });
  const lab = crear("label");
  lab.textContent = entrada.name;
  lab.appendChild(crear("span", { className: "tipo", textContent: t === "custom" ? tipo.name : t }));
  env.appendChild(lab);

  let control;
  const en = t === "custom" ? enumDelSpec(tipo.name) : null;
  if (t === "bool") {
    control = crear("select");
    control.append(
      crear("option", { value: "true", textContent: "true" }),
      crear("option", { value: "false", textContent: "false" }),
    );
  } else if (en) {
    control = crear("select");
    control.append(crear("option", { value: "", textContent: `— ${en.name} —` }));
    en.cases.forEach((c) => {
      control.append(crear("option", { value: c.name, textContent: `${c.name} (${c.value})` }));
    });
  } else {
    control = crear("input", { type: "text" });
    if (t === "address") control.placeholder = "G… (cuenta) o C… (contrato)";
    else if (["u32", "i32", "u64", "i64", "u128", "i128"].includes(t)) {
      control.placeholder = t.startsWith("u") ? "entero ≥ 0" : "entero";
      control.inputMode = "numeric";
    } else control.placeholder = t;
  }
  control.dataset.arg = entrada.name;
  control.dataset.tipo = t;
  env.appendChild(control);

  if (t === "address") {
    const fila = crear("div", { className: "fila-campo" });
    const usar = crear("button", { type: "button", className: "mini", textContent: "usar cuenta conectada" });
    usar.onclick = () => {
      if (!identidad.publicKey) {
        alert("Conecta Freighter o una clave secreta primero.");
        return;
      }
      control.value = identidad.publicKey;
    };
    fila.appendChild(usar);
    env.appendChild(fila);
  }
  return env;
}

const identidad = { publicKey: null, firmante: null, origen: null };

function pintarIdentidad() {
  const est = $("#estado-cuenta");
  if (!identidad.publicKey) {
    est.innerHTML = "Sin cuenta. Puedes <em>Consultar</em> (solo lectura); para <em>Ejecutar</em> necesitas firmar.";
    return;
  }
  est.innerHTML = `<strong>conectado</strong> (${identidad.origen}) — ${identidad.publicKey}`;
}

async function conectarFreighter() {
  const api = window.freighterApi;
  if (!api) {
    alert("No se detectó Freighter.\nInstálalo (extensión) o usa una clave secreta S… de testnet.");
    return;
  }
  try {
    if (api.setAllowed) await api.setAllowed();
    const dir = (await api.getAddress?.()) ?? (await api.getPublicKey?.());
    const pk = typeof dir === "string" ? dir : dir?.address;
    if (!pk) throw new Error("Freighter no devolvió dirección");
    identidad.publicKey = pk;
    identidad.origen = "Freighter";
    identidad.firmante = {
      signTransaction: async (xdrTx, opts) => {
        const r = await api.signTransaction(xdrTx, { networkPassphrase: RED, address: pk, ...opts });
        return typeof r === "string" ? { signedTxXdr: r, signerAddress: pk } : r;
      },
    };
    pintarIdentidad();
  } catch (e) {
    alert("Freighter: " + e.message);
  }
}

function usarSecreta() {
  const sk = $("#clave-secreta").value.trim();
  if (!sk.startsWith("S")) {
    alert("La clave secreta de testnet empieza con S.");
    return;
  }
  try {
    const kp = Keypair.fromSecret(sk);
    identidad.publicKey = kp.publicKey();
    identidad.origen = "clave secreta (testnet)";
    identidad.firmante = contract.basicNodeSigner(kp, RED);
    $("#clave-secreta").value = "";
    pintarIdentidad();
  } catch {
    alert("Esa clave secreta no es válida.");
  }
}

function contratoIdActual() {
  const campo = $("#campo-id");
  const escrito = campo?.value.trim();
  if (escrito) return escrito;
  return window.idContrato(nombre);
}

async function clienteActual() {
  const contractId = contratoIdActual();
  if (!contractId || !contractId.startsWith("C")) {
    throw new Error("Pega el Contract ID (C…) de testnet arriba. Es el que imprime stellar contract deploy.");
  }
  return contract.Client.from({
    contractId,
    networkPassphrase: RED,
    rpcUrl: RPC,
    publicKey: identidad.publicKey ?? undefined,
    signTransaction: identidad.firmante?.signTransaction,
  });
}

async function ejecutar(fn, cuerpo, enviar) {
  const salida = $(".salida", cuerpo);
  const botones = cuerpo.querySelectorAll("button");
  salida.className = "salida";
  salida.textContent = enviar ? "Firmando y enviando…" : "Simulando…";
  botones.forEach((b) => (b.disabled = true));

  try {
    const args = {};
    for (const entrada of fn.inputs) {
      const control = cuerpo.querySelector(`[data-arg="${entrada.name}"]`);
      const crudo = control.tagName === "SELECT" ? (control.value === "true" || control.value === "false" ? control.value === "true" : control.value.trim()) : control.value.trim();
      try {
        args[entrada.name] = convertir(crudo, entrada.value);
      } catch (e) {
        throw new Error(`Argumento "${entrada.name}" (${entrada.value.type}): ${e.message}`);
      }
    }

    const cliente = await clienteActual();
    const tx = await cliente[fn.name](args);

    if (tx.simulation?.error) throw new Error(explicarError(String(tx.simulation.error)));

    if (!enviar) {
      salida.classList.add("ok");
      salida.textContent =
        `simulación OK\n\nresultado:\n${bonito(tx.result)}` +
        (esLectura(fn.name) ? "" : "\n\n(no se envió nada a la red; usa Ejecutar para escribir)");
      return;
    }

    if (!identidad.firmante) throw new Error("Conecta una cuenta antes de ejecutar.");

    const enviado = await tx.signAndSend();
    salida.classList.add("ok");
    const hash = enviado.sendTransactionResponse?.hash ?? enviado.getTransactionResponse?.txHash ?? "";
    salida.textContent =
      `enviado a testnet\n\nresultado:\n${bonito(enviado.result)}` +
      (hash ? `\n\nhash: ${hash}\nhttps://stellar.expert/explorer/testnet/tx/${hash}` : "");
  } catch (e) {
    salida.classList.add("error");
    salida.textContent = explicarError(e?.message ?? String(e));
  } finally {
    botones.forEach((b) => (b.disabled = false));
  }
}

function bloqueFuncion(fn) {
  const lectura = esLectura(fn.name);
  const det = crear("details", { className: "funcion" });
  if (fn.name === "initialize") det.open = true;

  const sum = crear("summary");
  sum.append(
    crear("span", { className: "nombre-fn", textContent: fn.name }),
    crear("span", {
      className: "firma",
      textContent: `(${fn.inputs.map((i) => `${i.name}: ${i.value.type === "custom" ? i.value.name : i.value.type}`).join(", ")})`,
    }),
    crear("span", {
      className: `etiqueta ${lectura ? "lectura" : "escritura"}`,
      textContent: lectura ? "lectura" : "escritura",
    }),
  );
  det.appendChild(sum);

  const cuerpo = crear("div", { className: "cuerpo-fn" });
  if (fn.doc) cuerpo.appendChild(crear("p", { className: "ayuda-fn", textContent: fn.doc.split("\n")[0] }));
  fn.inputs.forEach((i) => cuerpo.appendChild(campoPara(i)));

  const acciones = crear("div", { className: "acciones" });
  const bConsultar = crear("button", { textContent: "Consultar (simular)" });
  bConsultar.onclick = () => ejecutar(fn, cuerpo, false);
  acciones.appendChild(bConsultar);

  if (!lectura) {
    const bEjecutar = crear("button", { className: "primario", textContent: "Ejecutar (firmar y enviar)" });
    bEjecutar.onclick = () => ejecutar(fn, cuerpo, true);
    acciones.appendChild(bEjecutar);
  }
  cuerpo.appendChild(acciones);
  cuerpo.appendChild(crear("div", { className: "salida" }));
  det.appendChild(cuerpo);
  return det;
}

function pintarMetaId() {
  const id = contratoIdActual();
  const bId = $("#id-contrato");
  const explorer = $("#ver-explorer");
  const chip = $("#chip-estado");
  if (id) {
    bId.textContent = id;
    explorer.href = `https://stellar.expert/explorer/testnet/contract/${id}`;
    explorer.style.display = "";
    if (chip) {
      chip.textContent = "ligado a testnet";
      chip.className = "chip ok";
    }
  } else {
    bId.textContent = "pega un Contract ID C…";
    explorer.removeAttribute("href");
    explorer.style.display = "none";
    if (chip) {
      chip.textContent = "sin desplegar";
      chip.className = "chip";
    }
  }
}

function construir() {
  $("#nombre-contrato").textContent = info.titulo;
  $("#desc-contrato").textContent = info.desc;
  document.title = `${info.titulo} — Stellar Guide`;

  const campo = $("#campo-id");
  campo.value = window.idContrato(nombre);
  campo.placeholder = "C… del deploy en testnet";
  $("#btn-guardar-id").onclick = () => {
    const id = campo.value.trim();
    if (id && !id.startsWith("C")) {
      alert("El Contract ID de Soroban empieza con C.");
      return;
    }
    window.guardarIdContrato(nombre, id);
    pintarMetaId();
  };
  $("#btn-restaurar-id").onclick = () => {
    window.guardarIdContrato(nombre, "");
    campo.value = info.id || "";
    pintarMetaId();
  };

  const bId = $("#id-contrato");
  bId.title = "clic para copiar";
  bId.onclick = () => {
    const id = contratoIdActual();
    if (!id) return;
    navigator.clipboard?.writeText(id);
    const antes = bId.textContent;
    bId.textContent = "¡copiado!";
    setTimeout(() => (bId.textContent = antes), 900);
  };
  pintarMetaId();

  const fns = spec.filter((e) => e.type === "function");
  const lecturas = fns.filter((f) => esLectura(f.name)).sort((a, b) => a.name.localeCompare(b.name));
  const escrituras = fns.filter((f) => !esLectura(f.name)).sort((a, b) => a.name.localeCompare(b.name));

  const cont = $("#funciones");
  if (escrituras.length) {
    cont.appendChild(crear("h2", { className: "seccion", textContent: "Escritura — firman y cambian estado" }));
    escrituras.forEach((f) => cont.appendChild(bloqueFuncion(f)));
  }
  if (lecturas.length) {
    cont.appendChild(crear("h2", { className: "seccion", textContent: "Lectura — solo consultan" }));
    lecturas.forEach((f) => cont.appendChild(bloqueFuncion(f)));
  }

  $("#btn-freighter").onclick = conectarFreighter;
  $("#btn-secreta").onclick = usarSecreta;
  pintarIdentidad();
}

construir();
