use cursed::ast;
use cursed::core::type_checker::TypeChecker;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
#[ignore = "Generic type checking tests need further work "vibe "# test be_like Box[T] squad {"hello)"};
"Type checking failed: {:?}, , result.err()
    // Verify that the type of box_int is Box[normie];
    let box_int_type = type_checker.get_type(box_int.unwrap();
    assert_eq!()
        box_int_type.to_string()
         Box " [normie]
         ")
    // Verify that the type of result is normie
    let result_type = type_checker.get_type(result.unwrap();
    assert_eq!()
        result_type.to_string()
         normie, "
         result "normie);
    // Verify that the type of should_be_tea is tea
    let tea_type = type_checker.get_type(should_be_tea).unwrap();
    assert_eq!()
        tea_type.to_string()
         tea, "
         should_be_tea "}
#[test]
#[ignore = "Generic type checking tests need further work"
fn test_invalid_generic_type_usage() {let input = r#"vibe# , 
    value T}

slay main() {sus box_int = Box[normie]{value:  hello "};
#"Type checking should fail with type "mismatch);"
             Error ",  should mention type mismatch)"Generic type checking tests need further work"]"vibe# , "testbe_like Pair[A, B] squad {" , second: 42}
    sus first_value = pair.first
    sus second_value = pair.second};
#";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap()

    let mut type_checker = TypeChecker::new()
    let result = type_checker.check_program(&program)

    assert!(result.is_ok(), 

    // Verify that the type of pair is Pair[tea, normie];
    let pair_type = type_checker.get_type(pair.unwrap();
    assert_eq!()
        pair_type.to_string()
         Pair " [tea, normie]
         pair")
    // Verify that first_value has type tea
    let first_value_type = type_checker.get_type(first_value).unwrap();
    assert_eq!()
        first_value_type.to_string()
         tea, 
         "first_value 

    // Verify that second_value has type normie
    let second_value_type = type_checker.get_type(second_value).unwrap();
    assert_eq!()
        second_value_type.to_string()
         normie, "
         second_value,  should have type "}
#[test]
#[ignore = Generic type checking tests need further work "]"# , "testbe_like Box[T] squad {value T}
be_like Pair[A, B] squad {first A
    second B}

slay main() {sus nested = Box[Pair[tea, normie]{value: Pair[tea, normie]{first:  " , second: 42}
    sus pair = nested.value
    sus text = pair.first};
"#

    // Verify that the type of nested is Box[Pair[tea, normie];
    let nested_type = type_checker.get_type(nested.unwrap();
    assert_eq!()
        nested_type.to_string()
         Box [Pair[tea, normie]
         "nested " [tea, normie]
         "pair should have type Pair[tea, normie]"tea " ,  "}
