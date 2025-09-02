# logz/backends.csd - Log Backend Implementations
# Multiple backend support: console, file, network, syslog

yeet "filez"
yeet "networkz"
yeet "stringz"
yeet "concurrenz"

# Console backend with color support
squad ConsoleBackend {
    sus formatter LogFormatter
    sus use_stderr lit
    
    slay new(formatter LogFormatter) ConsoleBackend {
        damn ConsoleBackend{ formatter: formatter, use_stderr: nah }
    }
    
    slay with_stderr(mut self ConsoleBackend) ConsoleBackend {
        self.use_stderr = based
        damn self
    }
}

give ConsoleBackend : LogBackend {
    slay write(self ConsoleBackend, entry LogEntry) yikes<tea> {
        sus formatted tea = self.formatter.format(entry)
        
        ready (self.use_stderr) {
            yeet "vibez"
            stderr_write(formatted + "\n") fam {
                when error -> yikes error
            }
        } otherwise {
            yeet "vibez"
            spill(formatted)
        }
        
        damn ""
    }
    
    slay flush(self ConsoleBackend) yikes<tea> {
        yeet "vibez"
        flush_stdout()
        damn ""
    }
    
    slay close(self ConsoleBackend) yikes<tea> {
        damn ""
    }
}

# File backend with rotation support
squad FileBackend {
    sus formatter LogFormatter
    sus file_path tea
    sus file_handle FileHandle
    sus max_size drip
    sus max_files drip
    sus current_size drip
    sus rotation_enabled lit
    
    slay new(formatter LogFormatter, file_path tea) yikes<FileBackend> {
        sus handle FileHandle = open_file_write(file_path) fam {
            when error -> yikes error
        }
        
        damn FileBackend{
            formatter: formatter,
            file_path: file_path,
            file_handle: handle,
            max_size: 100_000_000, # 100MB default
            max_files: 10,
            current_size: 0,
            rotation_enabled: based
        }
    }
    
    slay with_rotation(mut self FileBackend, max_size drip, max_files drip) FileBackend {
        self.max_size = max_size
        self.max_files = max_files
        self.rotation_enabled = based
        damn self
    }
    
    slay disable_rotation(mut self FileBackend) FileBackend {
        self.rotation_enabled = nah
        damn self
    }
    
    # Rotate log files when size limit reached
    slay rotate_if_needed(mut self FileBackend) yikes<tea> {
        ready (!self.rotation_enabled) {
            damn ""
        }
        
        ready (self.current_size < self.max_size) {
            damn ""
        }
        
        # Close current file
        close_file(self.file_handle) fam {
            when error -> yikes error
        }
        
        # Rotate existing files
        sus i drip = self.max_files - 1
        bestie (i >= 1) {
            sus old_name tea = self.file_path + "." + drip_to_string(i - 1)
            sus new_name tea = self.file_path + "." + drip_to_string(i)
            
            ready (file_exists(old_name)) {
                rename_file(old_name, new_name) fam {
                    when error -> # Ignore rename errors
                }
            }
            
            i = i - 1
        }
        
        # Move current file to .0
        rename_file(self.file_path, self.file_path + ".0") fam {
            when error -> yikes error
        }
        
        # Open new file
        self.file_handle = open_file_write(self.file_path) fam {
            when error -> yikes error
        }
        self.current_size = 0
        
        damn ""
    }
}

give FileBackend : LogBackend {
    slay write(mut self FileBackend, entry LogEntry) yikes<tea> {
        sus formatted tea = self.formatter.format(entry) + "\n"
        
        write_file_string(self.file_handle, formatted) fam {
            when error -> yikes error
        }
        
        self.current_size = self.current_size + len(formatted)
        self.rotate_if_needed() fam {
            when error -> yikes error
        }
        
        damn ""
    }
    
    slay flush(self FileBackend) yikes<tea> {
        flush_file(self.file_handle) fam {
            when error -> yikes error
        }
        damn ""
    }
    
    slay close(self FileBackend) yikes<tea> {
        close_file(self.file_handle) fam {
            when error -> yikes error
        }
        damn ""
    }
}

