import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

// Extension modules
import { CursedTaskProvider } from './taskProvider';
import { CursedProjectTreeDataProvider } from './projectView';
import { CursedDependencyProvider } from './dependencyView';
import { CursedStatusBar } from './statusBar';
import { CursedTerminalManager } from './terminalManager';
import { CursedOutputChannels } from './outputChannels';

let client: LanguageClient;
let taskProvider: CursedTaskProvider;
let projectProvider: CursedProjectTreeDataProvider;
let dependencyProvider: CursedDependencyProvider;
let statusBar: CursedStatusBar;
let terminalManager: CursedTerminalManager;
let outputChannels: CursedOutputChannels;

export function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Language Support extension is now active!');

    // Initialize output channels
    outputChannels = new CursedOutputChannels();
    context.subscriptions.push(...outputChannels.getChannels());

    // Initialize status bar
    statusBar = new CursedStatusBar();
    context.subscriptions.push(statusBar);

    // Initialize terminal manager
    terminalManager = new CursedTerminalManager(outputChannels);
    context.subscriptions.push(terminalManager);

    // Start the language server
    startLanguageServer(context);

    // Initialize providers
    initializeProviders(context);

    // Register custom commands
    registerCommands(context);

    // Register event handlers
    registerEventHandlers(context);

    // Show welcome message for first-time users
    showWelcomeMessage(context);
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

function initializeProviders(context: vscode.ExtensionContext) {
    // Task provider
    taskProvider = new CursedTaskProvider();
    const taskProviderDisposable = vscode.tasks.registerTaskProvider('cursed', taskProvider);
    context.subscriptions.push(taskProviderDisposable);

    // Project tree provider
    projectProvider = new CursedProjectTreeDataProvider(context);
    const projectTreeView = vscode.window.createTreeView('cursedProjectView', {
        treeDataProvider: projectProvider,
        showCollapseAll: true
    });
    context.subscriptions.push(projectTreeView);

    // Dependency provider
    dependencyProvider = new CursedDependencyProvider(context);
    const dependencyTreeView = vscode.window.createTreeView('cursedDependencies', {
        treeDataProvider: dependencyProvider,
        showCollapseAll: true
    });
    context.subscriptions.push(dependencyTreeView);
}

