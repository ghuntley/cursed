# CURSED Stdlib Dependency Analysis Report

## Overview
- **Total Modules**: 121
- **Total Dependencies**: 94
- **Circular Dependencies**: 0
- **Critical Modules**: 3
- **Leaf Modules**: 60
- **Root Modules**: 104

## Critical Modules (High Fan-In)
These modules are depended upon by many other modules:
- **testz**: 47 dependents
- **string**: 17 dependents
- **collections**: 9 dependents

## Leaf Modules (No Dependencies)
These modules don't depend on other stdlib modules:
- archive_handling
- atomic_drip
- big_mood
- binary_drip
- bytefit
- chadlogging
- chaos_mode
- collections
- command_line
- compare_mood
- complex_vibe
- core
- data_drip
- embed_that
- error_core
- error_drip
- exec_slay
- fs
- glyph_gang
- grammar_drip
- hash_drip
- heap_slay
- htmlrizzler
- image_processing
- logging
- main_character
- math
- math_float
- math_float_simple
- math_int
- math_rand_tea
- mathz
- memory
- net
- network
- no_cap
- pathing
- pem_drip
- process
- quick_test
- rpc_vibes
- sketchy_math
- slay_io
- smtp_tea
- sort_slay
- sorta_fresh
- spill_facts
- string
- string_pure
- test_result
- testz
- testz_simple
- time
- time_zone_drip
- tls_vibe
- url_parsing
- validation
- vibe_lock
- yeet_io
- zip_zilla

## Root Modules (No Dependents)
These modules are not used by other stdlib modules:
- archive_handling
- async
- atomic_drip
- big_mood
- bytefit
- chadlogging
- chaos_mode
- clock_bait
- collections_simple
- command_line
- compare_mood
- complex_vibe
- compression
- config
- csv
- cursed_pointer
- data_drip
- database
- debug_tea
- dropz
- elliptic_curve_tea
- embed_that
- encoding_flex
- error_drip
- error_handling
- exec_slay
- fmt
- fs
- glowup_http
- glyph_gang
- gob_encode_vibes
- grammar_drip
- hash_drip
- hashtag
- heap_slay
- htmlrizzler
- image_processing
- io_enhanced
- io_simple
- logging
- lookin_glass
- math
- math_float
- math_float_simple
- math_int
- math_rand_tea
- math_simple
- mathz
- memory
- mime_vibe
- mood_map
- network
- no_cap
- option
- packrat
- parser
- pathing
- plug_vibes
- plugin_system
- process
- quick_test
- reflect
- reflection
- regex
- result
- rizz_template
- rpc_vibes
- serialization
- signal_boost
- sketchy_math
- slay_io
- slices_on_slices
- smtp_tea
- sort_slay
- sorta_fresh
- spill_facts
- sql_slay
- stat_flexin
- string_pure
- string_simple
- sus_containers
- sus_log
- sys_core
- tag_core
- template_engine
- test_result
- testz_simple
- time_zone_drip
- tls_vibe
- trace_tea
- unicode
- url_parsing
- user_check
- validation
- vibe_context
- vibe_life
- vibe_lock
- vibe_mime
- vibecheck
- vibez
- web
- x509_certs_tea
- yeet_io
- zip_zilla

