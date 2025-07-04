fr fr Comprehensive struct demo showing full functionality

be_like Person squad {
    name tea
    age normie
    active vibes
}

be_like Company squad {
    name tea
    employee_count normie
}

slay main() {
    fr fr Create a person with struct literal
    sus alice Person = Person { name: "Alice Johnson", age: 28, active: based }
    
    fr fr Create a company
    sus company Company = Company { name: "Tech Corp", employee_count: 150 }
    
    fr fr Access struct fields
    vibez.spill("Employee name:")
    vibez.spill(alice.name)
    
    vibez.spill("Employee age:")
    vibez.spill(alice.age)
    
    vibez.spill("Company:")
    vibez.spill(company.name)
    
    vibez.spill("Employee count:")
    vibez.spill(company.employee_count)
    
    fr fr Check if employee is active
    lowkey alice.active {
        vibez.spill("Employee is active!")
    }
}