async function showWelcomeMessage(context: vscode.ExtensionContext) {
    const isFirstTime = !context.globalState.get('cursed.hasShownWelcome', false);
    
    if (isFirstTime) {
        const action = await vscode.window.showInformationMessage(
            'Welcome to CURSED! 🔥 The Gen Z programming language is ready to slay.',
            'Show Getting Started',
            'Create New Project',
            'Don\'t Show Again'
        );

        switch (action) {
            case 'Show Getting Started':
                vscode.commands.executeCommand('vscode.open', vscode.Uri.parse('https://github.com/ghuntley/cursed#getting-started'));
                break;
            case 'Create New Project':
                vscode.commands.executeCommand('cursed.newProject');
                break;
        }

        await context.globalState.update('cursed.hasShownWelcome', true);
    }
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

    // New Project Command
    const newProjectCommand = vscode.commands.registerCommand('cursed.newProject', async () => {
        const result = await vscode.window.showOpenDialog({
            canSelectFiles: false,
            canSelectFolders: true,
            canSelectMany: false,
            openLabel: 'Select Folder for New CURSED Project'
        });

        if (result && result[0]) {
            const projectPath = result[0].fsPath;
            const projectName = await vscode.window.showInputBox({
                prompt: 'Enter project name',
                value: path.basename(projectPath)
            });

            if (projectName) {
                await createNewProject(projectPath, projectName);
                vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath));
            }
        }
    });

    // Build Command
    const buildCommand = vscode.commands.registerCommand('cursed.build', async () => {
        statusBar.updateStatus('Building...');
        outputChannels.clear('build');
        outputChannels.show('build');

        try {
            await terminalManager.runTask('build', [], 'Building CURSED project...');
            statusBar.updateStatus('Build Complete', 3000);
            vscode.window.showInformationMessage('Build completed successfully!');
        } catch (error) {
            statusBar.updateStatus('Build Failed', 5000);
            vscode.window.showErrorMessage(`Build failed: ${error}`);
        }
    });

    // Run Command
    const runCommand = vscode.commands.registerCommand('cursed.run', async () => {
        statusBar.updateStatus('Running...');
        outputChannels.clear('run');
        outputChannels.show('run');

        try {
            await terminalManager.runTask('run', [], 'Running CURSED project...');
            statusBar.updateStatus('Run Complete', 3000);
        } catch (error) {
            statusBar.updateStatus('Run Failed', 5000);
            vscode.window.showErrorMessage(`Run failed: ${error}`);
        }
    });

    // Test Command
    const testCommand = vscode.commands.registerCommand('cursed.test', async () => {
        statusBar.updateStatus('Testing...');
        outputChannels.clear('test');
        outputChannels.show('test');

        try {
            await terminalManager.runTask('test', [], 'Running CURSED tests...');
            statusBar.updateStatus('Tests Complete', 3000);
            vscode.window.showInformationMessage('Tests completed successfully!');
        } catch (error) {
            statusBar.updateStatus('Tests Failed', 5000);
            vscode.window.showErrorMessage(`Tests failed: ${error}`);
        }
    });

    // Clean Command
    const cleanCommand = vscode.commands.registerCommand('cursed.clean', async () => {
        const confirm = await vscode.window.showWarningMessage(
            'This will clean all build artifacts. Continue?',
            'Yes', 'No'
        );

        if (confirm === 'Yes') {
            statusBar.updateStatus('Cleaning...');
            try {
                await terminalManager.runTask('clean', [], 'Cleaning build artifacts...');
                statusBar.updateStatus('Clean Complete', 3000);
                vscode.window.showInformationMessage('Clean completed successfully!');
            } catch (error) {
                statusBar.updateStatus('Clean Failed', 5000);
                vscode.window.showErrorMessage(`Clean failed: ${error}`);
            }
        }
    });

    // Open REPL Command
    const openReplCommand = vscode.commands.registerCommand('cursed.openRepl', async () => {
        terminalManager.openRepl();
    });

    // Package Install Command
    const packageInstallCommand = vscode.commands.registerCommand('cursed.packageInstall', async () => {
        statusBar.updateStatus('Installing packages...');
        outputChannels.clear('package');
        outputChannels.show('package');

        try {
            await terminalManager.runTask('install', [], 'Installing dependencies...');
            statusBar.updateStatus('Install Complete', 3000);
            dependencyProvider.refresh();
            vscode.window.showInformationMessage('Dependencies installed successfully!');
        } catch (error) {
            statusBar.updateStatus('Install Failed', 5000);
            vscode.window.showErrorMessage(`Package installation failed: ${error}`);
        }
    });

    // Package Update Command
    const packageUpdateCommand = vscode.commands.registerCommand('cursed.packageUpdate', async () => {
        statusBar.updateStatus('Updating packages...');
        outputChannels.clear('package');
        outputChannels.show('package');

        try {
            await terminalManager.runTask('update', [], 'Updating dependencies...');
            statusBar.updateStatus('Update Complete', 3000);
            dependencyProvider.refresh();
            vscode.window.showInformationMessage('Dependencies updated successfully!');
        } catch (error) {
            statusBar.updateStatus('Update Failed', 5000);
            vscode.window.showErrorMessage(`Package update failed: ${error}`);
        }
    });

    // Show Project Structure Command
    const showProjectStructureCommand = vscode.commands.registerCommand('cursed.showProjectStructure', async () => {
        const panel = vscode.window.createWebviewPanel(
            'cursedProjectStructure',
            'CURSED Project Structure',
            vscode.ViewColumn.Beside,
            { enableScripts: true }
        );

        panel.webview.html = await generateProjectStructureHtml();
    });

    // Generate Docs Command
    const generateDocsCommand = vscode.commands.registerCommand('cursed.generateDocs', async () => {
        statusBar.updateStatus('Generating docs...');
        try {
            await terminalManager.runTask('docs', [], 'Generating documentation...');
            statusBar.updateStatus('Docs Generated', 3000);
            vscode.window.showInformationMessage('Documentation generated successfully!');
        } catch (error) {
            statusBar.updateStatus('Docs Failed', 5000);
            vscode.window.showErrorMessage(`Documentation generation failed: ${error}`);
        }
    });

    // Benchmarks Command
    const benchmarksCommand = vscode.commands.registerCommand('cursed.benchmarks', async () => {
        statusBar.updateStatus('Running benchmarks...');
        outputChannels.clear('benchmark');
        outputChannels.show('benchmark');

        try {
            await terminalManager.runTask('benchmark', [], 'Running benchmarks...');
            statusBar.updateStatus('Benchmarks Complete', 3000);
            vscode.window.showInformationMessage('Benchmarks completed successfully!');
        } catch (error) {
            statusBar.updateStatus('Benchmarks Failed', 5000);
            vscode.window.showErrorMessage(`Benchmarks failed: ${error}`);
        }
    });

    // Show Diagnostics Command
    const showDiagnosticsCommand = vscode.commands.registerCommand('cursed.showDiagnostics', async () => {
        vscode.commands.executeCommand('workbench.action.problems.focus');
    });

    // Check Lints Command
    const checkLintsCommand = vscode.commands.registerCommand('cursed.checkLints', async () => {
        statusBar.updateStatus('Checking lints...');
        try {
            await terminalManager.runTask('lint', [], 'Running linter...');
            statusBar.updateStatus('Lints Complete', 3000);
        } catch (error) {
            statusBar.updateStatus('Lints Failed', 5000);
            vscode.window.showErrorMessage(`Linting failed: ${error}`);
        }
    });

    // Debug Start Command
    const debugStartCommand = vscode.commands.registerCommand('cursed.debugStart', async () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('Please open a CURSED file to start debugging');
            return;
        }

        const config = {
            type: 'cursed',
            request: 'launch',
            name: 'Debug CURSED',
            program: activeEditor.document.uri.fsPath,
            cwd: vscode.workspace.workspaceFolders?.[0].uri.fsPath || '.'
        };

        vscode.debug.startDebugging(vscode.workspace.workspaceFolders?.[0], config);
    });

    // Register all commands
    context.subscriptions.push(
        restartCommand,
        showAstCommand,
        showTypeInfoCommand,
        formatDocumentCommand,
        runLinterCommand,
        showGoroutineInfoCommand,
        showChannelInfoCommand,
        newProjectCommand,
        buildCommand,
        runCommand,
        testCommand,
        cleanCommand,
        openReplCommand,
        packageInstallCommand,
        packageUpdateCommand,
        showProjectStructureCommand,
        generateDocsCommand,
        benchmarksCommand,
        showDiagnosticsCommand,
        checkLintsCommand,
        debugStartCommand
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

    // Watch for configuration changes
    const configWatcher = vscode.workspace.onDidChangeConfiguration(event => {
        if (event.affectsConfiguration('cursed')) {
            // Update providers
            if (projectProvider) {
                projectProvider.refresh();
            }
            if (dependencyProvider) {
                dependencyProvider.refresh();
            }
            
            // Update status bar
            if (statusBar) {
                statusBar.updateConfiguration();
            }
        }
    });

    // Watch for file changes
    const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.{csd,toml}');
    fileWatcher.onDidCreate(() => {
        projectProvider?.refresh();
        dependencyProvider?.refresh();
    });
    fileWatcher.onDidDelete(() => {
        projectProvider?.refresh();
        dependencyProvider?.refresh();
    });
    fileWatcher.onDidChange(() => {
        projectProvider?.refresh();
    });

    context.subscriptions.push(formatOnSave, configWatcher, fileWatcher);
}

