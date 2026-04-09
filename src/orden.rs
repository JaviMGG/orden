use clap::Parser;
use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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

    let home = match env::var_os("HOME") {
        Some(h) => PathBuf::from(h),
        None => {
            println!("{}", "Error: No se pudo encontrar el HOME.".red());
            return;
        }
    };

    if let Ok(entradas) = fs::read_dir(ruta_base) {
        for entrada in entradas.flatten() {
            let path = entrada.path();

            if path.is_file() {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    let destino = match extension.to_lowercase().as_str() {
                        "jpg" | "png" | "gif" | "jpeg" => home.join("Imágenes"),
                        "pdf" | "docx" | "txt" => home.join("Documentos"),
                        "mp4" | "mkv" | "mov" => home.join("Videos"),
                        "zip" | "tar" | "gz" | "rar" => home.join("Documentos"), // o "Descargas"
                        "mp3" => home.join("Música"),
                        _ => home.join("Otros"),
                    };

                    organizar_archivo(&path, &destino);
                }
            }
        }
    }
}

fn organizar_archivo(archivo: &Path, carpeta_destino: &Path) {
    if !carpeta_destino.exists() {
        println!(
            "{} {:?}",
            "Advertencia: la carpeta destino no existe:".yellow(),
            carpeta_destino
        );
        return;
    }

    let nombre_archivo = archivo.file_name().unwrap();
    let destino_final = carpeta_destino.join(nombre_archivo);

    match fs::rename(archivo, &destino_final) {
        Ok(_) => println!(
            "{} {:?} a: {:?}",
            "Movido:".green(),
            nombre_archivo,
            destino_final
        ),
        Err(e) => println!("{} {:?} -> {}", "Error:".red(), nombre_archivo, e),
    }
}