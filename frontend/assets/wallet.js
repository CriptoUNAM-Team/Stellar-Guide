import {
  contract,
  Networks,
  Keypair,
} from "https://esm.sh/@stellar/stellar-sdk@16";

export const RPC = "https://soroban-testnet.stellar.org";
export const RED = Networks.TESTNET;

export const identidad = { publicKey: null, firmante: null, origen: null };

export function cortar(addr) {
  if (!addr) return "";
  const s = String(addr);
  return s.length <= 16 ? s : `${s.slice(0, 6)}…${s.slice(-6)}`;
}

export function nombreTaller(addr) {
  const a = String(addr || "");
  const t = window.TALLER || {};
  if (a === t.profe) return "profe";
  if (a === t.alumno) return "alumno";
  return "";
}

export function esProfe(adminAddr) {
  const pk = identidad.publicKey;
  if (!pk) return false;
  if (adminAddr && String(adminAddr) === pk) return true;
  return nombreTaller(pk) === "profe";
}

export function etiquetaCuenta(addr) {
  return nombreTaller(addr) || cortar(addr);
}

export function explicarError(texto, errores = {}) {
  const m = /Error\(Contract,\s*#(\d+)\)/.exec(String(texto));
  if (m) {
    const nom = errores[Number(m[1])];
    return nom ? `#${m[1]} ${nom}` : `Error del contrato #${m[1]}`;
  }
  if (/MissingValue/.test(String(texto))) return "Falta initialize o el ID no es de este contrato.";
  return String(texto);
}

export async function conectarFreighter() {
  let pk;
  let signTransaction;

  try {
    const api = await import("https://esm.sh/@stellar/freighter-api@3");
    const access = await api.requestAccess();
    if (access?.error) throw new Error(String(access.error));
    const dir = await api.getAddress();
    pk = dir?.address;
    if (!pk) throw new Error("Freighter no devolvió dirección.");
    signTransaction = async (xdrTx, opts) => {
      const r = await api.signTransaction(xdrTx, {
        networkPassphrase: RED,
        address: pk,
        ...opts,
      });
      if (r?.error) throw new Error(String(r.error));
      return { signedTxXdr: r.signedTxXdr ?? r, signerAddress: pk };
    };
  } catch (e) {
    const api = window.freighterApi;
    if (!api) {
      const msg = String(e?.message || e);
      if (/denied|reject|declin|cancel/i.test(msg)) throw e;
      throw new Error(
        "Instala la extensión Freighter, ponla en Testnet y recarga. Esta página no pide tu clave S…",
      );
    }
    if (api.setAllowed) await api.setAllowed();
    const dir = (await api.getAddress?.()) ?? (await api.getPublicKey?.());
    pk = typeof dir === "string" ? dir : dir?.address;
    if (!pk) throw new Error("Freighter no devolvió dirección.");
    signTransaction = async (xdrTx, opts) => {
      const r = await api.signTransaction(xdrTx, { networkPassphrase: RED, address: pk, ...opts });
      return typeof r === "string" ? { signedTxXdr: r, signerAddress: pk } : r;
    };
  }

  identidad.publicKey = pk;
  identidad.origen = "Freighter";
  identidad.firmante = { signTransaction };
  return pk;
}

export function usarSecreta(sk) {
  if (!sk?.startsWith("S")) throw new Error("La clave secreta de testnet empieza con S.");
  const kp = Keypair.fromSecret(sk);
  identidad.publicKey = kp.publicKey();
  identidad.origen = "clave de prueba";
  identidad.firmante = contract.basicNodeSigner(kp, RED);
  return identidad.publicKey;
}

export async function cliente(contractId) {
  if (!contractId?.startsWith("C")) throw new Error("Pega un Contract ID que empiece con C.");
  return contract.Client.from({
    contractId,
    networkPassphrase: RED,
    rpcUrl: RPC,
    publicKey: identidad.publicKey ?? undefined,
    signTransaction: identidad.firmante?.signTransaction,
  });
}

export function nativo(valor) {
  if (valor && typeof valor === "object" && "value" in valor && Object.keys(valor).length === 1) {
    return nativo(valor.value);
  }
  return valor;
}
