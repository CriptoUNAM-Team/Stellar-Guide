#!/usr/bin/env bash
# Reporte de métricas del ideathon a partir del historial de GitHub.
#
#   ./metricas.sh <ORG>/<REPO>                 # reporte completo
#   ./metricas.sh <ORG>/<REPO> --leaderboard   # solo el leaderboard (para proyectar)
#
# Requiere GitHub CLI autenticado:  gh auth login

set -euo pipefail

REPO="${1:-}"
MODO="${2:-completo}"

if [ -z "$REPO" ]; then
  echo "Uso: $0 <ORG>/<REPO> [--leaderboard]" >&2
  exit 1
fi

command -v gh >/dev/null 2>&1 || { echo "Falta GitHub CLI (gh). Instálalo: brew install gh" >&2; exit 1; }
gh auth status >/dev/null 2>&1 || { echo "gh no está autenticado. Corre: gh auth login" >&2; exit 1; }

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# ---------------------------------------------------------------- recolección

# Commits en main (lo ya mergeado)
gh api "repos/$REPO/commits?per_page=100" --paginate \
  --jq '.[].author.login // "sin-cuenta"' > "$TMP/autores-main.txt" 2>/dev/null || : > "$TMP/autores-main.txt"

# PRs (abiertos, cerrados y mergeados) con sus commits
gh pr list --repo "$REPO" --state all --limit 200 \
  --json number,title,author,state,mergedAt \
  > "$TMP/prs.json" 2>/dev/null || echo '[]' > "$TMP/prs.json"

: > "$TMP/autores-pr.txt"
: > "$TMP/prs.tsv"

while IFS=$'\t' read -r num titulo autor estado; do
  [ -z "${num:-}" ] && continue
  n_commits=$(gh api "repos/$REPO/pulls/$num/commits?per_page=100" --paginate --jq '.[].author.login // "sin-cuenta"' 2>/dev/null \
    | tee -a "$TMP/autores-pr.txt" | wc -l | tr -d ' ')
  # ¿hubo commit después del primer comentario de review? (métrica M6)
  primer_review=$(gh api "repos/$REPO/issues/$num/comments?per_page=100" --jq 'if length > 0 then .[0].created_at else "" end' 2>/dev/null || echo "")
  itero="no"
  if [ -n "$primer_review" ]; then
    ultimo_commit=$(gh api "repos/$REPO/pulls/$num/commits?per_page=100" --paginate --jq '.[].commit.committer.date' 2>/dev/null | sort | tail -1)
    [[ -n "$ultimo_commit" && "$ultimo_commit" > "$primer_review" ]] && itero="sí"
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$num" "$estado" "$autor" "$n_commits" "$itero" "$titulo" >> "$TMP/prs.tsv"
done < <(jq -r '.[] | [.number, .title, .author.login, .state] | @tsv' "$TMP/prs.json")

cat "$TMP/autores-main.txt" "$TMP/autores-pr.txt" | sort | uniq -c | sort -rn > "$TMP/leaderboard.txt"

# ------------------------------------------------------------------ salida

if [ "$MODO" = "--leaderboard" ]; then
  echo
  echo "🏆  LEADERBOARD DE COMMITS — $REPO"
  echo "════════════════════════════════════════════"
  awk '{printf "  %2d. %-28s %3d commits\n", NR, $2, $1}' "$TMP/leaderboard.txt"
  echo
  exit 0
fi

total_commits=$(awk '{s+=$1} END {print s+0}' "$TMP/leaderboard.txt")
autores_unicos=$(wc -l < "$TMP/leaderboard.txt" | tr -d ' ')
con_2mas=$(awk '$1>=2' "$TMP/leaderboard.txt" | wc -l | tr -d ' ')
forks=$(gh api "repos/$REPO/forks?per_page=100" --paginate --jq '.[].owner.login' 2>/dev/null | sort -u | wc -l | tr -d ' ')
prs_total=$(wc -l < "$TMP/prs.tsv" | tr -d ' ')
prs_merged=$(jq -r '[.[] | select(.mergedAt != null)] | length' "$TMP/prs.json")
prs_iteraron=$(awk -F'\t' '$5=="sí"' "$TMP/prs.tsv" | wc -l | tr -d ' ')
participantes=$(gh api "repos/$REPO/contents/participantes" \
  --jq '[.[] | select(.name | endswith(".md")) | select(.name != "README.md")] | length' 2>/dev/null || echo 0)

echo
echo "═══════════════════════════════════════════════════════════"
echo "  REPORTE DEL IDEATHON — $REPO"
echo "  generado: $(date '+%Y-%m-%d %H:%M')"
echo "═══════════════════════════════════════════════════════════"
echo
printf "  M1  Participantes con archivo propio ....... %s\n" "$participantes"
printf "  M2  Forks del repo ........................ %s\n" "$forks"
printf "  M3  Commits totales (main + PRs) .......... %s\n" "$total_commits"
printf "  M4  Pull Requests abiertos ................ %s\n" "$prs_total"
printf "  M5  Pull Requests mergeados ............... %s\n" "$prs_merged"
printf "  M6  Equipos que iteraron tras el review ... %s\n" "$prs_iteraron"
printf "  M8  Autores únicos ........................ %s  (con 2+ commits: %s)\n" "$autores_unicos" "$con_2mas"
echo
echo "───────────────────────────────────────────────────────────"
echo "  LEADERBOARD"
echo "───────────────────────────────────────────────────────────"
awk '{printf "  %2d. %-28s %3d commits\n", NR, $2, $1}' "$TMP/leaderboard.txt"
echo
echo "───────────────────────────────────────────────────────────"
echo "  PULL REQUESTS"
echo "───────────────────────────────────────────────────────────"
printf "  %-5s %-8s %-20s %-8s %-8s %s\n" "PR" "ESTADO" "AUTOR" "COMMITS" "ITERÓ" "TÍTULO"
awk -F'\t' '{printf "  #%-4s %-8s %-20s %-8s %-8s %s\n", $1, $2, $3, $4, $5, $6}' "$TMP/prs.tsv"
echo
echo "  Evidencia Testnet (M7): revisar 'evidencia.md' de cada equipo"
echo "  gh api repos/$REPO/contents/ideas --jq '.[].name'"
echo
