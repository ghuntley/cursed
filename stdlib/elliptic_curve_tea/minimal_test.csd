fr fr Minimal elliptic curve test

sus current_curve_name tea = "P-256"
sus current_curve_gx normie = 3
sus current_curve_gy normie = 10

slay get_curve_name() tea {
    damn current_curve_name
}

slay get_gx() normie {
    damn current_curve_gx
}

slay get_gy() normie {
    damn current_curve_gy
}

slay convert_to_string(value normie) tea {
    vibes value == 3 {
        damn "3"
    }
    vibes value == 10 {
        damn "10"
    }
    damn "unknown"
}

vibez.spill("🔐 Minimal Elliptic Curve Test")
vibez.spill("=============================")

sus name tea = get_curve_name()
sus gx normie = get_gx()
sus gy normie = get_gy()

vibez.spill("Curve: " + name)
vibez.spill("Generator X: " + convert_to_string(gx))
vibez.spill("Generator Y: " + convert_to_string(gy))

vibez.spill("✅ Minimal test completed!")
