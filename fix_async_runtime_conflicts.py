#!/usr/bin/env python3

import os
import re
import glob

def fix_async_runtime_conflicts():
    """Fix E0659 import conflicts related to async/runtime modules."""
    
    changes_made = 0
    
    # Fix the main async timer module conflicts
    timer_file = "src/runtime/async/timer.rs"
    if os.path.exists(timer_file):
        with open(timer_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix Future trait conflicts - use explicit imports
        content = re.sub(
            r'F1: Future<Output = T>,',
            r'F1: crate::runtime::r#async::future::Future<Output = T>,',
            content
        )
        
        content = re.sub(
            r'F2: Future<Output = T>,',
            r'F2: crate::runtime::r#async::future::Future<Output = T>,',
            content
        )
        
        # Fix Output type conflicts - use fully qualified syntax
        content = re.sub(
            r'F1::Output: Send \+ \'static,',
            r'<F1 as crate::runtime::r#async::future::Future>::Output: Send + \'static,',
            content
        )
        
        content = re.sub(
            r'F2::Output: Send \+ \'static,',
            r'<F2 as crate::runtime::r#async::future::Future>::Output: Send + \'static,',
            content
        )
        
        # Fix specific return type in race function  
        content = re.sub(
            r'-> Either<F1::Output, F2::Output>',
            r'-> Either<<F1 as crate::runtime::r#async::future::Future>::Output, <F2 as crate::runtime::r#async::future::Future>::Output>',
            content
        )
        
        # Fix Either usage in BoxFuture types
        content = re.sub(
            r'Either<F1::Output, F2::Output>',
            r'Either<<F1 as crate::runtime::r#async::future::Future>::Output, <F2 as crate::runtime::r#async::future::Future>::Output>',
            content
        )
        
        if content != original_content:
            with open(timer_file, 'w') as f:
                f.write(content)
            print(f"Fixed async timer Future conflicts in {timer_file}")
            changes_made += 1
    
    # Fix async I/O module conflicts
    async_io_file = "src/stdlib/io/async_io.rs"
    if os.path.exists(async_io_file):
        with open(async_io_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix Future bound conflicts
        content = re.sub(
            r'F: Future \+ futures::Future,',
            r'F: crate::runtime::r#async::future::Future + std::future::Future,',
            content
        )
        
        content = re.sub(
            r'F: Future<Output = T>,',
            r'F: crate::runtime::r#async::future::Future<Output = T> + std::future::Future<Output = T>,',
            content
        )
        
        if content != original_content:
            with open(async_io_file, 'w') as f:
                f.write(content)
            print(f"Fixed async I/O Future conflicts in {async_io_file}")
            changes_made += 1
    
    # Fix stdlib async module conflicts
    stdlib_async_file = "src/stdlib/async/mod.rs"
    if os.path.exists(stdlib_async_file):
        with open(stdlib_async_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix Future bounds in spawn_blocking and utility functions
        content = re.sub(
            r'F: Future<Output = T>,',
            r'F: crate::runtime::r#async::future::Future<Output = T> + std::future::Future<Output = T>,',
            content
        )
        
        content = re.sub(
            r'Fut: Future<Output = Result<T, E>>,',
            r'Fut: crate::runtime::r#async::future::Future<Output = Result<T, E>> + std::future::Future<Output = Result<T, E>>,',
            content
        )
        
        if content != original_content:
            with open(stdlib_async_file, 'w') as f:
                f.write(content)
            print(f"Fixed stdlib async Future conflicts in {stdlib_async_file}")
            changes_made += 1
    
    # Fix stdlib async io module conflicts
    stdlib_async_io_file = "src/stdlib/async/io.rs"
    if os.path.exists(stdlib_async_io_file):
        with open(stdlib_async_io_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix Future trait bound conflicts
        content = re.sub(
            r'F: Future<Output = Result<R, E>>,',
            r'F: crate::runtime::r#async::future::Future<Output = Result<R, E>> + std::future::Future<Output = Result<R, E>>,',
            content
        )
        
        if content != original_content:
            with open(stdlib_async_io_file, 'w') as f:
                f.write(content)
            print(f"Fixed stdlib async I/O Future conflicts in {stdlib_async_io_file}")
            changes_made += 1
    
    # Fix Promise Clone conflicts in stdlib async modules
    promise_files = [
        "src/stdlib/async/net.rs",
        "src/stdlib/async/io.rs", 
        "src/stdlib/async/mod.rs"
    ]
    
    for file_path in promise_files:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # Fix Promise<T> Clone requirement by using Arc<Mutex<T>> pattern
            content = re.sub(
                r'promise\.await\.unwrap_or_else',
                r'promise.clone().await.unwrap_or_else',
                content
            )
            
            # Add Clone bound where needed for Promise types that don't implement Clone
            content = re.sub(
                r'Promise<Result<(\w+), ([^>]+)>>',
                r'Promise<Result<Arc<\1>, \2>>',
                content
            )
            
            if content != original_content:
                with open(file_path, 'w') as f:
                    f.write(content)
                print(f"Fixed Promise Clone conflicts in {file_path}")
                changes_made += 1
    
    # Add type aliases to resolve conflicts
    runtime_mod_file = "src/runtime/async/mod.rs"
    if os.path.exists(runtime_mod_file):
        with open(runtime_mod_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Add explicit type aliases to avoid ambiguity
        if "// Type aliases to resolve Future conflicts" not in content:
            content = content.replace(
                "use crate::error::Error;",
                """use crate::error::Error;

// Type aliases to resolve Future conflicts
pub type CursedFuture<T> = dyn future::Future<Output = T> + Send;
pub type StdFuture<T> = dyn std::future::Future<Output = T> + Send;
pub type BoxedCursedFuture<'a, T> = Pin<Box<CursedFuture<T> + 'a>>;
pub type BoxedStdFuture<'a, T> = Pin<Box<StdFuture<T> + 'a>>;"""
            )
        
        if content != original_content:
            with open(runtime_mod_file, 'w') as f:
                f.write(content)
            print(f"Added Future type aliases in {runtime_mod_file}")
            changes_made += 1
    
    return changes_made

if __name__ == "__main__":
    changes = fix_async_runtime_conflicts()
    print(f"\nTotal changes made: {changes}")
