/**
 * Advanced CURSED Language Extension for VS Code
 * World-class IDE support with refactoring, code generation, and analysis
 */

import * as vscode from 'vscode';
import * as path from 'path';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    RevealOutputChannelOn,
    InitializeParams,
    ExecuteCommandParams,
    TextDocumentPositionParams,
    Range,
    Position,
    TextEdit,
    WorkspaceEdit,
    CodeLens,
    CodeAction,
    CodeActionKind,
    Diagnostic,
    DocumentSymbol,
    SymbolInformation,
    Location,
    Hover,
    CompletionList,
    SignatureHelp,
    ReferenceContext,
    CallHierarchyItem,
    TypeHierarchyItem,
    InlayHint,
} from 'vscode-languageclient/node';

import { CursedTaskProvider } from './taskProvider';
import { CursedDebugAdapterDescriptorFactory } from './debugAdapter';
import { CursedTestController } from './testController';
import { CursedCodeActionProvider } from './codeActionProvider';
import { CursedRefactoringProvider } from './refactoringProvider';
import { CursedCodeGenerationProvider } from './codeGenerationProvider';
import { CursedAnalysisProvider } from './analysisProvider';

let client: LanguageClient;
let outputChannel: vscode.OutputChannel;
let taskProvider: vscode.Disposable;
let debugAdapterFactory: vscode.Disposable;
let testController: CursedTestController;

export function activate(context: vscode.ExtensionContext) {
    console.log('Advanced CURSED Language Extension is activating...');
    
    // Create output channel
    outputChannel = vscode.window.createOutputChannel('CURSED Language Server');
    context.subscriptions.push(outputChannel);
    
    // Initialize LSP client
    initializeLanguageClient(context);
    
    // Register providers
    registerProviders(context);
    
    // Register commands
    registerCommands(context);
    
    // Setup task provider
    setupTaskProvider(context);
    
    // Setup debug adapter
    setupDebugAdapter(context);
    
    // Setup test controller
    setupTestController(context);
    
    // Setup file watchers
    setupFileWatchers(context);
    
    // Setup workspace events
    setupWorkspaceEvents(context);
    
    console.log('Advanced CURSED Language Extension is now active!');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    
    // Dispose of providers
    if (taskProvider) {
        taskProvider.dispose();
    }
    if (debugAdapterFactory) {
        debugAdapterFactory.dispose();
    }
    if (testController) {
        testController.dispose();
    }
    
    return client.stop();
}

function initializeLanguageClient(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('cursed');
    const lspPath = config.get<string>('lsp.path', 'cursed-lsp');
    const traceLevel = config.get<string>('lsp.trace', 'off');
    
    // Server options - either use the configured path or try to find it
    const serverOptions: ServerOptions = {
        command: lspPath,
        args: [],
        options: {
            env: process.env
        }
    };
    
    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'cursed' },
            { scheme: 'untitled', language: 'cursed' }
        ],
        synchronize: {
            fileEvents: [
                vscode.workspace.createFileSystemWatcher('**/*.csd'),
                vscode.workspace.createFileSystemWatcher('**/cursed.toml'),
                vscode.workspace.createFileSystemWatcher('**/Cursed.toml')
            ]
        },
        outputChannel: outputChannel,
        revealOutputChannelOn: RevealOutputChannelOn.Never,
        initializationOptions: {
            features: {
                completion: true,
                hover: true,
                definition: true,
                references: true,
                formatting: true,
                rename: true,
                semanticTokens: true,
                codeLens: config.get('codeLens.enabled', true),
                inlayHints: config.get('inlayHints.enabled', true),
                codeAction: true,
                callHierarchy: true,
                typeHierarchy: true,
                workspaceSymbol: true,
                // Advanced CURSED features
                cursedFeatures: {
                    securityAnalysis: config.get('analysis.security.enabled', true),
                    performanceAnalysis: config.get('analysis.performance.enabled', true),
                    memoryAnalysis: config.get('analysis.memory.enabled', true),
                    concurrencyAnalysis: config.get('analysis.concurrency.enabled', true),
                    refactoring: {
                        extractFunction: config.get('refactoring.extractFunction.enabled', true),
                        inlineVariable: config.get('refactoring.inlineVariable.enabled', true)
                    },
                    codeGeneration: {
                        autoImport: config.get('codeGeneration.autoImport.enabled', true),
                        generateTests: config.get('codeGeneration.generateTests.enabled', true)
                    }
                }
            }
        },
        middleware: {
            // Enhanced completion middleware
            provideCompletionItem: async (document, position, context, token, next) => {
                const result = await next(document, position, context, token);
                if (result && 'items' in result) {
                    // Post-process completion items for better UX
                    return enhanceCompletionItems(result);
                }
                return result;
            },
            
            // Enhanced hover middleware
            provideHover: async (document, position, token, next) => {
                const result = await next(document, position, token);
                if (result) {
                    return enhanceHover(result, document, position);
                }
                return result;
            },
            
            // Enhanced diagnostics middleware
            handleDiagnostics: (uri, diagnostics, next) => {
                // Filter and enhance diagnostics based on user preferences
                const filteredDiagnostics = filterDiagnostics(diagnostics);
                next(uri, filteredDiagnostics);
            }
        }
    };
    
    // Create and start the language client
    client = new LanguageClient(
        'cursedLanguageServer',
        'CURSED Language Server',
        serverOptions,
        clientOptions
    );
    
    // Start the client and server
    client.start().then(() => {
        outputChannel.appendLine('CURSED Language Server started successfully');
        
        // Register for client events
        client.onReady().then(() => {
            outputChannel.appendLine('CURSED Language Server is ready');
            
            // Setup custom request handlers
            setupCustomRequestHandlers();
        });
    }).catch((error) => {
        outputChannel.appendLine(`Failed to start CURSED Language Server: ${error}`);
        vscode.window.showErrorMessage(`Failed to start CURSED Language Server: ${error}`);
    });
    
    context.subscriptions.push(client);
}

