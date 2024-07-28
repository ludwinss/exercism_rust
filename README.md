# Rust Practicum - Exercism

Este repositorio contiene mi progreso en la pista de **Rust** de [Exercism](https://exercism.io/). Exercism es una plataforma que ofrece una serie de ejercicios de programación en varios lenguajes, proporcionando una oportunidad para mejorar las habilidades a través de la práctica y la retroalimentación.

## ¿Qué es Rust?

Rust es un lenguaje de programación de sistemas que se centra en la seguridad, especialmente en la concurrencia segura y la prevención de errores en tiempo de ejecución. Se ha ganado popularidad por su capacidad para escribir código eficiente y seguro sin sacrificar el rendimiento.

## Estructura del Repositorio

Cada carpeta en este repositorio corresponde a un ejercicio diferente de la pista de Rust en Exercism. Los ejercicios están organizados de la siguiente manera:

- `/ejercicio-nombre/` - Carpeta que contiene el código fuente y las pruebas del ejercicio.
  - `Cargo.toml` - Archivo de configuración del proyecto.
  - `src/` - Carpeta que contiene el código fuente del ejercicio.
  - `tests/` - Carpeta con pruebas adicionales, si las hay.

## Cómo Usar Este Repositorio

1. **Clonar el Repositorio**

Clona el repositorio en tu máquina local:

```bash
git clone https://github.com/tu-usuario/rust-exercism.git
```

2. Cambiar a la Rama de un Ejercicio Específico

Cada ejercicio se encuentra en una rama separada. Cambia a la rama del ejercicio que deseas revisar o trabajar:

```bash
   git checkout nombre-de-la-rama
```

Puedes ver todas las ramas disponibles con el siguiente comando:

```bash
git branch -a
```

3. Navegar a un Ejercicio

Entra en la carpeta del ejercicio:

```bash
cd ejercicio-nombre
```

4. Compilar y Ejecutar Pruebas

Utiliza Cargo, el gestor de paquetes y construcción de Rust, para compilar el código y ejecutar las pruebas:

```bash
cargo test
```

## Contribuciones

Este repositorio es principalmente para mi aprendizaje y práctica personal. Sin embargo, si tienes sugerencias o mejoras, ¡siéntete libre de abrir un issue o una pull request!

## Licencia

Este proyecto se distribuye bajo la Licencia MIT.
