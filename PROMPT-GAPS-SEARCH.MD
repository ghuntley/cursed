0. study specs/* to learn about the compiler specifications and fix_plan.md to understand plan so far.

1. The source code of the compiler is in src/*

2. The source code of the examples is in examples/* and the source code of the tree-sitter is in tree-sitter/*. Study them.

2. First task is to use up to 500 subagents to study existing source code in src/ and compare it against the compiler specifications. From that create/update a fix_plan.md which is a bullet point list sorted in priority of the items which have yet to be implemeneted. Think extra hard and use the oracle to plan. Consider searching for TODO, minimal implementations and placeholders. Study fix_plan.md to determine starting point for research and keep it up to date with items considered complete/incomplete.

2. Second task is to use up to 500 subagents to study existing source code in tree-sitter/ and examples/ then compare it against the compiler specifications. From that create/update a fix_plan.md which is a bullet point list sorted in priority of the items which have yet to be implemeneted. Think extra hard and use the oracle to plan. Consider searching for TODO, minimal implementations and placeholders. Study fix_plan.md to determine starting point for research and keep it up to date with items considered complete/incomplete.

3. IMPORTANT: The standard library should be built in cursed itself, not rust.

4. IMPORTANT: The specs/lexical.md was previously incorrect where "sus" was false and the codebase has incorrect usage of "sus". The keyword "false" is now called "cap" (previously mixed with vibe and false as "cap") make sure you look for this and also come up with migration plan.

5. IMPORTANT: The specs/lexical.md was previously incorrect where "cap" was nil and the codebase has incorrect usage of "cap". The keyword "nil" is now called "cringe" (previously mixed with nil as "cap") make sure you look for this and also come up with migration plan.
