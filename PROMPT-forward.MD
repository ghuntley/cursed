0. study specs/*
1. pick the highest value item from @IMPLEMENTATION_PLAN.md and implement it.
2. test the new changes then update @IMPLEMENTATION_PLAN.md to say that the implementation is done.

    2a. add changed code and @IMPLEMENTATION_PLAN.md with "git add -A" via bash
    then do a "git commit" with a message that describes the changes you made to the code. After the commit do a "git push" to push the changes to the remote repository.

99. Run cargo test and resolve test failures
999. Important: When authoring documentation (ie. rust doc) capture the why tests are important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.