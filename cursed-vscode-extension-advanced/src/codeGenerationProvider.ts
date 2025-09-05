/**
 * Advanced Code Generation Provider for CURSED
 * Handles intelligent code generation features
 */

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class CursedCodeGenerationProvider implements vscode.Disposable {
    private disposables: vscode.Disposable[] = [];
    
    constructor(private client: LanguageClient) {
        this.registerCodeGenerationCommands();
        this.setupAutoGenerationFeatures();
    }
    
    dispose() {
        this.disposables.forEach(d => d.dispose());
    }
    
    private registerCodeGenerationCommands() {
        // Function generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.function', async () => {
                await this.generateFunction();
            })
        );
        
        // Constructor generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.constructor', async () => {
                await this.generateConstructor();
            })
        );
        
        // Interface implementation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.implementInterface', async () => {
                await this.implementInterface();
            })
        );
        
        // Test generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.tests', async () => {
                await this.generateTests();
            })
        );
        
        // Getter/setter generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.gettersSetters', async () => {
                await this.generateGettersSetters();
            })
        );
        
        // Error handling generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.errorHandling', async () => {
                await this.addErrorHandling();
            })
        );
        
        // Documentation generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.documentation', async () => {
                await this.generateDocumentation();
            })
        );
        
        // Boilerplate generation
        this.disposables.push(
            vscode.commands.registerCommand('cursed.generate.boilerplate', async () => {
                await this.generateBoilerplate();
            })
        );
    }
    
    private setupAutoGenerationFeatures() {
        // Auto-import on completion
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument(async (event) => {
                if (event.document.languageId === 'cursed') {
                    await this.handleAutoImport(event);
                }
            })
        );
        
        // Auto-completion of partial constructs
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument(async (event) => {
                if (event.document.languageId === 'cursed') {
                    await this.handleAutoCompletion(event);
                }
            })
        );
    }
    
    async generateFunction(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const functionSpec = await this.promptForFunctionSpecification();
        if (!functionSpec) {
            return;
        }
        
        const generatedFunction = this.createFunctionTemplate(functionSpec);
        
        // Insert at cursor position
        const position = editor.selection.active;
        const edit = new vscode.WorkspaceEdit();
        edit.insert(editor.document.uri, position, generatedFunction);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Generated function '${functionSpec.name}'`);
        }
    }
    
    async generateConstructor(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const structInfo = await this.findStructAtCursor(editor);
        if (!structInfo) {
            vscode.window.showErrorMessage('Place cursor inside a struct to generate constructor');
            return;
        }
        
        const constructorOptions = await this.promptForConstructorOptions(structInfo);
        if (!constructorOptions) {
            return;
        }
        
        const constructor = this.createConstructorTemplate(structInfo, constructorOptions);
        
        // Insert after struct definition
        const insertPosition = this.findConstructorInsertPosition(editor, structInfo);
        const edit = new vscode.WorkspaceEdit();
        edit.insert(editor.document.uri, insertPosition, constructor);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Generated constructor for '${structInfo.name}'`);
        }
    }
    
    async implementInterface(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const interfaces = await this.findAvailableInterfaces(editor.document);
        if (interfaces.length === 0) {
            vscode.window.showErrorMessage('No interfaces found in workspace');
            return;
        }
        
        const selectedInterface = await vscode.window.showQuickPick(interfaces, {
            placeHolder: 'Select interface to implement'
        });
        
        if (!selectedInterface) {
            return;
        }
        
        const targetStruct = await this.findStructAtCursor(editor);
        if (!targetStruct) {
            vscode.window.showErrorMessage('Place cursor inside a struct to implement interface');
            return;
        }
        
        const implementation = await this.createInterfaceImplementation(selectedInterface, targetStruct);
        
        // Insert implementation
        const insertPosition = this.findImplementationInsertPosition(editor, targetStruct);
        const edit = new vscode.WorkspaceEdit();
        edit.insert(editor.document.uri, insertPosition, implementation);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Implemented interface '${selectedInterface.name}' for '${targetStruct.name}'`);
        }
    }
    
    async generateTests(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const testOptions = await this.promptForTestOptions();
        if (!testOptions) {
            return;
        }
        
        const functions = await this.findTestableFunctions(editor.document);
        const testFile = await this.createTestFile(editor.document, functions, testOptions);
        
        // Create or open test file
        const testFilePath = this.getTestFilePath(editor.document.uri);
        const testFileUri = vscode.Uri.file(testFilePath);
        
        try {
            await vscode.workspace.fs.writeFile(testFileUri, Buffer.from(testFile));
            const testDocument = await vscode.workspace.openTextDocument(testFileUri);
            await vscode.window.showTextDocument(testDocument);
            
            vscode.window.showInformationMessage(`Generated tests in ${testFilePath}`);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to create test file: ${error}`);
        }
    }
    
    async generateGettersSetters(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const structInfo = await this.findStructAtCursor(editor);
        if (!structInfo) {
            vscode.window.showErrorMessage('Place cursor inside a struct to generate getters/setters');
            return;
        }
        
        const fields = await this.promptForFieldSelection(structInfo.fields);
        if (!fields || fields.length === 0) {
            return;
        }
        
        const gettersSetters = this.createGettersSetters(structInfo, fields);
        
        // Insert after struct definition
        const insertPosition = this.findMethodInsertPosition(editor, structInfo);
        const edit = new vscode.WorkspaceEdit();
        edit.insert(editor.document.uri, insertPosition, gettersSetters);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Generated getters/setters for ${fields.length} fields`);
        }
    }
    
    async addErrorHandling(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const selection = editor.selection;
        if (selection.isEmpty) {
            vscode.window.showErrorMessage('Please select code to add error handling to');
            return;
        }
        
        const errorHandlingType = await vscode.window.showQuickPick([
            { label: 'Try-Catch (fam)', description: 'Wrap with CURSED error handling', value: 'fam' },
            { label: 'Result Type', description: 'Convert to result type pattern', value: 'result' },
            { label: 'Option Type', description: 'Convert to optional pattern', value: 'option' },
            { label: 'Custom Handler', description: 'Add custom error handler', value: 'custom' }
        ], {
            placeHolder: 'Select error handling pattern'
        });
        
        if (!errorHandlingType) {
            return;
        }
        
        const originalCode = editor.document.getText(selection);
        const wrappedCode = this.wrapWithErrorHandling(originalCode, errorHandlingType.value);
        
        const edit = new vscode.WorkspaceEdit();
        edit.replace(editor.document.uri, selection, wrappedCode);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Added ${errorHandlingType.label} error handling`);
        }
    }
    
    async generateDocumentation(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const functionInfo = await this.findFunctionAtCursor(editor);
        if (!functionInfo) {
            vscode.window.showErrorMessage('Place cursor on a function to generate documentation');
            return;
        }
        
        const docStyle = await vscode.window.showQuickPick([
            { label: 'Standard', description: 'Standard CURSED documentation', value: 'standard' },
            { label: 'Detailed', description: 'Detailed with examples', value: 'detailed' },
            { label: 'API', description: 'API documentation style', value: 'api' }
        ], {
            placeHolder: 'Select documentation style'
        });
        
        if (!docStyle) {
            return;
        }
        
        const documentation = this.generateFunctionDocumentation(functionInfo, docStyle.value);
        
        // Insert before function
        const insertPosition = new vscode.Position(functionInfo.range.start.line, 0);
        const edit = new vscode.WorkspaceEdit();
        edit.insert(editor.document.uri, insertPosition, documentation);
        
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            vscode.window.showInformationMessage(`Generated ${docStyle.label} documentation`);
        }
    }
    
    async generateBoilerplate(): Promise<void> {
        const boilerplateType = await vscode.window.showQuickPick([
            { label: 'Web Server', description: 'HTTP server boilerplate', value: 'webserver' },
            { label: 'CLI Application', description: 'Command-line application', value: 'cli' },
            { label: 'Library', description: 'Library with public API', value: 'library' },
            { label: 'Test Suite', description: 'Complete test suite', value: 'testsuite' },
            { label: 'Database Module', description: 'Database operations', value: 'database' },
            { label: 'Concurrent Worker', description: 'Goroutine-based worker', value: 'worker' }
        ], {
            placeHolder: 'Select boilerplate type'
        });
        
        if (!boilerplateType) {
            return;
        }
        
        const projectName = await vscode.window.showInputBox({
            prompt: 'Enter project/module name',
            placeHolder: 'my_project'
        });
        
        if (!projectName) {
            return;
        }
        
        const boilerplate = this.createBoilerplate(boilerplateType.value, projectName);
        
        // Create new file or insert in current file
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'cursed') {
            const edit = new vscode.WorkspaceEdit();
            edit.insert(editor.document.uri, editor.selection.active, boilerplate);
            
            const success = await vscode.workspace.applyEdit(edit);
            if (success) {
                vscode.window.showInformationMessage(`Generated ${boilerplateType.label} boilerplate`);
            }
        } else {
            // Create new file
            const fileName = `${projectName}.💀`;
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (workspaceFolder) {
                const filePath = vscode.Uri.joinPath(workspaceFolder.uri, fileName);
                await vscode.workspace.fs.writeFile(filePath, Buffer.from(boilerplate));
                const document = await vscode.workspace.openTextDocument(filePath);
                await vscode.window.showTextDocument(document);
                
                vscode.window.showInformationMessage(`Created ${fileName} with ${boilerplateType.label} boilerplate`);
            }
        }
    }
    
    // Auto-generation features
    
    private async handleAutoImport(event: vscode.TextDocumentChangeEvent): Promise<void> {
        const config = vscode.workspace.getConfiguration('cursed');
        if (!config.get('codeGeneration.autoImport.enabled', true)) {
            return;
        }
        
        for (const change of event.contentChanges) {
            const text = change.text;
            
            // Check for function calls that might need imports
            const functionCallMatch = text.match(/([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (functionCallMatch) {
                const functionName = functionCallMatch[1];
                const suggestedImport = this.getSuggestedImport(functionName);
                
                if (suggestedImport) {
                    await this.showImportSuggestion(event.document, suggestedImport, functionName);
                }
            }
        }
    }
    
    private async handleAutoCompletion(event: vscode.TextDocumentChangeEvent): Promise<void> {
        for (const change of event.contentChanges) {
            const text = change.text;
            
            // Auto-complete struct initialization
            if (text.includes('{') && !text.includes('}')) {
                await this.autoCompleteStructInit(event.document, change);
            }
            
            // Auto-complete function signature
            if (text.startsWith('slay ') && text.includes('(') && !text.includes(')')) {
                await this.autoCompleteFunctionSignature(event.document, change);
            }
        }
    }
    
    // Template creation methods
    
    private createFunctionTemplate(spec: FunctionSpecification): string {
        const params = spec.parameters.map(p => `${p.name} ${p.type}`).join(', ');
        const returnType = spec.returnType || 'void';
        
        let template = `slay ${spec.name}(${params}) ${returnType} {\n`;
        
        if (spec.generateBody) {
            template += this.generateFunctionBody(spec);
        } else {
            template += '    // TODO: Implement function\n';
            if (returnType !== 'void') {
                template += `    damn ${this.getDefaultValue(returnType)}\n`;
            }
        }
        
        template += '}\n';
        
        return template;
    }
    
    private createConstructorTemplate(structInfo: StructInfo, options: ConstructorOptions): string {
        const params = structInfo.fields
            .filter(f => options.includeFields.includes(f.name))
            .map(f => `${f.name} ${f.type}`)
            .join(', ');
        
        let constructor = `\nslay new_${structInfo.name}(${params}) ${structInfo.name} {\n`;
        constructor += `    sus instance ${structInfo.name} = ${structInfo.name} {\n`;
        
        for (const field of structInfo.fields) {
            if (options.includeFields.includes(field.name)) {
                constructor += `        ${field.name}: ${field.name},\n`;
            } else {
                constructor += `        ${field.name}: ${this.getDefaultValue(field.type)},\n`;
            }
        }
        
        constructor += '    }\n';
        constructor += '    damn instance\n';
        constructor += '}\n';
        
        return constructor;
    }
    
    private async createInterfaceImplementation(interfaceInfo: InterfaceInfo, structInfo: StructInfo): Promise<string> {
        let implementation = `\n// Implementation of ${interfaceInfo.name} for ${structInfo.name}\n`;
        
        for (const method of interfaceInfo.methods) {
            implementation += `slay ${method.name}(self *${structInfo.name}`;
            if (method.parameters.length > 0) {
                implementation += ', ' + method.parameters.map(p => `${p.name} ${p.type}`).join(', ');
            }
            implementation += `) ${method.returnType} {\n`;
            implementation += '    // TODO: Implement method\n';
            if (method.returnType !== 'void') {
                implementation += `    damn ${this.getDefaultValue(method.returnType)}\n`;
            }
            implementation += '}\n\n';
        }
        
        return implementation;
    }
    
    private async createTestFile(document: vscode.TextDocument, functions: FunctionInfo[], options: TestOptions): Promise<string> {
        const fileName = document.fileName.split('/').pop()?.replace('.💀', '') || 'module';
        
        let testFile = `// Tests for ${fileName}\n`;
        testFile += 'yeet "testz"\n';
        testFile += `yeet "${fileName}"\n\n`;
        
        for (const func of functions) {
            testFile += this.createTestFunction(func, options);
        }
        
        // Add test runner
        testFile += 'slay main() void {\n';
        testFile += '    testz.run_tests()\n';
        testFile += '}\n';
        
        return testFile;
    }
    
    private createTestFunction(func: FunctionInfo, options: TestOptions): string {
        let test = `slay test_${func.name}() lit {\n`;
        test += '    // Arrange\n';
        
        // Generate test data
        for (const param of func.parameters) {
            test += `    sus ${param.name} ${param.type} = ${this.generateTestValue(param.type)}\n`;
        }
        
        test += '\n    // Act\n';
        const paramNames = func.parameters.map(p => p.name).join(', ');
        
        if (func.returnType !== 'void') {
            test += `    sus result ${func.returnType} = ${func.name}(${paramNames})\n`;
            test += '\n    // Assert\n';
            test += `    testz.assert_eq(result, ${this.generateExpectedValue(func.returnType)})\n`;
        } else {
            test += `    ${func.name}(${paramNames})\n`;
            test += '\n    // Assert\n';
            test += '    // TODO: Add assertions\n';
        }
        
        test += '\n    damn based\n';
        test += '}\n\n';
        
        return test;
    }
    
    private createGettersSetters(structInfo: StructInfo, fields: FieldInfo[]): string {
        let code = '\n// Getters and Setters\n';
        
        for (const field of fields) {
            // Getter
            code += `slay get_${field.name}(self *${structInfo.name}) ${field.type} {\n`;
            code += `    damn self.${field.name}\n`;
            code += '}\n\n';
            
            // Setter
            code += `slay set_${field.name}(self *${structInfo.name}, value ${field.type}) void {\n`;
            code += `    self.${field.name} = value\n`;
            code += '}\n\n';
        }
        
        return code;
    }
    
    private wrapWithErrorHandling(code: string, type: string): string {
        switch (type) {
            case 'fam':
                return `${code} fam {\n    when _ -> {\n        // Handle error\n        vibez.spill("Error occurred")\n        damn cringe\n    }\n}`;
            
            case 'result':
                return `// TODO: Convert to Result<T, E> pattern\n${code}`;
            
            case 'option':
                return `// TODO: Convert to Option<T> pattern\n${code}`;
            
            case 'custom':
                return `ready (error_condition) {\n    // Handle error\n    vibez.spill("Custom error handling")\n    damn cringe\n}\n${code}`;
            
            default:
                return code;
        }
    }
    
    private generateFunctionDocumentation(func: FunctionInfo, style: string): string {
        let doc = '';
        
        switch (style) {
            case 'standard':
                doc += `// ${func.name} - Brief description\n`;
                break;
            
            case 'detailed':
                doc += `/**\n`;
                doc += ` * ${func.name} - Detailed description of what this function does\n`;
                doc += ` *\n`;
                for (const param of func.parameters) {
                    doc += ` * @param ${param.name} - Description of ${param.name}\n`;
                }
                if (func.returnType !== 'void') {
                    doc += ` * @returns - Description of return value\n`;
                }
                doc += ` */\n`;
                break;
            
            case 'api':
                doc += `/**\n`;
                doc += ` * API Function: ${func.name}\n`;
                doc += ` * \n`;
                doc += ` * Description: Brief description of the API function\n`;
                doc += ` * \n`;
                doc += ` * Parameters:\n`;
                for (const param of func.parameters) {
                    doc += ` *   - ${param.name} (${param.type}): Description\n`;
                }
                doc += ` * \n`;
                doc += ` * Returns: ${func.returnType}\n`;
                doc += ` * \n`;
                doc += ` * Example:\n`;
                doc += ` *   ${func.name}(${func.parameters.map(p => this.generateExampleValue(p.type)).join(', ')})\n`;
                doc += ` */\n`;
                break;
        }
        
        return doc;
    }
    
    private createBoilerplate(type: string, name: string): string {
        switch (type) {
            case 'webserver':
                return this.createWebServerBoilerplate(name);
            case 'cli':
                return this.createCliBoilerplate(name);
            case 'library':
                return this.createLibraryBoilerplate(name);
            case 'testsuite':
                return this.createTestSuiteBoilerplate(name);
            case 'database':
                return this.createDatabaseBoilerplate(name);
            case 'worker':
                return this.createWorkerBoilerplate(name);
            default:
                return `// ${type} boilerplate for ${name}\n`;
        }
    }
    
    private createWebServerBoilerplate(name: string): string {
        return `// Web Server: ${name}
yeet "httpz"
yeet "vibez"

squad ${name}Server {
    port drip
    host tea
}

slay new_${name}_server(port drip, host tea) ${name}Server {
    damn ${name}Server {
        port: port,
        host: host
    }
}

slay start(self *${name}Server) void {
    vibez.spill("Starting server on", self.host, ":", self.port)
    
    httpz.route("/", handle_index)
    httpz.route("/api/health", handle_health)
    
    httpz.listen(self.host, self.port)
}

slay handle_index(req httpz.Request) httpz.Response {
    damn httpz.response(200, "Welcome to ${name} Server!")
}

slay handle_health(req httpz.Request) httpz.Response {
    damn httpz.json_response(200, {
        "status": "healthy",
        "service": "${name}"
    })
}

slay main() void {
    sus server ${name}Server = new_${name}_server(3000, "localhost")
    server.start()
}
`;
    }
    
    private createCliBoilerplate(name: string): string {
        return `// CLI Application: ${name}
yeet "vibez"
yeet "argz"

squad ${name}Config {
    verbose lit
    input_file tea
    output_file tea
}

slay parse_args() ${name}Config {
    sus config ${name}Config = ${name}Config {
        verbose: cringe,
        input_file: "",
        output_file: ""
    }
    
    sus args []tea = argz.get_args()
    
    bestie (argz.has_flag("--verbose", args)) {
        config.verbose = based
    }
    
    config.input_file = argz.get_value("--input", args)
    config.output_file = argz.get_value("--output", args)
    
    damn config
}

slay run(config ${name}Config) void {
    ready (config.verbose) {
        vibez.spill("Running ${name} with config:", config)
    }
    
    // Main application logic here
    vibez.spill("Processing", config.input_file)
    
    ready (config.output_file != "") {
        vibez.spill("Output will be written to", config.output_file)
    }
}

slay main() void {
    sus config ${name}Config = parse_args()
    run(config)
}
`;
    }
    
    private createLibraryBoilerplate(name: string): string {
        return `// Library: ${name}
yeet "vibez"

// Public API types
squad ${name}Config {
    setting1 tea
    setting2 drip
}

// Main library struct
squad ${name} {
    config ${name}Config
    initialized lit
}

// Constructor
slay new_${name}(config ${name}Config) ${name} {
    damn ${name} {
        config: config,
        initialized: based
    }
}

// Public API methods
slay process(self *${name}, input tea) tea {
    ready (!self.initialized) {
        vibez.spill("Error: ${name} not initialized")
        damn ""
    }
    
    // Processing logic here
    damn input + "_processed"
}

slay configure(self *${name}, config ${name}Config) void {
    self.config = config
}

// Utility functions
slay validate_input(input tea) lit {
    damn input != ""
}

// Default configuration
slay default_config() ${name}Config {
    damn ${name}Config {
        setting1: "default",
        setting2: 42
    }
}
`;
    }
    
    private createTestSuiteBoilerplate(name: string): string {
        return `// Test Suite: ${name}
yeet "testz"

// Test setup and teardown
slay setup() void {
    vibez.spill("Setting up tests for ${name}")
}

slay teardown() void {
    vibez.spill("Cleaning up after tests")
}

// Example test cases
slay test_basic_functionality() lit {
    setup()
    
    // Arrange
    sus input tea = "test_input"
    sus expected tea = "expected_output"
    
    // Act
    sus result tea = process(input)
    
    // Assert
    testz.assert_eq(result, expected)
    
    teardown()
    damn based
}

slay test_edge_cases() lit {
    setup()
    
    // Test empty input
    testz.assert_eq(process(""), "")
    
    // Test null input
    testz.assert_throws(() -> { process(null) })
    
    teardown()
    damn based
}

slay test_performance() lit {
    setup()
    
    sus start_time drip = time.now()
    
    // Run performance test
    bestie (i := 0; i < 1000; i++) {
        process("performance_test_input")
    }
    
    sus duration drip = time.now() - start_time
    testz.assert_lt(duration, 1000) // Should complete in under 1 second
    
    teardown()
    damn based
}

// Test runner
slay main() void {
    vibez.spill("Running ${name} test suite")
    
    testz.run("Basic Functionality", test_basic_functionality)
    testz.run("Edge Cases", test_edge_cases)
    testz.run("Performance", test_performance)
    
    testz.summary()
}
`;
    }
    
    private createDatabaseBoilerplate(name: string): string {
        return `// Database Module: ${name}
yeet "dbz"
yeet "vibez"

squad ${name}DB {
    connection dbz.Connection
    config DBConfig
}

squad DBConfig {
    host tea
    port drip
    database tea
    username tea
    password tea
}

slay new_${name}_db(config DBConfig) ${name}DB {
    sus conn dbz.Connection = dbz.connect(config.host, config.port, config.database, config.username, config.password)
    
    damn ${name}DB {
        connection: conn,
        config: config
    }
}

slay create_tables(self *${name}DB) lit {
    sus sql tea = """
    CREATE TABLE IF NOT EXISTS ${name}_data (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    """
    
    damn self.connection.execute(sql) fam {
        when _ -> {
            vibez.spill("Failed to create tables")
            damn cringe
        }
    }
}

slay insert(self *${name}DB, name tea) drip {
    sus sql tea = "INSERT INTO ${name}_data (name) VALUES (?)"
    
    damn self.connection.execute_with_params(sql, [name]) fam {
        when _ -> {
            vibez.spill("Failed to insert data")
            damn -1
        }
    }
}

slay find_by_id(self *${name}DB, id drip) ?${name}Record {
    sus sql tea = "SELECT id, name, created_at FROM ${name}_data WHERE id = ?"
    
    sus result []${name}Record = self.connection.query(sql, [id]) fam {
        when _ -> damn null
    }
    
    ready (result.len() > 0) {
        damn result[0]
    }
    
    damn null
}

squad ${name}Record {
    id drip
    name tea
    created_at tea
}

slay close(self *${name}DB) void {
    self.connection.close()
}
`;
    }
    
    private createWorkerBoilerplate(name: string): string {
        return `// Concurrent Worker: ${name}
yeet "concurrenz"
yeet "vibez"

squad ${name}Worker {
    id drip
    tasks chan<Task>
    results chan<Result>
    done chan<lit>
}

squad Task {
    id drip
    data tea
}

squad Result {
    task_id drip
    output tea
    error tea
}

slay new_${name}_worker(id drip, tasks chan<Task>, results chan<Result>, done chan<lit>) ${name}Worker {
    damn ${name}Worker {
        id: id,
        tasks: tasks,
        results: results,
        done: done
    }
}

slay start(self *${name}Worker) void {
    vibez.spill("Worker", self.id, "starting")
    
    bestie (based) {
        sick (<-self.tasks) {
            when task Task -> {
                sus result Result = self.process_task(task)
                self.results <- result
            }
            when _ -> {
                vibez.spill("Worker", self.id, "shutting down")
                self.done <- based
                damn
            }
        }
    }
}

slay process_task(self *${name}Worker, task Task) Result {
    vibez.spill("Worker", self.id, "processing task", task.id)
    
    // Simulate work
    concurrenz.sleep(100)
    
    damn Result {
        task_id: task.id,
        output: "Processed: " + task.data,
        error: ""
    }
}

// Manager for coordinating workers
squad ${name}Manager {
    workers []${name}Worker
    tasks chan<Task>
    results chan<Result>
    done chan<lit>
}

slay new_${name}_manager(worker_count drip) ${name}Manager {
    sus tasks chan<Task> = concurrenz.make_channel()
    sus results chan<Result> = concurrenz.make_channel()
    sus done chan<lit> = concurrenz.make_channel()
    
    sus workers []${name}Worker = []
    
    bestie (i := 0; i < worker_count; i++) {
        sus worker ${name}Worker = new_${name}_worker(i, tasks, results, done)
        workers.append(worker)
        
        go {
            worker.start()
        }
    }
    
    damn ${name}Manager {
        workers: workers,
        tasks: tasks,
        results: results,
        done: done
    }
}

slay add_task(self *${name}Manager, task Task) void {
    self.tasks <- task
}

slay get_result(self *${name}Manager) Result {
    damn <-self.results
}

slay shutdown(self *${name}Manager) void {
    vibez.spill("Shutting down worker manager")
    
    // Close tasks channel
    concurrenz.close(self.tasks)
    
    // Wait for all workers to finish
    bestie (i := 0; i < self.workers.len(); i++) {
        <-self.done
    }
}

slay main() void {
    sus manager ${name}Manager = new_${name}_manager(4)
    
    // Add some tasks
    bestie (i := 0; i < 10; i++) {
        manager.add_task(Task {
            id: i,
            data: "Task " + i
        })
    }
    
    // Collect results
    bestie (i := 0; i < 10; i++) {
        sus result Result = manager.get_result()
        vibez.spill("Result:", result)
    }
    
    manager.shutdown()
}
`;
    }
    
    // Helper methods for prompting and analysis
    
    private async promptForFunctionSpecification(): Promise<FunctionSpecification | null> {
        const name = await vscode.window.showInputBox({
            prompt: 'Enter function name',
            placeHolder: 'my_function'
        });
        
        if (!name) return null;
        
        const parameterCount = await vscode.window.showInputBox({
            prompt: 'Number of parameters (0 for none)',
            placeHolder: '0'
        });
        
        const paramCount = parseInt(parameterCount || '0');
        const parameters: Parameter[] = [];
        
        for (let i = 0; i < paramCount; i++) {
            const paramName = await vscode.window.showInputBox({
                prompt: `Parameter ${i + 1} name`,
                placeHolder: `param${i + 1}`
            });
            
            if (!paramName) return null;
            
            const paramType = await vscode.window.showInputBox({
                prompt: `Parameter ${i + 1} type`,
                placeHolder: 'drip'
            });
            
            parameters.push({
                name: paramName,
                type: paramType || 'drip'
            });
        }
        
        const returnType = await vscode.window.showInputBox({
            prompt: 'Return type (void for no return)',
            placeHolder: 'void'
        });
        
        const generateBody = await vscode.window.showQuickPick([
            { label: 'Yes', picked: false },
            { label: 'No', picked: true }
        ], {
            placeHolder: 'Generate function body?'
        });
        
        return {
            name,
            parameters,
            returnType: returnType || 'void',
            generateBody: generateBody?.label === 'Yes'
        };
    }
    
    // Additional helper methods would go here...
    // (continuing with similar patterns for other prompt and analysis methods)
    
    private getDefaultValue(type: string): string {
        switch (type) {
            case 'drip': return '0';
            case 'tea': return '""';
            case 'lit': return 'cringe';
            default: return 'null';
        }
    }
    
    private generateTestValue(type: string): string {
        switch (type) {
            case 'drip': return '42';
            case 'tea': return '"test_value"';
            case 'lit': return 'based';
            default: return 'null';
        }
    }
    
    private generateExpectedValue(type: string): string {
        switch (type) {
            case 'drip': return '84';
            case 'tea': return '"expected"';
            case 'lit': return 'based';
            default: return 'null';
        }
    }
    
    private generateExampleValue(type: string): string {
        switch (type) {
            case 'drip': return '123';
            case 'tea': return '"example"';
            case 'lit': return 'based';
            default: return 'value';
        }
    }
    
    // Placeholder implementations for analysis methods
    private async findStructAtCursor(editor: vscode.TextEditor): Promise<StructInfo | null> {
        // Implementation would analyze AST to find struct at cursor
        return null;
    }
    
    private async findAvailableInterfaces(document: vscode.TextDocument): Promise<InterfaceInfo[]> {
        // Implementation would scan workspace for interfaces
        return [];
    }
    
    private async findTestableFunction(document: vscode.TextDocument): Promise<FunctionInfo[]> {
        // Implementation would extract functions that can be tested
        return [];
    }
}

// Type definitions for code generation

interface FunctionSpecification {
    name: string;
    parameters: Parameter[];
    returnType: string;
    generateBody: boolean;
}

interface Parameter {
    name: string;
    type: string;
}

interface StructInfo {
    name: string;
    fields: FieldInfo[];
    range: vscode.Range;
}

interface FieldInfo {
    name: string;
    type: string;
    range: vscode.Range;
}

interface InterfaceInfo {
    name: string;
    methods: MethodInfo[];
}

interface MethodInfo {
    name: string;
    parameters: Parameter[];
    returnType: string;
}

interface FunctionInfo {
    name: string;
    parameters: Parameter[];
    returnType: string;
    range: vscode.Range;
}

interface ConstructorOptions {
    includeFields: string[];
    generateBuilder: boolean;
}

interface TestOptions {
    includePrivateFunctions: boolean;
    generateMocks: boolean;
    style: 'unit' | 'integration' | 'both';
}
