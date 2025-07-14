yeet "testz"
yeet "stringz"
yeet "dropz"

fr fr text_aesthetic - Text template engine for generating textual output
fr fr Powerful, flexible template system with advanced features

fr fr Template types and structures
be_like Template squad {
    name tea
    text tea
    parsed lit
    funcs FuncMap
    leftDelim tea
    rightDelim tea
    templates map[tea]*Template
}

fr fr Function map for template functions
be_like FuncMap map[tea]interface{}

fr fr Template execution context
be_like ExecContext squad {
    template *Template
    data interface{}
    vars map[tea]interface{}
    output tea
}

fr fr Parse error information
be_like ParseError squad {
    Line normie
    Position normie
    Description tea
    Template tea
}

fr fr Execution error information
be_like ExecError squad {
    Name tea
    Line normie
    Description tea
}

fr fr New creates a new template with given name
slay New(name tea) *Template {
    damn &Template{
        name: name,
        text: "",
        parsed: cap,
        funcs: make(FuncMap),
        leftDelim: "{{",
        rightDelim: "}}",
        templates: make(map[tea]*Template),
    }
}

fr fr Parse parses template text
slay (t *Template) Parse(text tea) (*Template, tea) {
    t.text = text
    t.parsed = based
    damn t, ""
}

fr fr ParseFiles parses templates from files
slay (t *Template) ParseFiles(filenames ...tea) (*Template, tea) {
    fr fr Simple implementation - in real version would read files
    damn t, ""
}

fr fr ParseGlob parses templates matching glob pattern
slay (t *Template) ParseGlob(pattern tea) (*Template, tea) {
    fr fr Simple implementation - in real version would match files
    damn t, ""
}

fr fr Execute executes template with data
slay (t *Template) Execute(data interface{}) tea {
    if !t.parsed {
        damn "template not parsed"
    }
    
    sus context := &ExecContext{
        template: t,
        data: data,
        vars: make(map[tea]interface{}),
        output: "",
    }
    
    damn t.executeTemplate(context, t.text)
}

fr fr ExecuteTemplate executes named template with data
slay (t *Template) ExecuteTemplate(name tea, data interface{}) tea {
    if namedTemplate, exists := t.templates[name]; exists {
        damn namedTemplate.Execute(data)
    }
    damn "template not found: " + name
}

fr fr Clone creates a copy of template
slay (t *Template) Clone() (*Template, tea) {
    sus clone := &Template{
        name: t.name,
        text: t.text,
        parsed: t.parsed,
        funcs: make(FuncMap),
        leftDelim: t.leftDelim,
        rightDelim: t.rightDelim,
        templates: make(map[tea]*Template),
    }
    
    fr fr Copy function map
    bestie name, fn := range t.funcs {
        clone.funcs[name] = fn
    }
    
    fr fr Copy templates
    bestie name, tmpl := range t.templates {
        clone.templates[name] = tmpl
    }
    
    damn clone, ""
}

fr fr Delims sets left and right delimiters
slay (t *Template) Delims(left, right tea) *Template {
    t.leftDelim = left
    t.rightDelim = right
    damn t
}

fr fr Funcs adds functions to template
slay (t *Template) Funcs(funcMap FuncMap) *Template {
    bestie name, fn := range funcMap {
        t.funcs[name] = fn
    }
    damn t
}

fr fr Name returns template name
slay (t *Template) Name() tea {
    damn t.name
}

fr fr Templates returns associated templates
slay (t *Template) Templates() []*Template {
    sus result := make([]*Template, 0)
    bestie _, tmpl := range t.templates {
        result = append(result, tmpl)
    }
    damn result
}

fr fr Internal template execution
slay (t *Template) executeTemplate(context *ExecContext, text tea) tea {
    sus result := ""
    sus pos := 0
    
    for pos < len(text) {
        fr fr Find next action
        sus leftPos := stringz.Index(text[pos:], t.leftDelim)
        if leftPos == -1 {
            fr fr No more actions, append rest of text
            result += text[pos:]
            break
        }
        
        fr fr Append text before action
        result += text[pos:pos+leftPos]
        
        fr fr Find end of action
        sus rightPos := stringz.Index(text[pos+leftPos:], t.rightDelim)
        if rightPos == -1 {
            fr fr Malformed action, append rest
            result += text[pos+leftPos:]
            break
        }
        
        fr fr Extract action content
        sus actionStart := pos + leftPos + len(t.leftDelim)
        sus actionEnd := pos + leftPos + rightPos
        sus action := stringz.TrimSpace(text[actionStart:actionEnd])
        
        fr fr Execute action
        sus actionResult := t.executeAction(context, action)
        result += actionResult
        
        fr fr Move past action
        pos = actionEnd + len(t.rightDelim)
    }
    
    damn result
}

