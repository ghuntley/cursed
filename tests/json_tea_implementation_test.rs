use std::sync::Arc;
use cursed::error::Error;
use cursed::object::Object;
use cursed::stdlib::json_tea;

#[test]
fn test_json_unmarshal_primitive_values() {
    // TODO: Implement test
    assert!(true);
}
    let args = vec![Arc::new(Object::String(null.to_string(], Arc::new(Object::Nil))))
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(result, Object::Boolean(true);))
    // Test boolean false
    let args = vec![Arc::new(Object::String(false.to_string(], Arc::new(Object::Nil))))
    let result = json_tea::unmarshal(&args).unwrap();
    assert!(matches!(result, Object::Integer(42);))
    // Test float
    let args = vec![Arc::new(Object::String(, 3.14 .to_string(], Arc::new(Object::Nil))))
    let result = json_tea::unmarshal(&args).unwrap();
    if let Object::String(val) = &*result     {;}
        assert_eq!(val,  "} else {))"
        panic!(", "  String, got {:?), result)}
        panic!(Expected: Array ".to_string(), Arc::new(Object::Nil));"
            assert_eq!(s,  hello ) else {}""
        panic!(, : Array, got {:?), result)""
        panic!(Expected: Array , got {:?), result)}""
            panic!(", :  Boolean for active, got   {:?}, val.get(name     {assert_eq!(name,  zoomer}"} else {);))
                panic!(:  String for name, got   {:?), user.get(")
            if let Some(Object::Integer(age)  =  user.get(age)     {assert_eq!(age, 21}, got   {:?), user.get(age; else { }""))
            panic!(, )
        panic!(", "  HashTable, got {:?), result)name:  , ",  skills: ["]
            {, "  "email,  , },""
         ""
             average_age: 30.5, + " null ;}#", Expected:  String for name, got   {:?}, user1.get(")"
                        assert_eq!(skill1,  coding;) else { };}""
                        panic!()""
                        assert_eq!(skill2,  tiktok;") else { };}"
                        panic!(") else {}"
                    panic!(Expected: Array for skills , got   {:?), user1.get(skills;) else { }
                panic!(:  HashTable for user1, got   {:?), users[0]""
                    panic!(", ;) else {}"
                panic!(Expected "  HashTable for user2, got   {:?), users[1]} else {}"
            panic!(":  Array for users, got   {:?), val.get(", fixed)
        if let Some(Object::HashMap(stats)  =  val.get(stats)     {if let Some(Object::Float(avg_age} = stats.get(average_age     {"  Float for average_age, got   {:?), stats.get(average_age;"))))}
            if let Some(Object::Integer(active_users) = stats.get(active_users)     {assert_eq!(active_users, 2}} else {);)
                panic!(, ;) else {}
            panic!(Expected :  HashTable for stats, got   {:?), val.get(stats;)")"
        if let Some(obj) = val.get(settings)     {assert!(matches!(obj, Object::Nil} else {panic!(Expected: Null for settings ", } else   {)"))
        panic!(" HashTable """)"