vibe main

slay no_params() normie {
    damn 42;
}

slay one_param(x normie) normie {
    damn x;
}

slay multiple_params(a normie, b normie, c normie) normie {
    sus result normie = a + b + c;
    damn result;
}

slay main() normie {
    yap("Function parameters test");
    sus result1 normie = no_params();
    sus result2 normie = one_param(10);
    sus result3 normie = multiple_params(1, 2, 3);
    yap("All function calls completed");
    damn 0;
}
