collab Writer {
    slay write(data tea) normie
}

squad SimpleWriter {
    name tea
    
    slay write(data tea) normie {
        vibez.spill("Writing: " + data)
        damn 1
    }
}

slay main() {
    sus writer SimpleWriter = SimpleWriter{name: "test"}
    writer.write("Hello, interface!")
}
