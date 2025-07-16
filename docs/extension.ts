import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    // Get the CURSED LSP server path from configuration
    const config = vscode.workspace.getConfiguration('cursed');
    const serverPath = config.get<string>('lsp.path', 'cursed-lsp');

    // Server options for the CURSED language server
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: [],
        transport: TransportKind.stdio
    };

    // Client options for the language client
    const clientOptions: LanguageClientOptions = {
        // Register the server for CURSED documents
        documentSelector: [
            {
                scheme: 'file',
                language: 'cursed'
            }
        ],
        synchronize: {
            // Notify the server about file changes to '.csd' files contained in the workspace
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.csd')
        }
    };

    // Create the language client and start the client
    client = new LanguageClient(
        'cursedLanguageServer',
        'CURSED Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client. This will also launch the server
    client.start();

    // Register additional commands
    const commands = [
        vscode.commands.registerCommand('cursed.restart', () => {
            client.restart();
        }),
        vscode.commands.registerCommand('cursed.status', () => {
            const status = client.isRunning() ? 'Running' : 'Stopped';
            vscode.window.showInformationMessage(`CURSED Language Server: ${status}`);
        })
    ];

    context.subscriptions.push(...commands);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
