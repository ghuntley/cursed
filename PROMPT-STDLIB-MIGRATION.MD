0. study specs/* to learn about the compiler specifications and fix_plan.md to understand plan so far.

1. The source code of the compiler is in src/*

2. The source code of the examples is in examples/* and the source code of the tree-sitter is in tree-sitter/*. Study them.

2. First task is to use up to 500 subagents to study existing standard library ("stdlib") source code authored in rust and compare it against the stdlib implementation in the cursed language. From that create/update a fix_plan.md which is a bullet point list sorted in priority of the items which have yet to be implemeneted in the cursed programing language itself. Think extra hard and use the oracle to plan. Study fix_plan.md to determine starting point for research and keep it up to date with items considered complete/incomplete. IMPORTANT - WE WANT THE ENTIRE STDLIB AUTHORED IN CURSED, NOT RUST. LOOK FOR MISSING STDLIB WHEN COMPARED TO "specs/stdlib/*"

3. IMPORTANT: The standard library should be built in cursed itself, not rust.
