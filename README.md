# 🚀 Star Wars: Terminal Assault

Un juego tipo Space Invaders con temática de Star Wars desarrollado completamente en Rust para la terminal.

## ✨ Características

- 🎬 **Intro cinematográfica** tipo "crawl" de Star Wars
- 🎮 **Gameplay clásico** inspirado en Space Invaders
- 🎨 **Interfaz mejorada** con ASCII art y colores
- 💥 **Efectos visuales** (explosiones, láseres de colores)
- 📊 **Sistema de puntuación** con high score
- 🌊 **Oleadas progresivas** con dificultad incremental
- ❤️ **Sistema de vidas** del jugador
- 🎯 **Enemigos que disparan** de vuelta
- 📈 **Estadísticas detalladas** al final de cada partida

## 🎯 Objetivo

Defiende la base rebelde destruyendo todas las oleadas de cazas TIE del Imperio. ¡No dejes que lleguen al fondo!

## 📋 Requisitos

- **Rust** 1.70 o superior
- **Cargo** (incluido con Rust)
- Terminal con soporte para colores ANSI
- Fuente monoespaciada recomendada

### Instalación de Rust

Si no tienes Rust instalado:

```bash
# Windows (PowerShell)
winget install Rustlang.Rustup

# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🚀 Instalación y Ejecución

1. **Clonar o descargar** el proyecto en `c:\Users\cosmi\Desktop\rstarwars`

2. **Navegar al directorio**:
```bash
cd c:\Users\cosmi\Desktop\rstarwars
```

3. **Compilar y ejecutar**:
```bash
cargo run --release
```

## 🎮 Controles

| Tecla | Acción |
|-------|--------|
| `←` `→` | Mover la nave X-Wing |
| `SPACE` | Disparar cañones láser |
| `S` | Iniciar juego / Saltar intro |
| `H` | Ver ayuda |
| `Q` / `ESC` | Salir |

## 🎲 Mecánicas de Juego

### Puntuación
- **10 puntos** por cada caza TIE destruido
- El **High Score** se mantiene durante la sesión
- Completa oleadas para aumentar la dificultad

### Sistema de Vidas
- Comienzas con **3 vidas** (❤❤❤)
- Pierdes una vida si te alcanza un láser enemigo
- Game Over si pierdes todas las vidas o los enemigos llegan al fondo

### Dificultad Progresiva
- Cada oleada añade más enemigos y aumenta su velocidad
- Los enemigos disparan aleatoriamente
- Movimiento horizontal con descenso al tocar bordes (estilo Space Invaders)

## 🏗️ Estructura del Proyecto

```
rstarwars/
├── Cargo.toml          # Configuración y dependencias
├── README.md           # Este archivo
└── src/
    └── main.rs         # Código principal del juego
```

### Componentes Principales

#### `TerminalGuard`
- Maneja el estado del terminal (raw mode, cursor, alternate screen)
- Garantiza limpieza automática al salir

#### `Game`
- Estado del juego (jugador, enemigos, láseres, vidas, score)
- Lógica de actualización y renderizado
- Sistema de colisiones y explosiones

#### `Explosion`
- Efectos visuales de destrucción
- Duración configurable

#### Funciones de UI
- `star_wars_intro()`: Intro cinematográfica
- `show_menu()`: Menú principal con opciones
- `show_help()`: Pantalla de ayuda detallada
- `show_game_over()`: Estadísticas finales

## 🎨 Personalización

### Ajustar Dimensiones
```rust
const WIDTH: usize = 80;   // Ancho del área de juego
const HEIGHT: usize = 30;  // Alto del área de juego
```

### Modificar Dificultad Inicial
```rust
// En Game::new()
step_interval: 10,  // Mayor = más lento (rango: 3-15)
player_lives: 3,    // Vidas iniciales
```

### Cambiar Caracteres
```rust
const PLAYER_CHAR: char = '▲';
const ENEMY_CHAR: char = '◆';
const LASER_CHAR: char = '│';
```

### Personalizar Intro
Edita el vector `crawl` en `star_wars_intro()`:
```rust
let crawl = vec![
    "Tu texto personalizado",
    "Línea 2",
    // ...
];
```

## 🐛 Solución de Problemas

### El juego parpadea
- Usa una terminal moderna (Windows Terminal, iTerm2, etc.)
- Asegúrate de tener una fuente monoespaciada
- Reduce el tamaño de la ventana si es muy grande

### Los colores no se muestran
- Verifica que tu terminal soporte colores ANSI
- En Windows, usa PowerShell o Windows Terminal (no CMD antiguo)

### El juego va lento
- Compila en modo release: `cargo run --release`
- Cierra otros programas pesados

### Caracteres Unicode no se muestran
- Configura tu terminal para usar UTF-8
- Instala una fuente con soporte Unicode (Cascadia Code, Fira Code, etc.)

## 📚 Tutorial Básico

### Primera Partida

1. **Ejecuta el juego**: `cargo run --release`
2. **Disfruta la intro** o sáltala presionando `S`
3. **En el menú**: presiona `S` para comenzar
4. **Muévete** con flechas, **dispara** con SPACE
5. **Evita** los láseres enemigos (en rojo)
6. **Elimina** todos los TIE Fighters de cada oleada
7. **Sobrevive** el mayor tiempo posible para maximizar tu puntuación

### Estrategias Avanzadas

- **Mantente en movimiento**: Los enemigos disparan aleatoriamente
- **Dispara con precisión**: Cada disparo cuenta
- **Anticipa el movimiento**: Los enemigos bajan cuando tocan los bordes
- **Gestiona el espacio**: No te acorrales en las esquinas

## 🔧 Dependencias

- **crossterm** `0.27`: Manejo de terminal multiplataforma
- **rand** `0.8`: Generación de números aleatorios

## 📝 Notas Legales

Este es un proyecto educativo y de código abierto. No utiliza assets oficiales de Star Wars ni contenido protegido por derechos de autor. Todas las referencias son textuales y genéricas.

## 🤝 Contribuciones

Para mejorar el juego:
1. Añade más tipos de enemigos
2. Implementa power-ups
3. Añade efectos de sonido (con bibliotecas de audio)
4. Crea un sistema de guardado de high scores persistente
5. Añade más oleadas especiales o jefes

## 📜 Licencia

Este proyecto es de código abierto. Siéntete libre de modificarlo y mejorarlo.

---

**¡Que la Fuerza te acompañe!** ⚔️✨