async function createNewProject(projectPath: string, projectName: string): Promise<void> {
    const cursedPackageContent = `[package]
name = "${projectName}"
version = "0.1.0"
edition = "2023"

[dependencies]
# Add your dependencies here

[build]
# Build configuration
`;

    const mainCsdContent = `// Welcome to CURSED! 🔥
// This is your main entry point - time to slay some code!

// Import the standard library for basic I/O
import "stdlib::io"

slay main() {
    // Print a greeting that's absolutely fire
    println("Yo! Welcome to ${projectName} - this code is about to be lowkey legendary! 💯")?
    
    // Variables in CURSED are sus by default (mutable)
    sus message = "CURSED is absolutely sending me rn"
    println(message)?
    
    // Facts are immutable - they don't lie
    facts pi = 3.14159
    println("Pi is facts: " + string(pi))?
    
    // Control flow with Gen Z vibes
    sus mood = "excited"
    vibe_check mood {
        mood "excited" => {
            println("We're absolutely vibing with this code! 🎉")?
        }
        mood "tired" => {
            println("This code needs some coffee ☕")?
        }
        basic => {
            println("Mood is unknown but the code still slaps 🔥")?
        }
    }
    
    // Loops with style
    lowkey (sus i = 0; i < 3; i++) {
        println("Loop iteration: " + string(i) + " - still fire! 🚀")?
    }
    
    println("Project ${projectName} is ready to serve looks! ✨")?
}
`;

    try {
        // Create main directory if it doesn't exist
        if (!fs.existsSync(projectPath)) {
            fs.mkdirSync(projectPath, { recursive: true });
        }

        // Write CursedPackage.toml
        const packagePath = path.join(projectPath, 'CursedPackage.toml');
        fs.writeFileSync(packagePath, cursedPackageContent);

        // Write main.csd
        const mainPath = path.join(projectPath, 'main.csd');
        fs.writeFileSync(mainPath, mainCsdContent);

        // Create src directory
        const srcDir = path.join(projectPath, 'src');
        if (!fs.existsSync(srcDir)) {
            fs.mkdirSync(srcDir);
        }

        // Create tests directory
        const testsDir = path.join(projectPath, 'tests');
        if (!fs.existsSync(testsDir)) {
            fs.mkdirSync(testsDir);
        }

        // Create example test
        const testContent = `// Test file for ${projectName}
import "stdlib::testing"

test "basic_test" {
    facts expected = "test"
    facts actual = "test"
    assert_eq(expected, actual)
}
`;
        fs.writeFileSync(path.join(testsDir, 'basic_test.csd'), testContent);

        vscode.window.showInformationMessage(`New CURSED project "${projectName}" created successfully! 🎉`);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to create project: ${error}`);
        throw error;
    }
}

async function generateProjectStructureHtml(): Promise<string> {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders || workspaceFolders.length === 0) {
        return '<h1>No workspace folder found</h1>';
    }

    const rootPath = workspaceFolders[0].uri.fsPath;
    
    function scanDirectory(dirPath: string, level: number = 0): string {
        const items: string[] = [];
        const indent = '  '.repeat(level);
        
        try {
            const entries = fs.readdirSync(dirPath, { withFileTypes: true });
            
            for (const entry of entries) {
                if (entry.name.startsWith('.')) continue;
                
                const icon = entry.isDirectory() ? '📁' : getFileIcon(entry.name);
                const itemPath = path.join(dirPath, entry.name);
                
                items.push(`${indent}${icon} ${entry.name}`);
                
                if (entry.isDirectory() && level < 3) {
                    items.push(scanDirectory(itemPath, level + 1));
                }
            }
        } catch (error) {
            items.push(`${indent}❌ Error reading directory`);
        }
        
        return items.join('\\n');
    }
    
    function getFileIcon(fileName: string): string {
        const ext = path.extname(fileName).toLowerCase();
        switch (ext) {
            case '.csd': return '🔥';
            case '.toml': return '⚙️';
            case '.md': return '📝';
            case '.json': return '📄';
            default: return '📄';
        }
    }

    const structure = scanDirectory(rootPath);
    
    return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Project Structure</title>
            <style>
                body { 
                    font-family: 'Courier New', monospace; 
                    padding: 20px; 
                    background: #1e1e1e; 
                    color: #d4d4d4; 
                }
                h1 { color: #ff6b35; }
                pre { 
                    background: #2d2d30; 
                    padding: 15px; 
                    border-radius: 5px; 
                    overflow-x: auto; 
                }
                .highlight { color: #ff6b35; font-weight: bold; }
            </style>
        </head>
        <body>
            <h1>🔥 CURSED Project Structure</h1>
            <p>This project is absolutely <span class="highlight">fire</span>! Here's the structure:</p>
            <pre>${structure}</pre>
        </body>
        </html>
    `;
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
    console.log('CURSED Language Support extension is deactivating...');
    
    // Clean up status bar
    if (statusBar) {
        statusBar.dispose();
    }
    
    // Clean up terminal manager
    if (terminalManager) {
        terminalManager.dispose();
    }
    
    // Clean up output channels
    if (outputChannels) {
        outputChannels.dispose();
    }
    
    // Stop language client
    if (!client) {
        return undefined;
    }
    
    return client.stop().then(() => {
        console.log('CURSED Language Support extension deactivated successfully');
    });
}
