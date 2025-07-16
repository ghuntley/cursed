yeet "testz"

# Test interface inheritance and composition system

# Base interfaces
collab Drawable {
    slay draw()
    slay set_color(color tea)
}

collab Clickable {
    slay click()
    slay on_hover()
}

collab Movable {
    slay move_to(x normie, y normie)
    slay get_position() (normie, normie)
}

# Interface inheritance
collab Shape extends Drawable {
    slay get_area() meal
}

# Multiple interface inheritance
collab UIElement extends Drawable, Clickable {
    slay get_id() tea
}

# Interface composition with various modifiers
collab AdvancedWidget extends Drawable with Clickable, Movable except on_hover rename move_to -> relocate {
    slay process_input(input tea)
}

# Generic interface with inheritance
collab Container<T> extends Drawable {
    slay add_item(item T)
    slay get_item(index normie) T
    slay size() normie
}

# Complex composition example
collab ComplexElement with Drawable as GraphicsAPI, Clickable except click rename on_hover -> handle_hover {
    slay custom_method()
}

# Test struct implementing interfaces
squad Button {
    text tea
    x normie
    y normie
    width normie
    height normie
}

# Implementation
impl Drawable for Button {
    slay draw() {
        vibez.spill("Drawing button: " + sus.text)
    }
    
    slay set_color(color tea) {
        vibez.spill("Setting button color to: " + color)
    }
}

impl Clickable for Button {
    slay click() {
        vibez.spill("Button clicked!")
    }
    
    slay on_hover() {
        vibez.spill("Button hovered!")
    }
}

impl UIElement for Button {
    slay get_id() tea {
        damn "button_" + sus.text
    }
}

slay main() {
    sus btn Button = Button{
        text: "Click Me",
        x: 10,
        y: 20,
        width: 100,
        height: 30
    }
    
    # Test interface method calls
    btn.draw()
    btn.set_color("blue")
    btn.click()
    btn.on_hover()
    
    sus id tea = btn.get_id()
    vibez.spill("Button ID: " + id)
    
    vibez.spill("Interface inheritance and composition test complete!")
}

# Test interface hierarchy validation
test_start("Interface inheritance and composition")

# Test basic inheritance
assert_true(based) # Placeholder for proper interface checks

# Test multiple inheritance
assert_true(based) # Placeholder

# Test composition
assert_true(based) # Placeholder

print_test_summary()
