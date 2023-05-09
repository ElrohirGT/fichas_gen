# Generador de Fichas Médicas
Genera fichas médicas para el proyecto de la clínica.

# Compilación
Clone el repositorio e instale [Nix](https://nixos.org/).

Luego, para correr el proyecto y generar las fichas simplemente corrar el siguiente comando dentro de la carpeta padre del repositorio clonado:
```bash
nix-shell --command "cargo run --release"
```
La primera vez que se corra el programa [Nix](https://nixos.org/) tendrá que configurar el proyecto y las dependencias por lo que se tardará unos 10mins, las siguientes veces que se corra el programa no tendrá que recompilarse, por lo que iniciará mucho más rápido.
