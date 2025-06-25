# Space Station Manager - Documento de Diseño Completo

## 1. Visión General del Proyecto

### Concepto
Space Station Manager es un juego de gestión de estación espacial diseñado para sesiones cortas (2-3 minutos) durante breaks de trabajo. El juego mantiene progreso persistente sin procesos activos, calculando los cambios basados en el tiempo transcurrido entre sesiones.

### Objetivos del Juego
- **Expandir la estación**: Construir nuevos módulos y mejorar los existentes
- **Mantener la tripulación viva**: Gestionar recursos vitales (oxígeno, alimentos, energía)
- **Completar misiones**: Objetivos específicos que otorgan recompensas

### Público Objetivo
- Desarrolladores y profesionales que buscan un descanso mental corto
- Jugadores que aprecian la gestión de recursos y planificación estratégica
- Usuarios que valoran la persistencia de progreso sin comprometer el rendimiento del sistema

## 2. Arquitectura Técnica

### Stack Tecnológico
- **Lenguaje**: Rust
- **Interfaz**: Terminal con crossterm
- **Persistencia**: JSON con serde
- **Tiempo**: chrono
- **Aleatoriedad**: rand
- **Errores**: anyhow

### Estructura del Proyecto
```
src/
├── main.rs           # Entry point y loop principal
├── game/
│   ├── mod.rs       # Game module
│   ├── state.rs     # Game state management
│   ├── resources.rs # Resource management
│   └── events.rs    # Event system
├── models/
│   ├── mod.rs       # Models module
│   ├── station.rs   # Station structure
│   ├── module.rs    # Module types and traits
│   └── crew.rs      # Crew management
├── ui/
│   ├── mod.rs       # UI module
│   ├── display.rs   # Terminal rendering
│   └── input.rs     # Input handling
└── utils/
    ├── mod.rs       # Utilities module
    ├── time.rs      # Time calculations
    └── save.rs      # Save/load system
```

### Principios de Diseño
- **Modularidad**: Código organizado en módulos independientes
- **Robustez**: Manejo robusto de errores con Result/Option
- **Extensibilidad**: Fácil adición de nuevas características
- **Separación de responsabilidades**: Lógica de negocio separada de UI
- **Testabilidad**: Tests unitarios para lógica core

## 3. Sistemas de Juego

### 3.1 Sistema de Recursos

#### Tipos de Recursos
- **Energía**: Producida por módulos de generación, consumida por todos los sistemas
- **Oxígeno**: Consumido por tripulación, producido por sistemas de soporte vital
- **Alimentos**: Consumidos por tripulación, producidos por jardines hidropónicos
- **Materiales**: Para construcción/reparación de módulos
- **Créditos**: Moneda para comercio con otras estaciones

#### Mecánicas de Recursos
- Consumo automático por tripulación
- Producción basada en módulos activos
- Almacenamiento limitado por capacidad de almacén
- Escasez causa efectos negativos en tripulación

### 3.2 Sistema de Módulos

#### Tipos de Módulos

**Módulos Básicos:**
- **Habitaciones**: Alojamiento para tripulación
- **Almacén**: Aumenta capacidad de recursos
- **Pasillos**: Conectan módulos

**Módulos de Producción:**
- **Reactor Nuclear**: Genera energía (alto riesgo)
- **Paneles Solares**: Genera energía (bajo riesgo, menor producción)
- **Sistema de Soporte Vital**: Produce oxígeno
- **Jardines Hidropónicos**: Produce alimentos

**Módulos Especializados:**
- **Centro Médico**: Cura tripulación enferma/lesionada
- **Centro de Investigación**: Desbloquea nuevas tecnologías
- **Hangar de Naves**: Para misiones de exploración/comercio
- **Sistema de Defensa**: Protege contra amenazas
- **Centro de Recreación**: Mantiene moral de tripulación
- **Centro de Comunicaciones**: Mejora relaciones exteriores

#### Características de Módulos
- Costo de construcción en materiales y créditos
- Consumo de energía
- Requisitos de tripulación para operación
- Estado de integridad (daño/desgaste)
- Niveles de mejora disponibles

### 3.3 Sistema de Tripulación

#### Roles de Tripulación
- **Ingenieros**: Reparan módulos, optimizan sistemas
- **Médicos**: Curan tripulación, mantienen salud general
- **Científicos**: Investigan nuevas tecnologías
- **Pilotos**: Manejan naves para misiones
- **Seguridad**: Protegen contra amenazas
- **Comunicaciones**: Gestionan relaciones exteriores
- **Mantenimiento**: Reparaciones generales

#### Características de Tripulación
- **Salud**: Afectada por escasez de recursos
- **Moral**: Afectada por eventos y condiciones de vida
- **Experiencia**: Aumenta con el tiempo y acciones
- **Especialización**: Eficiencia en roles específicos

### 3.4 Sistema de Eventos

#### Tipos de Eventos

**Emergencias:**
- Falla en sistemas críticos
- Fuga de oxígeno
- Cortocircuito eléctrico
- Invasión de plagas en jardines

**Amenazas Externas:**
- Meteoritos/Escombros espaciales
- Piratas espaciales
- Erupciones solares
- Anomalías espaciales

**Oportunidades:**
- Naves mercantes
- Visitas diplomáticas
- Descubrimientos científicos
- Misiones lucrativas

**Eventos Aleatorios:**
- Anomalías espaciales beneficiosas
- Visitas de otras estaciones
- Oportunidades de comercio
- Descubrimientos de recursos

#### Mecánicas de Eventos
- Frecuencia basada en tiempo transcurrido
- Diferentes niveles de urgencia
- Requieren acción del jugador o tripulación
- Consecuencias positivas/negativas

