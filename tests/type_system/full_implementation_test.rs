use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::error::Error;

#[test]
fn test_type_system_integration() {
    // Test code with various type system features
    let input = "
        vibe main
        
        fr fr User-defined struct with generic parameters
        be_like Box[T] squad {
            value T
        }
        
        fr fr Another generic type with multiple parameters
        be_like Pair[A, B] squad {
            first A
            second B
        }
        
        fr fr Interface with generic parameter
        be_like Container[T] collab {
            add(item T)
            get(index normie) T
            size() normie
        }
        
        fr fr Implement Container for a generic list
        be_like List[T] squad {
            items []T
            count normie
        }
        
        fr fr Generic function with multiple type parameters
        slay map[T, U](items []T, transform slay(T) U) []U {
            sus result = make([]U, len(items))
            sus i = 0
            
            periodt i < len(items) {
                result[i] = transform(items[i])
                i = i + 1
            }
            
            yolo result
        }
        
        fr fr Function with channel parameters for concurrency
        slay worker(id normie, jobs dm<tea>, results dm<tea>) {
            periodt true {
                sus job = <-jobs
                results <- job + " done"
            }
        }
        
        slay main() {
            fr fr Create a Box of int
            sus box_int = Box[normie]{
                value: 42
            }
            
            fr fr Create a Pair of string and int
            sus pair = Pair[tea, normie]{
                first: "Hello",
                second: 123
            }
            
            fr fr Create a List of strings
            sus string_list = List[tea]{
                items: []tea{},
                count: 0
            }
            
            fr fr Create channels for concurrency
            sus jobs = make(dm<tea>, 10)
            sus results = make(dm<tea>, 10)
            
            fr fr Start worker goroutines
            stan worker(1, jobs, results)
            stan worker(2, jobs, results)
            
            fr fr Send jobs
            jobs <- "job1"
            jobs <- "job2"
            
            fr fr Get results
            sus r1 = <-results
            sus r2 = <-results
            
            puts(r1)
            puts(r2)
        }
    ";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // Verify the AST has the expected structure
    assert!(!program.statements.is_empty(), "Program should have statements");
    
    // Count the number of struct/interface definitions
    let struct_count = program.statements.iter()
        .filter(|stmt| stmt.as_any().downcast_ref::<cursed::ast::SquadStatement>().is_some())
        .count();
    
    let interface_count = program.statements.iter()
        .filter(|stmt| stmt.as_any().downcast_ref::<cursed::ast::CollabStatement>().is_some())
        .count();
    
    let function_count = program.statements.iter()
        .filter(|stmt| stmt.as_any().downcast_ref::<cursed::ast::FunctionStatement>().is_some())
        .count();
    
    // Verify counts
    assert_eq!(struct_count, 3, "Should have 3 struct definitions");
    assert_eq!(interface_count, 1, "Should have 1 interface definition");
    assert_eq!(function_count, 3, "Should have 3 function definitions");
    
    // TODO: Add more specific tests for type checking and generic instantiation
    // once the implementation is complete
}

#[test]
fn test_channel_type_parsing() {
    let input = "dm<normie>";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    // Parse the channel type
    let channel_type = parser.parse_type().unwrap();
    
    // Verify it's a channel type with the correct element type
    match channel_type {
        Type::Channel(elem_type) => {
            assert_eq!(*elem_type, Type::Normie);
        },
        _ => panic!("Expected channel type, got {:?}", channel_type),
    }
}

#[test]
fn test_nested_generic_type_parsing() {
    let input = "Pair[Box[tea], List[normie]]";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    // Parse the nested generic type
    let pair_type = parser.parse_type().unwrap();
    
    // Verify it's a Pair with the correct type arguments
    match pair_type {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Pair");
            assert_eq!(type_args.len(), 2);
            
            // First type argument should be Box[tea]
            match &*type_args[0] {
                Type::Struct(box_name, box_args) => {
                    assert_eq!(box_name, "Box");
                    assert_eq!(box_args.len(), 1);
                    assert_eq!(*box_args[0], Type::Tea);
                },
                _ => panic!("Expected Box type, got {:?}", type_args[0]),
            }
            
            // Second type argument should be List[normie]
            match &*type_args[1] {
                Type::Struct(list_name, list_args) => {
                    assert_eq!(list_name, "List");
                    assert_eq!(list_args.len(), 1);
                    assert_eq!(*list_args[0], Type::Normie);
                },
                _ => panic!("Expected List type, got {:?}", type_args[1]),
            }
        },
        _ => panic!("Expected Pair type, got {:?}", pair_type),
    }
}