"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    console.log('CURSED Language Support is now active!');
    // Get configuration
    const config = vscode.workspace.getConfiguration('cursed');
    const lspEnabled = config.get('lsp.enabled', true);
    if (!lspEnabled) {
        console.log('CURSED LSP is disabled');
        return;
    }
    // Get LSP server path from configuration
    let serverPath = config.get('lsp.path', 'cursed-lsp');
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
            }
            catch (error) {
                console.log(`Workspace CURSED LSP not found, using system path: ${serverPath}`);
            }
        }
    }
    // Server options - start the LSP server
    const serverOptions = {
        command: serverPath,
        args: [],
        transport: node_1.TransportKind.stdio,
        options: {
            cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
        }
    };
    // Client options - configure the language client
    const clientOptions = {
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
    client = new node_1.LanguageClient('cursedLanguageServer', 'CURSED Language Server', serverOptions, clientOptions);
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
exports.activate = activate;
function registerCommands(context) {
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
function deactivate() {
    if (!client) {
        return undefined;
    }
    console.log('Stopping CURSED Language Server...');
    return client.stop();
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map