import re

with open('tests/generational_gc_comprehensive_test.rs', 'r') as f:
    content = f.read()

# Replace ObjectId::new(1) with incrementing IDs
counter = 1
def replace_func(match):
    global counter
    result = f"ObjectId::new({counter})"
    counter += 1
    return result

# Replace all ObjectId::new(1) with incrementing IDs
content = re.sub(r'ObjectId::new\(1\)', replace_func, content)

with open('tests/generational_gc_comprehensive_test.rs', 'w') as f:
    f.write(content)

print(f"Fixed {counter-1} ObjectId instances")
