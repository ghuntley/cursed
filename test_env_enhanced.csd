fr fr Test Enhanced Environment Module
import env

sus home_val tea = env::get_env("HOME")
yap home_val

sus path_val tea = env::get_env("PATH")  
yap path_val

sus user_val tea = env::get_env("USER")
yap user_val

sus unknown_val tea = env::get_env("UNKNOWN")
yap unknown_val

sus has_home lit = env::has_env("HOME")
yap has_home

sus has_nonexistent lit = env::has_env("NONEXISTENT")  
yap has_nonexistent

sus set_result lit = env::set_env("TEST_VAR", "test_value")
yap set_result

sus unset_result lit = env::unset_env("TEST_VAR")
yap unset_result

sus env_count drip = env::list_env()
yap env_count

sus keys_count drip = env::get_all_keys()
yap keys_count
