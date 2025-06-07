0. study specs/*

1. The build is failing, look at @IMPLEMENTATION_PLAN.md for a summary of what the problems are. Pick the most important problem and resolve by either migration, implementation or whatever approach is best to resolve the error. Consider 10 options and chose the most likely one and resolve up to 25 parrallel subagents if required (your choice). After resolving, run the test for that unit of code.

2. When the tests pass update the @IMPLEMENTATION_PLAN.MD`, then add changed code and @IMPLEMENTATION_PLAN.md with "git add -A" via bash then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

999. Important: When authoring documentation (ie. rust doc) capture the why tests and the backing implementation is important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.
