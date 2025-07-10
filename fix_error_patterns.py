#!/usr/bin/env python3

import re

# Read the file
with open('src/execution/mod.rs', 'r') as f:
    content = f.read()

# Fix all ExecutionFlow::Error patterns in match statements
fixes = [
    # Fix all "ExecutionFlow::Error(_) => todo!()," patterns
    (r'(ExecutionFlow::NextIteration\(label\) => return Ok\(ExecutionFlow::NextIteration\(label\)\),)\s*}', 
     r'\1\n                            ExecutionFlow::Error(error) => return Ok(ExecutionFlow::Error(error)),\n                        }'),
    
    # Fix all "ExecutionFlow::Error(_) => todo!()," patterns (different format)
    (r'(ExecutionFlow::NextIteration\(_\) => return Ok\(ExecutionFlow::Continue\(CursedValue::Nil\)\),)\s*}', 
     r'\1\n                        ExecutionFlow::Error(error) => return Ok(ExecutionFlow::Error(error)),\n                    }'),
    
    # Fix CursedValue::Error patterns
    (r'(&CursedValue::Nil => nil,)\s*}', 
     r'\1\n            &CursedValue::Error { .. } => "error",\n        }'),
]

for pattern, replacement in fixes:
    content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

# Write back
with open('src/execution/mod.rs', 'w') as f:
    f.write(content)

print("Fixed error patterns")
