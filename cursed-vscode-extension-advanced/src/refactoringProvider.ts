/**
 * Advanced Refactoring Provider for CURSED
 * Handles complex refactoring operations
 */

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class CursedRefactoringProvider implements vscode.Disposable {
    private disposables: vscode.Disposable[] = [];
    
    constructor(private client: LanguageClient) {
        this.registerRefactoringCommands();
    }
    
    dispose() {
        this.disposables.forEach(d => d.dispose());
    }
    
    private registerRefactoringCommands() {
        // Register refactoring-specific commands
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.extractFunction', async (uri?: vscode.Uri, range?: vscode.Range) => {
                await this.extractFunction(uri, range);
            })
        );
        
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.extractVariable', async (uri?: vscode.Uri, range?: vscode.Range) => {
                await this.extractVariable(uri, range);
            })
        );
        
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.inlineVariable', async (uri?: vscode.Uri, position?: vscode.Position) => {
                await this.inlineVariable(uri, position);
            })
        );
        
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.moveSymbol', async (uri?: vscode.Uri, position?: vscode.Position) => {
                await this.moveSymbol(uri, position);
            })
        );
        
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.changeSignature', async (uri?: vscode.Uri, position?: vscode.Position) => {
                await this.changeSignature(uri, position);
            })
        );
        
        this.disposables.push(
            vscode.commands.registerCommand('cursed.refactor.convertToPatternMatching', async (uri?: vscode.Uri, range?: vscode.Range) => {
                await this.convertToPatternMatching(uri, range);
            })
        );
    }
    
    async extractFunction(uri?: vscode.Uri, range?: vscode.Range): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const selection = range || editor.selection;
        
        if (selection.isEmpty) {
            vscode.window.showErrorMessage('Please select code to extract into a function');
            return;
        }
        
        // Get function name from user
        const functionName = await vscode.window.showInputBox({
            prompt: 'Enter function name',
            placeHolder: 'extracted_function',
            validateInput: (value) => {
                if (!value || value.trim().length === 0) {
                    return 'Function name cannot be empty';
                }
                if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(value)) {
                    return 'Invalid function name';
                }
                return null;
            }
        });
        
        if (!functionName) {
            return;
        }
        
        try {
            // Analyze the selected code for parameters and return type
            const analysis = await this.analyzeCodeForExtraction(document, selection);
            
            // Generate function signature
            const signature = await this.generateFunctionSignature(functionName, analysis);
            
            // Perform the refactoring
            const edit = await this.performExtractFunction(document, selection, functionName, signature, analysis);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage(`Successfully extracted function '${functionName}'`);
                } else {
                    vscode.window.showErrorMessage('Failed to apply extract function refactoring');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Extract function failed: ${error}`);
        }
    }
    
    async extractVariable(uri?: vscode.Uri, range?: vscode.Range): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const selection = range || editor.selection;
        
        if (selection.isEmpty) {
            vscode.window.showErrorMessage('Please select an expression to extract into a variable');
            return;
        }
        
        const variableName = await vscode.window.showInputBox({
            prompt: 'Enter variable name',
            placeHolder: 'extracted_var',
            validateInput: (value) => {
                if (!value || value.trim().length === 0) {
                    return 'Variable name cannot be empty';
                }
                if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(value)) {
                    return 'Invalid variable name';
                }
                return null;
            }
        });
        
        if (!variableName) {
            return;
        }
        
        try {
            const analysis = await this.analyzeExpressionForExtraction(document, selection);
            const edit = await this.performExtractVariable(document, selection, variableName, analysis);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage(`Successfully extracted variable '${variableName}'`);
                } else {
                    vscode.window.showErrorMessage('Failed to apply extract variable refactoring');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Extract variable failed: ${error}`);
        }
    }
    
    async inlineVariable(uri?: vscode.Uri, position?: vscode.Position): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const pos = position || editor.selection.active;
        
        try {
            const variableInfo = await this.findVariableAtPosition(document, pos);
            if (!variableInfo) {
                vscode.window.showErrorMessage('No variable found at cursor position');
                return;
            }
            
            const references = await this.findVariableReferences(document, variableInfo);
            if (references.length === 0) {
                vscode.window.showErrorMessage('Variable has no references to inline');
                return;
            }
            
            const edit = await this.performInlineVariable(document, variableInfo, references);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage(`Successfully inlined variable '${variableInfo.name}'`);
                } else {
                    vscode.window.showErrorMessage('Failed to apply inline variable refactoring');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Inline variable failed: ${error}`);
        }
    }
    
    async moveSymbol(uri?: vscode.Uri, position?: vscode.Position): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const pos = position || editor.selection.active;
        
        try {
            const symbolInfo = await this.findSymbolAtPosition(document, pos);
            if (!symbolInfo) {
                vscode.window.showErrorMessage('No symbol found at cursor position');
                return;
            }
            
            // Show file picker for destination
            const workspaceFiles = await vscode.workspace.findFiles('**/*.csd');
            const fileItems = workspaceFiles.map(file => ({
                label: vscode.workspace.asRelativePath(file),
                description: file.fsPath,
                file: file
            }));
            
            const selectedFile = await vscode.window.showQuickPick(fileItems, {
                placeHolder: 'Select destination file'
            });
            
            if (!selectedFile) {
                return;
            }
            
            const edit = await this.performMoveSymbol(document, symbolInfo, selectedFile.file);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage(`Successfully moved symbol '${symbolInfo.name}'`);
                } else {
                    vscode.window.showErrorMessage('Failed to apply move symbol refactoring');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Move symbol failed: ${error}`);
        }
    }
    
    async changeSignature(uri?: vscode.Uri, position?: vscode.Position): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const pos = position || editor.selection.active;
        
        try {
            const functionInfo = await this.findFunctionAtPosition(document, pos);
            if (!functionInfo) {
                vscode.window.showErrorMessage('No function found at cursor position');
                return;
            }
            
            const newSignature = await this.promptForNewSignature(functionInfo);
            if (!newSignature) {
                return;
            }
            
            const edit = await this.performChangeSignature(document, functionInfo, newSignature);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage(`Successfully changed signature of '${functionInfo.name}'`);
                } else {
                    vscode.window.showErrorMessage('Failed to apply change signature refactoring');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Change signature failed: ${error}`);
        }
    }
    
    async convertToPatternMatching(uri?: vscode.Uri, range?: vscode.Range): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const document = uri ? await vscode.workspace.openTextDocument(uri) : editor.document;
        const selection = range || editor.selection;
        
        try {
            const ifElseChain = await this.analyzeIfElseChain(document, selection);
            if (!ifElseChain) {
                vscode.window.showErrorMessage('No suitable if-else chain found for conversion');
                return;
            }
            
            const edit = await this.performConvertToPatternMatching(document, ifElseChain);
            
            if (edit) {
                const success = await vscode.workspace.applyEdit(edit);
                if (success) {
                    vscode.window.showInformationMessage('Successfully converted to pattern matching');
                } else {
                    vscode.window.showErrorMessage('Failed to apply pattern matching conversion');
                }
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Convert to pattern matching failed: ${error}`);
        }
    }
    
    // Analysis methods
    
    private async analyzeCodeForExtraction(document: vscode.TextDocument, range: vscode.Range): Promise<ExtractionAnalysis> {
        const code = document.getText(range);
        
        // Analyze variables used in the selection
        const usedVariables = this.findUsedVariables(code);
        const definedVariables = this.findDefinedVariables(code);
        
        // Determine parameters (variables used but not defined)
        const parameters = usedVariables.filter(v => !definedVariables.includes(v));
        
        // Determine return type
        const returnStatements = this.findReturnStatements(code);
        const returnType = this.inferReturnType(returnStatements);
        
        return {
            parameters,
            returnType,
            usedVariables,
            definedVariables,
            returnStatements
        };
    }
    
    private async analyzeExpressionForExtraction(document: vscode.TextDocument, range: vscode.Range): Promise<ExpressionAnalysis> {
        const expression = document.getText(range);
        const inferredType = await this.inferExpressionType(expression);
        
        return {
            expression,
            inferredType,
            range
        };
    }
    
    private async findVariableAtPosition(document: vscode.TextDocument, position: vscode.Position): Promise<VariableInfo | null> {
        const line = document.lineAt(position.line);
        const text = line.text;
        
        // Simple variable detection - in practice would use AST
        const variableMatch = text.match(/sus\s+([a-zA-Z_][a-zA-Z0-9_]*)/);
        if (variableMatch && position.character >= line.text.indexOf(variableMatch[1])) {
            return {
                name: variableMatch[1],
                type: this.extractVariableType(text),
                range: new vscode.Range(position.line, line.text.indexOf(variableMatch[1]), position.line, line.text.indexOf(variableMatch[1]) + variableMatch[1].length)
            };
        }
        
        return null;
    }
    
    private async findVariableReferences(document: vscode.TextDocument, variable: VariableInfo): Promise<vscode.Range[]> {
        const references: vscode.Range[] = [];
        const text = document.getText();
        const regex = new RegExp(`\\b${variable.name}\\b`, 'g');
        
        let match;
        while ((match = regex.exec(text)) !== null) {
            const pos = document.positionAt(match.index);
            references.push(new vscode.Range(pos, document.positionAt(match.index + match[0].length)));
        }
        
        return references;
    }
    
    private async findSymbolAtPosition(document: vscode.TextDocument, position: vscode.Position): Promise<SymbolInfo | null> {
        // Use language server to find symbol information
        try {
            const result = await this.client.sendRequest('textDocument/documentSymbol', {
                textDocument: { uri: document.uri.toString() }
            });
            
            // Find symbol at position
            for (const symbol of result || []) {
                if (this.positionInRange(position, symbol.range)) {
                    return {
                        name: symbol.name,
                        kind: symbol.kind,
                        range: symbol.range
                    };
                }
            }
        } catch (error) {
            console.error('Failed to find symbol:', error);
        }
        
        return null;
    }
    
    private async findFunctionAtPosition(document: vscode.TextDocument, position: vscode.Position): Promise<FunctionInfo | null> {
        const line = document.lineAt(position.line);
        const text = line.text;
        
        // Simple function detection
        const functionMatch = text.match(/slay\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*([a-zA-Z_][a-zA-Z0-9_]*)?/);
        if (functionMatch) {
            return {
                name: functionMatch[1],
                parameters: this.parseParameters(functionMatch[2]),
                returnType: functionMatch[3] || 'void',
                range: new vscode.Range(position.line, 0, position.line, text.length)
            };
        }
        
        return null;
    }
    
    private async analyzeIfElseChain(document: vscode.TextDocument, range: vscode.Range): Promise<IfElseChain | null> {
        const code = document.getText(range);
        
        if (!code.includes('ready') || !code.includes('otherwise')) {
            return null;
        }
        
        // Parse if-else chain structure
        const conditions = this.extractConditions(code);
        const branches = this.extractBranches(code);
        
        if (conditions.length > 1) {
            return {
                conditions,
                branches,
                range
            };
        }
        
        return null;
    }
    
    // Refactoring implementation methods
    
    private async performExtractFunction(
        document: vscode.TextDocument,
        range: vscode.Range,
        functionName: string,
        signature: string,
        analysis: ExtractionAnalysis
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        const originalCode = document.getText(range);
        
        // Generate function call
        const parameterNames = analysis.parameters.map(p => p.name);
        const functionCall = `${functionName}(${parameterNames.join(', ')})`;
        
        // Generate function definition
        const functionDef = this.generateFunctionDefinition(functionName, signature, originalCode, analysis);
        
        // Replace original code with function call
        edit.replace(document.uri, range, functionCall);
        
        // Insert function definition at appropriate location
        const insertPosition = this.findFunctionInsertPosition(document);
        edit.insert(document.uri, insertPosition, `\n\n${functionDef}\n`);
        
        return edit;
    }
    
    private async performExtractVariable(
        document: vscode.TextDocument,
        range: vscode.Range,
        variableName: string,
        analysis: ExpressionAnalysis
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        const expression = analysis.expression;
        
        // Generate variable declaration
        const variableDeclaration = `sus ${variableName} ${analysis.inferredType} = ${expression}`;
        
        // Find insertion point (before the line containing the expression)
        const insertPosition = new vscode.Position(range.start.line, 0);
        edit.insert(document.uri, insertPosition, `${variableDeclaration}\n`);
        
        // Replace expression with variable name
        edit.replace(document.uri, range, variableName);
        
        return edit;
    }
    
    private async performInlineVariable(
        document: vscode.TextDocument,
        variable: VariableInfo,
        references: vscode.Range[]
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        
        // Get variable value
        const line = document.lineAt(variable.range.start.line);
        const valueMatch = line.text.match(/=\s*(.+)$/);
        if (!valueMatch) {
            return null;
        }
        
        const value = valueMatch[1].trim();
        
        // Replace all references with the value
        for (const ref of references) {
            edit.replace(document.uri, ref, value);
        }
        
        // Remove variable declaration
        const lineRange = new vscode.Range(variable.range.start.line, 0, variable.range.start.line + 1, 0);
        edit.delete(document.uri, lineRange);
        
        return edit;
    }
    
    private async performMoveSymbol(
        document: vscode.TextDocument,
        symbol: SymbolInfo,
        destinationFile: vscode.Uri
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        
        // Get symbol definition
        const symbolCode = document.getText(symbol.range);
        
        // Remove from source file
        edit.delete(document.uri, symbol.range);
        
        // Add to destination file
        const destDoc = await vscode.workspace.openTextDocument(destinationFile);
        const insertPosition = new vscode.Position(destDoc.lineCount, 0);
        edit.insert(destinationFile, insertPosition, `\n${symbolCode}\n`);
        
        return edit;
    }
    
    private async performChangeSignature(
        document: vscode.TextDocument,
        functionInfo: FunctionInfo,
        newSignature: string
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        
        // Replace function signature
        edit.replace(document.uri, functionInfo.range, newSignature);
        
        // TODO: Update all call sites
        
        return edit;
    }
    
    private async performConvertToPatternMatching(
        document: vscode.TextDocument,
        ifElseChain: IfElseChain
    ): Promise<vscode.WorkspaceEdit | null> {
        const edit = new vscode.WorkspaceEdit();
        
        // Generate pattern matching code
        const patternCode = this.generatePatternMatchingCode(ifElseChain);
        
        // Replace if-else chain
        edit.replace(document.uri, ifElseChain.range, patternCode);
        
        return edit;
    }
    
    // Helper methods
    
    private findUsedVariables(code: string): Parameter[] {
        const variables: Parameter[] = [];
        const regex = /\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g;
        let match;
        
        while ((match = regex.exec(code)) !== null) {
            const name = match[1];
            if (!this.isKeyword(name) && !variables.find(v => v.name === name)) {
                variables.push({
                    name,
                    type: 'unknown' // Would need type analysis
                });
            }
        }
        
        return variables;
    }
    
    private findDefinedVariables(code: string): string[] {
        const variables: string[] = [];
        const regex = /sus\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
        let match;
        
        while ((match = regex.exec(code)) !== null) {
            variables.push(match[1]);
        }
        
        return variables;
    }
    
    private findReturnStatements(code: string): string[] {
        const returns: string[] = [];
        const regex = /damn\s+(.+)/g;
        let match;
        
        while ((match = regex.exec(code)) !== null) {
            returns.push(match[1].trim());
        }
        
        return returns;
    }
    
    private inferReturnType(returnStatements: string[]): string {
        if (returnStatements.length === 0) {
            return 'void';
        }
        
        // Simple type inference
        const firstReturn = returnStatements[0];
        if (firstReturn.includes('"')) return 'tea';
        if (firstReturn === 'based' || firstReturn === 'cringe') return 'lit';
        if (/^\d+$/.test(firstReturn)) return 'drip';
        
        return 'unknown';
    }
    
    private async inferExpressionType(expression: string): Promise<string> {
        // Simple type inference
        if (expression.includes('"')) return 'tea';
        if (expression === 'based' || expression === 'cringe') return 'lit';
        if (/^\d+$/.test(expression)) return 'drip';
        if (expression.includes('[')) return '[]';
        
        return 'auto';
    }
    
    private extractVariableType(text: string): string {
        const typeMatch = text.match(/sus\s+[a-zA-Z_][a-zA-Z0-9_]*\s+([a-zA-Z_][a-zA-Z0-9_]*)/);
        return typeMatch ? typeMatch[1] : 'auto';
    }
    
    private async generateFunctionSignature(functionName: string, analysis: ExtractionAnalysis): Promise<string> {
        const params = analysis.parameters.map(p => `${p.name} ${p.type}`).join(', ');
        return `slay ${functionName}(${params}) ${analysis.returnType}`;
    }
    
    private generateFunctionDefinition(functionName: string, signature: string, body: string, analysis: ExtractionAnalysis): string {
        return `${signature} {\n    ${body.replace(/\n/g, '\n    ')}\n}`;
    }
    
    private findFunctionInsertPosition(document: vscode.TextDocument): vscode.Position {
        // Insert at end of file
        return new vscode.Position(document.lineCount, 0);
    }
    
    private parseParameters(paramString: string): Parameter[] {
        if (!paramString.trim()) {
            return [];
        }
        
        return paramString.split(',').map(param => {
            const parts = param.trim().split(/\s+/);
            return {
                name: parts[0],
                type: parts[1] || 'unknown'
            };
        });
    }
    
    private extractConditions(code: string): string[] {
        const conditions: string[] = [];
        const regex = /ready\s*\(([^)]+)\)/g;
        let match;
        
        while ((match = regex.exec(code)) !== null) {
            conditions.push(match[1]);
        }
        
        return conditions;
    }
    
    private extractBranches(code: string): string[] {
        // Simple branch extraction - would need proper parsing
        return code.split(/ready|otherwise/).filter(branch => branch.trim().length > 0);
    }
    
    private generatePatternMatchingCode(ifElseChain: IfElseChain): string {
        let patternCode = 'sick (value) {\n';
        
        for (let i = 0; i < ifElseChain.conditions.length; i++) {
            const condition = ifElseChain.conditions[i];
            const branch = ifElseChain.branches[i];
            patternCode += `    when ${condition} -> {\n        ${branch.trim()}\n    }\n`;
        }
        
        patternCode += '    when _ -> {\n        // Default case\n    }\n}';
        
        return patternCode;
    }
    
    private async promptForNewSignature(functionInfo: FunctionInfo): Promise<string | null> {
        const currentSignature = `slay ${functionInfo.name}(${functionInfo.parameters.map(p => `${p.name} ${p.type}`).join(', ')}) ${functionInfo.returnType}`;
        
        return await vscode.window.showInputBox({
            prompt: 'Enter new function signature',
            value: currentSignature,
            validateInput: (value) => {
                if (!value || !value.includes('slay')) {
                    return 'Invalid function signature';
                }
                return null;
            }
        });
    }
    
    private isKeyword(name: string): boolean {
        const keywords = ['sus', 'damn', 'slay', 'vibez', 'yeet', 'bestie', 'ready', 'otherwise', 'squad', 'collab', 'sick', 'when', 'based', 'cringe', 'drip', 'tea', 'lit'];
        return keywords.includes(name);
    }
    
    private positionInRange(position: vscode.Position, range: vscode.Range): boolean {
        return position.line >= range.start.line && position.line <= range.end.line &&
               position.character >= range.start.character && position.character <= range.end.character;
    }
}

// Type definitions

interface ExtractionAnalysis {
    parameters: Parameter[];
    returnType: string;
    usedVariables: Parameter[];
    definedVariables: string[];
    returnStatements: string[];
}

interface ExpressionAnalysis {
    expression: string;
    inferredType: string;
    range: vscode.Range;
}

interface Parameter {
    name: string;
    type: string;
}

interface VariableInfo {
    name: string;
    type: string;
    range: vscode.Range;
}

interface SymbolInfo {
    name: string;
    kind: number;
    range: vscode.Range;
}

interface FunctionInfo {
    name: string;
    parameters: Parameter[];
    returnType: string;
    range: vscode.Range;
}

interface IfElseChain {
    conditions: string[];
    branches: string[];
    range: vscode.Range;
}
