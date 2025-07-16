import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  console.log('CURSED Language Support is now active!');

  // Start the language server
  startLanguageServer(context);

  // Register commands
  registerCommands(context);

  // Register providers
  registerProviders(context);
}

function startLanguageServer(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('cursed.languageServer');
  
  if (!config.get('enabled', true)) {
    return;
  }

  const serverPath = config.get('path', 'cursed');
  const serverArgs = config.get('arguments', ['lsp']);

  const serverOptions: ServerOptions = {
    run: {
      command: serverPath,
      args: serverArgs,
      transport: TransportKind.stdio
    },
    debug: {
      command: serverPath,
      args: [...serverArgs, '--debug'],
      transport: TransportKind.stdio
    }
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'cursed' },
      { scheme: 'untitled', language: 'cursed' }
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.csd')
    },
    initializationOptions: {
      vibeMode: vscode.workspace.getConfiguration('cursed').get('vibe.mode', 'standard')
    }
  };

  client = new LanguageClient(
    'cursedLanguageServer',
    'CURSED Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client and server
  client.start().then(() => {
    console.log('CURSED Language Server started successfully');
    
    // Show status bar item
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.text = "$(check) CURSED LSP";
    statusBar.tooltip = "CURSED Language Server is running";
    statusBar.show();
    
    context.subscriptions.push(statusBar);
  }).catch(error => {
    console.error('Failed to start CURSED Language Server:', error);
    vscode.window.showErrorMessage(`Failed to start CURSED Language Server: ${error.message}`);
  });

  context.subscriptions.push(client);
}

function registerCommands(context: vscode.ExtensionContext) {
  // Compile command
  const compileCommand = vscode.commands.registerCommand('cursed.compile', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
      vscode.window.showErrorMessage('No CURSED file is currently open');
      return;
    }

    const filePath = editor.document.fileName;
    const terminal = vscode.window.createTerminal('CURSED Compile');
    terminal.sendText(`cursed -- compile "${filePath}"`);
    terminal.show();
  });

  // Run command
  const runCommand = vscode.commands.registerCommand('cursed.run', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
      vscode.window.showErrorMessage('No CURSED file is currently open');
      return;
    }

    const filePath = editor.document.fileName;
    const terminal = vscode.window.createTerminal('CURSED Run');
    terminal.sendText(`cursed "${filePath}"`);
    terminal.show();
  });

  // Format command
  const formatCommand = vscode.commands.registerCommand('cursed.format', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
      return;
    }

    try {
      await vscode.commands.executeCommand('editor.action.formatDocument');
    } catch (error) {
      vscode.window.showErrorMessage(`Failed to format document: ${error}`);
    }
  });

  // Show AST command
  const showASTCommand = vscode.commands.registerCommand('cursed.showAST', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
      vscode.window.showErrorMessage('No CURSED file is currently open');
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      'cursedAST',
      'CURSED AST',
      vscode.ViewColumn.Beside,
      {
        enableScripts: true
      }
    );

    panel.webview.html = getASTWebviewContent();
    
    // Request AST from language server
    if (client && client.isRunning()) {
      try {
        const result = await client.sendRequest('cursed/getAST', {
          textDocument: { uri: editor.document.uri.toString() }
        });
        
        panel.webview.postMessage({
          command: 'updateAST',
          ast: result
        });
      } catch (error) {
        console.error('Failed to get AST:', error);
      }
    }
  });

  // Toggle vibe mode
  const toggleVibeCommand = vscode.commands.registerCommand('cursed.toggleVibe', async () => {
    const config = vscode.workspace.getConfiguration('cursed');
    const currentMode = config.get('vibe.mode', 'standard');
    
    const modes = ['standard', 'extra', 'maximum'];
    const currentIndex = modes.indexOf(currentMode);
    const nextMode = modes[(currentIndex + 1) % modes.length];
    
    await config.update('vibe.mode', nextMode, vscode.ConfigurationTarget.Global);
    vscode.window.showInformationMessage(`CURSED vibe mode set to: ${nextMode}`);
  });

  context.subscriptions.push(
    compileCommand,
    runCommand,
    formatCommand,
    showASTCommand,
    toggleVibeCommand
  );
}

