use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

use crate::models::Producto;
use crate::utils::file_io::{guardar_en_archivo, cargar_desde_archivo};

/// Estructura principal para gestionar el inventario
pub struct InventarioManager {
    productos_por_id: HashMap<u32, Producto>,
    productos_por_tag: HashMap<String, HashSet<u32>>,
    productos_por_precio: BinaryHeap<Producto>,
    productos_bajo_stock: HashSet<u32>,
    siguiente_id: u32,
}

impl InventarioManager {
    pub fn new() -> Self {
        Self {
            productos_por_id: HashMap::new(),
            productos_por_tag: HashMap::new(),
            productos_por_precio: BinaryHeap::new(),
            productos_bajo_stock: HashSet::new(),
            siguiente_id: 1,
        }
    }
    
    ///Obtener todo
    pub fn obtener_todos_productos(&self) -> Vec<&Producto> {
        self.productos_por_id.values().collect()
    }

    /// Agregar un nuevo producto
    pub fn agregar_producto(&mut self, nombre: String, precio: u32, stock: u32, tags: Vec<String>) -> u32 {
        let id = self.siguiente_id;
        self.siguiente_id += 1;
        
        let tags_set: HashSet<String> = tags.into_iter().collect();
        let producto = Producto { id, nombre, precio, stock, tags: tags_set.clone() };
        
        // Actualizar estructuras de datos
        self.productos_por_id.insert(id, producto.clone());
        self.productos_por_precio.push(producto);
        
        // Indexar por tags
        for tag in tags_set {
            self.productos_por_tag
                .entry(tag)
                .or_insert_with(HashSet::new)
                .insert(id);
        }
        
        // Verificar stock bajo
        if stock < 5 {
            self.productos_bajo_stock.insert(id);
        }
        
        id
    }

    
    /// Actualizar stock de un producto
    pub fn actualizar_stock(&mut self, id: u32, nuevo_stock: u32) -> Result<(), String> {
        let producto = self.productos_por_id.get_mut(&id)
            .ok_or_else(|| format!("Producto con ID {} no encontrado", id))?;
            
        producto.stock = nuevo_stock;
        
        // Actualizar lista de bajo stock
        if nuevo_stock < 5 {
            self.productos_bajo_stock.insert(id);
        } else {
            self.productos_bajo_stock.remove(&id);
        }
        
        // Actualizar heap
        self.actualizar_heap(id);
        
        Ok(())
    }
    
    // Método privado para actualizar el heap después de cambios
    fn actualizar_heap(&mut self, id: u32) {
        let productos: Vec<Producto> = self.productos_por_precio
            .drain()
            .filter(|p| p.id != id)
            .collect();
        
        // Repoblar el heap
        for p in productos {
            self.productos_por_precio.push(p);
        }
        
        if let Some(producto) = self.productos_por_id.get(&id) {
            self.productos_por_precio.push(producto.clone());
        }
    }
    
    /// Buscar productos por tag
    pub fn buscar_por_tag(&self, tag: &str) -> Vec<&Producto> {
        self.productos_por_tag.get(tag)
            .map(|ids| ids.iter()
                .filter_map(|id| self.productos_por_id.get(id))
                .collect())
            .unwrap_or_default()
    }
    
    /// Obtener productos más caros
    pub fn productos_mas_caros(&self, n: usize) -> Vec<&Producto> {
        let mut heap_clon = self.productos_por_precio.clone();
        let mut resultado = Vec::with_capacity(n);
        
        while resultado.len() < n {
            match heap_clon.pop() {
                Some(producto) => {
                    if let Some(prod_ref) = self.productos_por_id.get(&producto.id) {
                        resultado.push(prod_ref);
                    }
                },
                None => break,
            }
        }
        
        resultado
    }
    
    /// Obtener productos más baratos
    pub fn productos_mas_baratos(&self, n: usize) -> Vec<&Producto> {
        let mut heap_inverso: BinaryHeap<Reverse<&Producto>> = 
            self.productos_por_id.values().map(Reverse).collect();
        
        let mut resultado = Vec::with_capacity(n);
        
        while resultado.len() < n {
            if let Some(Reverse(producto)) = heap_inverso.pop() {
                resultado.push(producto);
            } else {
                break;
            }
        }
        
        resultado
    }
    
    /// Obtener productos con bajo stock
    pub fn productos_con_bajo_stock(&self) -> Vec<&Producto> {
        self.productos_bajo_stock.iter()
            .filter_map(|id| self.productos_por_id.get(id))
            .collect()
    }
    
    /// Guardar inventario en archivo
    pub fn guardar_inventario(&self, ruta: &str) -> std::io::Result<()> {
        let productos: Vec<&Producto> = self.productos_por_id.values().collect();
        guardar_en_archivo(ruta, &productos)
    }
    
    /// Cargar inventario desde archivo
    pub fn cargar_inventario(&mut self, ruta: &str) -> std::io::Result<()> {
        // Limpiar estructuras
        self.productos_por_id.clear();
        self.productos_por_tag.clear();
        self.productos_por_precio.clear();
        self.productos_bajo_stock.clear();
        self.siguiente_id = 1;
        
        // Cargar productos desde archivo
        let productos = cargar_desde_archivo(ruta)?;
        
        // Repoblar estructuras
        for producto in productos {
            let id = producto.id;
            
            // Actualizar el siguiente ID
            if id >= self.siguiente_id {
                self.siguiente_id = id + 1;
            }
            
            // Añadir a todas las estructuras
            self.productos_por_id.insert(id, producto.clone());
            self.productos_por_precio.push(producto.clone());
            
            // Indexar por tags
            for tag in &producto.tags {
                self.productos_por_tag
                    .entry(tag.clone())
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }
            
            // Verificar stock bajo
            if producto.stock < 5 {
                self.productos_bajo_stock.insert(id);
            }
        }
        
        Ok(())
    }

    /// Eliminar un producto del inventario
    pub fn eliminar_producto(&mut self, id: u32) -> Result<(), String> {
        let producto = self.productos_por_id.remove(&id)
            .ok_or_else(|| format!("Producto con ID {} no encontrado", id))?;
            
        // Eliminar de productos_bajo_stock
        self.productos_bajo_stock.remove(&id);
        
        // Eliminar de productos_por_tag
        for tag in &producto.tags {
            if let Some(ids) = self.productos_por_tag.get_mut(tag) {
                ids.remove(&id);
                
                // Eliminar tag si ya no tiene productos
                if ids.is_empty() {
                    self.productos_por_tag.remove(tag);
                }
            }
        }
        
        // Actualizar heap
        self.actualizar_heap(id);
        
        Ok(())
    }
}
