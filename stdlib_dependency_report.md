# CURSED Stdlib Dependency Analysis Report

## Overview
- **Total Modules**: 273
- **Total Dependencies**: 419
- **Circular Dependencies**: 3
- **Critical Modules**: 18
- **Leaf Modules**: 87
- **Root Modules**: 209

## Critical Modules (High Fan-In)
These modules are depended upon by many other modules:
- **testz**: 136 dependents
- **stringz**: 51 dependents
- **vibez**: 25 dependents
- **mathz**: 19 dependents
- **concurrenz**: 14 dependents
- **timez**: 13 dependents
- **string**: 11 dependents
- **dropz**: 11 dependents
- **arrayz**: 11 dependents
- **filez**: 8 dependents
- **error_drip**: 7 dependents
- **core**: 7 dependents
- **atomic_drip**: 7 dependents
- **memoryz**: 6 dependents
- **runtime_core**: 6 dependents
- **io**: 5 dependents
- **encode_mood**: 5 dependents
- **time**: 5 dependents

## Leaf Modules (No Dependencies)
These modules don't depend on other stdlib modules:
- archive_handling
- arrayz
- atomic_drip
- big_mood
- binary_drip
- blockchainz
- bytefit
- chadlogging
- chaos_mode
- cloudz
- command_line
- compare_mood
- complex_vibe
- coverage_runtime
- data_drip
- deploymentz
- embed_that
- embeddedz
- encoding_flex
- enterprise_cloud
- enterprise_db
- enterprise_messaging
- enterprise_monitoring
- enterprise_security
- error_drip
- exec_vibez
- fs
- fs_test_vibe
- glyph_gang
- grammar_drip
- hash_drip
- hashz
- heap_slay
- io_simple
- ioz_enhanced
- kubernetesz
- logging
- main_character
- math_float
- math_float_simple
- math_int
- math_rand_tea
- mathz
- mathz_enhanced
- memoryz
- network
- no_cap
- oglogging
- pathing
- pem_drip
- plugin_system
- plugin_vibes
- pure_cursed_replacements
- regex_vibez
- rizz_template
- rpc_vibes
- scientificz
- signal_handling
- simple_math
- simple_test
- sketchy_math
- slay_io
- sort_slay
- sorta_fresh
- spill_facts
- string_pure
- stringz
- stringz_enhanced
- test_minimal_arrayz
- test_no_array
- test_result
- test_super_simple
- test_vibez_call
- test_vibez_minimal
- test_with_array
- testz
- testz_simple
- time_zone_drip
- typez
- url_parsing
- validation
- vibe_life
- vibe_lock
- vibez
- vibez_optimized
- wasm_mood
- yeet_io

