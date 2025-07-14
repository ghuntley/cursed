#!/bin/bash

# Fix RwLock operations in gc.rs
echo "Fixing GC RwLock operations..."

# Fix push operations
sed -i 's/\.stack_roots\.push(/\.stack_roots\.write()\.unwrap()\.push(/g' src/runtime/gc.rs
sed -i 's/\.global_roots\.push(/\.global_roots\.write()\.unwrap()\.push(/g' src/runtime/gc.rs  
sed -i 's/\.channel_roots\.push(/\.channel_roots\.write()\.unwrap()\.push(/g' src/runtime/gc.rs
sed -i 's/\.jit_roots\.push(/\.jit_roots\.write()\.unwrap()\.push(/g' src/runtime/gc.rs
sed -i 's/\.async_roots\.push(/\.async_roots\.write()\.unwrap()\.push(/g' src/runtime/gc.rs

# Fix iteration operations  
sed -i 's/for addr in stack_roots\.stack_roots/for addr in stack_roots.stack_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for addr in &roots\.stack_roots/for addr in roots.stack_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for &root_addr in &roots\.stack_roots/for root_addr in roots.stack_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for &root_addr in &roots\.global_roots/for root_addr in roots.global_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for &root_addr in &roots\.channel_roots/for root_addr in roots.channel_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for &root_addr in &roots\.jit_roots/for root_addr in roots.jit_roots.read().unwrap().iter()/g' src/runtime/gc.rs
sed -i 's/for &root_addr in &roots\.async_roots/for root_addr in roots.async_roots.read().unwrap().iter()/g' src/runtime/gc.rs

# Fix retain operations
sed -i 's/\.stack_roots\.retain(/\.stack_roots\.write()\.unwrap()\.retain(/g' src/runtime/gc.rs
sed -i 's/\.global_roots\.retain(/\.global_roots\.write()\.unwrap()\.retain(/g' src/runtime/gc.rs
sed -i 's/\.channel_roots\.retain(/\.channel_roots\.write()\.unwrap()\.retain(/g' src/runtime/gc.rs
sed -i 's/\.jit_roots\.retain(/\.jit_roots\.write()\.unwrap()\.retain(/g' src/runtime/gc.rs
sed -i 's/\.async_roots\.retain(/\.async_roots\.write()\.unwrap()\.retain(/g' src/runtime/gc.rs

# Fix Vec::new() issues in constructors
sed -i 's/stack_roots: Vec::new()/stack_roots: Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs
sed -i 's/global_roots: Vec::new()/global_roots: Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs
sed -i 's/channel_roots: Vec::new()/channel_roots: Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs
sed -i 's/jit_roots: Vec::new()/jit_roots: Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs
sed -i 's/async_roots: Vec::new()/async_roots: Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs

# Fix assignment to roots
sed -i 's/root_set\.stack_roots = Vec::new()/root_set.stack_roots = Arc::new(RwLock::new(Vec::new()))/g' src/runtime/gc.rs

echo "GC fixes applied"
