import {
  identidad,
  conectarFreighter,
  cortar,
  nombreTaller,
} from "./wallet.js";

const $ = (s) => document.querySelector(s);

export function aviso(msg, tipo = "") {
  const el = $("#aviso");
  if (!el) return;
  el.textContent = msg || "";
  el.className = "aviso-app " + tipo;
  el.hidden = !msg;
}

export function pintarCabecera() {
  const btn = $("#btn-freighter");
  const pill = $("#pill-cuenta");
  if (!btn) return;
  if (identidad.publicKey) {
    const nom = nombreTaller(identidad.publicKey);
    btn.textContent = "Cambiar cuenta";
    btn.classList.remove("primario");
    if (pill) {
      pill.hidden = false;
      pill.innerHTML = `<strong>${nom || "cuenta"}</strong> ${cortar(identidad.publicKey)}`;
    }
  } else {
    btn.textContent = "Conectar Freighter";
    btn.classList.add("primario");
    if (pill) pill.hidden = true;
  }
}

export function cablearFreighter(alConectar) {
  const btn = $("#btn-freighter");
  if (!btn) return;
  btn.addEventListener("click", async () => {
    aviso("");
    const texto = btn.textContent;
    btn.disabled = true;
    btn.textContent = "Esperando Freighter…";
    try {
      await conectarFreighter();
      pintarCabecera();
      await alConectar?.();
    } catch (e) {
      aviso(e.message, "error");
      pintarCabecera();
    } finally {
      btn.disabled = false;
      if (!identidad.publicKey) btn.textContent = texto;
    }
  });
}

export function cmdMarcar(contractId, sessionId = 1) {
  return `stellar contract invoke --id ${contractId} --source alumno --network testnet -- mark_present --session_id ${sessionId} --student alumno`;
}