## Root Modules (No Dependents)
These modules are not used by other stdlib modules:
- archive_handling
- arrayz_optimized
- async
- async_core
- async_runtime
- asyncz
- audioz
- benchmark_framework
- big_mood
- blockchainz
- build_system
- build_system_simple
- bytefit
- chadlogging
- channel_core
- chaos_mode
- clock_bait
- cloudz
- collections_advanced
- collections_core
- collections_enhanced
- collections_simple
- command_line
- compare_mood
- compiler_core
- complex_module
- complex_vibe
- compression
- compressionz
- compressz
- concurrency
- concurrency_advanced
- config
- config_package
- configz
- contract_testing
- coverage_analysis
- coverage_runtime
- crypto_complete
- crypto_secure
- crypto_subtle_drip
- csv
- csv_mood
- csv_rfc4180
- cursed_pointer
- data_drip
- database
- database_complete
- database_drivers
- database_orm
- database_production
- dbz
- debug_tea
- deploymentz
- drawz
- elliptic_curve_tea
- embed_that
- embeddedz
- encoding_flex
- enhanced_collections
- enhanced_error
- enterprise_cloud
- enterprise_db
- enterprise_messaging
- enterprise_monitoring
- enterprise_security
- env
- envz
- error_handling
- error_management
- exec_vibez
- filesystem_complete
- formatter
- fs_real
- fs_test_vibe
- gc
- glowup_http
- glyph_gang
- gob_encode_vibes
- goroutine_core
- grammar_drip
- hash_drip
- hash_map_enhanced
- hashtag
- heap_slay
- htmlrizzler
- image_processing
- io_enhanced
- io_simple
- io_test_vibe
- ioz
- ioz_enhanced
- ipc
- jit_vibes
- kubernetesz
- linter
- logging
- lookin_glass
- macro_slay
- math_float
- math_float_simple
- math_int
- math_rand_tea
- math_simple
- mathz_enhanced
- mathz_optimized
- memory_core
- memory_profiler
- mime_vibe
- module_resolver
- mood_map
- net_protocols
- net_real
- network
- networking_complete
- networkz_advanced
- nnz
- no_cap
- oglogging
- option
- package_manager
- packagz
- packrat
- panic_system
- parser
- path
- pathz
- performance_testing
- plug_vibes
- plugin_system
- plugin_vibes
- plugin_vibes_simple
- process
- process_real
- property_test
- property_testing
- pure_cursed_replacements
- pure_cursed_runtime
- pure_json
- quick_test
- reflect
- reflection
- regex
- regex_vibez
- regexz
- renderz
- result
- rizz_template
- rpc_vibes
- scientificz
- security_testing
- select_core
- self_hosted_builtins
- serialization
- signal_handling
- signalz
- simple_math
- simple_test
- sketchy_math
- slay_io
- slices_on_slices
- smtp_tea
- snapshot_testing
- sort_slay
- sorta_fresh
- spill_facts
- sqlz
- stat_flexin
- stdlib_linker
- string_energy
- string_pure
- stringz_enhanced
- sus_containers
- sus_log
- sysz
- tab_aesthetic
- tag_core
- tcpz
- template_engine
- test_minimal_arrayz
- test_no_array
- test_result
- test_super_simple
- test_vibes
- test_vibez_call
- test_vibez_minimal
- test_with_array
- testz_simple
- text_aesthetic
- time_zone_drip
- tlsz
- token_vibe
- trace_tea
- type_core
- unicode
- url_parsing
- user_check
- validation
- vibe_mime
- vibe_net
- vibez_optimized
- wasm_mood
- web
- web_vibez
- websocketz
- x509_certs_tea
- xmlz
- yeet_io
- zip_zilla

## Circular Dependencies ⚠️
The following circular dependencies were found:
1. filez → filez
2. beta → gamma → alpha → beta
3. module_b → module_a → module_b

