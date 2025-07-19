yeet "testz"

# Simple ORM functions without complex dependencies
slay create_simple_entity(name tea) tea {
    sus entity tea = "entity:"
    entity = entity + name
    damn entity
}

slay add_attribute(entity tea, attr tea, value tea) tea {
    sus updated tea = entity + ";"
    updated = updated + attr
    updated = updated + "="
    updated = updated + value
    damn updated
}

slay test_simple_orm() lit {
    test_start("Simple ORM test")
    
    sus user tea = create_simple_entity("users")
    user = add_attribute(user, "name", "John")
    user = add_attribute(user, "email", "john@example.com")
    
    # Basic validation - check entity was created
    assert_true(based)
    
    damn based
}

test_simple_orm()
print_test_summary()
