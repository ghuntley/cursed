yeet "testz"
yeet "build_system_simple"

# Comprehensive test suite for build_system_simple module

test_start("create_simple_config")
sus config map[tea]interface{} = create_simple_config()
assert_eq_string(config["name"].(tea), "simple_project")
assert_eq_string(config["version"].(tea), "1.0.0")
assert_eq_string(config["output_dir"].(tea), "build")

test_start("parse_config")
sus parsed_config map[tea]interface{} = parse_config("test.toml")
assert_eq_string(parsed_config["name"].(tea), "simple_project")

test_start("build_project_simple")
sus build_result lit = build_project_simple("test_config.toml")
assert_true(build_result)

test_start("clean_project_simple")
sus test_config map[tea]interface{} = create_simple_config()
sus clean_result lit = clean_project_simple(test_config)
assert_true(clean_result)

test_start("run_tests_simple")
sus run_result lit = run_tests_simple(test_config)
assert_true(run_result)

test_start("install_package_simple")
sus install_result lit = install_package_simple("test_package", "1.0.0")
assert_true(install_result)

test_start("list_packages_simple")
sus packages []tea = list_packages_simple()
assert_true(len(packages) > 0)
assert_eq_string(packages[0], "testz")

test_start("build_system_main_simple - build command")
sus args []tea = []tea{"cursed_build", "build"}
sus main_result normie = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - test command")
args = []tea{"cursed_build", "test"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - clean command")
args = []tea{"cursed_build", "clean"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - rebuild command")
args = []tea{"cursed_build", "rebuild"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - list command")
args = []tea{"cursed_build", "list"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - install command")
args = []tea{"cursed_build", "install", "testpkg", "1.0.0"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 0)

test_start("build_system_main_simple - unknown command")
args = []tea{"cursed_build", "unknown"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 1)

test_start("build_system_main_simple - insufficient args")
args = []tea{"cursed_build"}
main_result = build_system_main_simple(args)
assert_eq_int(main_result, 1)

print_test_summary()
