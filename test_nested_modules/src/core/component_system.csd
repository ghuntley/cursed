// Entity-Component System for game engine
export ComponentRegistry, Entity, Component, ComponentId

type ComponentId = int
type Entity = int

trait Component {
    func get_component_id(&self) -> ComponentId
    func serialize(&self) -> string
    func deserialize(data: string) -> Self
}

struct ComponentRegistry {
    entities: Vec<Entity>,
    components: Map<ComponentId, Vec<Box<dyn Component>>>,
    next_entity_id: Entity,
    component_type_counter: ComponentId
}

impl ComponentRegistry {
    func new() -> ComponentRegistry {
        println("Initializing Component Registry")
        
        return ComponentRegistry {
            entities: Vec.new(),
            components: Map.new(),
            next_entity_id: 1,
            component_type_counter: 1
        }
    }
    
    func create_entity(&mut self) -> Entity {
        let entity_id = self.next_entity_id
        self.next_entity_id += 1
        self.entities.push(entity_id)
        
        println("Created entity: " + entity_id.to_string())
        return entity_id
    }
    
    func destroy_entity(&mut self, entity: Entity) {
        // Remove entity from all component lists
        for (component_id, component_list) in &mut self.components {
            component_list.retain(|comp| comp.get_entity_id() != entity)
        }
        
        // Remove from entity list
        self.entities.retain(|&e| e != entity)
        println("Destroyed entity: " + entity.to_string())
    }
    
    func add_component<T: Component>(&mut self, entity: Entity, component: T) -> Result<(), string> {
        let component_id = component.get_component_id()
        
        if !self.entities.contains(&entity) {
            return Err("Entity does not exist")
        }
        
        if !self.components.contains_key(&component_id) {
            self.components.insert(component_id, Vec.new())
        }
        
        let component_list = self.components.get_mut(&component_id).unwrap()
        component_list.push(Box::new(component))
        
        println("Added component " + component_id.to_string() + " to entity " + entity.to_string())
        Ok(())
    }
    
    func get_component<T: Component>(&self, entity: Entity, component_id: ComponentId) -> Option<&T> {
        if let Some(component_list) = self.components.get(&component_id) {
            for component in component_list {
                if component.get_entity_id() == entity {
                    return component.downcast_ref::<T>()
                }
            }
        }
        None
    }
    
    func get_entities_with_component(&self, component_id: ComponentId) -> Vec<Entity> {
        let mut entities = Vec.new()
        
        if let Some(component_list) = self.components.get(&component_id) {
            for component in component_list {
                entities.push(component.get_entity_id())
            }
        }
        
        entities
    }
    
    func get_entity_count(&self) -> int {
        self.entities.len()
    }
    
    func get_component_type_count(&self) -> int {
        self.components.len()
    }
}

// Common component types for game entities
struct TransformComponent {
    entity_id: Entity,
    position: Vec3,
    rotation: Vec3,
    scale: Vec3
}

struct RenderComponent {
    entity_id: Entity,
    mesh_id: int,
    material_id: int,
    visible: bool
}

struct PhysicsComponent {
    entity_id: Entity,
    mass: float,
    velocity: Vec3,
    angular_velocity: Vec3,
    is_static: bool
}

struct AudioComponent {
    entity_id: Entity,
    sound_id: int,
    volume: float,
    is_playing: bool,
    is_looping: bool
}

// 3D Vector utility
struct Vec3 {
    x: float,
    y: float,
    z: float
}

impl Vec3 {
    func new(x: float, y: float, z: float) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    
    func zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

// Component implementations
impl Component for TransformComponent {
    func get_component_id(&self) -> ComponentId {
        1 // Transform component ID
    }
    
    func get_entity_id(&self) -> Entity {
        self.entity_id
    }
    
    func serialize(&self) -> string {
        return "TransformComponent{pos:" + self.position.x.to_string() + "," + 
               self.position.y.to_string() + "," + self.position.z.to_string() + "}"
    }
    
    func deserialize(data: string) -> Self {
        // Mock deserialization
        TransformComponent {
            entity_id: 0,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0)
        }
    }
}

impl Component for RenderComponent {
    func get_component_id(&self) -> ComponentId {
        2 // Render component ID
    }
    
    func get_entity_id(&self) -> Entity {
        self.entity_id
    }
    
    func serialize(&self) -> string {
        return "RenderComponent{mesh:" + self.mesh_id.to_string() + ",material:" + self.material_id.to_string() + "}"
    }
    
    func deserialize(data: string) -> Self {
        RenderComponent {
            entity_id: 0,
            mesh_id: 0,
            material_id: 0,
            visible: true
        }
    }
}

impl Component for PhysicsComponent {
    func get_component_id(&self) -> ComponentId {
        3 // Physics component ID
    }
    
    func get_entity_id(&self) -> Entity {
        self.entity_id
    }
    
    func serialize(&self) -> string {
        return "PhysicsComponent{mass:" + self.mass.to_string() + ",static:" + self.is_static.to_string() + "}"
    }
    
    func deserialize(data: string) -> Self {
        PhysicsComponent {
            entity_id: 0,
            mass: 1.0,
            velocity: Vec3::zero(),
            angular_velocity: Vec3::zero(),
            is_static: false
        }
    }
}

impl Component for AudioComponent {
    func get_component_id(&self) -> ComponentId {
        4 // Audio component ID
    }
    
    func get_entity_id(&self) -> Entity {
        self.entity_id
    }
    
    func serialize(&self) -> string {
        return "AudioComponent{sound:" + self.sound_id.to_string() + ",volume:" + self.volume.to_string() + "}"
    }
    
    func deserialize(data: string) -> Self {
        AudioComponent {
            entity_id: 0,
            sound_id: 0,
            volume: 1.0,
            is_playing: false,
            is_looping: false
        }
    }
}
