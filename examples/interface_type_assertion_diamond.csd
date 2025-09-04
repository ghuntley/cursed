vibe main;

fr fr Define interfaces for our diamond inheritance pattern
collab Movable {
    slay move(x lit, y lit);
    slay position() tea[]lit;
}

collab Drawable {
    slay draw();
    slay color() tea;
}

fr fr Interface inheriting from both (creates diamond potential)
collab AnimatedObject {
    slay animate();
}

fr fr Common base interface that others will inherit from
collab GameObject {
    slay id() lit;
    slay name() tea;
}

fr fr Drawable inherits from GameObject
slay (g GameObject) color() tea {
    return "default";
}

fr fr Movable inherits from GameObject
slay (g GameObject) position() tea[]lit {
    return [0, 0];
}

fr fr AnimatedObject inherits from both Drawable and Movable
fr fr This creates a diamond inheritance pattern through GameObject

fr fr Player concrete type - implements the inheritance diamond
squad Player {
    player_id lit,
    player_name tea,
    pos_x lit,
    pos_y lit,
    player_color tea,
    animation_frame lit
}

fr fr Implement GameObject for Player
slay (p Player) id() lit {
    return p.player_id;
}

slay (p Player) name() tea {
    return p.player_name;
}

fr fr Implement Movable for Player
slay (p Player) move(x lit, y lit) {
    p.pos_x = p.pos_x + x;
    p.pos_y = p.pos_y + y;
}

slay (p Player) position() tea[]lit {
    return [p.pos_x, p.pos_y];
}

fr fr Implement Drawable for Player
slay (p Player) draw() {
    vibez.spill("Drawing player " + p.player_name + " at position [" + p.pos_x + ", " + p.pos_y + "] with color " + p.player_color);
}

slay (p Player) color() tea {
    return p.player_color;
}

fr fr Implement AnimatedObject for Player
slay (p Player) animate() {
    p.animation_frame = (p.animation_frame + 1) % 60;
    vibez.spill("Player " + p.player_name + " animated at frame " + p.animation_frame);
}

fr fr Result type for error handling (similar to previous example)
squad Result<T, E> {
    value T,
    err E,
    isOk lit
}

fr fr Helper to create a successful result
slay ok<T, E>(value T) Result<T, E> {
    return Result<T, E>{
        value: value,
        err: nofr as E,
        isOk: 1
    };
}

fr fr Helper to create an error result
slay fail<T, E>(err E) Result<T, E> {
    return Result<T, E>{
        value: nofr as T,
        err: err,
        isOk: 0
    };
}

collab Error {
    slay error() tea;
}

squad TypeAssertionError {
    expected tea,
    actual tea
}

slay (e TypeAssertionError) error() tea {
    return "Type assertion failed: expected " + e.expected + " but got " + e.actual;
}

fr fr Function that navigates different paths in the diamond
slay testDiamondPaths(obj any) Result<tea, Error> {
    // First try to assert as GameObject (top of diamond)
    sus gameObj = obj.(GameObject)?;
    vibez.spill("Base GameObject: " + gameObj.name() + " (ID: " + gameObj.id() + ")");
    
    // Now try each path in the diamond
    // Path 1: GameObject -> Movable
    sus movable, movableOk = obj.(Movable);
    lowkey movableOk {
        sus pos = movable.position();
        vibez.spill("Movable path: position = [" + pos[0] + ", " + pos[1] + "]");
    }
    
    // Path 2: GameObject -> Drawable
    sus drawable = obj.(Drawable)?;
    vibez.spill("Drawable path: color = " + drawable.color());
    drawable.draw();
    
    // Bottom of diamond: AnimatedObject
    // Try assertion with ? operator to propagate errors
    sus animated = obj.(AnimatedObject)?;
    animated.animate();
    
    // Try to go directly from top to bottom of diamond
    sus player = obj.(Player)?;
    vibez.spill("Complete diamond traversed: Player " + player.player_name);
    
    return ok<tea, Error>("Diamond inheritance test completed successfully");
}

slay main_character() {
    // Create a Player instance
    sus player = Player{
        player_id: 42,
        player_name: "DiamondTester",
        pos_x: 10,
        pos_y: 20,
        player_color: "blue",
        animation_frame: 0
    };
    
    // Test with player object through diamond inheritance
    vibez.spill("\nTesting diamond inheritance with Player:\n");
    sus playerResult = testDiamondPaths(player);
    lowkey playerResult.isOk {
        vibez.spill("Result: " + playerResult.value);
    } no cap {
        vibez.spill("Error: " + playerResult.err.error());
    }
    
    // For comparison, try with an object that doesn't implement the full diamond
    // First, define a simple type that only implements GameObject
    squad SimpleObject {
        obj_id lit,
        obj_name tea
    }
    
    slay (s SimpleObject) id() lit {
        return s.obj_id;
    }
    
    slay (s SimpleObject) name() tea {
        return s.obj_name;
    }
    
    // Create a SimpleObject instance
    sus simpleObj = SimpleObject{
        obj_id: 99,
        obj_name: "SimpleTester"
    };
    
    // Test with simple object (should fail partway through the diamond)
    vibez.spill("\nTesting incomplete diamond inheritance with SimpleObject:\n");
    sus simpleResult = testDiamondPaths(simpleObj);
    lowkey simpleResult.isOk {
        vibez.spill("Result: " + simpleResult.value);
    } no cap {
        vibez.spill("Error: " + simpleResult.err.error());
    }
}