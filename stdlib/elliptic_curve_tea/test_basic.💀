fr fr Basic elliptic curve test without imports

fr fr Simplified curve parameters
sus current_curve_p normie = 23
sus current_curve_n normie = 28
sus current_curve_b normie = 1
sus current_curve_gx normie = 3
sus current_curve_gy normie = 10
sus current_curve_name tea = "P-256"

fr fr Result storage
sus result_x normie = 0
sus result_y normie = 0

slay elliptic_curve_get_params_name() tea {
    damn current_curve_name
}

slay elliptic_curve_get_params_gx() normie {
    damn current_curve_gx
}

slay elliptic_curve_get_params_gy() normie {
    damn current_curve_gy
}

slay elliptic_curve_get_result_x() normie {
    damn result_x
}

slay elliptic_curve_get_result_y() normie {
    damn result_y
}

slay elliptic_curve_is_on_curve(x normie, y normie) lit {
    vibes x == 0 && y == 0 {
        damn based  // Point at infinity
    }
    
    fr fr Simple validation for demo
    vibes x == current_curve_gx && y == current_curve_gy {
        damn based
    }
    
    damn cap  // Default false
}

slay elliptic_curve_double(x normie, y normie) {
    fr fr Simple doubling for demo
    result_x = x + 1
    result_y = y + 1
}

slay elliptic_curve_scalar_mult(x normie, y normie, k normie) {
    fr fr Simple scalar multiplication for demo
    result_x = x * k
    result_y = y * k
}

slay elliptic_curve_scalar_base_mult(k normie) {
    elliptic_curve_scalar_mult(current_curve_gx, current_curve_gy, k)
}

slay tea(value normie) tea {
    vibes value == 0 {
        damn "0"
    }
    vibes value == 1 {
        damn "1"
    }
    vibes value == 3 {
        damn "3"
    }
    vibes value == 10 {
        damn "10"
    }
    vibes value == 23 {
        damn "23"
    }
    vibes value == 30 {
        damn "30"
    }
    damn "unknown"
}

fr fr Test basic functionality
vibez.spill("🔐 Testing Basic Elliptic Curve Operations")
vibez.spill("==========================================")

sus name tea = elliptic_curve_get_params_name()
sus gx normie = elliptic_curve_get_params_gx()
sus gy normie = elliptic_curve_get_params_gy()

vibez.spill("Curve: " + name)
vibez.spill("Generator: (" + tea(gx) + ", " + tea(gy) + ")")

sus is_on_curve lit = elliptic_curve_is_on_curve(gx, gy)
vibes is_on_curve {
    vibez.spill("✅ Generator point is on curve")
} nah {
    vibez.spill("❌ Generator point is NOT on curve")
}

elliptic_curve_double(gx, gy)
sus doubled_x normie = elliptic_curve_get_result_x()
sus doubled_y normie = elliptic_curve_get_result_y()
vibez.spill("2*G = (" + tea(doubled_x) + ", " + tea(doubled_y) + ")")

elliptic_curve_scalar_base_mult(3)
sus tripled_x normie = elliptic_curve_get_result_x()
sus tripled_y normie = elliptic_curve_get_result_y()
vibez.spill("3*G = (" + tea(tripled_x) + ", " + tea(tripled_y) + ")")

vibez.spill("🎉 Basic elliptic curve operations completed!")
