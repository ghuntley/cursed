{
  "targets": [
    {
      "target_name": "tree_sitter_cursed_binding",
      "include_dirs": [
        "<!(node -e \"console.log(require('node-addon-api').include)\")",
        "src"
      ],
      "sources": [
        "bindings/node/binding.cc",
        "src/parser.c",
        "src/scanner.c"
      ],
      "cflags_c": [
        "-std=c99",
      ],
      "defines": [ "NAPI_VERSION=6" ]
    }
  ]
}
