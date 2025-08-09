import * as vscode from 'vscode';
import * as path from 'path';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Language Support is now active!');

    // Get configuration
    const config = vscode.workspace.getConfiguration('cursed');
    const lspEnabled = config.get<boolean>('lsp.enabled', true);
    
    if (!lspEnabled) {
        console.log('CURSED LSP is disabled');
        return;
    }

    // Get LSP server path from configuration
    let serverPath = config.get<string>('lsp.path', 'cursed-lsp');
    
    // If not absolute path, try to find cursed-lsp in workspace
    if (!path.isAbsolute(serverPath)) {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (workspaceFolders && workspaceFolders.length > 0) {
            const workspaceRoot = workspaceFolders[0].uri.fsPath;
            const workspaceLspPath = path.join(workspaceRoot, 'zig-out', 'bin', 'cursed-lsp');
            
            // Check if workspace LSP exists
            try {
                require('fs').accessSync(workspaceLspPath);
                serverPath = workspaceLspPath;
                console.log(`Using workspace CURSED LSP: ${serverPath}`);
            } catch (error) {
                console.log(`Workspace CURSED LSP not found, using system path: ${serverPath}`);
            }
        }
    }

    // Server options - start the LSP server
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: [],
        transport: TransportKind.stdio,
        options: {
            cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
        }
    };

    // Client options - configure the language client
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'cursed' }
        ],
        synchronize: {
            // Notify the server about file changes to .csd files
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.csd')
        },
        initializationOptions: {
            workspaceRoot: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
        },
        outputChannelName: 'CURSED Language Server',
        traceOutputChannel: vscode.window.createOutputChannel('CURSED LSP Trace')
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
        console.log('CURSED Language Server started successfully');
        
        // Show status message
        vscode.window.showInformationMessage('CURSED Language Server is ready!');
        
        // Register additional commands
        registerCommands(context);
        
    }).catch((error) => {
        console.error('Failed to start CURSED Language Server:', error);
        vscode.window.showErrorMessage(`Failed to start CURSED Language Server: ${error.message}`);
    });

    // Push client to context for proper disposal
    context.subscriptions.push(client);
}

function registerCommands(context: vscode.ExtensionContext) {
    // Register restart command
    const restartCommand = vscode.commands.registerCommand('cursed.restartLanguageServer', async () => {
        if (client) {
            await client.stop();
            await client.start();
            vscode.window.showInformationMessage('CURSED Language Server restarted');
        }
    });

    // Register show output command
    const showOutputCommand = vscode.commands.registerCommand('cursed.showLanguageServerOutput', () => {
        client.outputChannel.show();
    });

    // Register format document command (if not provided by LSP)
    const formatCommand = vscode.commands.registerCommand('cursed.formatDocument', () => {
        vscode.commands.executeCommand('editor.action.formatDocument');
    });

    context.subscriptions.push(restartCommand, showOutputCommand, formatCommand);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    console.log('Stopping CURSED Language Server...');
    return client.stop();
}
