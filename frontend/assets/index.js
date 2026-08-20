(() => {
  const aula = new Set(["attendance", "voting", "grades"]);
  const grupos = { Campus: [], Negocio: [], DeFi: [] };
  for (const [clave, info] of Object.entries(window.CONTRATOS)) {
    if (aula.has(clave)) continue;
    (grupos[info.grupo] || grupos.Negocio).push({ clave, ...info });
  }

  const fnCount = (clave) =>
    (window.SPECS[clave] || []).filter((e) => e.type === "function").length;

  const raiz = document.getElementById("catalogo");
  raiz.innerHTML = "";

  for (const [grupo, items] of Object.entries(grupos)) {
    if (!items.length) continue;
    raiz.appendChild(Object.assign(document.createElement("h2"), { className: "seccion", textContent: grupo }));
    const rejilla = Object.assign(document.createElement("div"), { className: "rejilla" });
    for (const item of items) {
      const id = window.idContrato(item.clave);
      const a = Object.assign(document.createElement("a"), {
        className: "tarjeta",
        href: `${item.clave}.html`,
      });
      a.innerHTML = `
        <h3>${item.titulo}</h3>
        <p>${item.desc}</p>
        <p class="cid">${fnCount(item.clave)} funciones · ${id ? window.acortarId(id) : "pega el C… al desplegar"}</p>
      `;
      rejilla.appendChild(a);
    }
    raiz.appendChild(rejilla);
  }
})();
