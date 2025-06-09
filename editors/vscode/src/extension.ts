import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Language Support extension is now active!');

    // Start the language server
    startLanguageServer(context);

    // Register custom commands
    registerCommands(context);

    // Register event handlers
    registerEventHandlers(context);
}

function startLanguageServer(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('cursed.languageServer');
    
    if (!config.get('enabled', true)) {
        console.log('CURSED Language Server is disabled');
        return;
    }

    const command = config.get('command', 'cursed-lsp');
    let args = config.get('args', []) as string[];
    
    // Add debug flag if enabled
    if (config.get('debug', false)) {
        args = ['--debug', ...args];
    }

    // Server options
    const serverOptions: ServerOptions = {
        command: command,
        args: args,
        transport: TransportKind.stdio
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'cursed' },
            { scheme: 'untitled', language: 'cursed' }
        ],
        synchronize: {
            configurationSection: 'cursed',
            fileEvents: [
                vscode.workspace.createFileSystemWatcher('**/*.csd'),
                vscode.workspace.createFileSystemWatcher('**/CursedPackage.toml'),
                vscode.workspace.createFileSystemWatcher('**/CursedBuild.toml')
            ]
        },
        initializationOptions: {
            settings: vscode.workspace.getConfiguration('cursed')
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
        console.log('CURSED Language Server started successfully');
        
        // Register for configuration changes
        vscode.workspace.onDidChangeConfiguration(event => {
            if (event.affectsConfiguration('cursed')) {
                client.sendNotification('workspace/didChangeConfiguration', {
                    settings: vscode.workspace.getConfiguration('cursed')
                });
            }
        });
    }).catch(error => {
        console.error('Failed to start CURSED Language Server:', error);
        vscode.window.showErrorMessage(
            'Failed to start CURSED Language Server. Make sure cursed-lsp is installed and available in PATH.'
        );
    });

    context.subscriptions.push(client);
}

function registerCommands(context: vscode.ExtensionContext) {
    // Restart Language Server
    const restartCommand = vscode.commands.registerCommand('cursed.restartLanguageServer', async () => {
        if (client) {
            await client.stop();
            await client.start();
            vscode.window.showInformationMessage('CURSED Language Server restarted');
        }
    });

    // Show AST Node
    const showAstCommand = vscode.commands.registerCommand('cursed.showAstNode', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const position = editor.selection.active;
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            position: { line: position.line, character: position.character },
            includeChildren: true,
            maxDepth: 3
        };

        try {
            const result = await client.sendRequest('cursed/getAstNode', params);
            const panel = vscode.window.createWebviewPanel(
                'cursedAst',
                'CURSED AST Node',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = getAstWebviewContent(result);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to get AST node: ${error}`);
        }
    });

    // Show Type Info
    const showTypeInfoCommand = vscode.commands.registerCommand('cursed.showTypeInfo', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const position = editor.selection.active;
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            position: { line: position.line, character: position.character },
            includeHierarchy: true
        };

        try {
            const result = await client.sendRequest('cursed/getTypeInfo', params);
            const panel = vscode.window.createWebviewPanel(
                'cursedTypeInfo',
                'CURSED Type Information',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = getTypeInfoWebviewContent(result);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to get type information: ${error}`);
        }
    });

    // Format Document
    const formatDocumentCommand = vscode.commands.registerCommand('cursed.formatDocument', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const formatConfig = vscode.workspace.getConfiguration('cursed.format');
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            options: {
                indentSize: formatConfig.get('indentSize', 4),
                useTabs: formatConfig.get('useTabs', false),
                lineWidth: formatConfig.get('lineWidth', 120),
                braceStyle: formatConfig.get('braceStyle', 'same-line'),
                spaceAroundOperators: true,
                spaceAfterComma: true,
                maxEmptyLines: 2,
                enforceCursedStyle: true
            }
        };

        try {
            const result = await client.sendRequest('cursed/formatDocument', params);
            if (result.formatted_content) {
                const edit = new vscode.WorkspaceEdit();
                const range = new vscode.Range(
                    editor.document.positionAt(0),
                    editor.document.positionAt(editor.document.getText().length)
                );
                edit.replace(editor.document.uri, range, result.formatted_content);
                await vscode.workspace.applyEdit(edit);
                vscode.window.showInformationMessage('Document formatted successfully');
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to format document: ${error}`);
        }
    });

    // Run Linter
    const runLinterCommand = vscode.commands.registerCommand('cursed.runLinter', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const lintConfig = vscode.workspace.getConfiguration('cursed.lint');
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            options: {
                checkStyle: lintConfig.get('checkStyle', true),
                checkPerformance: lintConfig.get('checkPerformance', true),
                checkSecurity: lintConfig.get('checkSecurity', true),
                checkBestPractices: true,
                severityLevel: 'warning'
            }
        };

        try {
            const result = await client.sendRequest('cursed/runLinter', params);
            const panel = vscode.window.createWebviewPanel(
                'cursedLinter',
                'CURSED Linter Results',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = getLinterWebviewContent(result);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to run linter: ${error}`);
        }
    });

    // Show Goroutine Info
    const showGoroutineInfoCommand = vscode.commands.registerCommand('cursed.showGoroutineInfo', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const position = editor.selection.active;
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            position: { line: position.line, character: position.character }
        };

        try {
            const result = await client.sendRequest('cursed/getGoroutineInfo', params);
            const panel = vscode.window.createWebviewPanel(
                'cursedGoroutines',
                'CURSED Goroutine Information',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = getGoroutineWebviewContent(result);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to get goroutine information: ${error}`);
        }
    });

    // Show Channel Info
    const showChannelInfoCommand = vscode.commands.registerCommand('cursed.showChannelInfo', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            return;
        }

        const position = editor.selection.active;
        const params = {
            textDocument: { uri: editor.document.uri.toString() },
            position: { line: position.line, character: position.character }
        };

        try {
            const result = await client.sendRequest('cursed/getChannelInfo', params);
            const panel = vscode.window.createWebviewPanel(
                'cursedChannels',
                'CURSED Channel Information',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = getChannelWebviewContent(result);
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to get channel information: ${error}`);
        }
    });

    // Register all commands
    context.subscriptions.push(
        restartCommand,
        showAstCommand,
        showTypeInfoCommand,
        formatDocumentCommand,
        runLinterCommand,
        showGoroutineInfoCommand,
        showChannelInfoCommand
    );
}