function registerProviders(context: vscode.ExtensionContext) {
  // Task provider for CURSED tasks
  const taskProvider = vscode.tasks.registerTaskProvider('cursed', {
    provideTasks: () => {
      const tasks: vscode.Task[] = [];
      
      // Compile task
      const compileTask = new vscode.Task(
        { type: 'cursed', task: 'compile' },
        vscode.TaskScope.Workspace,
        'compile',
        'cursed',
        new vscode.ShellExecution('cursed', ['--', 'compile', '${file}'])
      );
      compileTask.group = vscode.TaskGroup.Build;
      compileTask.problemMatchers = ['$cursed'];
      tasks.push(compileTask);

      // Run task
      const runTask = new vscode.Task(
        { type: 'cursed', task: 'run' },
        vscode.TaskScope.Workspace,
        'run',
        'cursed',
        new vscode.ShellExecution('cursed', ['${file}'])
      );
      runTask.group = vscode.TaskGroup.Test;
      tasks.push(runTask);

      return tasks;
    },
    resolveTask: (task: vscode.Task) => {
      return task;
    }
  });

  context.subscriptions.push(taskProvider);
}

function getASTWebviewContent(): string {
  return `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED AST</title>
    <style>
        body {
            font-family: 'Courier New', monospace;
            padding: 20px;
            background-color: var(--vscode-editor-background);
            color: var(--vscode-editor-foreground);
        }
        .ast-node {
            margin: 5px 0;
            padding: 5px;
            border-left: 2px solid var(--vscode-editor-foreground);
            margin-left: 20px;
        }
        .ast-type {
            color: var(--vscode-symbolIcon-functionForeground);
            font-weight: bold;
        }
        .ast-value {
            color: var(--vscode-symbolIcon-stringForeground);
        }
        .collapsible {
            cursor: pointer;
            user-select: none;
        }
        .collapsible:before {
            content: '▼ ';
        }
        .collapsed:before {
            content: '▶ ';
        }
        .content {
            overflow: hidden;
        }
        .content.collapsed {
            display: none;
        }
    </style>
</head>
<body>
    <h1>CURSED Abstract Syntax Tree</h1>
    <div id="ast-content">
        <p>Loading AST...</p>
    </div>

    <script>
        const vscode = acquireVsCodeApi();
        
        window.addEventListener('message', event => {
            const message = event.data;
            
            switch (message.command) {
                case 'updateAST':
                    displayAST(message.ast);
                    break;
            }
        });
        
        function displayAST(ast) {
            const content = document.getElementById('ast-content');
            content.innerHTML = renderASTNode(ast, 0);
            
            // Add collapsible functionality
            document.querySelectorAll('.collapsible').forEach(element => {
                element.addEventListener('click', function() {
                    this.classList.toggle('collapsed');
                    const content = this.nextElementSibling;
                    content.classList.toggle('collapsed');
                });
            });
        }
        
        function renderASTNode(node, depth) {
            if (typeof node === 'string' || typeof node === 'number' || typeof node === 'boolean') {
                return \`<span class="ast-value">\${node}</span>\`;
            }
            
            if (Array.isArray(node)) {
                let html = '<div class="collapsible">Array</div><div class="content">';
                node.forEach((item, index) => {
                    html += \`<div class="ast-node">[\${index}]: \${renderASTNode(item, depth + 1)}</div>\`;
                });
                html += '</div>';
                return html;
            }
            
            if (typeof node === 'object' && node !== null) {
                let html = \`<div class="collapsible"><span class="ast-type">\${node.type || 'Object'}</span></div><div class="content">\`;
                
                Object.entries(node).forEach(([key, value]) => {
                    if (key !== 'type') {
                        html += \`<div class="ast-node"><strong>\${key}:</strong> \${renderASTNode(value, depth + 1)}</div>\`;
                    }
                });
                
                html += '</div>';
                return html;
            }
            
            return String(node);
        }
    </script>
</body>
</html>`;
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