## Module Metrics
| Module | Dependencies | Dependents | Functions | Tests | Depth |
|--------|-------------|------------|-----------|-------|-------|
| alpha | 1 | 1 | 1 | ✅ | 3 |
| archive_handling | 0 | 0 | 43 | ✅ | 0 |
| arrayz | 0 | 11 | 40 | ✅ | 0 |
| arrayz_optimized | 1 | 0 | 32 | ✅ | 1 |
| asn1_mood | 1 | 1 | 33 | ✅ | 1 |
| ast_mood | 1 | 1 | 80 | ✅ | 1 |
| async | 5 | 0 | 55 | ✅ | 3 |
| async_core | 1 | 0 | 43 | ✅ | 1 |
| async_runtime | 1 | 0 | 85 | ✅ | 1 |
| asyncz | 3 | 0 | 56 | ✅ | 3 |
| atomic_drip | 0 | 7 | 100 | ✅ | 0 |
| audioz | 5 | 0 | 113 | ✅ | 2 |
| benchmark_framework | 4 | 0 | 23 | ✅ | 3 |
| beta | 1 | 1 | 1 | ✅ | 3 |
| big_mood | 0 | 0 | 14 | ✅ | 0 |
| binary_drip | 0 | 1 | 18 | ✅ | 0 |
| blockchainz | 0 | 0 | 0 | ✅ | 0 |
| build_system | 9 | 0 | 26 | ✅ | 3 |
| build_system_simple | 1 | 0 | 8 | ✅ | 1 |
| bytefit | 0 | 0 | 39 | ✅ | 0 |
| chadlogging | 0 | 0 | 26 | ✅ | 0 |
| channel_core | 1 | 0 | 4 | ✅ | 1 |
| chaos_mode | 0 | 0 | 30 | ✅ | 0 |
| clock_bait | 1 | 0 | 52 | ✅ | 1 |
| cloudz | 0 | 0 | 0 | ❌ | 0 |
| collections | 2 | 4 | 85 | ✅ | 1 |
| collections_advanced | 1 | 0 | 6 | ✅ | 1 |
| collections_core | 1 | 0 | 77 | ✅ | 2 |
| collections_enhanced | 1 | 0 | 36 | ✅ | 1 |
| collections_simple | 1 | 0 | 8 | ✅ | 1 |
| command_line | 0 | 0 | 17 | ✅ | 0 |
| compare_mood | 0 | 0 | 39 | ✅ | 0 |
| compiler_core | 3 | 0 | 69 | ✅ | 3 |
| complex_module | 4 | 0 | 8 | ✅ | 4 |
| complex_vibe | 0 | 0 | 27 | ✅ | 0 |
| compression | 1 | 0 | 22 | ✅ | 1 |
| compressionz | 1 | 0 | 17 | ✅ | 1 |
| compressz | 3 | 0 | 75 | ✅ | 1 |
| concurrency | 4 | 0 | 80 | ✅ | 2 |
| concurrency_advanced | 3 | 0 | 35 | ✅ | 2 |
| concurrenz | 4 | 14 | 72 | ✅ | 2 |
| config | 1 | 0 | 47 | ✅ | 1 |
| config_package | 4 | 0 | 27 | ✅ | 3 |
| configz | 5 | 0 | 81 | ✅ | 2 |
| contract_testing | 1 | 0 | 10 | ✅ | 1 |
| core | 1 | 7 | 65 | ✅ | 1 |
| coverage_analysis | 6 | 0 | 34 | ✅ | 3 |
| coverage_runtime | 0 | 0 | 7 | ✅ | 0 |
| crypto | 1 | 4 | 22 | ✅ | 1 |
| crypto_complete | 1 | 0 | 17 | ✅ | 1 |
| crypto_production | 1 | 2 | 38 | ✅ | 1 |
| crypto_secure | 6 | 0 | 32 | ✅ | 1 |
| crypto_subtle_drip | 1 | 0 | 15 | ✅ | 1 |
| cryptz | 3 | 4 | 95 | ✅ | 1 |
| csv | 1 | 0 | 18 | ✅ | 2 |
| csv_mood | 3 | 0 | 6 | ✅ | 3 |
| csv_rfc4180 | 1 | 0 | 30 | ✅ | 2 |
| cursed_pointer | 1 | 0 | 29 | ✅ | 1 |
| data_drip | 0 | 0 | 34 | ✅ | 0 |
| database | 1 | 0 | 32 | ✅ | 1 |
| database_complete | 7 | 0 | 33 | ✅ | 3 |
| database_drivers | 1 | 0 | 24 | ✅ | 1 |
| database_orm | 1 | 0 | 86 | ✅ | 1 |
| database_production | 2 | 0 | 56 | ✅ | 2 |
| dbz | 7 | 0 | 66 | ✅ | 2 |
| debug_tea | 1 | 0 | 34 | ✅ | 1 |
| debugz | 2 | 1 | 50 | ✅ | 1 |
| deploymentz | 0 | 0 | 0 | ❌ | 0 |
| drawz | 5 | 0 | 21 | ✅ | 2 |
| dropz | 2 | 11 | 31 | ✅ | 2 |
| elliptic_curve_tea | 1 | 0 | 31 | ✅ | 2 |
| embed_that | 0 | 0 | 53 | ✅ | 0 |
| embeddedz | 0 | 0 | 0 | ❌ | 0 |
| encode_mood | 1 | 5 | 30 | ✅ | 1 |
| encoding_flex | 0 | 0 | 9 | ✅ | 0 |
| enhanced_collections | 2 | 0 | 6 | ✅ | 3 |
| enhanced_error | 1 | 0 | 76 | ✅ | 1 |
| enterprise_cloud | 0 | 0 | 0 | ❌ | 0 |
| enterprise_db | 0 | 0 | 0 | ❌ | 0 |
| enterprise_messaging | 0 | 0 | 0 | ❌ | 0 |
| enterprise_monitoring | 0 | 0 | 0 | ❌ | 0 |
| enterprise_security | 0 | 0 | 0 | ❌ | 0 |
| env | 3 | 0 | 42 | ✅ | 1 |
| envz | 1 | 0 | 34 | ✅ | 1 |
| error_core | 2 | 4 | 62 | ✅ | 2 |
| error_drip | 0 | 7 | 16 | ✅ | 0 |
| error_handling | 1 | 0 | 9 | ✅ | 1 |
| error_management | 2 | 0 | 28 | ✅ | 2 |
| errorz | 1 | 2 | 64 | ✅ | 1 |
| exec_slay | 4 | 1 | 23 | ✅ | 2 |
| exec_vibez | 0 | 0 | 44 | ✅ | 0 |
| filesystem_complete | 3 | 0 | 70 | ✅ | 2 |
| filez | 5 | 8 | 63 | ✅ | 1 |
| fmt | 2 | 1 | 31 | ✅ | 1 |
| formatter | 4 | 0 | 64 | ✅ | 1 |
| fs | 0 | 1 | 56 | ✅ | 0 |
| fs_real | 1 | 0 | 38 | ✅ | 1 |
| fs_test_vibe | 0 | 0 | 19 | ✅ | 0 |
| gamma | 1 | 1 | 1 | ✅ | 3 |
| gc | 3 | 0 | 26 | ✅ | 2 |
| glowup_http | 2 | 0 | 2 | ✅ | 1 |
| glyph_gang | 0 | 0 | 60 | ✅ | 0 |
| gob_encode_vibes | 1 | 0 | 17 | ✅ | 1 |
| goroutine_core | 1 | 0 | 18 | ✅ | 1 |
| grammar_drip | 0 | 0 | 23 | ✅ | 0 |
| hash_drip | 0 | 0 | 14 | ✅ | 0 |
| hash_map_enhanced | 2 | 0 | 16 | ✅ | 2 |
| hashtag | 1 | 0 | 14 | ✅ | 1 |
| hashz | 0 | 1 | 55 | ✅ | 0 |
| heap_slay | 0 | 0 | 26 | ✅ | 0 |
| htmlrizzler | 2 | 0 | 31 | ✅ | 1 |
| httpz | 3 | 1 | 60 | ✅ | 3 |
| httpz_v2 | 3 | 1 | 46 | ✅ | 2 |
| image_processing | 1 | 0 | 90 | ✅ | 1 |
| imagez | 5 | 1 | 96 | ✅ | 2 |
| io | 1 | 5 | 71 | ✅ | 1 |
| io_enhanced | 4 | 0 | 35 | ✅ | 3 |
| io_simple | 0 | 0 | 15 | ✅ | 0 |
| io_test_vibe | 3 | 0 | 17 | ✅ | 3 |
| ioz | 2 | 0 | 3 | ✅ | 2 |
| ioz_enhanced | 0 | 0 | 43 | ✅ | 0 |
| ipc | 1 | 0 | 37 | ✅ | 3 |
| jit_vibes | 1 | 0 | 16 | ✅ | 1 |
| json | 1 | 2 | 34 | ✅ | 1 |
| json_tea | 1 | 1 | 80 | ✅ | 1 |
| jsonz | 3 | 4 | 41 | ✅ | 1 |
| kubernetesz | 0 | 0 | 0 | ❌ | 0 |
| linter | 3 | 0 | 74 | ✅ | 1 |
| logging | 0 | 0 | 6 | ✅ | 0 |
| lookin_glass | 1 | 0 | 32 | ✅ | 1 |
| macro_slay | 2 | 0 | 58 | ✅ | 2 |
| main_character | 0 | 1 | 26 | ✅ | 0 |
| math | 1 | 1 | 47 | ✅ | 1 |
| math_float | 0 | 0 | 70 | ✅ | 0 |
| math_float_simple | 0 | 0 | 13 | ✅ | 0 |
| math_int | 0 | 0 | 62 | ✅ | 0 |
| math_rand_tea | 0 | 0 | 6 | ✅ | 0 |
| math_simple | 1 | 0 | 28 | ✅ | 1 |
| mathz | 0 | 19 | 44 | ✅ | 0 |
| mathz_enhanced | 0 | 0 | 32 | ✅ | 0 |
| mathz_optimized | 1 | 0 | 33 | ✅ | 1 |
| memory | 4 | 3 | 20 | ✅ | 1 |
| memory_core | 1 | 0 | 20 | ✅ | 1 |
| memory_profiler | 4 | 0 | 15 | ✅ | 2 |
| memoryz | 0 | 6 | 44 | ✅ | 0 |
| mime_vibe | 1 | 0 | 19 | ✅ | 1 |
| mlz | 4 | 1 | 67 | ✅ | 2 |
| module_a | 1 | 1 | 1 | ✅ | 2 |
| module_b | 1 | 1 | 1 | ✅ | 2 |
| module_resolver | 1 | 0 | 4 | ✅ | 1 |
| mood_map | 1 | 0 | 22 | ✅ | 1 |
| net | 1 | 1 | 91 | ✅ | 1 |
| net_protocols | 2 | 0 | 33 | ✅ | 2 |
| net_real | 2 | 0 | 43 | ✅ | 1 |
| network | 0 | 0 | 43 | ✅ | 0 |
| networking_complete | 5 | 0 | 49 | ✅ | 2 |
| networkz | 3 | 4 | 7 | ✅ | 1 |
| networkz_advanced | 4 | 0 | 30 | ✅ | 3 |
| nnz | 5 | 0 | 101 | ✅ | 2 |
| no_cap | 0 | 0 | 30 | ✅ | 0 |
| oglogging | 0 | 0 | 6 | ✅ | 0 |
| option | 1 | 0 | 31 | ✅ | 1 |
| package_manager | 8 | 0 | 37 | ✅ | 4 |
| packagz | 15 | 0 | 19 | ✅ | 2 |
| packrat | 1 | 0 | 17 | ✅ | 1 |
| panic_system | 1 | 0 | 13 | ✅ | 1 |
| parser | 3 | 0 | 6 | ✅ | 2 |
| path | 3 | 0 | 40 | ✅ | 1 |
| pathing | 0 | 1 | 21 | ✅ | 0 |
| pathz | 1 | 0 | 35 | ✅ | 1 |
| pem_drip | 0 | 1 | 25 | ✅ | 0 |
| performance_testing | 1 | 0 | 11 | ✅ | 1 |
| plug_vibes | 1 | 0 | 24 | ✅ | 1 |
| plugin_system | 1 | 0 | 37 | ✅ | 0 |
| plugin_vibes | 0 | 0 | 64 | ✅ | 0 |
| plugin_vibes_simple | 1 | 0 | 3 | ✅ | 1 |
| process | 1 | 0 | 48 | ✅ | 1 |
| process_real | 2 | 0 | 66 | ✅ | 1 |
| property_test | 1 | 0 | 11 | ✅ | 1 |
| property_testing | 1 | 0 | 13 | ✅ | 1 |
| pure_cursed_replacements | 0 | 0 | 0 | ❌ | 0 |
| pure_cursed_runtime | 2 | 0 | 18 | ✅ | 1 |
| pure_json | 3 | 0 | 78 | ✅ | 4 |
| quick_test | 3 | 0 | 32 | ✅ | 2 |
| reflect | 4 | 0 | 95 | ✅ | 2 |
| reflection | 1 | 0 | 89 | ✅ | 1 |
| regex | 1 | 0 | 26 | ✅ | 1 |
| regex_vibez | 0 | 0 | 27 | ✅ | 0 |
| regexz | 5 | 0 | 28 | ✅ | 1 |
| renderz | 5 | 0 | 119 | ✅ | 3 |
| result | 1 | 0 | 40 | ✅ | 1 |
| rizz_template | 0 | 0 | 32 | ✅ | 0 |
| rpc_vibes | 0 | 0 | 36 | ✅ | 0 |
| runtime_core | 1 | 6 | 18 | ✅ | 1 |
| scientificz | 0 | 0 | 0 | ❌ | 0 |
| security_testing | 1 | 0 | 10 | ✅ | 1 |
| select_core | 1 | 0 | 29 | ✅ | 1 |
| self_hosted_builtins | 4 | 0 | 42 | ✅ | 3 |
| serialization | 1 | 0 | 72 | ✅ | 2 |
| signal_boost | 2 | 1 | 16 | ✅ | 2 |
| signal_handling | 0 | 0 | 7 | ✅ | 0 |
| signalz | 3 | 0 | 42 | ✅ | 3 |
| simple_math | 0 | 0 | 4 | ✅ | 0 |
| simple_test | 0 | 0 | 2 | ✅ | 0 |
| sketchy_math | 0 | 0 | 37 | ✅ | 0 |
| slay_io | 0 | 0 | 42 | ✅ | 0 |
| slices_on_slices | 1 | 0 | 40 | ✅ | 1 |
| smtp_tea | 1 | 0 | 23 | ✅ | 1 |
| snapshot_testing | 1 | 0 | 8 | ✅ | 1 |
| sort_slay | 0 | 0 | 29 | ✅ | 0 |
| sorta_fresh | 0 | 0 | 53 | ✅ | 0 |
| spill_facts | 0 | 0 | 25 | ✅ | 0 |
| sql_slay | 1 | 1 | 78 | ✅ | 1 |
| sqlz | 1 | 0 | 30 | ✅ | 2 |
| squish_core | 1 | 1 | 34 | ✅ | 3 |
| stat_flexin | 4 | 0 | 42 | ✅ | 3 |
| stdlib_linker | 1 | 0 | 4 | ✅ | 1 |
| string | 1 | 11 | 32 | ✅ | 1 |
| string_energy | 2 | 0 | 75 | ✅ | 1 |
| string_enhanced | 3 | 1 | 34 | ✅ | 2 |
| string_pure | 0 | 0 | 63 | ✅ | 0 |
| string_simple | 2 | 2 | 72 | ✅ | 3 |
| stringz | 0 | 51 | 82 | ✅ | 0 |
| stringz_enhanced | 0 | 0 | 41 | ✅ | 0 |
| sus_containers | 4 | 0 | 7 | ✅ | 2 |
| sus_log | 5 | 0 | 48 | ✅ | 3 |
| sys_core | 1 | 2 | 50 | ✅ | 1 |
| sysz | 2 | 0 | 131 | ✅ | 2 |
| tab_aesthetic | 2 | 0 | 19 | ✅ | 1 |
| tag_core | 4 | 0 | 61 | ✅ | 3 |
| tcpz | 2 | 0 | 29 | ✅ | 1 |
| template_engine | 2 | 0 | 35 | ✅ | 2 |
| tensorz | 2 | 2 | 49 | ✅ | 1 |
| test_minimal_arrayz | 0 | 0 | 1 | ✅ | 0 |
| test_no_array | 0 | 0 | 1 | ✅ | 0 |
| test_result | 0 | 0 | 8 | ✅ | 0 |
| test_super_simple | 0 | 0 | 1 | ✅ | 0 |
| test_vibes | 3 | 0 | 62 | ✅ | 3 |
| test_vibez_call | 0 | 0 | 1 | ✅ | 0 |
| test_vibez_minimal | 0 | 0 | 1 | ✅ | 0 |
| test_with_array | 0 | 0 | 1 | ✅ | 0 |
| testz | 0 | 136 | 16 | ✅ | 0 |
| testz_simple | 0 | 0 | 6 | ✅ | 0 |
| text_aesthetic | 3 | 0 | 11 | ✅ | 3 |
| time | 1 | 5 | 26 | ✅ | 1 |
| time_zone_drip | 0 | 0 | 19 | ✅ | 0 |
| timez | 3 | 13 | 68 | ✅ | 1 |
| tls_vibe | 2 | 1 | 28 | ✅ | 2 |
| tlsz | 8 | 0 | 29 | ✅ | 2 |
| token_vibe | 1 | 0 | 3 | ✅ | 1 |
| trace_tea | 5 | 0 | 40 | ✅ | 4 |
| type_core | 1 | 0 | 38 | ✅ | 1 |
| typez | 0 | 1 | 40 | ✅ | 0 |
| unicode | 1 | 0 | 16 | ✅ | 1 |
| url_parsing | 0 | 0 | 50 | ✅ | 0 |
| user_check | 3 | 0 | 54 | ✅ | 2 |
| validation | 0 | 0 | 53 | ✅ | 0 |
| vibe_context | 3 | 1 | 22 | ✅ | 3 |
| vibe_life | 0 | 1 | 32 | ✅ | 0 |
| vibe_lock | 0 | 1 | 20 | ✅ | 0 |
| vibe_mime | 3 | 0 | 17 | ✅ | 2 |
| vibe_net | 1 | 0 | 66 | ✅ | 1 |
| vibecheck | 2 | 1 | 35 | ✅ | 2 |
| vibez | 0 | 25 | 27 | ✅ | 0 |
| vibez_optimized | 0 | 0 | 27 | ✅ | 0 |
| wasm_mood | 1 | 0 | 47 | ✅ | 0 |
| web | 9 | 0 | 99 | ✅ | 3 |
| web_vibez | 3 | 0 | 61 | ✅ | 3 |
| websocketz | 4 | 0 | 51 | ✅ | 2 |
| x509_certs_tea | 4 | 0 | 17 | ✅ | 2 |
| xmlz | 5 | 0 | 40 | ✅ | 3 |
| yeet_io | 0 | 0 | 10 | ✅ | 0 |
| zip_zilla | 6 | 0 | 16 | ✅ | 3 |