function registerProviders(context: vscode.ExtensionContext) {
    // Register code action provider
    const codeActionProvider = new CursedCodeActionProvider();
    context.subscriptions.push(
        vscode.languages.registerCodeActionsProvider('cursed', codeActionProvider, {
            providedCodeActionKinds: [
                CodeActionKind.QuickFix,
                CodeActionKind.Refactor,
                CodeActionKind.RefactorExtract,
                CodeActionKind.RefactorInline,
                CodeActionKind.RefactorRewrite,
                CodeActionKind.Source,
                CodeActionKind.SourceOrganizeImports
            ]
        })
    );
    
    // Register refactoring provider
    const refactoringProvider = new CursedRefactoringProvider(client);
    context.subscriptions.push(refactoringProvider);
    
    // Register code generation provider
    const codeGenerationProvider = new CursedCodeGenerationProvider(client);
    context.subscriptions.push(codeGenerationProvider);
    
    // Register analysis provider
    const analysisProvider = new CursedAnalysisProvider(client, outputChannel);
    context.subscriptions.push(analysisProvider);
    
    // Register document formatting provider with cursed-fmt integration
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider('cursed', {
            async provideDocumentFormattingEdits(document: vscode.TextDocument): Promise<vscode.TextEdit[]> {
                return await formatDocument(document);
            }
        })
    );
    
    // Register document range formatting provider
    context.subscriptions.push(
        vscode.languages.registerDocumentRangeFormattingEditProvider('cursed', {
            async provideDocumentRangeFormattingEdits(
                document: vscode.TextDocument,
                range: vscode.Range
            ): Promise<vscode.TextEdit[]> {
                return await formatDocumentRange(document, range);
            }
        })
    );
    
    // Register on-type formatting provider
    context.subscriptions.push(
        vscode.languages.registerOnTypeFormattingEditProvider('cursed', {
            async provideOnTypeFormattingEdits(
                document: vscode.TextDocument,
                position: vscode.Position,
                ch: string
            ): Promise<vscode.TextEdit[]> {
                return await formatOnType(document, position, ch);
            }
        }, '}', '\n', ';')
    );
    
    // Register inlay hints provider
    context.subscriptions.push(
        vscode.languages.registerInlayHintsProvider('cursed', {
            async provideInlayHints(
                document: vscode.TextDocument,
                range: vscode.Range,
                token: vscode.CancellationToken
            ): Promise<vscode.InlayHint[]> {
                if (!vscode.workspace.getConfiguration('cursed').get('inlayHints.enabled', true)) {
                    return [];
                }
                return await client.sendRequest('textDocument/inlayHint', {
                    textDocument: { uri: document.uri.toString() },
                    range: {
                        start: { line: range.start.line, character: range.start.character },
                        end: { line: range.end.line, character: range.end.character }
                    }
                }) || [];
            }
        })
    );
    
    // Register semantic tokens provider
    const legend = new vscode.SemanticTokensLegend(
        ['keyword', 'string', 'number', 'comment', 'operator', 'namespace', 'type', 'class', 'interface', 'enum', 'function', 'method', 'variable', 'parameter', 'property', 'struct', 'typeParameter', 'decorator'],
        ['declaration', 'definition', 'readonly', 'static', 'deprecated', 'abstract', 'async', 'modification', 'documentation', 'defaultLibrary', 'generic']
    );
    
    context.subscriptions.push(
        vscode.languages.registerDocumentSemanticTokensProvider('cursed', {
            async provideDocumentSemanticTokens(
                document: vscode.TextDocument,
                token: vscode.CancellationToken
            ): Promise<vscode.SemanticTokens> {
                const result = await client.sendRequest('textDocument/semanticTokens/full', {
                    textDocument: { uri: document.uri.toString() }
                });
                return new vscode.SemanticTokens(new Uint32Array(result?.data || []));
            }
        }, legend)
    );
}

