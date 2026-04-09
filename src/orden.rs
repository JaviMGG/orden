use clap::Parser;
use colored::*;
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    /// La ruta de la carpeta que quieres organizar
    ruta: String,
}

fn main() {
    let args = Cli::parse();
    let ruta_base = Path::new(&args.ruta);

    if !ruta_base.is_dir() {
        println!("{}", "Error: La ruta proporcionada no es una carpeta.".red());
        return;
    }

    // Leemos los archivos de la carpeta
    if let Ok(entradas) = fs::read_dir(ruta_base) {
        for entrada in entradas.flatten() {
            let path = entrada.path();

            // Solo procesamos archivos, ignoramos carpetas
            if path.is_file() {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    let destino = match extension.to_lowercase().as_str() {
                        "jpg" | "png" | "gif" | "jpeg" => "Imágenes",
                        "pdf" | "docx" | "txt" => "Documentos",
                        "mp4" | "mkv" | "mov" => "Videos",
                        "zip" | "tar" | "gz" | "rar" => "Documentos Comprimidos",
                        _ => "Otros",
                    };

                    organizar_archivo(&path, ruta_base, destino);
                }
            }
        }
    }
}

fn organizar_archivo(archivo: &Path, ruta_base: &Path, carpeta_destino: &str) {
    let nueva_carpeta = ruta_base.join(carpeta_destino);
    
    // Creamos la carpeta de destino si no existe
    fs::create_dir_all(&nueva_carpeta).ok();

    let nombre_archivo = archivo.file_name().unwrap();
    let destino_final = nueva_carpeta.join(nombre_archivo);

    match fs::rename(archivo, &destino_final) {
        Ok(_) => println!("{} {:?}", "Movido:".green(), nombre_archivo),
        Err(e) => println!("{} {:?} -> {}", "Error:".red(), nombre_archivo, e),
    }
}