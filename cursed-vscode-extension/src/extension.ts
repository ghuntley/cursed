import * as vscode from 'vscode';
import * as path from 'path';
import { CursedLanguageClient } from './languageClient';
import { CursedDebugAdapterDescriptorFactory } from './debugAdapter';
import { CursedTaskProvider } from './taskProvider';

export function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Language Extension is now active!');

    // Initialize Language Server Client
    const languageClient = new CursedLanguageClient(context);
    context.subscriptions.push(languageClient);

    // Register Debug Adapter
    const debugAdapterFactory = new CursedDebugAdapterDescriptorFactory();
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory('cursed', debugAdapterFactory)
    );

    // Register Task Provider
    const taskProvider = new CursedTaskProvider();
    context.subscriptions.push(
        vscode.tasks.registerTaskProvider('cursed', taskProvider)
    );

    // Register Commands
    registerCommands(context);

    // Register File Watchers and Event Handlers
    registerEventHandlers(context);

    // Show welcome message
    showWelcomeMessage(context);
}

function registerCommands(context: vscode.ExtensionContext) {
    // Run CURSED Program
    const runCommand = vscode.commands.registerCommand('cursed.run', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently open');
            return;
        }

        const filePath = activeEditor.document.fileName;
        await runCursedProgram(filePath);
    });
    context.subscriptions.push(runCommand);

    // Build CURSED Program
    const buildCommand = vscode.commands.registerCommand('cursed.build', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently open');
            return;
        }

        const filePath = activeEditor.document.fileName;
        await buildCursedProgram(filePath);
    });
    context.subscriptions.push(buildCommand);

    // Format CURSED Code
    const formatCommand = vscode.commands.registerCommand('cursed.format', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently open');
            return;
        }

        await vscode.commands.executeCommand('editor.action.formatDocument');
    });
    context.subscriptions.push(formatCommand);

    // Lint CURSED Code
    const lintCommand = vscode.commands.registerCommand('cursed.lint', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently open');
            return;
        }

        const filePath = activeEditor.document.fileName;
        await lintCursedCode(filePath);
    });
    context.subscriptions.push(lintCommand);

    // New CURSED Project
    const newProjectCommand = vscode.commands.registerCommand('cursed.newProject', async () => {
        await createNewCursedProject();
    });
    context.subscriptions.push(newProjectCommand);

    // Show Documentation
    const docsCommand = vscode.commands.registerCommand('cursed.showDocumentation', async () => {
        const panel = vscode.window.createWebviewPanel(
            'cursedDocs',
            'CURSED Documentation',
            vscode.ViewColumn.Two,
            {
                enableScripts: true,
                retainContextWhenHidden: true
            }
        );

        panel.webview.html = getDocumentationContent();
    });
    context.subscriptions.push(docsCommand);
}

function registerEventHandlers(context: vscode.ExtensionContext) {
    // Format on save
    const formatOnSave = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (document.languageId === 'cursed') {
            const config = vscode.workspace.getConfiguration('cursed');
            if (config.get('format.onSave')) {
                await vscode.commands.executeCommand('cursed.format');
            }
            if (config.get('lint.onSave')) {
                await lintCursedCode(document.fileName);
            }
        }
    });
    context.subscriptions.push(formatOnSave);

    // Status bar integration
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBarItem.text = "$(play) CURSED";
    statusBarItem.command = 'cursed.run';
    statusBarItem.tooltip = 'Run CURSED Program';
    
    const updateStatusBar = () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'cursed') {
            statusBarItem.show();
        } else {
            statusBarItem.hide();
        }
    };

    updateStatusBar();
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(updateStatusBar),
        statusBarItem
    );
}

async function runCursedProgram(filePath: string) {
    const config = vscode.workspace.getConfiguration('cursed');
    const compilerPath = config.get('compiler.path', 'cursed');
    
    const terminal = vscode.window.createTerminal('CURSED Run');
    terminal.sendText(`${compilerPath} "${filePath}"`);
    terminal.show();
}

