# 📈 Habit Builder Solana



Sistema de seguimiento de hábitos desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para gestionar hábitos personales en blockchain, permitiendo llevar un control descentralizado del progreso diario:

- 🔑 Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad basada en firmas  

---

## 📚 Descripción

**Habit Builder Solana** permite a un usuario:

- Crear un gestor personal de hábitos  
- Registrar nuevos hábitos  
- Actualizar progreso (racha y tiempo invertido)  
- Eliminar hábitos  
- Consultar su rendimiento en blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir el tamaño de los datos para calcular correctamente la renta (*rent*).

### 📦 PDA Principal: `GestorHabitos`

Cuenta raíz que almacena todos los hábitos del usuario.

```rust
#[account]
#[derive(InitSpace)]
pub struct GestorHabitos {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(15)]
    pub lista_habitos: Vec<Habito>,
}
```

---

### 🧩 Estructura Interna: `Habito`

Cada hábito contiene:

- `nombre_habito (String)` → nombre del hábito  
- `racha_dias (u16)` → días consecutivos cumplidos  
- `minutos_invertidos (u8)` → tiempo dedicado  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Habito {
    #[max_len(40)]
    pub nombre_habito: String,
    pub racha_dias: u16,
    pub minutos_invertidos: u8,
}
```

---

## 🔒 Seguridad

El contrato asegura que solo el propietario pueda modificar su gestor:

```rust
require!(
    gestor.owner == ctx.accounts.owner.key(),
    Errores::NoAutorizado
);
```

✔ Protege los datos del usuario  
✔ Evita modificaciones no autorizadas  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Gestor

Crea la cuenta principal usando:

```rust
[b"habitos", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre del usuario  
- Lista vacía de hábitos  

---

### ➕ Agregar Hábito

- Recibe:
  - nombre  
  - racha inicial  
  - minutos invertidos  
- Inserta en el vector con `.push()`  

---

### ✏️ Actualizar Progreso

- Busca por `nombre_habito`  
- Actualiza:
  - racha  
  - minutos  

---

### ❌ Eliminar Hábito

```rust
.iter().position(|h| h.nombre_habito == nombre_buscado)
```

- Si existe → `.remove(index)`  
- Si no → error `HabitoNoEncontrado`  

---

### 📖 Consultar Hábitos

```rust
msg!("Lista de Hábitos: {:#?}", gestor.lista_habitos);
```

Muestra todos los hábitos en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar con el contrato usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `nombre: String`  
- `racha: u16`  
- `minutos: u8`  

---

## ⚠️ Manejo de Errores

```rust
#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para modificar este gestor.")]
    NoAutorizado,
    #[msg("El hábito solicitado no existe en tu lista.")]
    HabitoNoEncontrado,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Gestión de hábitos en blockchain  
- Seguridad mediante validación de firmas  
- Uso eficiente de estructuras dinámicas  
- Implementación de CRUD en un caso práctico (Habit Tracker)  

---

## 🚀 Próximos pasos

- Añadir recordatorios automáticos  
- Generar estadísticas de progreso  
- Integrar frontend tipo dashboard  
- Implementar recompensas con tokens  

---
