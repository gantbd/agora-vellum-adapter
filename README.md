# Agora–Vellum adapter

This separately owned, effect-free adapter converts only the restart-stable
identity vocabulary shared by an Agora artifact reference and a Vellum
document revision.

It does not resolve documents, authenticate callers, retain a Vellum
`VersionWitness`, read content, grant authority, or coordinate mutations. A
product must authenticate to Vellum and derive current read authority before
using a converted target.

The local path dependencies are development composition pins. This project is
not part of either engine's dependency graph or release claim.