function registerEventHandlers(context: vscode.ExtensionContext) {
    // Format on save
    const formatOnSave = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (document.languageId === 'cursed') {
            const config = vscode.workspace.getConfiguration('cursed.format');
            if (config.get('enable', true)) {
                await vscode.commands.executeCommand('cursed.formatDocument');
            }
        }
    });

    context.subscriptions.push(formatOnSave);
}

// Webview content generators
function getAstWebviewContent(astNode: any): string {
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED AST Node</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                .node { margin: 10px 0; padding: 10px; border: 1px solid #ccc; }
                .node-type { font-weight: bold; color: #0066cc; }
                .node-range { color: #666; font-size: 0.9em; }
                .node-text { background: #f5f5f5; padding: 5px; margin: 5px 0; }
                .children { margin-left: 20px; }
            </style>
        </head>
        <body>
            <h1>AST Node Information</h1>
            ${renderAstNode(astNode)}
        </body>
        </html>
    `;
}

function renderAstNode(node: any): string {
    let html = `
        <div class="node">
            <div class="node-type">${node.node_type}</div>
            <div class="node-range">Range: ${node.range.start.line}:${node.range.start.character} - ${node.range.end.line}:${node.range.end.character}</div>
            <div class="node-text">${escapeHtml(node.text)}</div>
    `;
    
    if (node.children && node.children.length > 0) {
        html += '<div class="children">';
        for (const child of node.children) {
            html += renderAstNode(child);
        }
        html += '</div>';
    }
    
    html += '</div>';
    return html;
}

function getTypeInfoWebviewContent(typeInfo: any): string {
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Type Information</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                .type-info { margin: 10px 0; }
                .type-name { font-weight: bold; font-size: 1.2em; color: #0066cc; }
                .section { margin: 15px 0; }
                .section-title { font-weight: bold; color: #333; }
                .method, .field { margin: 5px 0; padding: 5px; background: #f9f9f9; }
                .nullable { color: #ff6600; }
            </style>
        </head>
        <body>
            <h1>Type Information</h1>
            <div class="type-info">
                <div class="type-name">${typeInfo.type_name} ${typeInfo.nullable ? '<span class="nullable">(nullable)</span>' : ''}</div>
                ${typeInfo.base_type ? `<div>Base Type: ${typeInfo.base_type}</div>` : ''}
                ${typeInfo.documentation ? `<div class="section"><div class="section-title">Documentation:</div>${typeInfo.documentation}</div>` : ''}
                ${typeInfo.methods && typeInfo.methods.length > 0 ? 
                    `<div class="section">
                        <div class="section-title">Methods:</div>
                        ${typeInfo.methods.map((m: any) => `<div class="method">${m.name}(${m.parameters.map((p: any) => `${p.name}: ${p.param_type}`).join(', ')}) -> ${m.return_type || 'void'}</div>`).join('')}
                    </div>` : ''}
                ${typeInfo.fields && typeInfo.fields.length > 0 ?
                    `<div class="section">
                        <div class="section-title">Fields:</div>
                        ${typeInfo.fields.map((f: any) => `<div class="field">${f.name}: ${f.field_type}</div>`).join('')}
                    </div>` : ''}
            </div>
        </body>
        </html>
    `;
}

function getLinterWebviewContent(linterResult: any): string {
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Linter Results</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                .summary { background: #f0f0f0; padding: 10px; margin-bottom: 20px; }
                .diagnostic { margin: 10px 0; padding: 10px; border-left: 3px solid #ccc; }
                .error { border-left-color: #ff4444; }
                .warning { border-left-color: #ffaa00; }
                .info { border-left-color: #4444ff; }
                .hint { border-left-color: #44ff44; }
                .severity { font-weight: bold; text-transform: uppercase; }
                .location { color: #666; font-size: 0.9em; }
            </style>
        </head>
        <body>
            <h1>Linter Results</h1>
            <div class="summary">
                <div>Errors: ${linterResult.summary.errors_count}</div>
                <div>Warnings: ${linterResult.summary.warnings_count}</div>
                <div>Info: ${linterResult.summary.info_count}</div>
                <div>Hints: ${linterResult.summary.hints_count}</div>
                <div>Execution Time: ${linterResult.execution_time}ms</div>
            </div>
            ${linterResult.diagnostics.map((d: any) => `
                <div class="diagnostic ${d.severity?.toLowerCase() || 'info'}">
                    <div class="severity">${d.severity || 'info'}</div>
                    <div class="location">Line ${d.range.start.line + 1}, Column ${d.range.start.character + 1}</div>
                    <div>${d.message}</div>
                </div>
            `).join('')}
        </body>
        </html>
    `;
}

function getGoroutineWebviewContent(goroutineInfo: any): string {
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Goroutine Information</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                .section { margin: 15px 0; }
                .section-title { font-weight: bold; color: #333; margin-bottom: 10px; }
                .location { color: #666; }
                .warning { color: #ff6600; }
                .channel { background: #f9f9f9; padding: 5px; margin: 5px 0; }
            </style>
        </head>
        <body>
            <h1>Goroutine Information</h1>
            <div class="section">
                <div>Async Context: ${goroutineInfo.is_async_context ? 'Yes' : 'No'}</div>
            </div>
            ${goroutineInfo.spawn_locations.length > 0 ?
                `<div class="section">
                    <div class="section-title">Spawn Locations:</div>
                    ${goroutineInfo.spawn_locations.map((loc: any) => `<div class="location">Line ${loc.range.start.line + 1}</div>`).join('')}
                </div>` : ''}
            ${goroutineInfo.channel_usage.length > 0 ?
                `<div class="section">
                    <div class="section-title">Channel Usage:</div>
                    ${goroutineInfo.channel_usage.map((ch: any) => `
                        <div class="channel">
                            <div>${ch.channel_name}: ${ch.channel_type} ${ch.is_buffered ? `(buffered: ${ch.buffer_size})` : '(unbuffered)'}</div>
                            <div>Sends: ${ch.send_locations.length}, Receives: ${ch.receive_locations.length}</div>
                        </div>
                    `).join('')}
                </div>` : ''}
            ${goroutineInfo.potential_deadlocks.length > 0 ?
                `<div class="section">
                    <div class="section-title">Potential Deadlocks:</div>
                    ${goroutineInfo.potential_deadlocks.map((deadlock: any) => `
                        <div class="warning">⚠️ ${deadlock.message}</div>
                    `).join('')}
                </div>` : ''}
        </body>
        </html>
    `;
}

function getChannelWebviewContent(channelInfo: any): string {
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Channel Information</title>
            <style>
                body { font-family: monospace; padding: 20px; }
                .section { margin: 15px 0; }
                .section-title { font-weight: bold; color: #333; margin-bottom: 10px; }
                .operation { background: #f9f9f9; padding: 5px; margin: 5px 0; }
                .blocking { color: #ff6600; }
            </style>
        </head>
        <body>
            <h1>Channel Information</h1>
            <div class="section">
                <div>Name: ${channelInfo.channel_name}</div>
                <div>Element Type: ${channelInfo.element_type}</div>
                <div>Buffered: ${channelInfo.is_buffered ? `Yes (size: ${channelInfo.buffer_size})` : 'No'}</div>
            </div>
            ${channelInfo.send_operations.length > 0 ?
                `<div class="section">
                    <div class="section-title">Send Operations:</div>
                    ${channelInfo.send_operations.map((op: any) => `
                        <div class="operation">
                            Line ${op.location.range.start.line + 1}: ${op.operation_type} ${op.is_blocking ? '<span class="blocking">(blocking)</span>' : ''}
                        </div>
                    `).join('')}
                </div>` : ''}
            ${channelInfo.receive_operations.length > 0 ?
                `<div class="section">
                    <div class="section-title">Receive Operations:</div>
                    ${channelInfo.receive_operations.map((op: any) => `
                        <div class="operation">
                            Line ${op.location.range.start.line + 1}: ${op.operation_type} ${op.is_blocking ? '<span class="blocking">(blocking)</span>' : ''}
                        </div>
                    `).join('')}
                </div>` : ''}
        </body>
        </html>
    `;
}

function escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
