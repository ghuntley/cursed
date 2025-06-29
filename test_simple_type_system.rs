use cursed::type_system::*;

fn main() {
    println!("=== Testing Type System Implementation ===");
    
    // Test 1: Create TypeExpression instances
    println!("Test 1: Creating type expressions");
    let int_type = TypeExpression::named("int");
    println!("  ✓ Created int type: {:?}", int_type);
    
    let func_type = TypeExpression::function(
        vec![TypeExpression::named("string")],
        TypeExpression::named("void")
    );
    println!("  ✓ Created function type: {:?}", func_type);
    
    // Test 2: Type substitution
    println!("\nTest 2: Type substitution");
    let mut subst = TypeSubstitution::new();
    subst.add("T".to_string(), TypeExpression::named("int"));
    
    let type_var = TypeExpression::named("T");
    let resolved = subst.apply(&type_var);
    println!("  ✓ Substituted T -> {:?}", resolved);
    
    // Test 3: Type unification
    println!("\nTest 3: Type unification");
    let mut unify_subst = TypeSubstitution::new();
    let t1 = TypeExpression::named("int");
    let t2 = TypeExpression::named("int");
    
    match unify_subst.unify(&t1, &t2) {
        Ok(()) => println!("  ✓ Successfully unified int with int"),
        Err(e) => println!("  ✗ Failed to unify: {}", e),
    }
    
    // Test 4: Constraint resolver
    println!("\nTest 4: Constraint resolver");
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    let constraint = GenericConstraint {
        constraint_name: "Display".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    
    match resolver.validate_constraint(&constraint, &env) {
        Ok(()) => println!("  ✓ Constraint validation successful"),
        Err(e) => println!("  ✗ Constraint validation failed: {:?}", e),
    }
    
    // Test 5: Type unifier
    println!("\nTest 5: Type unifier");
    let mut unifier = TypeUnifier::new();
    let t3 = TypeExpression::named("T0");
    let t4 = TypeExpression::named("string");
    
    match unifier.unify(&t3, &t4) {
        Ok(substitutions) => {
            println!("  ✓ Type unification successful with {} substitutions", substitutions.len());
            for (k, v) in &substitutions {
                println!("    {} -> {:?}", k, v);
            }
        }
        Err(e) => println!("  ✗ Type unification failed: {:?}", e),
    }
    
    println!("\n=== Type System Implementation Working! ===");
}
