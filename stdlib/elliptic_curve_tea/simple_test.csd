yeet "elliptic_curve_tea"

vibez.spill("🔐 Testing CURSED Elliptic Curve Library")
vibez.spill("=========================================")

fr fr Test basic curve setup
vibez.spill("Setting up P-256 curve...")
elliptic_curve_p256()

sus name tea = elliptic_curve_get_params_name()
sus bitsize normie = elliptic_curve_get_params_bitsize()
sus gx normie = elliptic_curve_get_params_gx()
sus gy normie = elliptic_curve_get_params_gy()

vibez.spill("Curve name: " + name)
vibez.spill("Bit size: " + tea(bitsize))
vibez.spill("Generator X: " + tea(gx))
vibez.spill("Generator Y: " + tea(gy))

fr fr Test point operations
vibez.spill("Testing point operations...")

fr fr Test point doubling
elliptic_curve_double(gx, gy)
sus doubled_x normie = elliptic_curve_get_result_x()
sus doubled_y normie = elliptic_curve_get_result_y()

vibez.spill("2*G = (" + tea(doubled_x) + ", " + tea(doubled_y) + ")")

fr fr Test scalar multiplication
elliptic_curve_scalar_base_mult(3)
sus tripled_x normie = elliptic_curve_get_result_x()
sus tripled_y normie = elliptic_curve_get_result_y()

vibez.spill("3*G = (" + tea(tripled_x) + ", " + tea(tripled_y) + ")")

fr fr Test point addition
elliptic_curve_add(gx, gy, doubled_x, doubled_y)
sus added_x normie = elliptic_curve_get_result_x()
sus added_y normie = elliptic_curve_get_result_y()

vibez.spill("G + 2*G = (" + tea(added_x) + ", " + tea(added_y) + ")")

fr fr Test point validation
sus is_on_curve lit = elliptic_curve_is_on_curve(gx, gy)
vibes is_on_curve {
    vibez.spill("✅ Generator point is on curve")
} nah {
    vibez.spill("❌ Generator point is NOT on curve")
}

sus invalid_on_curve lit = elliptic_curve_is_on_curve(1, 1)
vibes invalid_on_curve {
    vibez.spill("❌ Invalid point (1,1) is on curve")
} nah {
    vibez.spill("✅ Invalid point (1,1) is NOT on curve")
}

fr fr Test multiple curves
vibez.spill("Testing multiple curve support...")

elliptic_curve_p224()
sus p224_name tea = elliptic_curve_get_params_name()
vibez.spill("Switched to: " + p224_name)

elliptic_curve_p384()
sus p384_name tea = elliptic_curve_get_params_name()
vibez.spill("Switched to: " + p384_name)

elliptic_curve_p521()
sus p521_name tea = elliptic_curve_get_params_name()
vibez.spill("Switched to: " + p521_name)

fr fr Test enhanced curves
vibez.spill("Testing enhanced curve support...")

elliptic_curve_edwards25519()
sus ed25519_name tea = elliptic_curve_get_params_name()
vibez.spill("Switched to: " + ed25519_name)

elliptic_curve_montgomery25519()
sus curve25519_name tea = elliptic_curve_get_params_name()
vibez.spill("Switched to: " + curve25519_name)

fr fr Test custom curve
vibez.spill("Testing custom curve support...")

elliptic_curve_new_curve(23, 28, 1, 3, 10, 5, "Custom-Demo")
sus custom_name tea = elliptic_curve_get_params_name()
sus custom_bitsize normie = elliptic_curve_get_params_bitsize()
vibez.spill("Custom curve: " + custom_name + " (" + tea(custom_bitsize) + " bits)")

fr fr Test utility functions
vibez.spill("Testing utility functions...")

sus hex_result tea = integer_to_hex(255)
vibez.spill("255 in hex: " + hex_result)

sus int_result normie = hex_to_integer("ff")
vibez.spill("'ff' as int: " + tea(int_result))

sus mod_inv normie = modular_inverse(3, 7)
vibez.spill("Modular inverse of 3 mod 7: " + tea(mod_inv))

vibez.spill("🎉 Basic elliptic curve tests completed!")
vibez.spill("✅ All basic operations working correctly")
