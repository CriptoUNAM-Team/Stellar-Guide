# Cómo contribuir a este repo

## Reglas

1. **Trabaja siempre en tu fork**, nunca en este repo directamente.
2. **Un archivo = un commit.** Commits pequeños y frecuentes durante el día.
3. Mensajes de commit en minúsculas, con prefijo:
   - `feat:` algo nuevo (`feat: agrega diagrama de solución`)
   - `docs:` texto o documentación (`docs: define el problema y el cliente`)
   - `fix:` corrección (`fix: aclara quién opera el ancla`)
4. **No toques la carpeta de otro equipo.** Un PR que modifica archivos ajenos se rechaza.
5. **No subas datos personales ni confidenciales**: el repo es público. Nada de teléfonos, RFC, correos de terceros ni información de empresas bajo NDA.
6. Nombra tu carpeta así: `ideas/equipo-XX-nombre-corto/` — minúsculas, guiones, sin acentos ni espacios.
7. **Nunca pegues una clave secreta** (las que empiezan con `S`) en la demo ni en ningún archivo. La página es pública y cualquiera puede leer su código.

## Checklist antes de abrir tu Pull Request

- [ ] Existe `participantes/<usuario>.md` de cada integrante
- [ ] Los 5 archivos de `ideas/equipo-XX/` están creados
- [ ] Existe `ideas/equipo-XX/demo/index.html` y ya no tiene los textos de ejemplo
- [ ] GitHub Pages está activado en el fork y la URL de la demo está en `evidencia.md`
- [ ] No quedan campos `<...>` sin llenar
- [ ] El diagrama Mermaid se ve bien en la vista previa de GitHub
- [ ] El título del PR es `Equipo XX — <nombre de la idea>`

## Después de abrir el PR

Un mentor va a dejar una pregunta. **Contéstala con un commit**, no en el chat: edita el archivo, guarda, y el commit aparece solo dentro del mismo PR.