# Network backend for centralized logging
squad NetworkBackend {
    sus formatter LogFormatter
    sus target_host tea
    sus target_port drip
    sus protocol tea  # "tcp" or "udp"
    sus connection NetworkConnection
    sus reconnect_attempts drip
    sus max_reconnect_attempts drip
    sus buffer_size drip
    sus send_buffer chan<tea>
    sus worker_active lit
    
    slay new(formatter LogFormatter, host tea, port drip, protocol tea) yikes<NetworkBackend> {
        sus backend NetworkBackend = NetworkBackend{
            formatter: formatter,
            target_host: host,
            target_port: port,
            protocol: protocol,
            connection: NetworkConnection{},
            reconnect_attempts: 0,
            max_reconnect_attempts: 5,
            buffer_size: 1000,
            send_buffer: make_channel<tea>(1000),
            worker_active: nah
        }
        
        backend.connect() fam {
            when error -> yikes error
        }
        
        backend.start_sender_worker()
        damn backend
    }
    
    slay connect(mut self NetworkBackend) yikes<tea> {
        ready (self.protocol == "tcp") {
            self.connection = tcp_connect(self.target_host, self.target_port) fam {
                when error -> yikes error
            }
        } otherwise ready (self.protocol == "udp") {
            self.connection = udp_connect(self.target_host, self.target_port) fam {
                when error -> yikes error
            }
        } otherwise {
            yikes "unsupported protocol: " + self.protocol
        }
        
        self.reconnect_attempts = 0
        damn ""
    }
    
    slay reconnect(mut self NetworkBackend) yikes<tea> {
        ready (self.reconnect_attempts >= self.max_reconnect_attempts) {
            yikes "max reconnection attempts exceeded"
        }
        
        self.reconnect_attempts = self.reconnect_attempts + 1
        close_connection(self.connection) fam {
            when error -> # Ignore close errors
        }
        
        # Exponential backoff
        yeet "timez"
        sus backoff drip = self.reconnect_attempts * self.reconnect_attempts * 1000
        sleep_milliseconds(backoff)
        
        self.connect() fam {
            when error -> yikes error
        }
        
        damn ""
    }
    
    slay start_sender_worker(mut self NetworkBackend) {
        ready (!self.worker_active) {
            self.worker_active = based
            go {
                bestie (self.worker_active) {
                    sus message tea = <-self.send_buffer fam {
                        when _ -> break
                    }
                    
                    self.send_message(message) fam {
                        when error -> {
                            # Try to reconnect and resend
                            self.reconnect() fam {
                                when reconnect_error -> {
                                    # Log to stderr as fallback
                                    yeet "vibez"
                                    stderr_write("Network logging failed: " + error + "\n")
                                }
                                otherwise -> {
                                    # Retry send
                                    self.send_message(message) fam {
                                        when retry_error -> {
                                            stderr_write("Network retry failed: " + retry_error + "\n")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    slay send_message(self NetworkBackend, message tea) yikes<tea> {
        network_write(self.connection, message) fam {
            when error -> yikes error
        }
        damn ""
    }
}

give NetworkBackend : LogBackend {
    slay write(mut self NetworkBackend, entry LogEntry) yikes<tea> {
        sus formatted tea = self.formatter.format(entry) + "\n"
        
        # Send to async worker buffer
        self.send_buffer <- formatted fam {
            when _ -> {
                # Buffer full, drop message or write to stderr
                yeet "vibez"
                stderr_write("Network log buffer full, dropping message\n")
            }
        }
        
        damn ""
    }
    
    slay flush(self NetworkBackend) yikes<tea> {
        # Wait for buffer to empty (simplified)
        yeet "timez"
        sleep_milliseconds(100)
        damn ""
    }
    
    slay close(mut self NetworkBackend) yikes<tea> {
        self.worker_active = nah
        close_channel(self.send_buffer)
        close_connection(self.connection) fam {
            when error -> yikes error
        }
        damn ""
    }
}

# Syslog backend for Unix systems
squad SyslogBackend {
    sus formatter LogFormatter
    sus facility drip
    sus identity tea
    sus syslog_handle SyslogHandle
    
    slay new(formatter LogFormatter, identity tea, facility drip) yikes<SyslogBackend> {
        sus handle SyslogHandle = open_syslog(identity, facility) fam {
            when error -> yikes error
        }
        
        damn SyslogBackend{
            formatter: formatter,
            facility: facility,
            identity: identity,
            syslog_handle: handle
        }
    }
    
    # Convert CURSED log level to syslog priority
    slay level_to_priority(level LogLevel) drip {
        ready (level.name == "DEBUG") {
            damn 7  # LOG_DEBUG
        } otherwise ready (level.name == "INFO") {
            damn 6  # LOG_INFO
        } otherwise ready (level.name == "WARN") {
            damn 4  # LOG_WARNING
        } otherwise ready (level.name == "ERROR") {
            damn 3  # LOG_ERR
        } otherwise ready (level.name == "FATAL") {
            damn 2  # LOG_CRIT
        } otherwise {
            damn 6  # Default to INFO
        }
    }
}

give SyslogBackend : LogBackend {
    slay write(self SyslogBackend, entry LogEntry) yikes<tea> {
        sus priority drip = self.level_to_priority(entry.level)
        sus message tea = self.formatter.format(entry)
        
        syslog_write(self.syslog_handle, priority, message) fam {
            when error -> yikes error
        }
        
        damn ""
    }
    
    slay flush(self SyslogBackend) yikes<tea> {
        # Syslog handles flushing automatically
        damn ""
    }
    
    slay close(self SyslogBackend) yikes<tea> {
        close_syslog(self.syslog_handle) fam {
            when error -> yikes error
        }
        damn ""
    }
}

# Multi-backend aggregator
squad MultiBackend {
    sus backends LogBackend[value]
    
    slay new() MultiBackend {
        damn MultiBackend{ backends: LogBackend[value]{} }
    }
    
    slay add(mut self MultiBackend, backend LogBackend) MultiBackend {
        append(&self.backends, backend)
        damn self
    }
}

give MultiBackend : LogBackend {
    slay write(self MultiBackend, entry LogEntry) yikes<tea> {
        sus errors tea[value] = tea[value]{}
        
        bestie (backend in self.backends) {
            backend.write(entry) fam {
                when error -> append(&errors, error)
            }
        }
        
        ready (len(errors) > 0) {
            yikes join(errors, "; ")
        }
        
        damn ""
    }
    
    slay flush(self MultiBackend) yikes<tea> {
        bestie (backend in self.backends) {
            backend.flush() fam {
                when error -> # Continue with other backends
            }
        }
        damn ""
    }
    
    slay close(self MultiBackend) yikes<tea> {
        bestie (backend in self.backends) {
            backend.close() fam {
                when error -> # Continue with other backends
            }
        }
        damn ""
    }
}

# Buffered backend wrapper
squad BufferedBackend {
    sus backend LogBackend
    sus buffer LogEntry[value]
    sus buffer_size drip
    sus flush_interval drip
    sus last_flush drip
    
    slay new(backend LogBackend, buffer_size drip, flush_interval drip) BufferedBackend {
        sus buffered BufferedBackend = BufferedBackend{
            backend: backend,
            buffer: LogEntry[value]{},
            buffer_size: buffer_size,
            flush_interval: flush_interval,
            last_flush: current_timestamp()
        }
        
        # Start periodic flush
        go {
            bestie (based) {
                yeet "timez"
                sleep_milliseconds(flush_interval)
                buffered.flush_if_needed()
            }
        }
        
        damn buffered
    }
    
    slay flush_if_needed(mut self BufferedBackend) {
        sus now drip = current_timestamp()
        ready (len(self.buffer) >= self.buffer_size || 
               (now - self.last_flush) >= self.flush_interval) {
            self.do_flush()
        }
    }
    
    slay do_flush(mut self BufferedBackend) {
        bestie (entry in self.buffer) {
            self.backend.write(entry) fam {
                when error -> {
                    yeet "vibez"
                    stderr_write("Buffered backend flush error: " + error + "\n")
                }
            }
        }
        self.buffer = LogEntry[value]{}
        self.last_flush = current_timestamp()
        
        self.backend.flush() fam {
            when error -> # Ignore flush errors
        }
    }
}

give BufferedBackend : LogBackend {
    slay write(mut self BufferedBackend, entry LogEntry) yikes<tea> {
        append(&self.buffer, entry)
        self.flush_if_needed()
        damn ""
    }
    
    slay flush(mut self BufferedBackend) yikes<tea> {
        self.do_flush()
        damn ""
    }
    
    slay close(mut self BufferedBackend) yikes<tea> {
        self.do_flush()
        self.backend.close() fam {
            when error -> yikes error
        }
        damn ""
    }
}

# Factory functions for easy backend creation
slay console_backend() ConsoleBackend {
    damn ConsoleBackend.new(TextFormatter.default())
}

slay json_console_backend() ConsoleBackend {
    damn ConsoleBackend.new(JsonFormatter.new(nah))
}

slay file_backend(path tea) yikes<FileBackend> {
    damn FileBackend.new(TextFormatter.default(), path)
}

slay json_file_backend(path tea) yikes<FileBackend> {
    damn FileBackend.new(JsonFormatter.new(based), path)
}

slay network_backend(host tea, port drip) yikes<NetworkBackend> {
    damn NetworkBackend.new(JsonFormatter.new(nah), host, port, "tcp")
}

slay syslog_backend(identity tea) yikes<SyslogBackend> {
    damn SyslogBackend.new(TextFormatter.default(), identity, 16) # LOG_LOCAL0
}
