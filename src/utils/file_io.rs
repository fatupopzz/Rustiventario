use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashSet;

use crate::models::Producto;

/// Guardar productos en un archivo CSV
pub fn guardar_en_archivo(ruta: &str, productos: &[&Producto]) -> io::Result<()> {
    let mut archivo = File::create(ruta)?;
    
    for producto in productos {
        let tags_str = producto.tags.iter().cloned().collect::<Vec<_>>().join(",");
        writeln!(
            archivo, 
            "{},{},{},{},{}", 
            producto.id, 
            producto.nombre, 
            producto.precio, 
            producto.stock, 
            tags_str
        )?;
    }
    
    Ok(())
}

/// Cargar productos desde un archivo CSV
pub fn cargar_desde_archivo(ruta: &str) -> io::Result<Vec<Producto>> {
    let archivo = File::open(ruta)?;
    let lector = BufReader::new(archivo);
    let mut productos = Vec::new();
    
    for resultado_linea in lector.lines() {
        let linea = resultado_linea?;
        let partes: Vec<&str> = linea.split(',').collect();
        
        if partes.len() >= 5 {
            let id: u32 = partes[0].parse().unwrap_or(0);
            let nombre = partes[1].to_string();
            let precio: u32 = partes[2].parse().unwrap_or(0);
            let stock: u32 = partes[3].parse().unwrap_or(0);
            
            // Las tags están en la posición 4 y pueden contener comas adicionales
            let tags_str = partes[4..].join(",");
            let tags: HashSet<String> = tags_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            let producto = Producto {
                id,
                nombre,
                precio,
                stock,
                tags,
            };
            
            productos.push(producto);
        }
    }
    
    Ok(productos)
}