## Recommendations
1. **Critical Modules**: Focus testing efforts on critical modules with high fan-in
2. **Circular Dependencies**: Resolve circular dependencies
3. **Test Coverage**: Ensure all modules have comprehensive tests
4. **Integration Testing**: Test critical dependency pairs together
5. **Module Isolation**: Consider reducing dependencies in high-depth modules

## Test Order Recommendation
Based on dependency analysis, test modules in this order:
1. net_protocols
2. image_processing
3. enterprise_security
4. concurrency_advanced
5. arrayz_optimized
6. coverage_analysis
7. zip_zilla
8. ioz
9. build_system
10. simple_math
11. mime_vibe
12. slay_io
13. x509_certs_tea
14. option
15. plug_vibes
16. test_minimal_arrayz
17. blockchainz
18. database_drivers
19. config_package
20. quick_test
21. collections_advanced
22. plugin_vibes
23. pure_cursed_runtime
24. envz
25. test_vibez_call
26. error_management
27. hash_map_enhanced
28. csv_mood
29. channel_core
30. csv
31. chadlogging
32. networking_complete
33. mathz_enhanced
34. elliptic_curve_tea
35. compare_mood
36. lookin_glass
37. no_cap
38. stat_flexin
39. big_mood
40. stringz_enhanced
41. collections_enhanced
42. config
43. io_enhanced
44. database_complete
45. sysz
46. template_engine
47. validation
48. yeet_io
49. cloudz
50. contract_testing
51. property_test
52. deploymentz
53. spill_facts
54. env
55. slices_on_slices
56. glowup_http
57. string_pure
58. kubernetesz
59. networkz_advanced
60. string_energy
61. url_parsing
62. self_hosted_builtins
63. enhanced_collections
64. jit_vibes
65. path
66. packagz
67. reflection
68. dbz
69. data_drip
70. user_check
71. math_float_simple
72. embed_that
73. regex_vibez
74. tlsz
75. sqlz
76. time_zone_drip
77. nnz
78. rpc_vibes
79. xmlz
80. logging
81. error_handling
82. macro_slay
83. enterprise_messaging
84. test_result
85. ioz_enhanced
86. test_no_array
87. gob_encode_vibes
88. testz_simple
89. security_testing
90. build_system_simple
91. enterprise_db
92. smtp_tea
93. tcpz
94. tag_core
95. oglogging
96. test_super_simple
97. vibez_optimized
98. concurrency
99. goroutine_core
100. math_simple
101. benchmark_framework
102. encoding_flex
103. packrat
104. test_vibez_minimal
105. serialization
106. math_rand_tea
107. compression
108. async
109. net_real
110. hash_drip
111. compressz
112. signalz
113. embeddedz
114. glyph_gang
115. io_test_vibe
116. renderz
117. crypto_complete
118. tab_aesthetic
119. enterprise_cloud
120. drawz
121. simple_test
122. heap_slay
123. regexz
124. performance_testing
125. htmlrizzler
126. linter
127. web_vibez
128. sus_containers
129. text_aesthetic
130. cursed_pointer
131. enhanced_error
132. csv_rfc4180
133. math_float
134. fs_test_vibe
135. module_resolver
136. io_simple
137. fs_real
138. configz
139. pure_json
140. property_testing
141. select_core
142. debug_tea
143. rizz_template
144. trace_tea
145. async_runtime
146. complex_vibe
147. panic_system
148. mathz_optimized
149. math_int
150. result
151. web
152. network
153. wasm_mood
154. token_vibe
155. async_core
156. chaos_mode
157. complex_module
158. formatter
159. grammar_drip
160. memory_profiler
161. type_core
162. plugin_system
163. crypto_secure
164. scientificz
165. collections_core
166. collections_simple
167. compiler_core
168. websocketz
169. clock_bait
170. test_with_array
171. pure_cursed_replacements
172. database_orm
173. exec_vibez
174. memory_core
175. signal_handling
176. regex
177. ipc
178. package_manager
179. audioz
180. crypto_subtle_drip
181. asyncz
182. unicode
183. vibe_net
184. filesystem_complete
185. coverage_runtime
186. process
187. stdlib_linker
188. compressionz
189. vibe_mime
190. sorta_fresh
191. database
192. mood_map
193. bytefit
194. sus_log
195. enterprise_monitoring
196. hashtag
197. sort_slay
198. test_vibes
199. plugin_vibes_simple
200. snapshot_testing
201. command_line
202. reflect
203. pathz
204. archive_handling
205. parser
206. sketchy_math
207. gc
208. process_real
209. database_production
