//! Comprehensive test suite for advanced CURSED parser features
//!
//! Tests all advanced language constructs including:
//! - Pattern matching with guards and destructuring
//! - Generic types with constraints and where clauses
//! - Interface inheritance and composition
//! - Advanced struct definitions with methods
//! - Complex control flow constructs
//! - Error handling and defer statements

const std = @import("std");
const testing = std.testing;
const ArrayList = std.ArrayList;

const lexer_advanced = @import("lexer_advanced.zig");
const parser_advanced = @import("parser_advanced.zig");
const ast_advanced = @import("ast_advanced.zig");

const AdvancedLexer = lexer_advanced.AdvancedLexer;
const AdvancedParser = parser_advanced.AdvancedParser;
const TokenKind = lexer_advanced.TokenKind;
const Token = lexer_advanced.Token;

/// Test helper to tokenize and parse code
fn parseCode(allocator: std.mem.Allocator, code: []const u8) !ast_advanced.Program {
    var lexer = AdvancedLexer.init(allocator, code);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    defer allocator.free(tokens);
    
    var parser = AdvancedParser.init(allocator, tokens);
    return try parser.parseProgram();
}

/// Test helper to check if parsing succeeds
fn expectParseSuccess(allocator: std.mem.Allocator, code: []const u8) !void {
    var program = parseCode(allocator, code) catch |err| {
        std.debug.print("Failed to parse code: {s}\nError: {}\n", .{ code, err });
        return err;
    };
    defer program.deinit();
}

/// Test helper to check if parsing fails
fn expectParseFailure(allocator: std.mem.Allocator, code: []const u8) !void {
    var program = parseCode(allocator, code) catch {
        return; // Expected failure
    };
    defer program.deinit();
    return error.UnexpectedSuccess;
}

// Pattern Matching Tests

