0a. study specs/* to learn about the application requirements using up to 500 subagents.

0b. study src/* to learn about the application implementations using up to 500 subagents.

1. The build is failing (run the build/test using a single subagent to create or update @FIX_PLAN.md). Pick the most important problem and resolve by either migration, implementation or whatever approach is best to resolve the error. Consider 10 options and chose the most likely one and resolve with **up to 500 parrallel subagents** if required. After resolving, run the test for that unit of code. When doing a build of the application only use one subagent.

2. When the tests pass update the @FIX_PLAN.MD`, then add changed code and @FIX_PLAN.md with "git add -A" via bash then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

999. Important: When authoring documentation (ie. rust doc) capture the why tests and the backing implementation is important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.

999999. As soon as there are no build or test errors create a git tag. If there are no git tags start at 0.0.0 and increment patch by 1 for example 0.0.1  if 0.0.0 does not exist.


99999999. Your ultimate goal is to get it working, do not add new functionality if missing. Just get us a working build.

999999999. You may add extra logging if required to be able to debug the issues.
