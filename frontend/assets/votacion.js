import {
  identidad,
  cliente,
  esProfe,
  explicarError,
  nativo,
} from "./wallet.js";
import { aviso, pintarCabecera, cablearFreighter } from "./aula.js";

const ERRORES = {
  1: "Falta initialize",
  2: "Ya estaba inicializado",
  3: "No existe esa propuesta",
  4: "La votación ya cerró",
  5: "Esta cuenta ya votó",
};

const $ = (s) => document.querySelector(s);

function id() {
  return ($("#campo-id")?.value || window.idContrato("voting") || "").trim();
}

async function simular(fn, args = {}) {
  const x = await (await cliente(id()))[fn](args);
  if (x.simulation?.error) throw new Error(explicarError(x.simulation.error, ERRORES));
  return nativo(x.result);
}
async function enviar(fn, args = {}) {
  if (!identidad.firmante) throw new Error("Conecta Freighter para firmar.");
  const x = await (await cliente(id()))[fn](args);
  if (x.simulation?.error) throw new Error(explicarError(x.simulation.error, ERRORES));
  return nativo((await x.signAndSend()).result);
}

function pid() {
  const n = Number($("#proposal-id").value || 1);
  if (!n) throw new Error("Indica el número de propuesta.");
  return BigInt(n);
}

function pintarRol() {
  const panel = $("#vista-profe");
  if (panel) panel.hidden = !esProfe();
  const ayuda = $("#ayuda-voto");
  if (!ayuda) return;
  if (!identidad.publicKey) ayuda.textContent = "Conecta Freighter para emitir tu voto. Un address = un voto.";
  else if (esProfe()) ayuda.textContent = "Conectado como admin. Puedes publicar una pregunta y también votar.";
  else ayuda.textContent = "Freighter te pedirá firmar. Esta página no ve tu clave S…";
}

async function cargar() {
  pintarRol();
  try {
    const p = await simular("get_proposal", { proposal_id: pid() });
    const yes = Number(p.yes ?? 0);
    const no = Number(p.no ?? 0);
    const tot = yes + no || 1;
    $("#panel-prop").innerHTML = `
      <p class="kicker">${p.open ? "Votación abierta" : "Cerrada"} · propuesta #${p.proposal_id ?? $("#proposal-id").value}</p>
      <h2>${p.title || "Propuesta"}</h2>
      <p class="conteo"><b>${yes}</b> a favor · <b>${no}</b> en contra</p>
      <div class="barra-votos"><span class="si" style="width:${(yes / tot) * 100}%"></span><span class="no" style="width:${(no / tot) * 100}%"></span></div>
    `;
    document.querySelectorAll(".voto-btn").forEach((b) => {
      b.disabled = !identidad.publicKey || p.open === false;
    });
  } catch (e) {
    $("#panel-prop").innerHTML = `<p class="vacio">${e.message}</p>`;
    document.querySelectorAll(".voto-btn").forEach((b) => {
      b.disabled = true;
    });
  }
}

async function votar(support) {
  aviso("");
  try {
    await enviar("vote", {
      proposal_id: pid(),
      voter: identidad.publicKey,
      support,
    });
    aviso("Voto registrado en Testnet.", "ok");
    await cargar();
  } catch (e) {
    aviso(explicarError(e.message, ERRORES), "error");
  }
}

function init() {
  const campo = $("#campo-id");
  campo.value = window.idContrato("voting");
  $("#btn-guardar-id").onclick = () => {
    window.guardarIdContrato("voting", campo.value.trim());
    aviso("ID guardado.", "ok");
    cargar();
  };
  $("#btn-si").onclick = () => votar(true);
  $("#btn-no").onclick = () => votar(false);
  $("#btn-ver").onclick = cargar;
  $("#btn-crear").onclick = async () => {
    aviso("");
    try {
      const title = $("#titulo").value.trim();
      if (!title) throw new Error("Escribe la pregunta.");
      const n = await enviar("create_proposal", { title });
      $("#proposal-id").value = String(n);
      aviso(`Propuesta #${n} creada. Comparte el número con el grupo.`, "ok");
      await cargar();
    } catch (e) {
      aviso(explicarError(e.message, ERRORES), "error");
    }
  };
  pintarCabecera();
  cablearFreighter(cargar);
  cargar();
}

init();
