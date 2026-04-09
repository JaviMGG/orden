# Orden

Una herramienta de línea de comandos escrita en Rust para organizar archivos en una carpeta basándose en sus extensiones. Los archivos se mueven automáticamente a subcarpetas apropiadas como "Imágenes", "Documentos", "Videos", etc.

## Requisitos previos

- [Rust](https://www.rust-lang.org/) instalado en tu sistema (versión 1.56 o superior recomendada).

## Instalación

1. Clona este repositorio:
   ```bash
   git clone https://github.com/tu-usuario/orden.git
   cd orden
   ```

2. Instala la herramienta usando Cargo:
   ```bash
   cargo install --path .
   ```

   Esto instalará el binario `orden` en tu PATH, permitiéndote ejecutarlo desde cualquier lugar.

## Uso

Después de la instalación, puedes usar el comando `orden` seguido de la ruta a la carpeta que deseas organizar.

### Sintaxis
```bash
orden <ruta-a-la-carpeta>
```

Si es en la carpeta actual:
```bash
orden .
```

### Ejemplo
```bash
orden /home/usuario/MiCarpetaDesordenada
```

Esto organizará todos los archivos en `/home/usuario/MiCarpetaDesordenada` en subcarpetas basadas en sus extensiones.

## Categorías de archivos

Los archivos se organizan en las siguientes categorías:

- **Imágenes**: `.jpg`, `.png`, `.gif`, `.jpeg`
- **Documentos**: `.pdf`, `.docx`, `.txt`
- **Videos**: `.mp4`, `.mkv`, `.mov`
- **Documentos Comprimidos**: `.zip`, `.tar`, `.gz`, `.rar`
- **Musica**: `.mp3`
- **Otros**: Cualquier extensión no listada arriba

## Notas importantes

- Solo se procesan archivos; las carpetas se ignoran.
- Si una carpeta de destino ya existe, los archivos se moverán allí.
- Si ocurre un error al mover un archivo (por ejemplo, permisos insuficientes), se mostrará un mensaje de error en rojo.
- Los archivos se mueven, no se copian. Asegúrate de tener una copia de seguridad si es necesario.

## Desarrollo

Si deseas contribuir o modificar el código:

1. Clona el repositorio.
2. Instala las dependencias:
   ```bash
   cargo build
   ```
3. Ejecuta el proyecto:
   ```bash
   cargo run -- <ruta-a-la-carpeta>
   ```

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo LICENSE para más detalles.