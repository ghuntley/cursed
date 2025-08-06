import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Language Extension is now active!');

    // Get configuration
    const config = vscode.workspace.getConfiguration('cursed');
    const lspEnabled = config.get<boolean>('lsp.enabled', true);
    
    if (!lspEnabled) {
        console.log('CURSED LSP is disabled in configuration');
        return;
    }

    // Language Server setup
    const serverPath = config.get<string>('lsp.serverPath', 'cursed-lsp');
    
    // Server options
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: [],
        transport: TransportKind.stdio
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'cursed' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{csd,cursed}')
        },
        outputChannelName: 'CURSED Language Server'
    };

    // Create and start the language client
    client = new LanguageClient(
        'cursed-lsp',
        'CURSED Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (this will also launch the server)
    client.start();

    // Register commands
    registerCommands(context);

    // Register providers
    registerProviders(context);

    // Status bar item
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = "$(symbol-method) CURSED";
    statusBarItem.tooltip = "CURSED Language Active";
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    console.log('CURSED Language Server client started');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function registerCommands(context: vscode.ExtensionContext) {
    // Compile command
    const compileCommand = vscode.commands.registerCommand('cursed.compile', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No active CURSED file to compile');
            return;
        }

        const terminal = vscode.window.createTerminal('CURSED Compiler');
        const filePath = activeEditor.document.uri.fsPath;
        terminal.sendText(`cursed "${filePath}"`);
        terminal.show();
    });

    // Format command
    const formatCommand = vscode.commands.registerCommand('cursed.format', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No active CURSED file to format');
            return;
        }

        await vscode.commands.executeCommand('editor.action.formatDocument');
    });

    // Lint command
    const lintCommand = vscode.commands.registerCommand('cursed.lint', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No active CURSED file to lint');
            return;
        }

        const terminal = vscode.window.createTerminal('CURSED Linter');
        const filePath = activeEditor.document.uri.fsPath;
        terminal.sendText(`cursed-lint "${filePath}"`);
        terminal.show();
    });

    // Restart LSP command
    const restartLspCommand = vscode.commands.registerCommand('cursed.restart-lsp', async () => {
        if (client) {
            await client.stop();
            client.start();
            vscode.window.showInformationMessage('CURSED Language Server restarted');
        }
    });

    context.subscriptions.push(compileCommand, formatCommand, lintCommand, restartLspCommand);
}

function registerProviders(context: vscode.ExtensionContext) {
    // Code completion provider for enhanced snippets
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'cursed',
        {
            provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
                const linePrefix = document.lineAt(position).text.substr(0, position.character);
                
                // Custom completion items
                const items: vscode.CompletionItem[] = [];

                // Function template
                if (linePrefix.endsWith('slay ')) {
                    const funcItem = new vscode.CompletionItem('function template', vscode.CompletionItemKind.Snippet);
                    funcItem.insertText = new vscode.SnippetString('${1:function_name}(${2:params}) {\n\t$0\n}');
                    funcItem.documentation = 'Create a new CURSED function';
                    items.push(funcItem);
                }

                // Struct template
                if (linePrefix.endsWith('squad ')) {
                    const structItem = new vscode.CompletionItem('struct template', vscode.CompletionItemKind.Snippet);
                    structItem.insertText = new vscode.SnippetString('${1:StructName} {\n\tspill ${2:field_name} ${3:field_type}\n\t$0\n}');
                    structItem.documentation = 'Create a new CURSED struct';
                    items.push(structItem);
                }

                // Interface template
                if (linePrefix.endsWith('collab ')) {
                    const interfaceItem = new vscode.CompletionItem('interface template', vscode.CompletionItemKind.Snippet);
                    interfaceItem.insertText = new vscode.SnippetString('${1:InterfaceName} {\n\tslay ${2:method_name}(${3:params}) ${4:return_type}\n\t$0\n}');
                    interfaceItem.documentation = 'Create a new CURSED interface';
                    items.push(interfaceItem);
                }

                return items;
            }
        },
        ' ' // Trigger on space
    );

    // Hover provider for additional information
    const hoverProvider = vscode.languages.registerHoverProvider('cursed', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);

            // Enhanced hover information
            const genZKeywords: { [key: string]: string } = {
                'slay': 'Function definition keyword - defines a function that slays (executes)',
                'sus': 'Variable declaration keyword - creates a suspicious (mutable) variable',
                'facts': 'Conditional keyword - checks if something is facts (true)',
                'lowkey': 'Else keyword - used when something is lowkey false',
                'bestie': 'Loop keyword - iterates with your bestie (loop variable)',
                'squad': 'Struct keyword - defines a squad (data structure)',
                'collab': 'Interface keyword - defines a collab (interface)',
                'vibez': 'I/O module - handles all the vibez (input/output operations)',
                'based': 'Boolean true - when something is based (true)',
                'cringe': 'Boolean false - when something is cringe (false)',
                'normie': 'Integer type - for normie (normal integer) values',
                'tea': 'String type - for spilling the tea (string data)',
                'lit': 'Boolean type - for lit (boolean) values',
                'drip': 'Float type - for drip (floating point) values'
            };

            if (genZKeywords[word]) {
                const hoverText = new vscode.MarkdownString();
                hoverText.appendCodeblock(word, 'cursed');
                hoverText.appendMarkdown(`**${genZKeywords[word]}**`);
                hoverText.appendMarkdown('\n\n*CURSED Language - Gen Z Programming Syntax*');
                return new vscode.Hover(hoverText);
            }

            return null;
        }
    });

    context.subscriptions.push(completionProvider, hoverProvider);
}

// Status monitoring
vscode.workspace.onDidChangeConfiguration((event) => {
    if (event.affectsConfiguration('cursed')) {
        vscode.window.showInformationMessage(
            'CURSED configuration changed. Restart the LSP for changes to take effect.',
            'Restart LSP'
        ).then((selection) => {
            if (selection === 'Restart LSP') {
                vscode.commands.executeCommand('cursed.restart-lsp');
            }
        });
    }
});
