#!/bin/bash
export LD_LIBRARY_PATH=/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:$LD_LIBRARY_PATH
export RUSTFLAGS="-L /nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib"
exec "$@"
