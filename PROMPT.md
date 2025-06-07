study OUTCOME.md
the project uses devenv.sh - see devenv.nix - you may adjust it to install libraries if required. library names and packages can be determined by searching for nixpkgs at https://search.nixos.org/packages
run cargo test and summarize any build failures in OUTCOME.MD
if you need to install something via devenv, do it, then rerun the tests
if the tests pass, do a git commit and push it
then study specs/* to understand the application and use a subagent to resolve the failing test. make sure you run cargo test for that test to ensure you have fixed it.
