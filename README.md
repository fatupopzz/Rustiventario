# 🏪 Rustiventario

Un sistema de gestión de inventario desarrollado en Rust que combina eficiencia, seguridad y una interfaz gráfica moderna. Este proyecto demuestra la potencia del lenguaje Rust para crear aplicaciones de gestión robustas.



## 🌟 Características

- ✅ Gestión completa de productos con nombre, precio, stock y etiquetas
- 🔍 Búsqueda y filtrado por múltiples criterios
- ⚠️ Alertas automáticas de bajo stock
- 💻 Interfaz gráfica moderna con [egui](https://github.com/emilk/egui)
- 💾 Persistencia de datos en formato CSV
- ⚡ Operaciones de alta eficiencia gracias a estructuras de datos optimizadas

## 🧠 Fundamentos Técnicos

El sistema utiliza una combinación inteligente de estructuras de datos para lograr un rendimiento óptimo:

- `HashMap` para acceso rápido por ID y etiquetas (O(1))
- `HashSet` para conjuntos únicos de etiquetas y productos con bajo stock
- `BinaryHeap` para mantener productos ordenados por precio

Esta arquitectura permite operaciones eficientes como:
- Búsqueda por ID o etiqueta en tiempo constante
- Obtención de productos más caros/baratos en O(log n)
- Verificación instantánea de productos con bajo stock

## 🚀 Ventajas de Rust

Este proyecto aprovecha varias ventajas únicas de Rust:

- 🔒 Seguridad de memoria garantizada sin recolector de basura
- 🧪 Sistema de tipos estricto que previene errores en compilación
- 🔄 Gestión de concurrencia segura sin condiciones de carrera
- 🛠️ Rendimiento cercano a lenguajes de bajo nivel como C/C++

## 📋 Requisitos

- Rust 1.70.0 o superior
- Dependencias:
  - eframe 0.24.0
  - egui 0.24.0
  - egui_extras 0.24.0
  - rfd 0.12.0

## 🚀 Instalación y Uso

1. **Clonar el repositorio:**
   ```bash
   git clone https://github.com/fatupopzz/Rustiventario.git
   cd Rustiventario
