# CURSED Lexer Keyword Coverage Analysis

## Keywords from specs/lexical.md vs Implementation

### ✅ Keywords Present in Implementation:
| Go Keyword | CURSED Keyword | Status |
|------------|---------------|---------|
| func       | slay          | ✅ Present |
| return     | yolo          | ✅ Present |
| var        | sus           | ✅ Present |
| const      | facts         | ✅ Present |
| if         | lowkey        | ✅ Present |
| else       | highkey       | ✅ Present |
| while      | periodt       | ✅ Present |
| go         | stan          | ✅ Present |
| for        | bestie        | ✅ Present |
| range      | flex          | ✅ Present |
| break      | ghosted       | ✅ Present |
| continue   | simp          | ✅ Present |
| struct     | squad         | ✅ Present |
| interface  | collab        | ✅ Present |
| package    | vibe          | ✅ Present |
| import     | yeet          | ✅ Present |
| type       | be_like       | ✅ Present |
| switch     | vibe_check    | ✅ Present |
| case       | mood          | ✅ Present |
| default    | basic         | ✅ Present |
| chan       | dm            | ✅ Present |

### ❌ Keywords MISSING from Implementation:
| Go Keyword | CURSED Keyword | Status |
|------------|---------------|---------|
| map        | tea           | ❌ MISSING - tea is tokenized as Tea type, not map |
| defer      | later         | ❌ MISSING |
| true       | based         | ❌ MISSING - uses generic Boolean token |
| false      | sus           | ❌ MISSING - conflicts with variable sus |
| nil        | cap           | ❌ MISSING - uses Cap but should be nil literal |

### ⚠️ Issues Found:

1. **`tea` keyword conflict**: In the lexer, "tea" is mapped to `TokenType::Tea` (string type), but the specs show it should map to `map` keyword. This is a semantic conflict.

2. **`sus` keyword conflict**: The specs show both `var → sus` and `false → sus`. The lexer only handles the variable declaration meaning.

3. **Boolean literals**: The specs show `true → based` and `false → sus`, but the lexer uses generic `Boolean` token for "true" and "false" literals.

4. **Missing `later` keyword**: The `defer → later` keyword is not implemented.

5. **Nil literal**: The specs show `nil → cap`, but the lexer uses `Cap` for bool type and `NoCap` for nil.

6. **Comment tokens**: The specs define comments as `fr fr` (line) and `no cap...on god` (block), but these are not tokenized in the implementation.

## Additional Tokens in Implementation Not in Specs:
- `YeetError` (panic)
- `Catch` (catch/recover)  
- `Normie` (int type)
- `MainCharacter` (main function)
- `Arrow` (->)
- `Match` (alternative to vibe_check)
- `If` (alternative to lowkey)
- `Async`/`Await`

## Total Coverage:
- **Keywords from specs**: 25 keywords
- **Implemented correctly**: 20 keywords  
- **Missing or incorrect**: 5 keywords
- **Coverage**: 80%

## Critical Fixes Needed:
1. Add `Later` token for defer keyword
2. Add `Based` token for true literal  
3. Resolve `sus` conflict (false vs variable)
4. Resolve `tea` conflict (map vs string type)
5. Fix `cap` to be nil literal, not bool type
6. Add comment tokenization for `fr fr` and `no cap...on god`
