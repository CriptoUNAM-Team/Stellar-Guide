// IDs de contratos en Testnet.
// El valor de datos.js es el default del repo.
// Si pegas otro C… en una página, queda en localStorage y pisa el default.

const CLAVE_IDS = "stellar-guide-contract-ids";

function idsGuardados() {
  try {
    return JSON.parse(localStorage.getItem(CLAVE_IDS) || "{}");
  } catch {
    return {};
  }
}

function idContrato(nombre) {
  const extra = idsGuardados()[nombre];
  if (extra && extra.startsWith("C")) return extra.trim();
  return (window.CONTRATOS?.[nombre]?.id || "").trim();
}

function guardarIdContrato(nombre, id) {
  const mapa = idsGuardados();
  const limpio = (id || "").trim();
  if (!limpio) delete mapa[nombre];
  else mapa[nombre] = limpio;
  localStorage.setItem(CLAVE_IDS, JSON.stringify(mapa));
}

function acortarId(id) {
  if (!id) return "sin desplegar";
  return id.slice(0, 12) + "…";
}

window.idContrato = idContrato;
window.guardarIdContrato = guardarIdContrato;
window.acortarId = acortarId;
window.idsGuardados = idsGuardados;

window.TALLER = {
  profe: "GAREE3LHODLGPCZQD6EXYFOE5NEOC2W7VWHUEIBTQONQLGJG6BC4Z5TY",
  alumno: "GDRVFWUCR7FDAROL5UZY2A55YZRB5URQ34VWOET5G2LCMPXVU764TGRD",
};
