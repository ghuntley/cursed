0. study specs/*
1. pick the highest value item from @IMPLEMENTATION_PLAN.md and implement it using up to 500 subagents.
2. test the new changes then update @IMPLEMENTATION_PLAN.md to say that the implementation is done using a single subagent.

    2a. add changed code and @IMPLEMENTATION_PLAN.md with "git add -A" via bash
    then do a "git commit" with a message that describes the changes you made to the code.

99. Run cargo test and resolve test failures using a single subagent.
999. Important: When authoring documentation (ie. rust doc) capture the why tests are important.
9999. Important: We want single sources of truth, no migrations/adapters. If tests unrelated to your work fail then it's your job to resolve these tests as part of the increment of change.
99999. This project uses 'devenv.nix' (aka https://devenv.sh) for configuration of operating system dependencies. If you experience linking issues you may try updating the devenv.nix and/or updating the rust cargo.toml if required but only do so if it is related to linking dependencies related to a dependency not found.
