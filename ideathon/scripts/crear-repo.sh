#!/usr/bin/env bash
# Crea y publica el repo del ideathon a partir de la semilla en ideathon/repo-participantes.
#
#   ./crear-repo.sh <ORG-o-USUARIO>/<nombre-repo>
#   ./crear-repo.sh MarxMad/Ideathon-Stellar-BAF-Canacintra
#
# Requiere: gh auth login

set -euo pipefail

DESTINO="${1:-}"
if [ -z "$DESTINO" ]; then
  echo "Uso: $0 <ORG-o-USUARIO>/<nombre-repo>" >&2
  exit 1
fi

RAIZ="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SEMILLA="$RAIZ/repo-participantes"
TRABAJO="$(mktemp -d)"

echo "▸ Preparando contenido…"
cp -R "$SEMILLA/." "$TRABAJO/"
cp -R "$RAIZ/plantillas" "$TRABAJO/plantillas"   # incluye plantillas/demo/index.html
cp "$RAIZ/guia-github-participantes.md" "$TRABAJO/guia-github.md"
cp "$RAIZ/rubrica.md" "$TRABAJO/rubrica.md"
cp "$RAIZ/plan-de-trabajo.html" "$TRABAJO/plan-de-trabajo.html"

# En el repo de participantes no viven los scripts del facilitador:
# se quita ese enlace para no dejar una liga rota.
python3 - "$TRABAJO/rubrica.md" <<'PY_FIX'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
s = s.replace(
    "Se calcula con [`scripts/metricas.sh`](scripts/metricas.sh). No la juzga nadie: sale del historial.",
    "La calcula un script a partir del historial de Git. No la juzga nadie: sale de los commits.")
open(p, "w", encoding="utf-8").write(s)
PY_FIX

# Los enlaces relativos al material docente apuntan a Stellar-Guide en GitHub
sed -i '' 's|(guia-github-participantes.md)|(guia-github.md)|g' "$TRABAJO/README.md" 2>/dev/null || \
  sed -i 's|(guia-github-participantes.md)|(guia-github.md)|g' "$TRABAJO/README.md"

cd "$TRABAJO"
git init -q -b main
git add -A
git commit -q -m "chore: estructura inicial del ideathon"

echo "▸ Creando $DESTINO en GitHub…"
gh repo create "$DESTINO" --public --source=. --push \
  --description "Ideathon Stellar × BAF × CANACINTRA — ideas, demos y entregables de los equipos"

echo "▸ Configurando el repo…"
# Solo merge commits: preserva la autoría individual de cada participante (ver metricas.md)
gh api -X PATCH "repos/$DESTINO" \
  -F allow_merge_commit=true \
  -F allow_squash_merge=false \
  -F allow_rebase_merge=false \
  -F has_issues=true \
  -F has_projects=false \
  -F has_wiki=false >/dev/null

gh api -X PUT "repos/$DESTINO/topics" \
  -f 'names[]=ideathon' -f 'names[]=stellar' -f 'names[]=canacintra' -f 'names[]=baf' -f 'names[]=blockchain' >/dev/null || true

# GitHub Pages desde main: aquí quedan las demos ya mergeadas.
# El primer intento suele fallar porque el repo aún se está inicializando; se reintenta.
for intento in 1 2 3; do
  if gh api -X POST "repos/$DESTINO/pages" -f 'source[branch]=main' -f 'source[path]=/' >/dev/null 2>&1; then
    echo "  Pages habilitado."
    break
  fi
  [ "$intento" -eq 3 ] && echo "  ⚠️ No se pudo habilitar Pages automáticamente. Actívalo a mano en Settings → Pages."
  sleep 3
done

echo
echo "✅ Listo: https://github.com/$DESTINO"
echo
echo "Siguientes pasos:"
echo "  1. Revisa que el README y la guía apunten a '$DESTINO'."
echo "  2. Prueba el flujo completo con una cuenta de GitHub ajena (fork → commit → PR)."
echo "  3. Genera el QR del repo para imprimir en las mesas."
echo "  4. Verifica la demo de ejemplo: https://$(echo "$DESTINO" | cut -d/ -f1).github.io/$(echo "$DESTINO" | cut -d/ -f2)/ideas/equipo-00-ejemplo/demo/"
echo "  5. El día del evento:  $RAIZ/scripts/metricas.sh $DESTINO --leaderboard"
echo

cd "$RAIZ"
rm -rf "$TRABAJO"
