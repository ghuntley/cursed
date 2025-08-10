import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    ExecutableOptions
} from 'vscode-languageclient/node';
import * as path from 'path';

export class CursedLanguageClient implements vscode.Disposable {
    private client: LanguageClient | undefined;
    private context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.startLanguageServer();
    }

    private async startLanguageServer() {
        const config = vscode.workspace.getConfiguration('cursed');
        
        if (!config.get('lsp.enabled', true)) {
            return;
        }

        // Try to find the CURSED LSP server
        const serverPath = await this.findLspServer();
        
        if (!serverPath) {
            vscode.window.showErrorMessage(
                'CURSED Language Server not found. Please ensure CURSED is installed and available in PATH.',
                'Install CURSED'
            ).then(selection => {
                if (selection === 'Install CURSED') {
                    vscode.env.openExternal(vscode.Uri.parse('https://cursed-lang.org/install'));
                }
            });
            return;
        }

        // Configure server options
        const serverOptions: ServerOptions = {
            command: serverPath,
            args: ['--lsp'],
            options: {
                env: { ...process.env }
            } as ExecutableOptions
        };

        // Configure client options
        const clientOptions: LanguageClientOptions = {
            documentSelector: [
                { scheme: 'file', language: 'cursed' },
                { scheme: 'untitled', language: 'cursed' }
            ],
            synchronize: {
                fileEvents: [
                    vscode.workspace.createFileSystemWatcher('**/*.csd'),
                    vscode.workspace.createFileSystemWatcher('**/CursedPackage.toml'),
                    vscode.workspace.createFileSystemWatcher('**/CursedBuild.toml')
                ]
            },
            initializationOptions: {
                debug: config.get('lsp.debug', false),
                trace: config.get('lsp.debug', false) ? 'verbose' : 'off'
            },
            outputChannel: vscode.window.createOutputChannel('CURSED Language Server'),
            traceOutputChannel: config.get('lsp.debug', false) 
                ? vscode.window.createOutputChannel('CURSED LSP Trace')
                : undefined
        };

        // Create and start the language client
        this.client = new LanguageClient(
            'cursedLanguageServer',
            'CURSED Language Server',
            serverOptions,
            clientOptions
        );

        // Register client capabilities
        this.registerClientCapabilities();

        try {
            await this.client.start();
            vscode.window.showInformationMessage('CURSED Language Server started successfully');
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to start CURSED Language Server: ${error}`);
        }
    }

    private async findLspServer(): Promise<string | null> {
        const config = vscode.workspace.getConfiguration('cursed');
        const compilerPath = config.get('compiler.path', 'cursed');

        // Try different possible locations for the CURSED compiler/LSP
        const possiblePaths = [
            compilerPath,
            'cursed',
            './zig-out/bin/cursed-zig',
            './zig-out/bin/cursed-lsp',
            path.join(vscode.workspace.workspaceFolders?.[0]?.uri?.fsPath || '', 'zig-out/bin/cursed-zig'),
            path.join(process.env.HOME || '', '.cursed/bin/cursed'),
            '/usr/local/bin/cursed',
            '/usr/bin/cursed'
        ];

        for (const testPath of possiblePaths) {
            if (await this.isExecutable(testPath)) {
                return testPath;
            }
        }

        return null;
    }

    private async isExecutable(filePath: string): Promise<boolean> {
        try {
            const { spawn } = require('child_process');
            return new Promise<boolean>((resolve) => {
                const process = spawn(filePath, ['--version'], { stdio: 'pipe' });
                
                process.on('close', (code: number) => {
                    resolve(code === 0);
                });
                
                process.on('error', () => {
                    resolve(false);
                });

                // Timeout after 2 seconds
                setTimeout(() => {
                    process.kill();
                    resolve(false);
                }, 2000);
            });
        } catch (error) {
            return false;
        }
    }

    private registerClientCapabilities() {
        if (!this.client) return;

        // Register custom commands that the LSP server might provide
        this.context.subscriptions.push(
            vscode.commands.registerCommand('cursed.lsp.restart', async () => {
                await this.restart();
            })
        );

        // Handle server notifications
        this.client.onDidChangeState((event) => {
            if (event.newState === 3) { // Running
                vscode.window.setStatusBarMessage('CURSED LSP: Ready', 2000);
            } else if (event.newState === 1) { // Starting
                vscode.window.setStatusBarMessage('CURSED LSP: Starting...', 2000);
            } else if (event.newState === 2) { // Stopped
                vscode.window.setStatusBarMessage('CURSED LSP: Stopped', 2000);
            }
        });

        // Register hover provider enhancement
        this.registerHoverEnhancements();

        // Register completion enhancements
        this.registerCompletionEnhancements();

        // Register diagnostic enhancements
        this.registerDiagnosticEnhancements();
    }

    private registerHoverEnhancements() {
        vscode.languages.registerHoverProvider('cursed', {
            provideHover(document, position) {
                const range = document.getWordRangeAtPosition(position);
                const word = document.getText(range);

                // Provide enhanced hover information for CURSED keywords
                const keywordDocs: { [key: string]: string } = {
                    'sus': 'Variable declaration keyword\n\nExample: `sus name tea = "value"`',
                    'slay': 'Function definition keyword\n\nExample: `slay functionName(params) returnType { body }`',
                    'damn': 'Return statement keyword\n\nExample: `damn value`',
                    'vibez': 'I/O operations module\n\nExample: `vibez.spill("Hello, world!")`',
                    'ready': 'If statement keyword\n\nExample: `ready (condition) { body }`',
                    'otherwise': 'Else clause keyword\n\nExample: `otherwise { body }`',
                    'bestie': 'While loop keyword\n\nExample: `bestie (condition) { body }`',
                    'squad': 'Struct definition keyword\n\nExample: `squad Name { fields }`',
                    'collab': 'Interface definition keyword\n\nExample: `collab Name { methods }`',
                    'yeet': 'Import module keyword\n\nExample: `yeet "module_name"`',
                    'based': 'Boolean true value',
                    'cringe': 'Boolean false value',
                    'drip': 'Integer numeric type',
                    'tea': 'String type',
                    'lit': 'Boolean type'
                };

                if (keywordDocs[word]) {
                    return new vscode.Hover(
                        new vscode.MarkdownString(`**${word}** (CURSED keyword)\n\n${keywordDocs[word]}`)
                    );
                }

                return null;
            }
        });
    }

    private registerCompletionEnhancements() {
        vscode.languages.registerCompletionItemProvider('cursed', {
            provideCompletionItems(document, position) {
                const completions: vscode.CompletionItem[] = [];

                // Add stdlib module completions
                const stdlibModules = [
                    { name: 'mathz', description: 'Mathematical operations' },
                    { name: 'stringz', description: 'String manipulation' },
                    { name: 'arrayz', description: 'Array operations' },
                    { name: 'testz', description: 'Testing framework' },
                    { name: 'cryptz', description: 'Cryptographic functions' },
                    { name: 'filez', description: 'File I/O operations' },
                    { name: 'httpz', description: 'HTTP client/server' },
                    { name: 'timez', description: 'Time and date operations' },
                    { name: 'jsonz', description: 'JSON parsing and generation' },
                    { name: 'vibez', description: 'Core I/O operations' },
                    { name: 'concurrenz', description: 'Concurrency primitives' }
                ];

                for (const module of stdlibModules) {
                    const item = new vscode.CompletionItem(module.name, vscode.CompletionItemKind.Module);
                    item.detail = module.description;
                    item.insertText = module.name;
                    item.documentation = new vscode.MarkdownString(`Import with: \`yeet "${module.name}"\``);
                    completions.push(item);
                }

                // Add common functions
                const functions = [
                    { name: 'vibez.spill', description: 'Print to console', snippet: 'vibez.spill(${1:value})' },
                    { name: 'len', description: 'Get length of array/string', snippet: 'len(${1:value})' },
                    { name: 'abs_normie', description: 'Absolute value (mathz)', snippet: 'abs_normie(${1:value})' },
                    { name: 'slice_tea', description: 'String slice (stringz)', snippet: 'slice_tea(${1:string}, ${2:start}, ${3:end})' }
                ];

                for (const func of functions) {
                    const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
                    item.detail = func.description;
                    item.insertText = new vscode.SnippetString(func.snippet);
                    completions.push(item);
                }

                return completions;
            }
        }, '.'); // Trigger on dot
    }

    private registerDiagnosticEnhancements() {
        // Enhanced diagnostic collection for CURSED-specific issues
        const diagnosticCollection = vscode.languages.createDiagnosticCollection('cursed-enhanced');
        this.context.subscriptions.push(diagnosticCollection);

        const updateDiagnostics = (document: vscode.TextDocument) => {
            if (document.languageId !== 'cursed') return;

            const diagnostics: vscode.Diagnostic[] = [];
            const text = document.getText();
            const lines = text.split('\n');

            lines.forEach((line, lineIndex) => {
                // Check for common CURSED issues
                
                // Missing semicolons (simplified check)
                if (line.trim().match(/^(sus|slay|damn)\s+.*[^;}]$/)) {
                    const diagnostic = new vscode.Diagnostic(
                        new vscode.Range(lineIndex, line.length - 1, lineIndex, line.length),
                        'Consider adding a semicolon',
                        vscode.DiagnosticSeverity.Hint
                    );
                    diagnostic.code = 'missing-semicolon';
                    diagnostic.source = 'cursed-enhanced';
                    diagnostics.push(diagnostic);
                }

                // Unused imports (yeet statements not referenced)
                const importMatch = line.match(/yeet\s+"([^"]+)"/);
                if (importMatch) {
                    const moduleName = importMatch[1];
                    if (!text.includes(`${moduleName}.`) && moduleName !== 'vibez') {
                        const startPos = line.indexOf(`"${moduleName}"`);
                        const diagnostic = new vscode.Diagnostic(
                            new vscode.Range(lineIndex, startPos, lineIndex, startPos + moduleName.length + 2),
                            `Unused import: ${moduleName}`,
                            vscode.DiagnosticSeverity.Warning
                        );
                        diagnostic.code = 'unused-import';
                        diagnostic.source = 'cursed-enhanced';
                        diagnostics.push(diagnostic);
                    }
                }
            });

            diagnosticCollection.set(document.uri, diagnostics);
        };

        // Update diagnostics on document changes
        vscode.workspace.onDidChangeTextDocument(event => {
            updateDiagnostics(event.document);
        });

        vscode.workspace.onDidOpenTextDocument(updateDiagnostics);

        // Update all open CURSED documents
        vscode.workspace.textDocuments.forEach(updateDiagnostics);
    }

    public async restart() {
        if (this.client) {
            await this.client.stop();
        }
        await this.startLanguageServer();
    }

    public dispose() {
        if (this.client) {
            this.client.stop();
        }
    }
}
