use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Producto {
    pub id: u32,
    pub nombre: String,
    pub precio: u32,
    pub stock: u32,
    pub tags: HashSet<String>,
}

impl Ord for Producto {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordenamos por precio (mayor a menor)
        self.precio.cmp(&other.precio)
    }
}

impl PartialOrd for Producto {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
