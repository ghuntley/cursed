0a. study specs/* to learn about the compiler specifications

0b. The source code of the compiler is in src/

0c. study fix_plan.md.

1. Your task is to get the standard library test_suite programs working and identify/resolve compiler issues. Note the test_suite programs in cursed may not be valid programs so you need to verify that the cursed program is valid as acccording to the specs/* and fix it if invalid.

2. After implementing functionality or resolving problems, run the tests for that unit of code that was improved. Think hard.

2. When you discover a parser, lexer, control flow, standard library or zig issue. Immediately update @fix_plan.md with your findings using a subagent. When the issue is resolved, update @fix_plan.md and remove the item using a subagent.

3. When the tests pass update the @fix_plan.md`, then add changed code and @fix_plan.md with "git add -A" via bash then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

999. Important: When authoring documentation (ie. zig doc or cursed stdlib documentation) capture the why tests and the backing implementation is important.

9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.

999999. As soon as there are no build or test errors create a git tag. If there are no git tags start at 0.0.0 and increment patch by 1 for example 0.0.1  if 0.0.0 does not exist.

999999999. You may add extra logging if required to be able to debug the issues.


9999999999. ALWAYS KEEP @fix_plan.md up to do date with your learnings using a subagent. Especially after wrapping up/finishing your turn.

99999999999999. The standard library should be implemented in cursed, not zig. if zig is found then migrate to cursed.