## Module Metrics
| Module | Dependencies | Dependents | Functions | Tests | Depth |
|--------|-------------|------------|-----------|-------|-------|
| archive_handling | 0 | 0 | 43 | ✅ | 0 |
| asn1_mood | 1 | 1 | 33 | ✅ | 1 |
| async | 5 | 0 | 76 | ✅ | 2 |
| atomic_drip | 0 | 0 | 24 | ✅ | 0 |
| big_mood | 0 | 0 | 14 | ✅ | 0 |
| binary_drip | 0 | 1 | 18 | ✅ | 0 |
| bytefit | 0 | 0 | 39 | ✅ | 0 |
| chadlogging | 0 | 0 | 26 | ✅ | 0 |
| chaos_mode | 0 | 0 | 30 | ✅ | 0 |
| clock_bait | 1 | 0 | 50 | ✅ | 1 |
| collections | 1 | 9 | 79 | ✅ | 0 |
| collections_simple | 1 | 0 | 12 | ✅ | 1 |
| command_line | 0 | 0 | 14 | ✅ | 0 |
| compare_mood | 0 | 0 | 39 | ✅ | 0 |
| complex_vibe | 0 | 0 | 27 | ✅ | 0 |
| compression | 1 | 0 | 32 | ✅ | 1 |
| concurrency | 4 | 1 | 80 | ✅ | 1 |
| concurrenz | 1 | 1 | 40 | ✅ | 1 |
| config | 3 | 0 | 19 | ✅ | 2 |
| core | 0 | 1 | 16 | ✅ | 0 |
| crypto | 1 | 2 | 40 | ✅ | 1 |
| csv | 1 | 0 | 18 | ✅ | 1 |
| cursed_pointer | 1 | 0 | 29 | ✅ | 1 |
| data_drip | 0 | 0 | 34 | ✅ | 0 |
| database | 4 | 0 | 23 | ✅ | 2 |
| debug_tea | 1 | 0 | 34 | ✅ | 1 |
| dropz | 1 | 0 | 16 | ✅ | 1 |
| elliptic_curve_tea | 1 | 0 | 31 | ✅ | 2 |
| embed_that | 0 | 0 | 53 | ✅ | 0 |
| encoding_flex | 1 | 0 | 19 | ✅ | 1 |
| error_core | 0 | 1 | 28 | ✅ | 0 |
| error_drip | 0 | 0 | 12 | ✅ | 0 |
| error_handling | 1 | 0 | 9 | ✅ | 1 |
| exec_slay | 0 | 0 | 20 | ✅ | 0 |
| fmt | 2 | 0 | 31 | ✅ | 2 |
| fs | 0 | 0 | 27 | ✅ | 0 |
| glowup_http | 1 | 0 | 48 | ✅ | 1 |
| glyph_gang | 0 | 0 | 60 | ✅ | 0 |
| gob_encode_vibes | 1 | 0 | 17 | ✅ | 1 |
| grammar_drip | 0 | 0 | 23 | ✅ | 0 |
| hash_drip | 0 | 0 | 14 | ✅ | 0 |
| hashtag | 1 | 0 | 14 | ✅ | 1 |
| heap_slay | 0 | 0 | 26 | ✅ | 0 |
| htmlrizzler | 0 | 0 | 24 | ✅ | 0 |
| image_processing | 0 | 0 | 33 | ✅ | 0 |
| io | 1 | 1 | 34 | ✅ | 1 |
| io_enhanced | 1 | 0 | 43 | ✅ | 1 |
| io_simple | 1 | 0 | 12 | ✅ | 1 |
| json | 2 | 4 | 67 | ✅ | 1 |
| logging | 0 | 0 | 40 | ✅ | 0 |
| lookin_glass | 1 | 0 | 18 | ✅ | 1 |
| main_character | 0 | 1 | 26 | ✅ | 0 |
| math | 0 | 0 | 8 | ✅ | 0 |
| math_float | 0 | 0 | 70 | ✅ | 0 |
| math_float_simple | 0 | 0 | 13 | ✅ | 0 |
| math_int | 0 | 0 | 62 | ✅ | 0 |
| math_rand_tea | 0 | 0 | 6 | ✅ | 0 |
| math_simple | 1 | 0 | 14 | ✅ | 1 |
| mathz | 0 | 0 | 18 | ✅ | 0 |
| memory | 6 | 0 | 12 | ✅ | 0 |
| mime_vibe | 1 | 0 | 19 | ✅ | 1 |
| mood_map | 1 | 0 | 22 | ✅ | 1 |
| net | 0 | 1 | 91 | ✅ | 0 |
| network | 0 | 0 | 43 | ✅ | 0 |
| no_cap | 0 | 0 | 30 | ✅ | 0 |
| option | 1 | 0 | 31 | ✅ | 1 |
| packrat | 1 | 0 | 17 | ✅ | 1 |
| parser | 4 | 0 | 54 | ✅ | 2 |
| pathing | 0 | 0 | 21 | ✅ | 0 |
| pem_drip | 0 | 1 | 25 | ✅ | 0 |
| plug_vibes | 1 | 0 | 24 | ✅ | 1 |
| plugin_system | 1 | 0 | 34 | ✅ | 1 |
| process | 0 | 0 | 23 | ✅ | 0 |
| quick_test | 0 | 0 | 29 | ✅ | 0 |
| reflect | 1 | 0 | 52 | ✅ | 1 |
| reflection | 1 | 0 | 30 | ✅ | 1 |
| regex | 1 | 0 | 25 | ✅ | 1 |
| result | 1 | 0 | 40 | ✅ | 1 |
| rizz_template | 1 | 0 | 23 | ✅ | 1 |
| rpc_vibes | 0 | 0 | 36 | ✅ | 0 |
| serialization | 1 | 0 | 48 | ✅ | 1 |
| signal_boost | 2 | 0 | 38 | ✅ | 1 |
| sketchy_math | 0 | 0 | 55 | ✅ | 0 |
| slay_io | 0 | 0 | 42 | ✅ | 0 |
| slices_on_slices | 1 | 0 | 40 | ✅ | 1 |
| smtp_tea | 0 | 0 | 20 | ✅ | 0 |
| sort_slay | 0 | 0 | 29 | ✅ | 0 |
| sorta_fresh | 0 | 0 | 53 | ✅ | 0 |
| spill_facts | 0 | 0 | 25 | ✅ | 0 |
| sql_slay | 1 | 0 | 78 | ✅ | 1 |
| stat_flexin | 1 | 0 | 11 | ✅ | 1 |
| string | 0 | 17 | 8 | ✅ | 0 |
| string_pure | 0 | 0 | 63 | ✅ | 0 |
| string_simple | 1 | 0 | 8 | ✅ | 1 |
| stringz | 1 | 2 | 22 | ✅ | 1 |
| sus_containers | 1 | 0 | 7 | ✅ | 1 |
| sus_log | 1 | 0 | 11 | ✅ | 1 |
| sys_core | 1 | 0 | 19 | ✅ | 1 |
| tag_core | 1 | 0 | 20 | ✅ | 1 |
| template_engine | 2 | 0 | 35 | ✅ | 1 |
| test_result | 0 | 0 | 8 | ✅ | 0 |
| testz | 0 | 47 | 14 | ✅ | 0 |
| testz_simple | 0 | 0 | 8 | ✅ | 0 |
| time | 0 | 3 | 67 | ✅ | 0 |
| time_zone_drip | 0 | 0 | 19 | ✅ | 0 |
| tls_vibe | 0 | 0 | 19 | ✅ | 0 |
| trace_tea | 2 | 0 | 27 | ✅ | 1 |
| unicode | 1 | 0 | 27 | ✅ | 1 |
| url_parsing | 0 | 0 | 50 | ✅ | 0 |
| user_check | 3 | 0 | 33 | ✅ | 1 |
| validation | 0 | 0 | 50 | ✅ | 0 |
| vibe_context | 3 | 0 | 22 | ✅ | 2 |
| vibe_life | 1 | 0 | 27 | ✅ | 1 |
| vibe_lock | 0 | 0 | 20 | ✅ | 0 |
| vibe_mime | 3 | 0 | 17 | ✅ | 2 |
| vibecheck | 1 | 0 | 38 | ✅ | 1 |
| vibez | 2 | 0 | 23 | ✅ | 2 |
| web | 5 | 0 | 43 | ✅ | 2 |
| x509_certs_tea | 4 | 0 | 17 | ✅ | 2 |
| yeet_io | 0 | 0 | 10 | ✅ | 0 |
| zip_zilla | 0 | 0 | 14 | ✅ | 0 |