async function buildCursedProgram(filePath: string) {
    const config = vscode.workspace.getConfiguration('cursed');
    const compilerPath = config.get('compiler.path', 'cursed');
    const optimization = config.get('build.optimization', 'debug');
    
    const outputChannel = vscode.window.createOutputChannel('CURSED Build');
    outputChannel.show();
    
    const { spawn } = require('child_process');
    const args = ['--compile', filePath];
    
    if (optimization !== 'debug') {
        args.push(`--optimize=${optimization}`);
    }
    
    const process = spawn(compilerPath, args);
    
    process.stdout.on('data', (data: Buffer) => {
        outputChannel.append(data.toString());
    });
    
    process.stderr.on('data', (data: Buffer) => {
        outputChannel.append(data.toString());
    });
    
    process.on('close', (code: number) => {
        if (code === 0) {
            vscode.window.showInformationMessage('CURSED build completed successfully');
        } else {
            vscode.window.showErrorMessage(`CURSED build failed with exit code ${code}`);
        }
    });
}

async function lintCursedCode(filePath: string) {
    const config = vscode.workspace.getConfiguration('cursed');
    const compilerPath = config.get('compiler.path', 'cursed');
    
    const outputChannel = vscode.window.createOutputChannel('CURSED Lint');
    
    const { spawn } = require('child_process');
    const process = spawn(compilerPath, ['--lint', filePath]);
    
    let output = '';
    
    process.stdout.on('data', (data: Buffer) => {
        output += data.toString();
    });
    
    process.stderr.on('data', (data: Buffer) => {
        output += data.toString();
    });
    
    process.on('close', (code: number) => {
        outputChannel.clear();
        outputChannel.append(output);
        
        if (code === 0) {
            vscode.window.showInformationMessage('CURSED lint passed');
        } else {
            outputChannel.show();
            vscode.window.showWarningMessage('CURSED lint found issues');
        }
    });
}

