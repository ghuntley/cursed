collab TestInterface {
    slay test_method() lit
}

struct TestStruct {}

impl TestInterface for TestStruct {
    slay test_method() lit {
        damn based
    }
}

sus test_obj TestStruct = TestStruct{}
sus interface_obj TestInterface = test_obj as TestInterface
interface_obj.test_method()