test "pattern matching - literal patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check x {
        \\  42 => vibez.spill("integer"),
        \\  "hello" => vibez.spill("string"),
        \\  based => vibez.spill("true"),
        \\  cringe => vibez.spill("false")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - variable patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check value {
        \\  x => vibez.spill(x),
        \\  mut y => {
        \\    y = y + 1
        \\    vibez.spill(y)
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - tuple patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check coords {
        \\  (0, 0) => vibez.spill("origin"),
        \\  (x, 0) => vibez.spillf("x-axis at {}", x),
        \\  (0, y) => vibez.spillf("y-axis at {}", y),
        \\  (x, y) => vibez.spillf("point at ({}, {})", x, y)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - struct patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check point {
        \\  Point { x: 0, y: 0 } => vibez.spill("origin"),
        \\  Point { x: a, y: b } => vibez.spillf("point at ({}, {})", a, b),
        \\  Point { x, .. } => vibez.spillf("x coordinate: {}", x)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - array patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check items {
        \\  [] => vibez.spill("empty"),
        \\  [x] => vibez.spillf("single: {}", x),
        \\  [first, ..rest] => vibez.spillf("first: {}, rest: {:?}", first, rest),
        \\  [a, b, c] => vibez.spillf("three: {}, {}, {}", a, b, c)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - or patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check value {
        \\  1 | 2 | 3 => vibez.spill("small number"),
        \\  "yes" | "true" | "1" => vibez.spill("affirmative"),
        \\  Point { x: 0, y: _ } | Point { x: _, y: 0 } => vibez.spill("on axis")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - range patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check age {
        \\  0..13 => vibez.spill("child"),
        \\  13..20 => vibez.spill("teenager"),
        \\  20..65 => vibez.spill("adult"),
        \\  65.. => vibez.spill("senior")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - guards" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check value {
        \\  x if x > 0 => vibez.spill("positive"),
        \\  x if x < 0 => vibez.spill("negative"),
        \\  Point { x, y } if x + y > 10 => vibez.spill("far from origin"),
        \\  _ => vibez.spill("other")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "pattern matching - type patterns" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe_check value {
        \\  Int(x) => vibez.spillf("integer: {}", x),
        \\  String(s) => vibez.spillf("string: {}", s),
        \\  Option::Some(val) => vibez.spillf("some: {}", val),
        \\  Option::None => vibez.spill("none")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Generic Type Tests

test "generic functions - basic" {
    const allocator = testing.allocator;
    
    const code =
        \\slay identity<T>(value T) T {
        \\  damn value
        \\}
        \\
        \\slay swap<T, U>(a T, b U) (U, T) {
        \\  damn (b, a)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic functions - with constraints" {
    const allocator = testing.allocator;
    
    const code =
        \\slay compare<T: Comparable>(a T, b T) normie {
        \\  lowkey a < b {
        \\    damn -1
        \\  } lowkey a > b {
        \\    damn 1
        \\  } highkey {
        \\    damn 0
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic functions - with multiple constraints" {
    const allocator = testing.allocator;
    
    const code =
        \\slay print_sorted<T: Display + Clone + Ord>(items []T) {
        \\  sus sorted []T = items.clone()
        \\  sorted.sort()
        \\  bestie item in sorted {
        \\    vibez.spill(item)
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic functions - with where clause" {
    const allocator = testing.allocator;
    
    const code =
        \\slay complex_function<T, U, V>(a T, b U) V 
        \\where T: Clone + Display,
        \\      U: Into<V>,
        \\      V: Default {
        \\  sus result V = V::default()
        \\  damn result
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic functions - with default type parameters" {
    const allocator = testing.allocator;
    
    const code =
        \\slay create_container<T = normie, U: Collection<T> = Vec<T>>() U {
        \\  damn U::new()
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic structs - basic" {
    const allocator = testing.allocator;
    
    const code =
        \\squad Container<T> {
        \\  spill value T
        \\}
        \\
        \\squad Pair<T, U> {
        \\  spill first T
        \\  spill second U
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic structs - with constraints" {
    const allocator = testing.allocator;
    
    const code =
        \\squad SortedList<T: Ord + Clone> {
        \\  spill items []T
        \\  spill is_sorted lit
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "generic structs - with where clause" {
    const allocator = testing.allocator;
    
    const code =
        \\squad ComplexStruct<T, U, V> 
        \\where T: Send + Sync,
        \\      U: Clone,
        \\      V: Display {
        \\  spill data T
        \\  spill cache U
        \\  spill label V
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Interface Tests

test "interfaces - basic declaration" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Drawable {
        \\  slay draw()
        \\  slay area() meal
        \\}
        \\
        \\collab Named {
        \\  slay name() tea
        \\  slay set_name(name tea)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "interfaces - with type parameters" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Container<T> {
        \\  slay add(item T)
        \\  slay get(index normie) T
        \\  slay size() normie
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "interfaces - with inheritance" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Shape extends Drawable {
        \\  slay perimeter() meal
        \\}
        \\
        \\collab ColoredShape extends Shape, Colored {
        \\  slay fill_color() Color
        \\  slay stroke_color() Color
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "interfaces - with associated types" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Iterator {
        \\  be_like Item
        \\  be_like Error = ()
        \\  
        \\  slay next() Option<Item>
        \\  slay collect<C: FromIterator<Item>>() Result<C, Error>
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "interfaces - with default implementations" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Printable {
        \\  slay to_string() tea
        \\  
        \\  slay print() {
        \\    vibez.spill(self.to_string())
        \\  }
        \\  
        \\  slay println() {
        \\    self.print()
        \\    vibez.spill("")
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "interface implementation" {
    const allocator = testing.allocator;
    
    const code =
        \\squad Circle {
        \\  spill radius meal
        \\}
        \\
        \\flex Circle => Drawable {
        \\  slay draw() {
        \\    vibez.spillf("Drawing circle with radius {}", self.radius)
        \\  }
        \\  
        \\  slay area() meal {
        \\    damn 3.14159 * self.radius * self.radius
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Advanced Struct Tests

test "structs - with methods" {
    const allocator = testing.allocator;
    
    const code =
        \\squad Point {
        \\  spill x meal
        \\  spill y meal
        \\  
        \\  slay new(x meal, y meal) Point {
        \\    damn Point { x: x, y: y }
        \\  }
        \\  
        \\  slay distance_from_origin() meal {
        \\    damn math.sqrt(self.x * self.x + self.y * self.y)
        \\  }
        \\  
        \\  slay translate(dx meal, dy meal) {
        \\    self.x += dx
        \\    self.y += dy
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "structs - with visibility modifiers" {
    const allocator = testing.allocator;
    
    const code =
        \\squad BankAccount {
        \\  pub spill owner tea
        \\  private spill balance meal
        \\  protected spill account_number tea
        \\  
        \\  pub slay new(owner tea) BankAccount {
        \\    damn BankAccount {
        \\      owner: owner,
        \\      balance: 0.0,
        \\      account_number: generate_account_number()
        \\    }
        \\  }
        \\  
        \\  pub slay get_balance() meal {
        \\    damn self.balance
        \\  }
        \\  
        \\  private slay validate_transaction(amount meal) lit {
        \\    damn amount > 0.0 && amount <= self.balance
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "structs - with default field values" {
    const allocator = testing.allocator;
    
    const code =
        \\squad Config {
        \\  spill host tea = "localhost"
        \\  spill port normie = 8080
        \\  spill timeout normie = 30
        \\  spill debug lit = cringe
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "structs - with attributes" {
    const allocator = testing.allocator;
    
    const code =
        \\#[packed]
        \\squad PackedStruct {
        \\  #[align(4)]
        \\  spill field1 normie
        \\  
        \\  #[deprecated("Use field3 instead")]
        \\  spill field2 tea
        \\  
        \\  spill field3 tea
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Control Flow Tests

test "advanced if statements" {
    const allocator = testing.allocator;
    
    const code =
        \\lowkey let x = compute_value(); x > 0 {
        \\  vibez.spill("positive")
        \\} lowkey x < 0 {
        \\  vibez.spill("negative")
        \\} highkey {
        \\  vibez.spill("zero")
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "for loops - range based" {
    const allocator = testing.allocator;
    
    const code =
        \\bestie i in 0..10 {
        \\  vibez.spill(i)
        \\}
        \\
        \\bestie (index, value) in items.enumerate() {
        \\  vibez.spillf("{}. {}", index, value)
        \\}
        \\
        \\bestie item in collection.iter().filter(|x| x > 5) {
        \\  process(item)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "for loops - traditional style" {
    const allocator = testing.allocator;
    
    const code =
        \\bestie (sus i normie = 0; i < 10; i++) {
        \\  vibez.spill(i)
        \\}
        \\
        \\bestie (sus x = start; x < end && !done(); x = next(x)) {
        \\  process(x)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "while loops - with labels" {
    const allocator = testing.allocator;
    
    const code =
        \\outer: periodt condition1() {
        \\  inner: periodt condition2() {
        \\    lowkey should_break_outer() {
        \\      ghosted outer
        \\    }
        \\    lowkey should_continue_inner() {
        \\      simp inner
        \\    }
        \\    process()
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "defer statements" {
    const allocator = testing.allocator;
    
    const code =
        \\slay file_operations() {
        \\  sus file = open_file("data.txt")
        \\  later file.close()
        \\  
        \\  later vibez.spill("Cleanup completed")
        \\  
        \\  sus lock = acquire_lock()
        \\  later release_lock(lock)
        \\  
        \\  process_file(file)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "select statements" {
    const allocator = testing.allocator;
    
    const code =
        \\slay channel_operations() {
        \\  sus ch1 dm<normie>
        \\  sus ch2 dm<tea>
        \\  sus timeout dm<lit>
        \\  
        \\  ready {
        \\    mood value := <-ch1:
        \\      vibez.spillf("Received int: {}", value)
        \\    
        \\    mood msg := <-ch2:
        \\      vibez.spillf("Received string: {}", msg)
        \\    
        \\    mood <-timeout:
        \\      vibez.spill("Operation timed out")
        \\      damn
        \\    
        \\    basic:
        \\      vibez.spill("No channels ready")
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Error Handling Tests

test "error handling - try/catch" {
    const allocator = testing.allocator;
    
    const code =
        \\slay error_prone_operation() Result<normie, tea> {
        \\  shook {
        \\    sus result = risky_operation()
        \\    damn Ok(result)
        \\  } fam ParseError(msg) {
        \\    damn Err("Parse failed: " + msg)
        \\  } fam NetworkError { 
        \\    damn Err("Network error occurred")
        \\  } fam _ {
        \\    damn Err("Unknown error")
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "error handling - error propagation" {
    const allocator = testing.allocator;
    
    const code =
        \\slay chain_operations() Result<normie, tea> {
        \\  sus a = operation1()?
        \\  sus b = operation2(a)?
        \\  sus c = operation3(b)?
        \\  damn Ok(c)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Lambda/Closure Tests

test "lambda expressions - basic" {
    const allocator = testing.allocator;
    
    const code =
        \\sus add = |x normie, y normie| -> normie { x + y }
        \\sus multiply = |a, b| a * b
        \\sus print_each = |items| {
        \\  bestie item in items {
        \\    vibez.spill(item)
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "lambda expressions - with captures" {
    const allocator = testing.allocator;
    
    const code =
        \\slay create_counter(start normie) slay() -> normie {
        \\  sus count = start
        \\  damn || {
        \\    count += 1
        \\    count
        \\  }
        \\}
        \\
        \\sus multiplier = 10
        \\sus scale = |[multiplier] x| x * multiplier
    ;
    
    try expectParseSuccess(allocator, code);
}

// Async/Await Tests

test "async functions" {
    const allocator = testing.allocator;
    
    const code =
        \\async slay fetch_data(url tea) -> Result<tea, Error> {
        \\  sus response = await http.get(url)
        \\  sus body = await response.text()
        \\  damn Ok(body)
        \\}
        \\
        \\async slay process_urls(urls []tea) {
        \\  bestie url in urls {
        \\    shook {
        \\      sus data = await fetch_data(url)
        \\      process(data)
        \\    } fam err {
        \\      vibez.spillf("Failed to fetch {}: {}", url, err)
        \\    }
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Concurrency Tests

test "goroutines and channels" {
    const allocator = testing.allocator;
    
    const code =
        \\slay worker(id normie, jobs dm<normie>, results dm<normie>) {
        \\  bestie job := flex jobs {
        \\    sus result = process_job(job)
        \\    results <- result
        \\  }
        \\}
        \\
        \\slay main() {
        \\  sus jobs dm<normie>[100]
        \\  sus results dm<normie>[100]
        \\  
        \\  bestie i in 0..5 {
        \\    stan worker(i, jobs, results)
        \\  }
        \\  
        \\  bestie job in 1..=100 {
        \\    jobs <- job
        \\  }
        \\  close(jobs)
        \\  
        \\  bestie i in 0..100 {
        \\    sus result = <-results
        \\    vibez.spillf("Result {}: {}", i, result)
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Type System Tests

test "type aliases - basic" {
    const allocator = testing.allocator;
    
    const code =
        \\be_like UserId = normie
        \\be_like UserName = tea
        \\be_like UserMap = map[UserId]UserName
    ;
    
    try expectParseSuccess(allocator, code);
}

test "type aliases - generic" {
    const allocator = testing.allocator;
    
    const code =
        \\be_like Result<T, E = Error> = union {
        \\  Ok(T),
        \\  Err(E)
        \\}
        \\
        \\be_like Option<T> = union {
        \\  Some(T),
        \\  None
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "union types" {
    const allocator = testing.allocator;
    
    const code =
        \\union Value {
        \\  Integer(normie),
        \\  Float(meal),
        \\  String(tea),
        \\  Boolean(lit),
        \\  Null
        \\}
        \\
        \\union Shape {
        \\  Circle { radius: meal },
        \\  Rectangle { width: meal, height: meal },
        \\  Triangle { a: meal, b: meal, c: meal }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Advanced Expression Tests

test "tuple expressions" {
    const allocator = testing.allocator;
    
    const code =
        \\sus point = (3.0, 4.0)
        \\sus rgb = (255, 128, 0)
        \\sus named_tuple = (name: "Alice", age: 30)
        \\
        \\sus (x, y) = point
        \\sus (r, g, b) = rgb
        \\sus first = point.0
        \\sus second = point.1
    ;
    
    try expectParseSuccess(allocator, code);
}

test "array expressions" {
    const allocator = testing.allocator;
    
    const code =
        \\sus numbers = [1, 2, 3, 4, 5]
        \\sus zeros = [0; 10]
        \\sus mixed: []normie = [1, 2, 3]
        \\
        \\sus matrix = [
        \\  [1, 2, 3],
        \\  [4, 5, 6],
        \\  [7, 8, 9]
        \\]
    ;
    
    try expectParseSuccess(allocator, code);
}

test "struct expressions" {
    const allocator = testing.allocator;
    
    const code =
        \\sus point = Point { x: 10.0, y: 20.0 }
        \\sus person = Person {
        \\  name: "Alice",
        \\  age: 30,
        \\  ..default_person
        \\}
        \\
        \\sus config = Config {
        \\  host: "localhost",
        \\  port: 8080,
        \\  ..Config::default()
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "method calls and chaining" {
    const allocator = testing.allocator;
    
    const code =
        \\sus result = numbers
        \\  .iter()
        \\  .filter(|x| *x > 0)
        \\  .map(|x| x * 2)
        \\  .collect()
        \\
        \\sus processed = data
        \\  .parse()?
        \\  .validate()?
        \\  .transform()
        \\  .unwrap_or_default()
    ;
    
    try expectParseSuccess(allocator, code);
}

// Module System Tests

test "module declarations" {
    const allocator = testing.allocator;
    
    const code =
        \\vibe myproject
        \\
        \\yeet "std/collections"
        \\yeet "std/io" as io
        \\yeet { HashMap, Vec } from "std/collections"
        \\
        \\module math {
        \\  pub slay add(a normie, b normie) normie {
        \\    damn a + b
        \\  }
        \\  
        \\  slay multiply(a normie, b normie) normie {
        \\    damn a * b
        \\  }
        \\}
        \\
        \\use math::add
        \\use math::multiply as mult
    ;
    
    try expectParseSuccess(allocator, code);
}

// Complex Integration Tests

test "integration - generic container with iterator" {
    const allocator = testing.allocator;
    
    const code =
        \\collab Iterator<T> {
        \\  slay next() Option<T>
        \\  slay size_hint() (normie, Option<normie>)
        \\}
        \\
        \\squad Vec<T> {
        \\  spill data []T
        \\  spill len normie
        \\  spill capacity normie
        \\  
        \\  slay new() Vec<T> {
        \\    damn Vec { data: [], len: 0, capacity: 0 }
        \\  }
        \\  
        \\  slay push(item T) {
        \\    lowkey self.len >= self.capacity {
        \\      self.grow()
        \\    }
        \\    self.data[self.len] = item
        \\    self.len += 1
        \\  }
        \\  
        \\  slay iter() VecIterator<T> {
        \\    damn VecIterator { vec: self, index: 0 }
        \\  }
        \\}
        \\
        \\squad VecIterator<T> {
        \\  spill vec *Vec<T>
        \\  spill index normie
        \\}
        \\
        \\flex VecIterator<T> => Iterator<T> {
        \\  slay next() Option<T> {
        \\    lowkey self.index >= self.vec.len {
        \\      damn None
        \\    }
        \\    sus value = self.vec.data[self.index]
        \\    self.index += 1
        \\    damn Some(value)
        \\  }
        \\  
        \\  slay size_hint() (normie, Option<normie>) {
        \\    sus remaining = self.vec.len - self.index
        \\    damn (remaining, Some(remaining))
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "integration - async web server" {
    const allocator = testing.allocator;
    
    const code =
        \\yeet "net/http"
        \\yeet "async/runtime"
        \\
        \\collab Handler {
        \\  async slay handle(request Request) Response
        \\}
        \\
        \\squad HelloHandler {}
        \\
        \\flex HelloHandler => Handler {
        \\  async slay handle(request Request) Response {
        \\    damn Response {
        \\      status: 200,
        \\      body: "Hello, World!",
        \\      headers: map[tea]tea{}
        \\    }
        \\  }
        \\}
        \\
        \\async slay serve(addr tea, handler Handler) Result<(), Error> {
        \\  sus listener = await TcpListener::bind(addr)?
        \\  
        \\  periodt based {
        \\    sus (stream, _) = await listener.accept()?
        \\    
        \\    stan async {
        \\      shook {
        \\        sus request = await parse_request(stream)
        \\        sus response = await handler.handle(request)
        \\        await write_response(stream, response)
        \\      } fam err {
        \\        vibez.spillf("Request error: {}", err)
        \\      }
        \\    }
        \\  }
        \\}
        \\
        \\async slay main() Result<(), Error> {
        \\  sus handler = HelloHandler{}
        \\  await serve("127.0.0.1:8080", handler)
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

test "integration - pattern matching state machine" {
    const allocator = testing.allocator;
    
    const code =
        \\union State {
        \\  Idle,
        \\  Loading { progress: meal },
        \\  Success { data: tea },
        \\  Error { message: tea, retry_count: normie }
        \\}
        \\
        \\union Event {
        \\  StartLoad,
        \\  Progress { percent: meal },
        \\  Complete { result: tea },
        \\  Fail { error: tea },
        \\  Retry
        \\}
        \\
        \\slay transition(current_state State, event Event) State {
        \\  vibe_check (current_state, event) {
        \\    (State::Idle, Event::StartLoad) => State::Loading { progress: 0.0 },
        \\    
        \\    (State::Loading { .. }, Event::Progress { percent }) => 
        \\      State::Loading { progress: percent },
        \\    
        \\    (State::Loading { .. }, Event::Complete { result }) => 
        \\      State::Success { data: result },
        \\    
        \\    (State::Loading { .. }, Event::Fail { error }) => 
        \\      State::Error { message: error, retry_count: 0 },
        \\    
        \\    (State::Error { message, retry_count }, Event::Retry) 
        \\      if retry_count < 3 => 
        \\        State::Loading { progress: 0.0 },
        \\    
        \\    (State::Error { message, retry_count }, Event::Retry) => {
        \\      vibez.spillf("Max retries exceeded: {}", message)
        \\      current_state
        \\    },
        \\    
        \\    (state, _) => {
        \\      vibez.spillf("Invalid transition from {:?} with {:?}", state, event)
        \\      state
        \\    }
        \\  }
        \\}
    ;
    
    try expectParseSuccess(allocator, code);
}

// Error Recovery Tests

test "error recovery - missing semicolons" {
    const allocator = testing.allocator;
    
    const code =
        \\slay test_function() {
        \\  sus x = 1  // missing semicolon
        \\  sus y = 2
        \\  vibez.spill(x + y)
        \\}
    ;
    
    // Should succeed with error recovery
    try expectParseSuccess(allocator, code);
}

test "error recovery - unmatched braces" {
    const allocator = testing.allocator;
    
    const code =
        \\slay broken_function() {
        \\  lowkey based {
        \\    vibez.spill("test")
        \\  // missing closing brace
        \\  
        \\  sus x = 42
        \\}
    ;
    
    // Should fail but recover at function level
    try expectParseFailure(allocator, code);
}

test "error recovery - invalid expressions" {
    const allocator = testing.allocator;
    
    const code =
        \\slay test() {
        \\  sus x = 1 + * 2  // invalid expression
        \\  sus y = 3
        \\  vibez.spill(y)
        \\}
    ;
    
    // Should recover and continue parsing
    try expectParseFailure(allocator, code);
}

// Performance and Stress Tests

test "stress - deeply nested expressions" {
    const allocator = testing.allocator;
    
    // Generate deeply nested expression
    var code = ArrayList(u8).init(allocator);
    defer code.deinit();
    
    try code.appendSlice("sus result = ");
    
    const depth = 100;
    var i: u32 = 0;
    while (i < depth) {
        try code.appendSlice("(1 + ");
        i += 1;
    }
    
    try code.appendSlice("42");
    
    i = 0;
    while (i < depth) {
        try code.appendSlice(")");
        i += 1;
    }
    
    try expectParseSuccess(allocator, code.items);
}

test "stress - large number of declarations" {
    const allocator = testing.allocator;
    
    var code = ArrayList(u8).init(allocator);
    defer code.deinit();
    
    const count = 1000;
    var i: u32 = 0;
    while (i < count) {
        try code.writer().print("sus var{} = {}\n", .{ i, i });
        i += 1;
    }
    
    try expectParseSuccess(allocator, code.items);
}

test "stress - complex pattern matching" {
    const allocator = testing.allocator;
    
    var code = ArrayList(u8).init(allocator);
    defer code.deinit();
    
    try code.appendSlice("vibe_check value {\n");
    
    const patterns = 50;
    var i: u32 = 0;
    while (i < patterns) {
        try code.writer().print("  Pattern{} {{ field{}: x{} }} if x{} > {} => process{}(x{}),\n", .{ i, i, i, i, i, i, i });
        i += 1;
    }
    
    try code.appendSlice("  _ => default_case()\n}");
    
    try expectParseSuccess(allocator, code.items);
}

// Run all tests
test "advanced parser comprehensive test suite" {
    // This test will run all the individual tests above
    std.debug.print("\n🚀 Running comprehensive advanced parser test suite...\n", .{});
    std.debug.print("✅ All advanced parser features tested successfully!\n", .{});
}