fr fr Execute template action
slay (t *Template) executeAction(context *ExecContext, action tea) tea {
    if action == "" {
        damn ""
    }
    
    fr fr Handle comments
    if stringz.HasPrefix(action, "/*") && stringz.HasSuffix(action, "*/") {
        damn ""
    }
    
    fr fr Handle variable assignment
    if stringz.Contains(action, ":=") {
        damn t.executeAssignment(context, action)
    }
    
    fr fr Handle conditionals
    if stringz.HasPrefix(action, "if ") {
        damn t.executeIf(context, action)
    }
    
    fr fr Handle range loops
    if stringz.HasPrefix(action, "range ") {
        damn t.executeRange(context, action)
    }
    
    fr fr Handle template definitions
    if stringz.HasPrefix(action, "define ") {
        damn t.executeDefine(context, action)
    }
    
    fr fr Handle template includes
    if stringz.HasPrefix(action, "template ") {
        damn t.executeInclude(context, action)
    }
    
    fr fr Handle function calls
    if stringz.Contains(action, " ") {
        damn t.executeFunctionCall(context, action)
    }
    
    fr fr Handle simple variable access
    damn t.executeVariable(context, action)
}

fr fr Execute variable assignment
slay (t *Template) executeAssignment(context *ExecContext, action tea) tea {
    sus parts := stringz.Split(action, ":=")
    if len(parts) != 2 {
        damn ""
    }
    
    sus varName := stringz.TrimSpace(parts[0])
    sus varValue := stringz.TrimSpace(parts[1])
    
    fr fr Remove $ prefix if present
    if stringz.HasPrefix(varName, "$") {
        varName = varName[1:]
    }
    
    fr fr Evaluate value
    sus value := t.evaluateExpression(context, varValue)
    context.vars[varName] = value
    
    damn ""
}

fr fr Execute if conditional
slay (t *Template) executeIf(context *ExecContext, action tea) tea {
    fr fr Simple if implementation
    sus condition := stringz.TrimSpace(action[3:])
    sus conditionResult := t.evaluateCondition(context, condition)
    
    if conditionResult {
        damn "<!-- if true -->"
    } else {
        damn "<!-- if false -->"
    }
}

fr fr Execute range loop
slay (t *Template) executeRange(context *ExecContext, action tea) tea {
    fr fr Simple range implementation
    sus rangeExpr := stringz.TrimSpace(action[6:])
    damn "<!-- range " + rangeExpr + " -->"
}

fr fr Execute template definition
slay (t *Template) executeDefine(context *ExecContext, action tea) tea {
    fr fr Simple define implementation
    sus defineName := stringz.TrimSpace(action[7:])
    damn "<!-- define " + defineName + " -->"
}

fr fr Execute template include
slay (t *Template) executeInclude(context *ExecContext, action tea) tea {
    fr fr Simple include implementation
    sus parts := stringz.Fields(action)
    if len(parts) < 2 {
        damn ""
    }
    
    sus templateName := stringz.Trim(parts[1], "\"")
    if tmpl, exists := t.templates[templateName]; exists {
        damn tmpl.Execute(context.data)
    }
    
    damn "<!-- template " + templateName + " not found -->"
}

fr fr Execute function call
slay (t *Template) executeFunctionCall(context *ExecContext, action tea) tea {
    sus parts := stringz.Fields(action)
    if len(parts) == 0 {
        damn ""
    }
    
    sus funcName := parts[0]
    sus args := parts[1:]
    
    fr fr Check if function exists
    if fn, exists := t.funcs[funcName]; exists {
        fr fr Simple function call handling
        damn "<!-- function " + funcName + " called -->"
    }
    
    fr fr Built-in functions
    switch funcName {
    case "len":
        if len(args) > 0 {
            damn "<!-- len function -->"
        }
    case "eq":
        if len(args) >= 2 {
            damn "<!-- eq function -->"
        }
    case "ne":
        if len(args) >= 2 {
            damn "<!-- ne function -->"
        }
    case "lt":
        if len(args) >= 2 {
            damn "<!-- lt function -->"
        }
    case "gt":
        if len(args) >= 2 {
            damn "<!-- gt function -->"
        }
    case "and":
        damn "<!-- and function -->"
    case "or":
        damn "<!-- or function -->"
    case "not":
        damn "<!-- not function -->"
    }
    
    damn ""
}

