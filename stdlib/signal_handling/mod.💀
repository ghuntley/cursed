sus SIGNAL_INT normie = 2
sus SIGNAL_TERM normie = 15
sus SIGNAL_KILL normie = 9
sus SIGNAL_USR1 normie = 10
sus SIGNAL_USR2 normie = 12
sus SIGNAL_HUP normie = 1
sus SIGNAL_QUIT normie = 3
sus SIGNAL_PIPE normie = 13
sus SIGNAL_ALRM normie = 14
sus SIGNAL_CHLD normie = 17

sus signal_handlers_registered normie = 0
sus signal_system_active lit = cap
sus test_signal_received lit = cap

slay signal_init() lit {
    signal_handlers_registered = 0
    signal_system_active = based
    test_signal_received = cap
    damn based
}

slay signal_register(signal normie, handler_id normie) lit {
    signal_handlers_registered = signal_handlers_registered + 1
    damn based
}

slay signal_get_stats_handlers_registered() normie {
    damn signal_handlers_registered
}

slay signal_get_stats_is_active() normie {
    lowkey signal_system_active {
        damn 1
    }
    damn 0
}

slay signal_name(signal normie) tea {
    lowkey signal == SIGNAL_INT {
        damn "SIGINT"
    }
    lowkey signal == SIGNAL_TERM {
        damn "SIGTERM"
    }
    damn "UNKNOWN"
}

slay get_test_signal_received() lit {
    damn test_signal_received
}

slay reset_test_signal_state() {
    test_signal_received = cap
}