function registerCommands(context: vscode.ExtensionContext) {
    // Language server commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.restartLanguageServer', async () => {
            await client.stop();
            await client.start();
            vscode.window.showInformationMessage('CURSED Language Server restarted');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.showOutput', () => {
            outputChannel.show();
        })
    );
    
    // Formatting commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.format', async () => {
            await vscode.commands.executeCommand('editor.action.formatDocument');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.organizeImports', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            const actions = await vscode.commands.executeCommand<vscode.CodeAction[]>(
                'vscode.executeCodeActionProvider',
                editor.document.uri,
                editor.selection,
                CodeActionKind.SourceOrganizeImports
            );
            
            if (actions && actions.length > 0) {
                const edit = actions[0].edit;
                if (edit) {
                    await vscode.workspace.applyEdit(edit);
                }
            }
        })
    );
    
    // Refactoring commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.extractFunction', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            if (editor.selection.isEmpty) {
                vscode.window.showErrorMessage('Please select code to extract into a function');
                return;
            }
            
            const result = await client.sendRequest('cursed/extractFunction', {
                textDocument: { uri: editor.document.uri.toString() },
                range: {
                    start: { line: editor.selection.start.line, character: editor.selection.start.character },
                    end: { line: editor.selection.end.line, character: editor.selection.end.character }
                }
            });
            
            if (result && result.edit) {
                await vscode.workspace.applyEdit(result.edit);
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.extractVariable', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            // TODO: Implement extract variable
            vscode.window.showInformationMessage('Extract variable feature coming soon!');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.inlineVariable', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            // TODO: Implement inline variable
            vscode.window.showInformationMessage('Inline variable feature coming soon!');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.renameSymbol', async () => {
            await vscode.commands.executeCommand('editor.action.rename');
        })
    );
    
    // Code generation commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.generateFunction', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            const functionName = await vscode.window.showInputBox({
                prompt: 'Enter function name',
                placeHolder: 'function_name'
            });
            
            if (!functionName) {
                return;
            }
            
            const result = await client.sendRequest('cursed/generateFunction', {
                textDocument: { uri: editor.document.uri.toString() },
                position: { line: editor.selection.active.line, character: editor.selection.active.character },
                functionName: functionName
            });
            
            if (result && result.edit) {
                await vscode.workspace.applyEdit(result.edit);
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.generateConstructor', async () => {
            // TODO: Implement generate constructor
            vscode.window.showInformationMessage('Generate constructor feature coming soon!');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.generateTests', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            const result = await client.sendRequest('cursed/generateTests', {
                textDocument: { uri: editor.document.uri.toString() }
            });
            
            if (result && result.edit) {
                await vscode.workspace.applyEdit(result.edit);
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.generateInterface', async () => {
            // TODO: Implement generate interface
            vscode.window.showInformationMessage('Generate interface feature coming soon!');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.addErrorHandling', async () => {
            // TODO: Implement add error handling
            vscode.window.showInformationMessage('Add error handling feature coming soon!');
        })
    );
    
    // Navigation commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.showCallHierarchy', async () => {
            await vscode.commands.executeCommand('references-view.showCallHierarchy');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.showTypeHierarchy', async () => {
            await vscode.commands.executeCommand('references-view.showTypeHierarchy');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.showReferences', async () => {
            await vscode.commands.executeCommand('editor.action.goToReferences');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.showImplementations', async () => {
            await vscode.commands.executeCommand('editor.action.goToImplementation');
        })
    );
    
    // Analysis commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.analysis.security', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            outputChannel.show();
            outputChannel.appendLine('Running security analysis...');
            
            const result = await client.sendRequest('cursed/securityAnalysis', {
                textDocument: { uri: editor.document.uri.toString() }
            });
            
            if (result && result.vulnerabilities) {
                outputChannel.appendLine(`Found ${result.vulnerabilities.length} security issues`);
                for (const vuln of result.vulnerabilities) {
                    outputChannel.appendLine(`- ${vuln.message} (${vuln.category})`);
                }
            } else {
                outputChannel.appendLine('No security issues found');
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.analysis.performance', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cursed') {
                return;
            }
            
            outputChannel.show();
            outputChannel.appendLine('Running performance analysis...');
            
            const result = await client.sendRequest('cursed/performanceHints', {
                textDocument: { uri: editor.document.uri.toString() }
            });
            
            if (result && result.hints) {
                outputChannel.appendLine(`Found ${result.hints.length} performance suggestions`);
                for (const hint of result.hints) {
                    outputChannel.appendLine(`- ${hint.message}: ${hint.suggestion}`);
                }
            } else {
                outputChannel.appendLine('No performance issues found');
            }
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.analysis.memory', async () => {
            // TODO: Implement memory analysis
            vscode.window.showInformationMessage('Memory analysis feature coming soon!');
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.analysis.concurrency', async () => {
            // TODO: Implement concurrency analysis
            vscode.window.showInformationMessage('Concurrency analysis feature coming soon!');
        })
    );
    
    // Build and run commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.build', async () => {
            const terminal = vscode.window.createTerminal('CURSED Build');
            terminal.sendText('zig build');
            terminal.show();
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.run', async () => {
            const terminal = vscode.window.createTerminal('CURSED Run');
            terminal.sendText('zig build && ./zig-out/bin/cursed-zig main.csd');
            terminal.show();
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.test', async () => {
            const terminal = vscode.window.createTerminal('CURSED Test');
            terminal.sendText('zig test');
            terminal.show();
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.benchmark', async () => {
            const terminal = vscode.window.createTerminal('CURSED Benchmark');
            terminal.sendText('zig build benchmark');
            terminal.show();
        })
    );
    
    // Project commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.newProject', async () => {
            const projectName = await vscode.window.showInputBox({
                prompt: 'Enter project name',
                placeHolder: 'my-cursed-project'
            });
            
            if (!projectName) {
                return;
            }
            
            const folderUri = await vscode.window.showOpenDialog({
                canSelectFolders: true,
                canSelectFiles: false,
                canSelectMany: false,
                openLabel: 'Select Project Location'
            });
            
            if (!folderUri || folderUri.length === 0) {
                return;
            }
            
            // TODO: Create new CURSED project structure
            vscode.window.showInformationMessage(`Creating new CURSED project: ${projectName}`);
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.newFile', async () => {
            const fileName = await vscode.window.showInputBox({
                prompt: 'Enter file name',
                placeHolder: 'example.csd'
            });
            
            if (!fileName) {
                return;
            }
            
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (!workspaceFolder) {
                vscode.window.showErrorMessage('No workspace folder open');
                return;
            }
            
            const filePath = path.join(workspaceFolder.uri.fsPath, fileName);
            const fileUri = vscode.Uri.file(filePath);
            
            try {
                await vscode.workspace.fs.writeFile(fileUri, new Uint8Array());
                const document = await vscode.workspace.openTextDocument(fileUri);
                await vscode.window.showTextDocument(document);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to create file: ${error}`);
            }
        })
    );
}

function setupTaskProvider(context: vscode.ExtensionContext) {
    taskProvider = vscode.tasks.registerTaskProvider('cursed', new CursedTaskProvider());
}

function setupDebugAdapter(context: vscode.ExtensionContext) {
    debugAdapterFactory = vscode.debug.registerDebugAdapterDescriptorFactory(
        'cursed',
        new CursedDebugAdapterDescriptorFactory()
    );
}

function setupTestController(context: vscode.ExtensionContext) {
    testController = new CursedTestController(context);
}

function setupFileWatchers(context: vscode.ExtensionContext) {
    // Watch for CURSED files
    const cursedWatcher = vscode.workspace.createFileSystemWatcher('**/*.csd');
    
    cursedWatcher.onDidCreate((uri) => {
        outputChannel.appendLine(`Created: ${uri.fsPath}`);
    });
    
    cursedWatcher.onDidChange((uri) => {
        outputChannel.appendLine(`Changed: ${uri.fsPath}`);
    });
    
    cursedWatcher.onDidDelete((uri) => {
        outputChannel.appendLine(`Deleted: ${uri.fsPath}`);
    });
    
    context.subscriptions.push(cursedWatcher);
    
    // Watch for project files
    const projectWatcher = vscode.workspace.createFileSystemWatcher('**/cursed.toml');
    
    projectWatcher.onDidChange(() => {
        // Restart language server when project configuration changes
        vscode.commands.executeCommand('cursed.restartLanguageServer');
    });
    
    context.subscriptions.push(projectWatcher);
}

function setupWorkspaceEvents(context: vscode.ExtensionContext) {
    // Auto-format on save
    context.subscriptions.push(
        vscode.workspace.onWillSaveTextDocument((event) => {
            const document = event.document;
            if (document.languageId !== 'cursed') {
                return;
            }
            
            const config = vscode.workspace.getConfiguration('cursed');
            if (config.get('format.onSave', true)) {
                event.waitUntil(formatDocument(document));
            }
        })
    );
    
    // Configuration changes
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration((event) => {
            if (event.affectsConfiguration('cursed')) {
                // Restart language server on configuration changes
                vscode.commands.executeCommand('cursed.restartLanguageServer');
            }
        })
    );
}

function setupCustomRequestHandlers() {
    // Handle custom LSP requests here if needed
}

// Helper functions

async function formatDocument(document: vscode.TextDocument): Promise<vscode.TextEdit[]> {
    try {
        const result = await client.sendRequest('textDocument/formatting', {
            textDocument: { uri: document.uri.toString() },
            options: {
                tabSize: vscode.workspace.getConfiguration('editor').get('tabSize', 4),
                insertSpaces: vscode.workspace.getConfiguration('editor').get('insertSpaces', true),
                insertFinalNewline: vscode.workspace.getConfiguration('cursed').get('format.insertFinalNewline', true),
                trimTrailingWhitespace: vscode.workspace.getConfiguration('cursed').get('format.trimTrailingWhitespace', true)
            }
        });
        
        return result || [];
    } catch (error) {
        outputChannel.appendLine(`Format error: ${error}`);
        return [];
    }
}

async function formatDocumentRange(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.TextEdit[]> {
    try {
        const result = await client.sendRequest('textDocument/rangeFormatting', {
            textDocument: { uri: document.uri.toString() },
            range: {
                start: { line: range.start.line, character: range.start.character },
                end: { line: range.end.line, character: range.end.character }
            },
            options: {
                tabSize: vscode.workspace.getConfiguration('editor').get('tabSize', 4),
                insertSpaces: vscode.workspace.getConfiguration('editor').get('insertSpaces', true)
            }
        });
        
        return result || [];
    } catch (error) {
        outputChannel.appendLine(`Range format error: ${error}`);
        return [];
    }
}

async function formatOnType(document: vscode.TextDocument, position: vscode.Position, ch: string): Promise<vscode.TextEdit[]> {
    try {
        const result = await client.sendRequest('textDocument/onTypeFormatting', {
            textDocument: { uri: document.uri.toString() },
            position: { line: position.line, character: position.character },
            ch: ch,
            options: {
                tabSize: vscode.workspace.getConfiguration('editor').get('tabSize', 4),
                insertSpaces: vscode.workspace.getConfiguration('editor').get('insertSpaces', true)
            }
        });
        
        return result || [];
    } catch (error) {
        outputChannel.appendLine(`On-type format error: ${error}`);
        return [];
    }
}

function enhanceCompletionItems(completionList: CompletionList): CompletionList {
    // Add custom enhancement logic for completion items
    return completionList;
}

function enhanceHover(hover: Hover, document: vscode.TextDocument, position: vscode.Position): Hover {
    // Add custom enhancement logic for hover information
    return hover;
}

function filterDiagnostics(diagnostics: Diagnostic[]): Diagnostic[] {
    const config = vscode.workspace.getConfiguration('cursed');
    
    // Filter diagnostics based on user preferences
    return diagnostics.filter(diagnostic => {
        // Filter security diagnostics if disabled
        if (diagnostic.source === 'cursed-security' && !config.get('analysis.security.enabled', true)) {
            return false;
        }
        
        // Filter performance diagnostics if disabled
        if (diagnostic.source === 'cursed-performance' && !config.get('analysis.performance.enabled', true)) {
            return false;
        }
        
        return true;
    });
}
