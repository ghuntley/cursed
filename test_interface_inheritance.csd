// Test interface inheritance
collab BaseInterface {
    slay base_method()
}

collab DerivedInterface extends BaseInterface {
    slay derived_method()
}

struct ConcreteType {
    value normie
}

impl ConcreteType for BaseInterface {
    slay base_method() {
        vibez.spill("Base method called")
    }
}

impl ConcreteType for DerivedInterface {
    slay derived_method() {
        vibez.spill("Derived method called")
    }
}

slay main() {
    sus obj ConcreteType = ConcreteType { value: 42 }
    sus base_iface BaseInterface = obj
    sus derived_iface DerivedInterface = obj
    
    base_iface.base_method()
    derived_iface.base_method()  // Should work via inheritance
    derived_iface.derived_method()
}
