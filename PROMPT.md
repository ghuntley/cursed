0. study specs/* to learn about the compiler specifications and fix_plan.md to understand plan so far.

1. The source code of the compiler is in src/*

2. The source code of the examples is in examples/* and the source code of the tree-sitter is in tree-sitter/*. Study them.

2. The source code of the stdlib is in src/stdlib/*. Study them.

2. First task is to study @fix_plan.md (it may be incorrect) and is to use up to 500 subagents to study existing source code in src/ and compare it against the compiler specifications. From that create/update a @fix_plan.md which is a bullet point list sorted in priority of the items which have yet to be implemeneted. Think extra hard and use the oracle to plan. Consider searching for TODO, minimal implementations and placeholders. Study @fix_plan.md to determine starting point for research and keep it up to date with items considered complete/incomplete using subagents.

2. Second task is to use up to 500 subagents to study existing source code in examples/ then compare it against the compiler specifications. From that create/update a fix_plan.md which is a bullet point list sorted in priority of the items which have yet to be implemeneted. Think extra hard and use the oracle to plan. Consider searching for TODO, minimal implementations and placeholders. Study fix_plan.md to determine starting point for research and keep it up to date with items considered complete/incomplete.

3. IMPORTANT: The standard library in src/stdlib should be built in cursed itself, not rust. If you find stdlib authored in rust then it must be noted that it needs to be migrated.

4. ULTIMATE GOAL we want to achieve a self-hosting compiler release with full standard library (stdlib). Consider missing stdlib modules and plan. If the stdlib is missing then author the specification at specs/stdlib/FILENAME.md (do NOT assume that it does not exist, search before creating). The naming of the module should be GenZ named and not conflict with another stdlib module name. If you create a new stdlib module then document the plan to implement in @fix_plan.md

5. The stage2 compiler is in src/bootstrap/*
