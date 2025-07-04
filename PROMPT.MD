0a. study specs/* to learn about the compiler specifications

0b. The source code of the compiler is in src/*

0c. study fix_plan.md.

1. Your task is to implement missing functionality and produce an compiled application in the cursed language via LLVM for that functionality. Follow the fix_plan.md and choose the most important thing. Before making changes search codebase (don't assume not implemented) using subagents. Think extra hard. You may use up to 500 subagents for search and edit operations but only 1 subagent for build/tests.

2. After implementing functionality or resolving problems, run the tests for that unit of code that was improved. If functionality is missing then it's your job to add it as per the application specifications. Think extra hard.

2. When you discover a parser, lexer, control flow or LLVM issue. Immediately update @fix_plan.md with your findings using a subagent. When the issue is resolved, update @fix_plan.md and remove the item using a subagent.

3. When the tests pass update the @fix_plan.md`, then add changed code and @fix_plan.md with "git add -A" via bash then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

999. Important: When authoring documentation (ie. rust doc) capture the why tests and the backing implementation is important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.

999999. As soon as there are no build or test errors create a git tag. If there are no git tags start at 0.0.0 and increment patch by 1 for example 0.0.1  if 0.0.0 does not exist.

999999999. You may add extra logging if required to be able to debug the issues.


9999999999. ALWAYS KEEP @fix_plan.md up to do date with your learnings using a subagent. Especially after wrapping up/finishing your turn.