async function createNewCursedProject() {
    const folderUri = await vscode.window.showOpenDialog({
        canSelectFolders: true,
        canSelectFiles: false,
        canSelectMany: false,
        openLabel: 'Select Project Folder'
    });

    if (!folderUri || folderUri.length === 0) {
        return;
    }

    const projectPath = folderUri[0].fsPath;
    const projectName = await vscode.window.showInputBox({
        prompt: 'Enter project name',
        placeHolder: 'my-cursed-project'
    });

    if (!projectName) {
        return;
    }

    const fullProjectPath = path.join(projectPath, projectName);
    
    try {
        const fs = require('fs');
        
        // Create project directory
        fs.mkdirSync(fullProjectPath, { recursive: true });
        
        // Create main.💀
        const mainContent = `// CURSED Project: ${projectName}
yeet "vibez"

slay main() drip {
    vibez.spill("Hello, CURSED!")
    damn 0
}
`;
        fs.writeFileSync(path.join(fullProjectPath, 'main.💀'), mainContent);
        
        // Create CursedPackage.toml
        const packageContent = `[package]
name = "${projectName}"
version = "0.1.0"
description = "A new CURSED project"
authors = ["Your Name <you@example.com>"]

[dependencies]
# Add dependencies here

[build]
optimization = "debug"
target = "native"
`;
        fs.writeFileSync(path.join(fullProjectPath, 'CursedPackage.toml'), packageContent);
        
        // Create README.md
        const readmeContent = `# ${projectName}

A CURSED programming language project.

## Building

\`\`\`bash
cursed build
\`\`\`

## Running

\`\`\`bash
cursed run main.💀
\`\`\`
`;
        fs.writeFileSync(path.join(fullProjectPath, 'README.md'), readmeContent);
        
        // Open the new project
        const uri = vscode.Uri.file(fullProjectPath);
        vscode.commands.executeCommand('vscode.openFolder', uri, true);
        
        vscode.window.showInformationMessage(`Created new CURSED project: ${projectName}`);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to create project: ${error}`);
    }
}

function getDocumentationContent(): string {
    return `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Documentation</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            margin: 20px;
            color: var(--vscode-foreground);
            background-color: var(--vscode-editor-background);
        }
        h1, h2, h3 { color: var(--vscode-textLink-foreground); }
        code {
            background-color: var(--vscode-textCodeBlock-background);
            padding: 2px 4px;
            border-radius: 3px;
            font-family: 'Monaco', 'Cascadia Code', monospace;
        }
        pre {
            background-color: var(--vscode-textCodeBlock-background);
            padding: 10px;
            border-radius: 5px;
            overflow-x: auto;
        }
        .keyword { color: #569CD6; }
        .string { color: #CE9178; }
        .comment { color: #6A9955; }
    </style>
</head>
<body>
    <h1>CURSED Language Documentation</h1>
    
    <h2>Quick Start</h2>
    <p>CURSED is a modern systems programming language with expressive syntax.</p>
    
    <h3>Basic Syntax</h3>
    <pre><code><span class="comment">// Variable declaration</span>
<span class="keyword">sus</span> name <span class="keyword">tea</span> = <span class="string">"CURSED"</span>
<span class="keyword">sus</span> count <span class="keyword">drip</span> = 42

<span class="comment">// Function definition</span>
<span class="keyword">slay</span> greet(name <span class="keyword">tea</span>) <span class="keyword">tea</span> {
    <span class="keyword">damn</span> <span class="string">"Hello, "</span> + name
}

<span class="comment">// Main function</span>
<span class="keyword">slay</span> main() <span class="keyword">drip</span> {
    vibez.spill(greet(<span class="string">"World"</span>))
    <span class="keyword">damn</span> 0
}</code></pre>

    <h3>Control Flow</h3>
    <pre><code><span class="comment">// If statement</span>
<span class="keyword">ready</span> (condition) {
    vibez.spill(<span class="string">"True branch"</span>)
} <span class="keyword">otherwise</span> {
    vibez.spill(<span class="string">"False branch"</span>)
}

<span class="comment">// While loop</span>
<span class="keyword">bestie</span> (count > 0) {
    vibez.spill(count)
    count = count - 1
}</code></pre>

    <h3>Data Structures</h3>
    <pre><code><span class="comment">// Struct definition</span>
<span class="keyword">squad</span> Person {
    <span class="keyword">spill</span> name <span class="keyword">tea</span>
    <span class="keyword">spill</span> age <span class="keyword">drip</span>
}

<span class="comment">// Interface definition</span>
<span class="keyword">collab</span> Greeter {
    <span class="keyword">slay</span> greet() <span class="keyword">tea</span>
}</code></pre>

    <h3>Standard Library</h3>
    <ul>
        <li><strong>vibez</strong> - I/O operations</li>
        <li><strong>mathz</strong> - Mathematical functions</li>
        <li><strong>stringz</strong> - String manipulation</li>
        <li><strong>arrayz</strong> - Array operations</li>
        <li><strong>cryptz</strong> - Cryptographic functions</li>
        <li><strong>filez</strong> - File I/O</li>
        <li><strong>httpz</strong> - HTTP client/server</li>
        <li><strong>timez</strong> - Time and date operations</li>
    </ul>

    <h3>Error Handling</h3>
    <pre><code><span class="keyword">sus</span> result = risky_operation() <span class="keyword">fam</span> {
    <span class="keyword">when</span> ErrorType -> {
        vibez.spill(<span class="string">"Handle error"</span>)
    }
}</code></pre>

    <h2>Resources</h2>
    <ul>
        <li><a href="https://cursed-lang.org">Official Website</a></li>
        <li><a href="https://github.com/ghuntley/cursed">GitHub Repository</a></li>
        <li><a href="https://playground.cursed-lang.org">Online Playground</a></li>
    </ul>
</body>
</html>
    `;
}

function showWelcomeMessage(context: vscode.ExtensionContext) {
    const hasShownWelcome = context.globalState.get('cursed.hasShownWelcome', false);
    
    if (!hasShownWelcome) {
        vscode.window.showInformationMessage(
            'Welcome to CURSED! 🔥',
            'Show Documentation',
            'Create New Project'
        ).then(selection => {
            if (selection === 'Show Documentation') {
                vscode.commands.executeCommand('cursed.showDocumentation');
            } else if (selection === 'Create New Project') {
                vscode.commands.executeCommand('cursed.newProject');
            }
        });
        
        context.globalState.update('cursed.hasShownWelcome', true);
    }
}

export function deactivate() {
    console.log('CURSED Language Extension is now deactivated');
}
