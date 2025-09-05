fr fr CURSED Logging Module - Incremental Build

sus LOG_TRACE normie = 0
sus LOG_DEBUG normie = 1
sus LOG_INFO normie = 2
sus LOG_WARN normie = 3
sus LOG_ERROR normie = 4
sus LOG_FATAL normie = 5

sus global_log_prefix tea = "[CURSED]"

slay get_timestamp() tea {
    damn "2025-01-13T12:00:00Z"
}

slay log_info(message tea) tea {
    sus timestamp tea = get_timestamp()
    sus formatted tea = global_log_prefix + " [" + timestamp + "] INFO: " + message
    damn formatted
}

slay log_warn(message tea) tea {
    sus timestamp tea = get_timestamp()
    sus formatted tea = global_log_prefix + " [" + timestamp + "] WARN: " + message
    damn formatted
}

slay log_error(message tea) tea {
    sus timestamp tea = get_timestamp()
    sus formatted tea = global_log_prefix + " [" + timestamp + "] ERROR: " + message
    damn formatted
}

slay set_log_prefix(prefix tea) {
    global_log_prefix = prefix
}

slay create_logger(name tea) tea {
    sus prefix tea = global_log_prefix + "[" + name + "]"
    damn prefix
}