## Recommendations
1. **Critical Modules**: Focus testing efforts on critical modules with high fan-in
2. **Circular Dependencies**: No circular dependencies found ✅
3. **Test Coverage**: Ensure all modules have comprehensive tests
4. **Integration Testing**: Test critical dependency pairs together
5. **Module Isolation**: Consider reducing dependencies in high-depth modules

## Test Order Recommendation
Based on dependency analysis, test modules in this order:
1. complex_vibe
2. math_int
3. csv
4. vibez
5. sql_slay
6. packrat
7. async
8. sketchy_math
9. data_drip
10. unicode
11. mime_vibe
12. database
13. hashtag
14. heap_slay
15. mathz
16. signal_boost
17. cursed_pointer
18. math_float
19. sorta_fresh
20. serialization
21. compare_mood
22. stat_flexin
23. regex
24. plugin_system
25. io_enhanced
26. string_pure
27. compression
28. zip_zilla
29. math_simple
30. vibecheck
31. yeet_io
32. error_drip
33. sus_log
34. string_simple
35. web
36. glowup_http
37. rizz_template
38. sys_core
39. tls_vibe
40. quick_test
41. pathing
42. fmt
43. user_check
44. math_float_simple
45. error_handling
46. chaos_mode
47. math_rand_tea
48. memory
49. reflect
50. vibe_mime
51. option
52. process
53. archive_handling
54. testz_simple
55. image_processing
56. big_mood
57. math
58. parser
59. dropz
60. vibe_life
61. fs
62. smtp_tea
63. sus_containers
64. tag_core
65. mood_map
66. plug_vibes
67. url_parsing
68. time_zone_drip
69. x509_certs_tea
70. elliptic_curve_tea
71. exec_slay
72. bytefit
73. grammar_drip
74. command_line
75. network
76. htmlrizzler
77. lookin_glass
78. vibe_context
79. slay_io
80. test_result
81. hash_drip
82. template_engine
83. trace_tea
84. gob_encode_vibes
85. spill_facts
86. io_simple
87. sort_slay
88. rpc_vibes
89. glyph_gang
90. config
91. no_cap
92. chadlogging
93. atomic_drip
94. vibe_lock
95. encoding_flex
96. reflection
97. slices_on_slices
98. embed_that
99. clock_bait
100. result
101. logging
102. debug_tea
103. validation
104. collections_simple
