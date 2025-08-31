vibe main

sus no_params() -> std_int {
    damn 42;
}

sus one_param(x drip) -> std_int {
    damn x;
}

sus multiple_params(a drip, b drip, c drip) -> std_int {
    sus result drip = a + b + c;
    damn result;
}

sus main() -> std_int {
    yap("Function parameters test");
    sus result1 drip = no_params();
    sus result2 drip = one_param(10);
    sus result3 drip = multiple_params(1, 2, 3);
    yap("All function calls completed");
    damn 0;
}
