/**
 * Advanced Code Action Provider for CURSED
 * Provides refactoring, quick fixes, and source actions
 */

import * as vscode from 'vscode';

export class CursedCodeActionProvider implements vscode.CodeActionProvider {
    
    async provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];
        
        // Quick fixes for diagnostics
        for (const diagnostic of context.diagnostics) {
            const quickFixes = await this.getQuickFixes(document, diagnostic);
            actions.push(...quickFixes);
        }
        
        // Refactoring actions
        if (!range.isEmpty) {
            actions.push(...this.getRefactoringActions(document, range));
        }
        
        // Source actions
        actions.push(...this.getSourceActions(document, range));
        
        return actions;
    }
    
    private async getQuickFixes(document: vscode.TextDocument, diagnostic: vscode.Diagnostic): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];
        
        // Add missing import
        if (diagnostic.message.includes('undefined module') || diagnostic.message.includes('unknown identifier')) {
            const missingSymbol = this.extractSymbolFromDiagnostic(diagnostic.message);
            if (missingSymbol) {
                const addImportAction = new vscode.CodeAction(
                    `Add import for '${missingSymbol}'`,
                    vscode.CodeActionKind.QuickFix
                );
                addImportAction.edit = await this.createAddImportEdit(document, missingSymbol);
                addImportAction.diagnostics = [diagnostic];
                actions.push(addImportAction);
            }
        }
        
        // Fix type annotations
        if (diagnostic.message.includes('missing type annotation')) {
            const fixTypeAction = new vscode.CodeAction(
                'Add type annotation',
                vscode.CodeActionKind.QuickFix
            );
            fixTypeAction.edit = await this.createAddTypeAnnotationEdit(document, diagnostic.range);
            fixTypeAction.diagnostics = [diagnostic];
            actions.push(fixTypeAction);
        }
        
        // Fix error handling
        if (diagnostic.message.includes('unhandled error') || diagnostic.message.includes('missing error handling')) {
            const addErrorHandlingAction = new vscode.CodeAction(
                'Add error handling',
                vscode.CodeActionKind.QuickFix
            );
            addErrorHandlingAction.edit = await this.createAddErrorHandlingEdit(document, diagnostic.range);
            addErrorHandlingAction.diagnostics = [diagnostic];
            actions.push(addErrorHandlingAction);
        }
        
        // Security fixes
        if (diagnostic.source === 'cursed-security') {
            const securityFix = await this.getSecurityFix(document, diagnostic);
            if (securityFix) {
                actions.push(securityFix);
            }
        }
        
        // Performance fixes
        if (diagnostic.source === 'cursed-performance') {
            const performanceFix = await this.getPerformanceFix(document, diagnostic);
            if (performanceFix) {
                actions.push(performanceFix);
            }
        }
        
        return actions;
    }
    
    private getRefactoringActions(document: vscode.TextDocument, range: vscode.Range): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];
        
        // Extract function
        if (this.isValidForExtractFunction(document, range)) {
            const extractFunctionAction = new vscode.CodeAction(
                'Extract function',
                vscode.CodeActionKind.RefactorExtract
            );
            extractFunctionAction.command = {
                title: 'Extract function',
                command: 'cursed.extractFunction'
            };
            actions.push(extractFunctionAction);
        }
        
        // Extract variable
        if (this.isValidForExtractVariable(document, range)) {
            const extractVariableAction = new vscode.CodeAction(
                'Extract variable',
                vscode.CodeActionKind.RefactorExtract
            );
            extractVariableAction.command = {
                title: 'Extract variable',
                command: 'cursed.extractVariable'
            };
            actions.push(extractVariableAction);
        }
        
        // Inline variable
        if (this.isValidForInlineVariable(document, range)) {
            const inlineVariableAction = new vscode.CodeAction(
                'Inline variable',
                vscode.CodeActionKind.RefactorInline
            );
            inlineVariableAction.command = {
                title: 'Inline variable',
                command: 'cursed.inlineVariable'
            };
            actions.push(inlineVariableAction);
        }
        
        // Convert to pattern matching
        if (this.isValidForPatternMatching(document, range)) {
            const convertToPatternAction = new vscode.CodeAction(
                'Convert to pattern matching',
                vscode.CodeActionKind.RefactorRewrite
            );
            convertToPatternAction.edit = this.createConvertToPatternMatchingEdit(document, range);
            actions.push(convertToPatternAction);
        }
        
        return actions;
    }
    
    private getSourceActions(document: vscode.TextDocument, range: vscode.Range): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];
        
        // Organize imports
        const organizeImportsAction = new vscode.CodeAction(
            'Organize imports',
            vscode.CodeActionKind.SourceOrganizeImports
        );
        organizeImportsAction.command = {
            title: 'Organize imports',
            command: 'cursed.organizeImports'
        };
        actions.push(organizeImportsAction);
        
        // Generate tests
        const generateTestsAction = new vscode.CodeAction(
            'Generate tests',
            vscode.CodeActionKind.Source
        );
        generateTestsAction.command = {
            title: 'Generate tests',
            command: 'cursed.generateTests'
        };
        actions.push(generateTestsAction);
        
        // Generate constructor
        if (this.isInStructDeclaration(document, range)) {
            const generateConstructorAction = new vscode.CodeAction(
                'Generate constructor',
                vscode.CodeActionKind.Source
            );
            generateConstructorAction.command = {
                title: 'Generate constructor',
                command: 'cursed.generateConstructor'
            };
            actions.push(generateConstructorAction);
        }
        
        // Implement interface
        if (this.isValidForInterfaceImplementation(document, range)) {
            const implementInterfaceAction = new vscode.CodeAction(
                'Implement interface',
                vscode.CodeActionKind.Source
            );
            implementInterfaceAction.command = {
                title: 'Implement interface',
                command: 'cursed.generateInterface'
            };
            actions.push(implementInterfaceAction);
        }
        
        return actions;
    }
    
    private extractSymbolFromDiagnostic(message: string): string | null {
        // Extract symbol name from error message
        const match = message.match(/'([^']+)'/);
        return match ? match[1] : null;
    }
    
    private async createAddImportEdit(document: vscode.TextDocument, symbol: string): Promise<vscode.WorkspaceEdit> {
        const edit = new vscode.WorkspaceEdit();
        
        // Find appropriate import statement to add
        const moduleName = this.inferModuleForSymbol(symbol);
        if (moduleName) {
            const importStatement = `yeet "${moduleName}"\n`;
            const insertPosition = this.findImportInsertPosition(document);
            edit.insert(document.uri, insertPosition, importStatement);
        }
        
        return edit;
    }
    
    private async createAddTypeAnnotationEdit(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.WorkspaceEdit> {
        const edit = new vscode.WorkspaceEdit();
        
        // Analyze the context to suggest appropriate type
        const text = document.getText(range);
        const inferredType = this.inferTypeFromContext(document, range);
        
        if (inferredType) {
            edit.replace(document.uri, range, `${text} ${inferredType}`);
        }
        
        return edit;
    }
    
    private async createAddErrorHandlingEdit(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.WorkspaceEdit> {
        const edit = new vscode.WorkspaceEdit();
        
        // Wrap the problematic code with error handling
        const text = document.getText(range);
        const errorHandlingCode = `${text} fam {\n    when _ -> {\n        // Handle error\n        vibez.spill("Error occurred")\n        damn cringe\n    }\n}`;
        
        edit.replace(document.uri, range, errorHandlingCode);
        
        return edit;
    }
    
    private async getSecurityFix(document: vscode.TextDocument, diagnostic: vscode.Diagnostic): Promise<vscode.CodeAction | null> {
        const message = diagnostic.message;
        
        if (message.includes('SQL injection')) {
            const action = new vscode.CodeAction(
                'Use parameterized query',
                vscode.CodeActionKind.QuickFix
            );
            action.edit = this.createSqlInjectionFix(document, diagnostic.range);
            action.diagnostics = [diagnostic];
            return action;
        }
        
        if (message.includes('weak cryptography')) {
            const action = new vscode.CodeAction(
                'Use stronger cryptographic hash',
                vscode.CodeActionKind.QuickFix
            );
            action.edit = this.createWeakCryptoFix(document, diagnostic.range);
            action.diagnostics = [diagnostic];
            return action;
        }
        
        return null;
    }
    
    private async getPerformanceFix(document: vscode.TextDocument, diagnostic: vscode.Diagnostic): Promise<vscode.CodeAction | null> {
        const message = diagnostic.message;
        
        if (message.includes('nested loops')) {
            const action = new vscode.CodeAction(
                'Optimize algorithm complexity',
                vscode.CodeActionKind.QuickFix
            );
            action.edit = this.createAlgorithmOptimizationFix(document, diagnostic.range);
            action.diagnostics = [diagnostic];
            return action;
        }
        
        if (message.includes('string concatenation')) {
            const action = new vscode.CodeAction(
                'Use StringBuilder pattern',
                vscode.CodeActionKind.QuickFix
            );
            action.edit = this.createStringBuilderFix(document, diagnostic.range);
            action.diagnostics = [diagnostic];
            return action;
        }
        
        return null;
    }
    
    private createSqlInjectionFix(document: vscode.TextDocument, range: vscode.Range): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        const text = document.getText(range);
        
        // Replace string concatenation with parameterized query
        const fixedCode = text.replace(/sql\s*\+\s*.*/, 'sql_prepare(query, params)');
        edit.replace(document.uri, range, fixedCode);
        
        return edit;
    }
    
    private createWeakCryptoFix(document: vscode.TextDocument, range: vscode.Range): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        const text = document.getText(range);
        
        // Replace weak hash functions with stronger alternatives
        const fixedCode = text
            .replace(/md5\(/g, 'sha256(')
            .replace(/sha1\(/g, 'sha256(');
        
        edit.replace(document.uri, range, fixedCode);
        
        return edit;
    }
    
    private createAlgorithmOptimizationFix(document: vscode.TextDocument, range: vscode.Range): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        
        // Add comment suggesting algorithm optimization
        const comment = '// TODO: Consider using a more efficient algorithm (e.g., HashMap for O(1) lookups)\n';
        edit.insert(document.uri, range.start, comment);
        
        return edit;
    }
    
    private createStringBuilderFix(document: vscode.TextDocument, range: vscode.Range): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        
        // Add comment suggesting StringBuilder pattern
        const comment = '// TODO: Use StringBuilder pattern for efficient string concatenation\n';
        edit.insert(document.uri, range.start, comment);
        
        return edit;
    }
    
    private createConvertToPatternMatchingEdit(document: vscode.TextDocument, range: vscode.Range): vscode.WorkspaceEdit {
        const edit = new vscode.WorkspaceEdit();
        const text = document.getText(range);
        
        // Convert if-else chain to pattern matching
        if (text.includes('ready') && text.includes('otherwise')) {
            const patternCode = this.convertIfElseToPatternMatching(text);
            edit.replace(document.uri, range, patternCode);
        }
        
        return edit;
    }
    
    private convertIfElseToPatternMatching(code: string): string {
        // Simplified conversion - in practice, this would need more sophisticated parsing
        return `sick (value) {\n    when pattern1 -> result1\n    when pattern2 -> result2\n    when _ -> default_result\n}`;
    }
    
    private isValidForExtractFunction(document: vscode.TextDocument, range: vscode.Range): boolean {
        const text = document.getText(range);
        return text.trim().length > 0 && text.includes('\n');
    }
    
    private isValidForExtractVariable(document: vscode.TextDocument, range: vscode.Range): boolean {
        const text = document.getText(range);
        return text.trim().length > 0 && !text.includes('\n') && !text.startsWith('sus ');
    }
    
    private isValidForInlineVariable(document: vscode.TextDocument, range: vscode.Range): boolean {
        const line = document.lineAt(range.start.line);
        return line.text.trim().startsWith('sus ') && line.text.includes(' = ');
    }
    
    private isValidForPatternMatching(document: vscode.TextDocument, range: vscode.Range): boolean {
        const text = document.getText(range);
        return text.includes('ready') && text.includes('otherwise');
    }
    
    private isInStructDeclaration(document: vscode.TextDocument, range: vscode.Range): boolean {
        // Check if cursor is inside a struct declaration
        for (let i = range.start.line; i >= 0; i--) {
            const line = document.lineAt(i);
            if (line.text.trim().startsWith('squad ')) {
                return true;
            }
            if (line.text.trim() === '}') {
                return false;
            }
        }
        return false;
    }
    
    private isValidForInterfaceImplementation(document: vscode.TextDocument, range: vscode.Range): boolean {
        // Check if there are unimplemented interface methods
        const text = document.getText();
        return text.includes('collab ') && text.includes('slay ');
    }
    
    private inferModuleForSymbol(symbol: string): string | null {
        // Simple heuristic to map symbols to modules
        const moduleMap: { [key: string]: string } = {
            'spill': 'vibez',
            'sqrt': 'mathz',
            'len': 'arrayz',
            'parse': 'stringz',
            'assert_eq': 'testz',
            'hash': 'cryptz',
            'read_file': 'filez',
            'get': 'httpz',
            'now': 'timez',
            'parse_json': 'jsonz',
            'go': 'concurrenz'
        };
        
        return moduleMap[symbol] || null;
    }
    
    private inferTypeFromContext(document: vscode.TextDocument, range: vscode.Range): string | null {
        const text = document.getText(range);
        
        // Simple type inference based on patterns
        if (text.includes('"')) return 'tea';
        if (text.includes('based') || text.includes('cringe')) return 'lit';
        if (/^\d+$/.test(text.trim())) return 'drip';
        if (text.includes('[') && text.includes(']')) return '[]';
        
        return null;
    }
    
    private findImportInsertPosition(document: vscode.TextDocument): vscode.Position {
        // Find the position to insert new import statements
        for (let i = 0; i < document.lineCount; i++) {
            const line = document.lineAt(i);
            if (!line.text.trim().startsWith('yeet ') && line.text.trim().length > 0) {
                return new vscode.Position(i, 0);
            }
        }
        return new vscode.Position(0, 0);
    }
}
