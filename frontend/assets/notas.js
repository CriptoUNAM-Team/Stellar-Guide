import {
  identidad,
  cliente,
  esProfe,
  explicarError,
  nativo,
  cortar,
} from "./wallet.js";
import { aviso, pintarCabecera, cablearFreighter } from "./aula.js";

const ERRORES = {
  1: "Falta initialize",
  2: "Ya estaba inicializado",
  3: "Puntaje inválido",
  4: "No hay esa calificación",
};

const $ = (s) => document.querySelector(s);
function id() {
  return ($("#campo-id")?.value || window.idContrato("grades") || "").trim();
}
async function enviar(fn, args) {
  if (!identidad.firmante) throw new Error("Conecta Freighter para firmar.");
  const x = await (await cliente(id()))[fn](args);
  if (x.simulation?.error) throw new Error(explicarError(x.simulation.error, ERRORES));
  return nativo((await x.signAndSend()).result);
}
async function simular(fn, args) {
  const x = await (await cliente(id()))[fn](args);
  if (x.simulation?.error) throw new Error(explicarError(x.simulation.error, ERRORES));
  return nativo(x.result);
}

function pintarRol() {
  const panel = $("#vista-profe");
  if (panel) panel.hidden = !esProfe();
  if (identidad.publicKey && !$("#alumno-ver").value) {
    $("#alumno-ver").value = identidad.publicKey;
  }
}

function init() {
  $("#campo-id").value = window.idContrato("grades");
  $("#btn-guardar-id").onclick = () => {
    window.guardarIdContrato("grades", $("#campo-id").value.trim());
    aviso("ID guardado.", "ok");
  };
  $("#btn-ver").onclick = async () => {
    aviso("");
    try {
      const student = $("#alumno-ver").value.trim() || identidad.publicKey;
      const assignment = $("#tarea-ver").value.trim();
      if (!student || !assignment) throw new Error("Cuenta del alumno y nombre de la actividad.");
      const g = await simular("get_grade", { student, assignment });
      $("#boleta").innerHTML = `
        <p class="kicker">${g.assignment || assignment}</p>
        <div class="nota-grande">${g.score}<span style="font-size:22px;color:var(--suave)"> / ${g.max_score}</span></div>
        <p class="vacio">${cortar(g.student || student)}</p>
      `;
    } catch (e) {
      aviso(explicarError(e.message, ERRORES), "error");
    }
  };
  $("#btn-registrar").onclick = async () => {
    aviso("");
    try {
      await enviar("record_grade", {
        student: $("#alumno").value.trim(),
        assignment: $("#tarea").value.trim(),
        score: Number($("#score").value),
        max_score: Number($("#max").value || 100),
      });
      aviso("Calificación publicada en Testnet.", "ok");
    } catch (e) {
      aviso(explicarError(e.message, ERRORES), "error");
    }
  };
  $("#usar-mia").onclick = () => {
    if (!identidad.publicKey) return aviso("Conecta Freighter primero.", "error");
    $("#alumno-ver").value = identidad.publicKey;
  };
  pintarCabecera();
  cablearFreighter(() => {
    pintarRol();
  });
  pintarRol();
}

init();