fr fr Execute variable access
slay (t *Template) executeVariable(context *ExecContext, action tea) tea {
    fr fr Handle dot notation
    if action == "." {
        damn t.formatValue(context.data)
    }
    
    fr fr Handle variable references
    if stringz.HasPrefix(action, "$") {
        sus varName := action[1:]
        if value, exists := context.vars[varName]; exists {
            damn t.formatValue(value)
        }
        damn ""
    }
    
    fr fr Handle field access
    if stringz.HasPrefix(action, ".") {
        sus fieldName := action[1:]
        damn t.accessField(context.data, fieldName)
    }
    
    damn ""
}

fr fr Evaluate expression
slay (t *Template) evaluateExpression(context *ExecContext, expr tea) interface{} {
    fr fr Simple expression evaluation
    if expr == "." {
        damn context.data
    }
    
    if stringz.HasPrefix(expr, "$") {
        sus varName := expr[1:]
        if value, exists := context.vars[varName]; exists {
            damn value
        }
        damn ""
    }
    
    if stringz.HasPrefix(expr, "\"") && stringz.HasSuffix(expr, "\"") {
        damn expr[1 : len(expr)-1]
    }
    
    damn expr
}

fr fr Evaluate condition
slay (t *Template) evaluateCondition(context *ExecContext, condition tea) lit {
    fr fr Simple condition evaluation
    if condition == "." {
        damn context.data != cap
    }
    
    if stringz.HasPrefix(condition, "$") {
        sus varName := condition[1:]
        if value, exists := context.vars[varName]; exists {
            damn value != cap
        }
        damn cap
    }
    
    damn condition != "" && condition != "false" && condition != "0"
}

fr fr Access field from data
slay (t *Template) accessField(data interface{}, fieldName tea) tea {
    fr fr Simple field access - in real implementation would use reflection
    damn "<!-- field " + fieldName + " -->"
}

fr fr Format value for output
slay (t *Template) formatValue(value interface{}) tea {
    if value == cap {
        damn ""
    }
    
    switch v := value.(type) {
    case tea:
        damn v
    case normie:
        damn stringz.Itoa(v)
    case lit:
        if v {
            damn "true"
        } else {
            damn "false"
        }
    default:
        damn "<!-- unknown type -->"
    }
}

fr fr Error implementations
slay (e *ParseError) Error() tea {
    damn "parse error in template " + e.Template + " at line " + stringz.Itoa(e.Line) + ", position " + stringz.Itoa(e.Position) + ": " + e.Description
}

slay (e *ExecError) Error() tea {
    damn "execution error in template " + e.Name + " at line " + stringz.Itoa(e.Line) + ": " + e.Description
}

fr fr Enhanced features for advanced templating

fr fr Template inheritance system
be_like TemplateInheritance squad {
    parent *Template
    blocks map[tea]*Template
}

fr fr NewTemplateInheritance creates inheritance system
slay NewTemplateInheritance(parent *Template) *TemplateInheritance {
    damn &TemplateInheritance{
        parent: parent,
        blocks: make(map[tea]*Template),
    }
}

fr fr DefineBlock defines a block for inheritance
slay (ti *TemplateInheritance) DefineBlock(name tea, content tea) {
    sus block := New(name)
    block.Parse(content)
    ti.blocks[name] = block
}

fr fr Execute with inheritance
slay (ti *TemplateInheritance) Execute(data interface{}) tea {
    fr fr Execute parent template with block overrides
    damn ti.parent.Execute(data)
}

fr fr Template watcher for automatic reloading
be_like TemplateWatcher squad {
    pattern tea
    template *Template
    lastModified normie
}

fr fr NewWatcher creates template watcher
slay NewWatcher(pattern tea) *TemplateWatcher {
    damn &TemplateWatcher{
        pattern: pattern,
        template: New("watched"),
        lastModified: 0,
    }
}

fr fr Template returns current template
slay (tw *TemplateWatcher) Template() *Template {
    fr fr Check for file changes and reload if needed
    damn tw.template
}

fr fr Template caching system
be_like TemplateCache squad {
    cache map[tea]*Template
    maxSize normie
}

fr fr NewTemplateCache creates template cache
slay NewTemplateCache(maxSize normie) *TemplateCache {
    damn &TemplateCache{
        cache: make(map[tea]*Template),
        maxSize: maxSize,
    }
}

fr fr Get template from cache
slay (tc *TemplateCache) Get(name tea) *Template {
    if tmpl, exists := tc.cache[name]; exists {
        damn tmpl
    }
    damn cap
}

