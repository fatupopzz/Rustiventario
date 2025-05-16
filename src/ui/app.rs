use std::sync::{Arc, Mutex};
use egui::{Color32, RichText, ScrollArea, Ui};
use egui_extras::{Column, TableBuilder};
use rfd::FileDialog;

use crate::inventario::InventarioManager;
use crate::models::Producto;

pub struct InventarioApp {
    inventario: Arc<Mutex<InventarioManager>>,
    filtro_nombre: String,
    filtro_tag: String,
    mostrar_bajo_stock: bool,
    mensaje: Option<String>,
    // Estados para el formulario de agregar producto
    nuevo_producto: NuevoProducto,
    modo_edicion: Option<u32>,
}

struct NuevoProducto {
    nombre: String,
    precio: String,
    stock: String,
    tags: String,
}

impl Default for NuevoProducto {
    fn default() -> Self {
        Self {
            nombre: String::new(),
            precio: String::new(),
            stock: String::new(),
            tags: String::new(),
        }
    }
}

impl InventarioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Personalizar la fuente si lo deseamos
        cc.egui_ctx.style_mut(|style| {
            style.text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 16.0;
        });
        
        Self {
            inventario: Arc::new(Mutex::new(InventarioManager::new())),
            filtro_nombre: String::new(),
            filtro_tag: String::new(),
            mostrar_bajo_stock: false,
            mensaje: None,
            nuevo_producto: NuevoProducto::default(),
            modo_edicion: None,
        }
    }
    
    fn mostrar_sidebar(&mut self, ui: &mut Ui) {
        ui.heading("Filtros");
        
        ui.horizontal(|ui| {
            ui.label("Nombre:");
            ui.text_edit_singleline(&mut self.filtro_nombre);
        });
        
        ui.horizontal(|ui| {
            ui.label("Etiqueta:");
            ui.text_edit_singleline(&mut self.filtro_tag);
        });
        
        ui.checkbox(&mut self.mostrar_bajo_stock, "Solo productos con bajo stock");
        
        ui.separator();
        
        ui.heading("Acciones");
        
        if ui.button("Reiniciar filtros").clicked() {
            self.filtro_nombre.clear();
            self.filtro_tag.clear();
            self.mostrar_bajo_stock = false;
        }
        
        if ui.button("Guardar inventario").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("CSV", &["csv"])
                .set_file_name("inventario.csv")
                .save_file() 
            {
                let path_str = path.to_string_lossy().to_string();
                match self.inventario.lock().unwrap().guardar_inventario(&path_str) {
                    Ok(_) => self.mensaje = Some(format!("Inventario guardado en {}", path_str)),
                    Err(e) => self.mensaje = Some(format!("Error al guardar: {}", e)),
                }
            }
        }
        
        if ui.button("Cargar inventario").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("CSV", &["csv"])
                .pick_file() 
            {
                let path_str = path.to_string_lossy().to_string();
                match self.inventario.lock().unwrap().cargar_inventario(&path_str) {
                    Ok(_) => self.mensaje = Some(format!("Inventario cargado desde {}", path_str)),
                    Err(e) => self.mensaje = Some(format!("Error al cargar: {}", e)),
                }
            }
        }
        
        // Agregar algunos productos de ejemplo para demo
        if ui.button("Cargar datos de ejemplo").clicked() {
            let mut inventario = self.inventario.lock().unwrap();
            
            inventario.agregar_producto(
                "Laptop Gamer".to_string(), 
                1200, 
                10, 
                vec!["electrónica".to_string(), "computadoras".to_string()]
            );
            
            inventario.agregar_producto(
                "Smartphone".to_string(), 
                800, 
                15, 
                vec!["electrónica".to_string(), "móviles".to_string()]
            );
            
            inventario.agregar_producto(
                "Teclado Mecánico".to_string(), 
                120, 
                3, 
                vec!["electrónica".to_string(), "periféricos".to_string()]
            );
            
            inventario.agregar_producto(
                "Mouse Inalámbrico".to_string(), 
                50, 
                2, 
                vec!["electrónica".to_string(), "periféricos".to_string()]
            );
            
            inventario.agregar_producto(
                "Monitor 4K".to_string(), 
                350, 
                7, 
                vec!["electrónica".to_string(), "periféricos".to_string()]
            );
            
            self.mensaje = Some("Datos de ejemplo cargados".to_string());
        }
    }
    
    fn mostrar_formulario_producto(&mut self, ui: &mut Ui) {
        let titulo = if self.modo_edicion.is_some() {
            "Editar Producto"
        } else {
            "Agregar Nuevo Producto"
        };
        
        ui.heading(titulo);
        
        ui.horizontal(|ui| {
            ui.label("Nombre:");
            ui.text_edit_singleline(&mut self.nuevo_producto.nombre);
        });
        
        ui.horizontal(|ui| {
            ui.label("Precio:");
            ui.text_edit_singleline(&mut self.nuevo_producto.precio);
        });
        
        ui.horizontal(|ui| {
            ui.label("Stock:");
            ui.text_edit_singleline(&mut self.nuevo_producto.stock);
        });
        
        ui.horizontal(|ui| {
            ui.label("Etiquetas (separadas por comas):");
            ui.text_edit_singleline(&mut self.nuevo_producto.tags);
        });
        
        ui.horizontal(|ui| {
            if ui.button("Cancelar").clicked() {
                self.nuevo_producto = NuevoProducto::default();
                self.modo_edicion = None;
            }
            
            let boton_texto = if self.modo_edicion.is_some() { "Actualizar" } else { "Agregar" };
            
            if ui.button(boton_texto).clicked() {
                let precio = self.nuevo_producto.precio.parse::<u32>().unwrap_or(0);
                let stock = self.nuevo_producto.stock.parse::<u32>().unwrap_or(0);
                let tags: Vec<String> = self.nuevo_producto.tags
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                
                let mut inventario = self.inventario.lock().unwrap();
                
                if let Some(id) = self.modo_edicion {
                    // Implementar actualización
                    // Nota: Necesitaríamos añadir un método para actualizar completamente un producto
                    // Por ahora solo actualizamos el stock como ejemplo
                    if let Err(e) = inventario.actualizar_stock(id, stock) {
                        self.mensaje = Some(format!("Error: {}", e));
                    } else {
                        self.mensaje = Some(format!("Producto ID {} actualizado", id));
                    }
                } else {
                    // Agregar nuevo producto
                    let id = inventario.agregar_producto(
                        self.nuevo_producto.nombre.clone(),
                        precio,
                        stock,
                        tags
                    );
                    self.mensaje = Some(format!("Producto agregado con ID {}", id));
                }
                
                // Limpiar formulario
                self.nuevo_producto = NuevoProducto::default();
                self.modo_edicion = None;
            }
        });
    }
    
    fn mostrar_tabla_productos(&mut self, ui: &mut Ui) {
        // En lugar de mantener el lock durante todo el renderizado,
        // primero obtenemos todos los datos necesarios y luego liberamos el mutex
        
        // Estructura para almacenar los datos de producto que necesitamos para la UI
        #[derive(Clone)]
        struct ProductoUI {
            id: u32,
            nombre: String,
            precio: u32,
            stock: u32,
            tags: Vec<String>,
        }
        
        // Recopilamos todos los datos mientras tenemos el mutex bloqueado
        let productos_ui: Vec<ProductoUI> = {
            let inventario = self.inventario.lock().unwrap();
            inventario.obtener_todos_productos()
                .into_iter()
                .filter(|p| {
                    let coincide_nombre = self.filtro_nombre.is_empty() || 
                        p.nombre.to_lowercase().contains(&self.filtro_nombre.to_lowercase());
                    
                    let coincide_tag = self.filtro_tag.is_empty() || 
                        p.tags.iter().any(|t| t.to_lowercase().contains(&self.filtro_tag.to_lowercase()));
                    
                    let coincide_stock = !self.mostrar_bajo_stock || p.stock < 5;
                    
                    coincide_nombre && coincide_tag && coincide_stock
                })
                .map(|p| ProductoUI {
                    id: p.id,
                    nombre: p.nombre.clone(),
                    precio: p.precio,
                    stock: p.stock,
                    tags: p.tags.iter().cloned().collect(),
                })
                .collect()
        }; // El mutex se libera aquí
        
        ui.heading(format!("Productos ({})", productos_ui.len()));
        
        if productos_ui.is_empty() {
            ui.label("No hay productos que coincidan con los filtros aplicados.");
            return;
        }
        
        // Crear tabla
        ScrollArea::vertical().show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().at_least(50.0)) // ID
                .column(Column::remainder().at_least(200.0)) // Nombre
                .column(Column::auto().at_least(100.0)) // Precio
                .column(Column::auto().at_least(100.0)) // Stock
                .column(Column::remainder().at_least(200.0)) // Etiquetas
                .column(Column::auto()) // Acciones
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.strong("ID"); });
                    header.col(|ui| { ui.strong("Nombre"); });
                    header.col(|ui| { ui.strong("Precio"); });
                    header.col(|ui| { ui.strong("Stock"); });
                    header.col(|ui| { ui.strong("Etiquetas"); });
                    header.col(|ui| { ui.strong("Acciones"); });
                })
                .body(|mut body| {
                    for producto in &productos_ui {
                        body.row(30.0, |mut row| {
                            row.col(|ui| { ui.label(producto.id.to_string()); });
                            row.col(|ui| { ui.label(&producto.nombre); });
                            row.col(|ui| { ui.label(format!("${}", producto.precio)); });
                            
                            // Color rojo para bajo stock
                            row.col(|ui| { 
                                let text = if producto.stock < 5 {
                                    RichText::new(producto.stock.to_string()).color(Color32::RED)
                                } else {
                                    RichText::new(producto.stock.to_string())
                                };
                                ui.label(text);
                            });
                            
                            // Etiquetas
                            row.col(|ui| { 
                                ui.horizontal_wrapped(|ui| {
                                    for tag in &producto.tags {
                                        ui.label(
                                            RichText::new(tag)
                                                .background_color(Color32::from_rgb(230, 230, 250))
                                        );
                                        ui.add_space(2.0);
                                    }
                                });
                            });
                            
                            // Acciones
                            row.col(|ui| { 
                                let id = producto.id;
                                ui.horizontal(|ui| {
                                    if ui.button("Editar").clicked() {
                                        // Preparar para edición
                                        self.modo_edicion = Some(id);
                                        self.nuevo_producto.nombre = producto.nombre.clone();
                                        self.nuevo_producto.precio = producto.precio.to_string();
                                        self.nuevo_producto.stock = producto.stock.to_string();
                                        self.nuevo_producto.tags = producto.tags.join(", ");
                                    }
                                    
                                    if ui.button("Eliminar").clicked() {
                                        match self.inventario.lock().unwrap().eliminar_producto(id) {
                                            Ok(_) => self.mensaje = Some(format!("Producto ID {} eliminado", id)),
                                            Err(e) => self.mensaje = Some(format!("Error: {}", e)),
                                        }
                                    }
                                });
                            });
                        });
                    }
                });
        });
    }
}

impl eframe::App for InventarioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Sistema de Gestión de Inventario").size(24.0));
            
            // Mensaje de estado
            let mostrar_cierre = if let Some(msg) = &self.mensaje {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(msg).color(Color32::from_rgb(0, 100, 0)));
                    ui.button("×").clicked()
                }).inner
            } else {
                false
            };

            if mostrar_cierre {
                self.mensaje = None;
            }
            
            egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Agregar Producto").clicked() {
                        self.nuevo_producto = NuevoProducto::default();
                        self.modo_edicion = None;
                    }
                });
            });
            
            egui::SidePanel::left("side_panel")
                .resizable(true)
                .default_width(200.0)
                .width_range(150.0..=300.0)
                .show_inside(ui, |ui| {
                    self.mostrar_sidebar(ui);
                });
            
            egui::TopBottomPanel::bottom("form_panel")
                .resizable(true)
                .default_height(150.0)
                .height_range(100.0..=200.0)
                .show_inside(ui, |ui| {
                    self.mostrar_formulario_producto(ui);
                });
            
            // Contenido principal
            egui::CentralPanel::default().show_inside(ui, |ui| {
                self.mostrar_tabla_productos(ui);
            });
        });
    }
}
