#!/usr/bin/env python3

import re

# Fix goroutine_spawn.rs
with open('src/ast/expressions/goroutine_spawn.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'use crate::ast::traits::Expression;',
    'use crate::ast::traits::{Expression, Node};'
)

content = re.sub(
    r'impl Expression for GoroutineSpawn \{\s*fn to_string\(&self\) -> String \{\s*format!\("stan \{\}", self\.function_call\.to_string\(\)\)\s*\}\s*fn as_any\(&self\) -> &dyn Any \{\s*self\s*\}\s*\}',
    '''impl Node for GoroutineSpawn {
    fn string(&self) -> String {
        format!("stan {}", self.function_call.string())
    }
    
    fn token_literal(&self) -> String {
        "stan".to_string()
    }
}

impl Expression for GoroutineSpawn {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}''',
    content,
    flags=re.MULTILINE | re.DOTALL
)

with open('src/ast/expressions/goroutine_spawn.rs', 'w') as f:
    f.write(content)

# Fix error_propagation.rs
with open('src/ast/expressions/error_propagation.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'use crate::ast::traits::Expression;',
    'use crate::ast::traits::{Expression, Node};'
)

content = re.sub(
    r'impl Expression for ErrorPropagation \{\s*fn to_string\(&self\) -> String \{\s*format!\("\{\}\?", self\.expression\.to_string\(\)\)\s*\}\s*fn as_any\(&self\) -> &dyn Any \{\s*self\s*\}\s*\}',
    '''impl Node for ErrorPropagation {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    }
    
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for ErrorPropagation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}''',
    content,
    flags=re.MULTILINE | re.DOTALL
)

with open('src/ast/expressions/error_propagation.rs', 'w') as f:
    f.write(content)

# Fix block.rs
with open('src/ast/expressions/block.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'use crate::ast::traits::Expression;',
    'use crate::ast::traits::{Expression, Node};'
)

content = re.sub(
    r'impl Expression for BlockExpression \{\s*fn to_string\(&self\) -> String \{\s*self\.block\.string\(\)\s*\}\s*fn as_any\(&self\) -> &dyn Any \{\s*self\s*\}\s*\}',
    '''impl Node for BlockExpression {
    fn string(&self) -> String {
        self.block.string()
    }
    
    fn token_literal(&self) -> String {
        self.block.token_literal()
    }
}

impl Expression for BlockExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}''',
    content,
    flags=re.MULTILINE | re.DOTALL
)

with open('src/ast/expressions/block.rs', 'w') as f:
    f.write(content)

print("Fixed remaining expression files")
