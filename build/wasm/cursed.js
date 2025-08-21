// Basic CURSED WASM JavaScript bindings
class CursedModule {
    constructor() {
        this.instance = null;
        this.memory = null;
    }
    
    async load(wasmPath) {
        const wasmModule = await WebAssembly.instantiateStreaming(fetch(wasmPath), {
            js: {
                console_log: (ptr, len) => console.log(this.readString(ptr, len)),
                alert: (ptr, len) => alert(this.readString(ptr, len))
            }
        });
        
        this.instance = wasmModule.instance;
        this.memory = this.instance.exports.memory;
        return this;
    }
    
    readString(ptr, len) {
        const bytes = new Uint8Array(this.memory.buffer, ptr, len);
        return new TextDecoder().decode(bytes);
    }
    
    call(functionName, ...args) {
        return this.instance.exports[functionName](...args);
    }
}

window.CursedModule = CursedModule;
