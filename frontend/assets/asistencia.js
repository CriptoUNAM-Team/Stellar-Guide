import {
  identidad,
  cliente,
  esProfe,
  etiquetaCuenta,
  explicarError,
  nativo,
} from "./wallet.js";
import { aviso, pintarCabecera, cablearFreighter, cmdMarcar } from "./aula.js";

const ERRORES = {
  1: "El contrato no está listo (initialize).",
  2: "Ya estaba inicializado.",
  3: "Todavía no hay sesión.",
  4: "Esta cuenta ya pasó lista.",
};

const VIEJO = "CAIV7NKO23LPMPXWQRCK44OUBJN5UI4FD7EUHAHMLFGCFJJWI6CYQZRO";
const $ = (s) => document.querySelector(s);

let admin = null;
let ocupado = false;

function contractId() {
  const guardado = window.idContrato("attendance");
  if (guardado === VIEJO) return window.CONTRATOS.attendance.id;
  return ($("#campo-id")?.value || guardado || "").trim();
}

async function simular(fn, args = {}) {
  const c = await cliente(contractId());
  const tx = await c[fn](args);
  if (tx.simulation?.error) throw new Error(explicarError(tx.simulation.error, ERRORES));
  return nativo(tx.result);
}

async function enviar(fn, args = {}) {
  if (!identidad.firmante) throw new Error("Conecta Freighter para firmar.");
  const c = await cliente(contractId());
  const tx = await c[fn](args);
  if (tx.simulation?.error) throw new Error(explicarError(tx.simulation.error, ERRORES));
  return nativo((await tx.signAndSend()).result);
}

function esc(s) {
  return String(s).replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function pintarAccion({ haySesion, yaEstoy, sid }) {
  const box = $("#accion");
  if (!box) return;
  const yo = identidad.publicKey;
  const profe = esProfe(admin);

  if (!haySesion) {
    box.innerHTML = profe
      ? `<p class="ayuda">Eres el admin. Abre la sesión abajo para que el grupo pueda firmar.</p>`
      : `<p class="cta-invitado">Todavía no hay clase abierta. El admin la abre con Freighter o con la CLI.</p>`;
    return;
  }

  if (profe) {
    box.innerHTML = `<p class="ayuda">Sesión ${sid} abierta. Pide al grupo que conecte Freighter y toque <em>Estoy presente</em>.</p>`;
    return;
  }

  if (!yo) {
    box.innerHTML = `
      <p class="cta-invitado">Puedes ver la lista sin wallet. Para quedar en ella, conecta Freighter (arriba). Te pedirá firmar; la <code>S…</code> no sale de la extensión.</p>`;
    return;
  }

  if (yaEstoy) {
    box.innerHTML = `<p class="ok-enlista">Ya quedaste en la lista. Eso está en Testnet.</p>`;
    return;
  }

  box.innerHTML = `<button id="btn-presente" class="cta" type="button">Estoy presente</button>
    <p class="ayuda mini">Freighter va a pedirte firmar <code>mark_present</code> con tu cuenta.</p>`;
  $("#btn-presente").onclick = pasarLista;
}

async function cargarSesion() {
  if (ocupado) return;
  const panel = $("#panel-clase");
  const rosterEl = $("#roster");
  const vistaProfe = $("#vista-profe");
  if (!panel) return;

  try {
    admin = String(await simular("get_admin"));
  } catch {
    admin = window.TALLER?.profe || null;
  }

  try {
    const next = Number(await simular("next_session_id"));
    const haySesion = next > 1;
    if (vistaProfe) vistaProfe.hidden = !(esProfe(admin) && !haySesion);

    const cmd = $("#cmd-cli");
    if (cmd) cmd.textContent = cmdMarcar(contractId(), haySesion ? next - 1 : 1);

    if (!haySesion) {
      panel.innerHTML = `<p class="kicker">Sin sesión</p><h2>Nadie ha abierto la clase</h2>
        <p class="tema">Cuando el admin firme <code>open_session</code>, aquí aparece el tablero.</p>`;
      if (rosterEl) rosterEl.innerHTML = `<li class="vacio">Aún no hay lista.</li>`;
      pintarAccion({ haySesion: false, yaEstoy: false, sid: 0 });
      return;
    }

    const sid = next - 1;
    const s = await simular("get_session", { session_id: BigInt(sid) });
    const lista = (await simular("list_present", { session_id: BigInt(sid) })) || [];
    const addrs = Array.from(lista).map(String);
    const yo = identidad.publicKey;
    const yaEstoy = yo && addrs.includes(yo);
    const course = s.course ?? (typeof s.get === "function" ? s.get("course") : "") ?? "Clase";
    const topic = s.topic ?? (typeof s.get === "function" ? s.get("topic") : "") ?? "";

    panel.innerHTML = `
      <p class="kicker">En vivo · sesión ${sid}</p>
      <h2>${esc(course)}</h2>
      <p class="tema">${esc(topic)}</p>
      <p class="conteo"><b>${addrs.length}</b> ${addrs.length === 1 ? "presente" : "presentes"}</p>
    `;
    pintarAccion({ haySesion: true, yaEstoy, sid });

    if (rosterEl) {
      rosterEl.innerHTML = addrs.length
        ? addrs
            .map((a) => {
              const mio = a === yo;
              return `<li class="${mio ? "yo" : ""}"><strong>${esc(etiquetaCuenta(a))}</strong>${
                mio ? "<em>tú</em>" : ""
              }</li>`;
            })
            .join("")
        : `<li class="vacio">Nadie ha pasado lista todavía.</li>`;
    }
  } catch (e) {
    panel.innerHTML = `<p class="vacio">${esc(explicarError(e.message, ERRORES))}</p>`;
    pintarAccion({ haySesion: false, yaEstoy: false, sid: 0 });
  }
}

async function pasarLista() {
  aviso("");
  ocupado = true;
  const btn = $("#btn-presente");
  if (btn) {
    btn.disabled = true;
    btn.textContent = "Esperando firma…";
  }
  try {
    const next = Number(await simular("next_session_id"));
    const sid = next - 1;
    if (sid < 1) throw new Error("El admin todavía no abre la clase.");
    await enviar("mark_present", { session_id: BigInt(sid), student: identidad.publicKey });
    aviso("Quedaste en la lista. Eso ya está en Testnet.", "ok");
  } catch (e) {
    aviso(explicarError(e.message, ERRORES), "error");
  } finally {
    ocupado = false;
    await cargarSesion();
  }
}

async function abrirSesion() {
  aviso("");
  ocupado = true;
  const btn = $("#btn-abrir");
  if (btn) btn.disabled = true;
  try {
    const course = $("#curso").value.trim();
    const topic = $("#tema").value.trim();
    if (!course || !topic) throw new Error("Escribe curso y tema.");
    const id = await enviar("open_session", { course, topic });
    aviso(`Sesión ${id} abierta. El grupo ya puede firmar presente.`, "ok");
  } catch (e) {
    aviso(explicarError(e.message, ERRORES), "error");
  } finally {
    ocupado = false;
    if (btn) btn.disabled = false;
    await cargarSesion();
  }
}

function init() {
  const campo = $("#campo-id");
  if (campo) campo.value = contractId();
  $("#btn-guardar-id")?.addEventListener("click", () => {
    window.guardarIdContrato("attendance", campo.value.trim());
    aviso("ID guardado.", "ok");
    cargarSesion();
  });
  $("#btn-abrir")?.addEventListener("click", abrirSesion);
  pintarCabecera();
  cablearFreighter(cargarSesion);
  cargarSesion();
  setInterval(cargarSesion, 8000);
}

init();
