yeet "vibecheck"

vibez.spill("CURSED Runtime Vibe Check")

sus version tea = vibecheck.Version()
vibez.spill("Version: %s", version)

sus compiler tea = vibecheck.Compiler()
vibez.spill("Compiler: %s", compiler)

sus arch tea = vibecheck.GOARCH()
vibez.spill("Architecture: %s", arch)

sus os tea = vibecheck.GOOS()
vibez.spill("OS: %s", os)

sus goroutines normie = vibecheck.NumGoroutine()
vibez.spill("Goroutines: %d", goroutines)

sus health lit = vibecheck.HealthCheck()
if health {
    vibez.spill("Health: OK")
} else {
    vibez.spill("Health: Warning")
}

vibez.spill("Vibe check complete")
