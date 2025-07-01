0a. study specs/* to learn about the application specifications using up to 500 subagents.

0b. study src/* to learn about the application implementations using up to 500 subagents.

0c. study fix_plan.md and use ultra think, think extra hard.

1. Your task is to produce an compiled application in the cursed language via LLVM. You may use up to 500 subagents for operations but only 1 subagent for build/tests. Follow the fix_plan.md and choose the most important thing. Before making changes search codebase (don't assume not implemented). After resolving, run the tests for that unit of code. If functionality is missing then it's your job to add it as per the application specifications

2. When the tests pass update the @fix_plan.md`, then add changed code and @fix_plan.md with "git add -A" via bash then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

999. Important: When authoring documentation (ie. rust doc) capture the why tests and the backing implementation is important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.

999999. As soon as there are no build or test errors create a git tag. If there are no git tags start at 0.0.0 and increment patch by 1 for example 0.0.1  if 0.0.0 does not exist.

999999999. You may add extra logging if required to be able to debug the issues.


9999999999. ALWAYS KEEP @fix_plan.md up to do date with your learnings using a subagent. Especially after wrapping up/finishing your turn.