### 3.5 Sistema de Tiempo

#### Cálculo de Progreso
- Timestamp de última sesión guardado
- Al cargar, calcula tiempo transcurrido
- Aplica cambios basados en tiempo:
  - Consumo de recursos
  - Producción de recursos
  - Progreso de eventos
  - Desgaste de módulos
  - Experiencia de tripulación

#### Consideraciones de Tiempo
- Máximo tiempo entre sesiones (ej: 24 horas)
- Aceleración de tiempo para sesiones largas
- Eventos que requieren atención inmediata

## 4. Interfaz de Usuario

### 4.1 Diseño de Pantalla
```
┌─────────────────────────────────────────────────────────────┐
│                    SPACE STATION MANAGER                    │
├─────────────────────────────────────────────────────────────┤
│ Recursos: [⚡100] [🫧85%] [🍎75] [🔧50] [💰1000]           │
├─────────────────────────────────────────────────────────────┤
│ Módulos Activos: 8/12  |  Tripulación: 6/8  |  Día: 45    │
├─────────────────────────────────────────────────────────────┤
│ [MÓDULOS]                    │ [TRIPULACIÓN]               │
│ ⚡ Reactor Nuclear (95%)     │ 👨‍🔬 Dr. Smith (Salud: 90%)   │
│ 🫧 Soporte Vital (88%)       │ 👨‍⚕️ Dr. Jones (Salud: 85%)   │
│ 🍎 Jardines (92%)            │ 👨‍💻 Eng. Garcia (Salud: 95%) │
│ 🔧 Almacén (100%)            │ 👨‍🚀 Pilot Lee (Salud: 88%)   │
├─────────────────────────────────────────────────────────────┤
│ [EVENTOS ACTIVOS]                                           │
│ ⚠️  Alerta: Fuga de oxígeno en Sector B                    │
│ 💰 Oportunidad: Mercante ofrece 200 créditos por materiales│
├─────────────────────────────────────────────────────────────┤
│ [COMANDOS] [Q]Salir [N]Nuevo módulo [R]Reparar [E]Eventos  │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Controles
- **Navegación**: Flechas o WASD
- **Acciones**: Teclas específicas para cada acción
- **Menús**: Enter para seleccionar, Esc para cancelar
- **Salida**: Q para guardar y salir

### 4.3 Información Mostrada
- Estado actual de recursos
- Lista de módulos con estado
- Tripulación con salud/moral
- Eventos activos
- Comandos disponibles

## 5. Sistema de Guardado

### 5.1 Formato de Datos
```json
{
  "station": {
    "name": "Estación Alpha",
    "modules": [...],
    "crew": [...],
    "resources": {...},
    "last_update": "2024-01-15T10:30:00Z"
  },
  "game_state": {
    "current_day": 45,
    "active_events": [...],
    "completed_missions": [...]
  }
}
```

### 5.2 Mecánicas de Guardado
- Guardado automático al salir
- Backup de archivo anterior
- Validación de integridad de datos
- Recuperación de errores de guardado

## 6. Progresión del Juego

### 6.1 Objetivos a Largo Plazo
- Expandir estación a 20+ módulos
- Mantener tripulación de 12+ miembros
- Completar 50+ misiones
- Alcanzar autosuficiencia total

### 6.2 Sistema de Logros
- Constructor: Construir X módulos
- Explorador: Completar X misiones
- Gestor: Mantener estación X días
- Científico: Investigar X tecnologías

### 6.3 Dificultad Progresiva
- Eventos más complejos con el tiempo
- Requisitos de recursos más altos
- Nuevos tipos de amenazas
- Misiones más desafiantes

## 7. Consideraciones de Implementación

### 7.1 Manejo de Errores
- Result<T, E> para operaciones que pueden fallar
- Logging de errores para debugging
- Recuperación graceful de errores críticos
- Validación de datos de entrada

### 7.2 Performance
- Cálculos eficientes de tiempo transcurrido
- Renderizado optimizado de UI
- Serialización rápida de datos
- Minimizar uso de memoria

### 7.3 Extensibilidad
- Traits para diferentes tipos de módulos
- Sistema de plugins para eventos
- Configuración externa de parámetros
- API para mods futuros

## 8. Plan de Desarrollo

### Fase 1: Fundación (Semanas 1-2)
- [ ] Configurar proyecto Rust
- [ ] Implementar estructuras de datos básicas
- [ ] Sistema de tiempo y guardado
- [ ] UI básica de terminal

### Fase 2: Sistemas Core (Semanas 3-4)
- [ ] Sistema de recursos
- [ ] Módulos básicos
- [ ] Tripulación simple
- [ ] Eventos básicos

### Fase 3: Contenido (Semanas 5-6)
- [ ] Módulos especializados
- [ ] Eventos complejos
- [ ] Sistema de misiones
- [ ] Progresión del juego

### Fase 4: Pulido (Semanas 7-8)
- [ ] Balance de juego
- [ ] Tests unitarios
- [ ] Documentación
- [ ] Optimizaciones

## 9. Métricas de Éxito

### Técnicas
- Código modular y mantenible
- Tests con >80% cobertura
- Tiempo de carga <2 segundos
- Uso de memoria <50MB

### Jugabilidad
- Sesiones de 2-3 minutos satisfactorias
- Progresión clara y motivadora
- Eventos interesantes y variados
- Dificultad balanceada

### Usuario
- Fácil de aprender
- Satisfactorio de jugar
- Motivación para volver
- Progreso visible

---

Este documento servirá como guía durante todo el desarrollo del proyecto, asegurando que mantengamos la visión original mientras construimos un juego sólido y entretenido. 