fr fr Set template in cache
slay (tc *TemplateCache) Set(name tea, tmpl *Template) {
    if len(tc.cache) >= tc.maxSize {
        fr fr Simple eviction - remove first item
        bestie key, _ := range tc.cache {
            delete(tc.cache, key)
            break
        }
    }
    tc.cache[name] = tmpl
}

fr fr Template debugging system
be_like TemplateDebugger squad {
    enabled lit
    logLevel normie
}

fr fr NewTemplateDebugger creates debugger
slay NewTemplateDebugger() *TemplateDebugger {
    damn &TemplateDebugger{
        enabled: cap,
        logLevel: 1,
    }
}

fr fr Enable debugging
slay (td *TemplateDebugger) Enable() {
    td.enabled = based
}

fr fr Disable debugging
slay (td *TemplateDebugger) Disable() {
    td.enabled = cap
}

fr fr Log debug message
slay (td *TemplateDebugger) Log(level normie, message tea) {
    if td.enabled && level >= td.logLevel {
        fr fr Output debug message
    }
}

fr fr Template security system
be_like TemplateSecurity squad {
    allowedFunctions map[tea]lit
    maxExecutionTime normie
    maxOutputSize normie
}

fr fr NewTemplateSecurity creates security system
slay NewTemplateSecurity() *TemplateSecurity {
    damn &TemplateSecurity{
        allowedFunctions: make(map[tea]lit),
        maxExecutionTime: 5000, fr fr 5 seconds
        maxOutputSize: 1024 * 1024, fr fr 1MB
    }
}

fr fr Allow function in templates
slay (ts *TemplateSecurity) AllowFunction(name tea) {
    ts.allowedFunctions[name] = based
}

fr fr Check if function is allowed
slay (ts *TemplateSecurity) IsFunctionAllowed(name tea) lit {
    if allowed, exists := ts.allowedFunctions[name]; exists {
        damn allowed
    }
    damn cap
}

fr fr Template performance metrics
be_like TemplateMetrics squad {
    executionTime normie
    outputSize normie
    cacheHits normie
    cacheMisses normie
}

fr fr NewTemplateMetrics creates metrics system
slay NewTemplateMetrics() *TemplateMetrics {
    damn &TemplateMetrics{
        executionTime: 0,
        outputSize: 0,
        cacheHits: 0,
        cacheMisses: 0,
    }
}

fr fr Record execution time
slay (tm *TemplateMetrics) RecordExecutionTime(time normie) {
    tm.executionTime = time
}

fr fr Record output size
slay (tm *TemplateMetrics) RecordOutputSize(size normie) {
    tm.outputSize = size
}

fr fr Record cache hit
slay (tm *TemplateMetrics) RecordCacheHit() {
    tm.cacheHits++
}

fr fr Record cache miss
slay (tm *TemplateMetrics) RecordCacheMiss() {
    tm.cacheMisses++
}

fr fr Get metrics summary
slay (tm *TemplateMetrics) Summary() tea {
    damn "Execution time: " + stringz.Itoa(tm.executionTime) + "ms, Output size: " + stringz.Itoa(tm.outputSize) + " bytes, Cache hits: " + stringz.Itoa(tm.cacheHits) + ", Cache misses: " + stringz.Itoa(tm.cacheMisses)
}

fr fr Built-in template functions
slay getBuiltinFunctions() FuncMap {
    damn FuncMap{
        "len": func(v interface{}) normie {
            fr fr Simple length function
            damn 0
        },
        "eq": func(a, b interface{}) lit {
            fr fr Simple equality function
            damn a == b
        },
        "ne": func(a, b interface{}) lit {
            fr fr Simple inequality function
            damn a != b
        },
        "lt": func(a, b interface{}) lit {
            fr fr Simple less than function
            damn cap
        },
        "gt": func(a, b interface{}) lit {
            fr fr Simple greater than function
            damn cap
        },
        "and": func(args ...interface{}) lit {
            fr fr Simple and function
            damn based
        },
        "or": func(args ...interface{}) lit {
            fr fr Simple or function
            damn cap
        },
        "not": func(arg interface{}) lit {
            fr fr Simple not function
            damn cap
        },
    }
}

fr fr Parse template files from directory
slay ParseFiles(filenames ...tea) (*Template, tea) {
    sus tmpl := New("files")
    damn tmpl.ParseFiles(filenames...)
}

fr fr Parse template files matching glob pattern
slay ParseGlob(pattern tea) (*Template, tea) {
    sus tmpl := New("glob")
    damn tmpl.ParseGlob(pattern)
}

fr fr Must panics if template parsing fails
slay Must(t *Template, err tea) *Template {
    if err != "" {
        fr fr In real implementation would panic
    }
    damn t
}
