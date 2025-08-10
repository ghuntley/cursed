import * as vscode from 'vscode';
import * as path from 'path';
import { 
    LanguageClient, 
    LanguageClientOptions, 
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';
import { CursedProjectExplorer } from './projectExplorer';
import { CursedDependencyProvider } from './dependencyProvider';
import { CursedDebugProvider } from './debugProvider';
import { CursedTaskProvider } from './taskProvider';
import { CursedTerminalManager } from './terminalManager';
import { CursedPerformanceAnalyzer } from './performanceAnalyzer';
import { CursedCodeLensProvider } from './codeLensProvider';
import { CursedDocumentFormatter } from './formatter';
import { CursedDocumentSymbolProvider } from './symbolProvider';
import { CursedReferenceProvider } from './referenceProvider';
import { CursedRenameProvider } from './renameProvider';
import { CursedDefinitionProvider } from './definitionProvider';
import { CursedHoverProvider } from './hoverProvider';
import { CursedCompletionProvider } from './completionProvider';
import { CursedDiagnosticProvider } from './diagnosticProvider';
import { CursedSemanticTokensProvider } from './semanticTokensProvider';

let client: LanguageClient;
let projectExplorer: CursedProjectExplorer;
let dependencyProvider: CursedDependencyProvider;
let terminalManager: CursedTerminalManager;
let performanceAnalyzer: CursedPerformanceAnalyzer;

export async function activate(context: vscode.ExtensionContext) {
    console.log('CURSED Advanced Extension activating...');

    // Initialize terminal manager
    terminalManager = new CursedTerminalManager();
    
    // Initialize performance analyzer
    performanceAnalyzer = new CursedPerformanceAnalyzer(context);
    
    // Check if CURSED compiler is available
    const compilerPath = vscode.workspace.getConfiguration('cursed').get<string>('compiler.path', 'cursed-zig');
    const compilerAvailable = await checkCompilerAvailability(compilerPath);
    
    if (!compilerAvailable) {
        vscode.window.showWarningMessage(
            'CURSED compiler not found. Some features may not work properly.',
            'Install CURSED',
            'Configure Path'
        ).then(selection => {
            if (selection === 'Install CURSED') {
                vscode.env.openExternal(vscode.Uri.parse('https://cursed-lang.org/install'));
            } else if (selection === 'Configure Path') {
                vscode.commands.executeCommand('workbench.action.openSettings', 'cursed.compiler.path');
            }
        });
    }

    // Start Language Server
    await startLanguageServer(context);
    
    // Initialize project explorer
    projectExplorer = new CursedProjectExplorer(context);
    vscode.window.registerTreeDataProvider('cursedProjectExplorer', projectExplorer);
    
    // Initialize dependency provider
    dependencyProvider = new CursedDependencyProvider(context);
    vscode.window.registerTreeDataProvider('cursedDependencies', dependencyProvider);
    
    // Register providers
    const documentSelector = [
        { language: 'cursed', scheme: 'file' },
        { language: 'cursed', scheme: 'untitled' }
    ];
    
    // Code lens provider for performance hints and actions
    context.subscriptions.push(
        vscode.languages.registerCodeLensProvider(
            documentSelector,
            new CursedCodeLensProvider()
        )
    );
    
    // Document formatter
    context.subscriptions.push(
        vscode.languages.registerDocumentFormattingEditProvider(
            documentSelector,
            new CursedDocumentFormatter()
        )
    );
    
    // Symbol provider for outline view
    context.subscriptions.push(
        vscode.languages.registerDocumentSymbolProvider(
            documentSelector,
            new CursedDocumentSymbolProvider()
        )
    );
    
    // Reference provider
    context.subscriptions.push(
        vscode.languages.registerReferenceProvider(
            documentSelector,
            new CursedReferenceProvider()
        )
    );
    
    // Rename provider
    context.subscriptions.push(
        vscode.languages.registerRenameProvider(
            documentSelector,
            new CursedRenameProvider()
        )
    );
    
    // Definition provider
    context.subscriptions.push(
        vscode.languages.registerDefinitionProvider(
            documentSelector,
            new CursedDefinitionProvider()
        )
    );
    
    // Hover provider
    context.subscriptions.push(
        vscode.languages.registerHoverProvider(
            documentSelector,
            new CursedHoverProvider()
        )
    );
    
    // Completion provider
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider(
            documentSelector,
            new CursedCompletionProvider(),
            '.', ' ', '\n'
        )
    );
    
    // Semantic tokens provider for advanced syntax highlighting
    context.subscriptions.push(
        vscode.languages.registerDocumentSemanticTokensProvider(
            documentSelector,
            new CursedSemanticTokensProvider(),
            CursedSemanticTokensProvider.legend
        )
    );
    
    // Diagnostic provider for real-time error checking
    const diagnosticProvider = new CursedDiagnosticProvider();
    context.subscriptions.push(diagnosticProvider);
    
    // Debug configuration provider
    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider(
            'cursed',
            new CursedDebugProvider()
        )
    );
    
    // Task provider
    context.subscriptions.push(
        vscode.tasks.registerTaskProvider(
            'cursed',
            new CursedTaskProvider()
        )
    );
    
    // Register commands
    registerCommands(context);
    
    // Set up file watchers
    setupFileWatchers(context);
    
    // Check if we're in a CURSED project
    updateProjectContext();
    
    console.log('CURSED Advanced Extension activated successfully!');
}

async function startLanguageServer(context: vscode.ExtensionContext): Promise<void> {
    const lspEnabled = vscode.workspace.getConfiguration('cursed').get<boolean>('lsp.enabled', true);
    if (!lspEnabled) {
        return;
    }

    // Find LSP server executable
    const serverPath = await findLspServer();
    if (!serverPath) {
        vscode.window.showErrorMessage('CURSED Language Server not found');
        return;
    }

    const serverOptions: ServerOptions = {
        run: { command: serverPath, transport: TransportKind.stdio },
        debug: { command: serverPath, transport: TransportKind.stdio }
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'cursed' },
            { scheme: 'untitled', language: 'cursed' }
        ],
        synchronize: {
            fileEvents: [
                vscode.workspace.createFileSystemWatcher('**/*.csd'),
                vscode.workspace.createFileSystemWatcher('**/CursedPackage.toml'),
                vscode.workspace.createFileSystemWatcher('**/CursedWorkspace.toml')
            ]
        },
        initializationOptions: {
            enableSemanticTokens: true,
            enableCodeLens: true,
            enableInlayHints: true,
            enableDiagnostics: true,
            experimentalFeatures: vscode.workspace.getConfiguration('cursed').get('experimental.features', false)
        }
    };

    client = new LanguageClient(
        'cursed-lsp',
        'CURSED Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client and server
    try {
        await client.start();
        console.log('CURSED Language Server started successfully');
    } catch (error) {
        console.error('Failed to start CURSED Language Server:', error);
        vscode.window.showErrorMessage(`Failed to start CURSED Language Server: ${error}`);
    }
}

async function findLspServer(): Promise<string | null> {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (workspaceFolder) {
        // Try local build first
        const localPath = path.join(workspaceFolder.uri.fsPath, 'zig-out', 'bin', 'cursed-lsp');
        if (await fileExists(localPath)) {
            return localPath;
        }
    }
    
    // Try global installation
    const globalPaths = [
        'cursed-lsp',
        '/usr/local/bin/cursed-lsp',
        '/opt/cursed/bin/cursed-lsp'
    ];
    
    for (const path of globalPaths) {
        if (await commandExists(path)) {
            return path;
        }
    }
    
    return null;
}

function registerCommands(context: vscode.ExtensionContext) {
    // Core commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cursed.run', runCurrentFile),
        vscode.commands.registerCommand('cursed.build', buildCurrentFile),
        vscode.commands.registerCommand('cursed.buildRelease', buildRelease),
        vscode.commands.registerCommand('cursed.test', runTests),
        vscode.commands.registerCommand('cursed.format', formatCurrentFile),
        vscode.commands.registerCommand('cursed.lint', lintCurrentFile),
        vscode.commands.registerCommand('cursed.newProject', createNewProject),
        vscode.commands.registerCommand('cursed.newPackage', createNewPackage),
        vscode.commands.registerCommand('cursed.showDocumentation', showDocumentation),
        
        // Debug commands
        vscode.commands.registerCommand('cursed.showAST', showAST),
        vscode.commands.registerCommand('cursed.showTokens', showTokens),
        
        // Performance commands
        vscode.commands.registerCommand('cursed.analyzePerformance', analyzePerformance),
        vscode.commands.registerCommand('cursed.benchmarkCode', benchmarkCode),
        vscode.commands.registerCommand('cursed.profile', profileCode),
        vscode.commands.registerCommand('cursed.memoryCheck', memoryCheck),
        
        // Cross-compilation
        vscode.commands.registerCommand('cursed.crossCompile', crossCompile),
        
        // Project management
        vscode.commands.registerCommand('cursed.projectExplorer.refresh', () => projectExplorer.refresh()),
        vscode.commands.registerCommand('cursed.dependencies.add', addDependency),
        vscode.commands.registerCommand('cursed.dependencies.update', updateDependencies)
    );
}

function setupFileWatchers(context: vscode.ExtensionContext) {
    // Watch for CURSED package files
    const packageWatcher = vscode.workspace.createFileSystemWatcher('**/CursedPackage.toml');
    packageWatcher.onDidChange(() => updateProjectContext());
    packageWatcher.onDidCreate(() => updateProjectContext());
    packageWatcher.onDidDelete(() => updateProjectContext());
    context.subscriptions.push(packageWatcher);
    
    // Watch for CURSED source files
    const sourceWatcher = vscode.workspace.createFileSystemWatcher('**/*.csd');
    sourceWatcher.onDidChange((uri) => {
        // Real-time analysis on file changes
        if (vscode.workspace.getConfiguration('cursed').get('lint.onType', false)) {
            setTimeout(() => lintFile(uri), 500); // Debounce
        }
    });
    context.subscriptions.push(sourceWatcher);
}

function updateProjectContext() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        vscode.commands.executeCommand('setContext', 'cursed.projectOpen', false);
        return;
    }
    
    const packageFile = path.join(workspaceFolder.uri.fsPath, 'CursedPackage.toml');
    fileExists(packageFile).then(exists => {
        vscode.commands.executeCommand('setContext', 'cursed.projectOpen', exists);
        if (exists) {
            projectExplorer?.refresh();
            dependencyProvider?.refresh();
        }
    });
}

// Command implementations
async function runCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(`cursed-zig "${filePath}"`, 'CURSED Run');
}

async function buildCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    const optimization = vscode.workspace.getConfiguration('cursed').get('build.optimization', 'debug');
    
    terminalManager.executeCommand(
        `cursed-zig --compile --optimize=${optimization} "${filePath}"`, 
        'CURSED Build'
    );
}

async function buildRelease() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }
    
    terminalManager.executeCommand('zig build -Doptimize=ReleaseFast', 'CURSED Release Build');
}

async function runTests() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }
    
    terminalManager.executeCommand('zig build test', 'CURSED Tests');
}

async function formatCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(`cursed-fmt "${filePath}"`, 'CURSED Format');
}

async function lintCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        return;
    }
    
    const filePath = editor.document.fileName;
    await lintFile(vscode.Uri.file(filePath));
}

async function lintFile(uri: vscode.Uri) {
    terminalManager.executeCommand(`cursed-lint "${uri.fsPath}"`, 'CURSED Lint');
}

async function createNewProject() {
    const name = await vscode.window.showInputBox({
        prompt: 'Enter project name',
        validateInput: (value) => {
            if (!value) return 'Project name is required';
            if (!/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(value)) {
                return 'Invalid project name. Use letters, numbers, hyphens, and underscores only.';
            }
            return null;
        }
    });
    
    if (!name) return;
    
    const location = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: 'Select Project Location'
    });
    
    if (!location || location.length === 0) return;
    
    const projectPath = path.join(location[0].fsPath, name);
    terminalManager.executeCommand(
        `cursed-pkg new "${name}"`,
        'CURSED New Project',
        location[0].fsPath
    );
    
    // Open the new project
    setTimeout(() => {
        vscode.commands.executeCommand('vscode.openFolder', vscode.Uri.file(projectPath));
    }, 2000);
}

async function createNewPackage() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }
    
    const name = await vscode.window.showInputBox({
        prompt: 'Enter package name',
        validateInput: (value) => {
            if (!value) return 'Package name is required';
            if (!/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(value)) {
                return 'Invalid package name';
            }
            return null;
        }
    });
    
    if (!name) return;
    
    terminalManager.executeCommand(`cursed-pkg new-package "${name}"`, 'CURSED New Package');
}

async function showDocumentation() {
    const selection = await vscode.window.showQuickPick([
        'Language Reference',
        'Standard Library',
        'Getting Started',
        'Advanced Topics',
        'API Documentation'
    ], {
        placeHolder: 'Select documentation topic'
    });
    
    const baseUrl = 'https://docs.cursed-lang.org';
    const urls: { [key: string]: string } = {
        'Language Reference': `${baseUrl}/language`,
        'Standard Library': `${baseUrl}/stdlib`,
        'Getting Started': `${baseUrl}/getting-started`,
        'Advanced Topics': `${baseUrl}/advanced`,
        'API Documentation': `${baseUrl}/api`
    };
    
    if (selection && urls[selection]) {
        vscode.env.openExternal(vscode.Uri.parse(urls[selection]));
    }
}

async function showAST() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    const filePath = editor.document.fileName;
    const result = await terminalManager.executeCommandSilent(`cursed-zig --ast "${filePath}"`);
    
    const doc = await vscode.workspace.openTextDocument({
        content: result,
        language: 'json'
    });
    
    vscode.window.showTextDocument(doc, { viewColumn: vscode.ViewColumn.Beside });
}

async function showTokens() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    const filePath = editor.document.fileName;
    const result = await terminalManager.executeCommandSilent(`cursed-zig --tokens "${filePath}"`);
    
    const doc = await vscode.workspace.openTextDocument({
        content: result,
        language: 'json'
    });
    
    vscode.window.showTextDocument(doc, { viewColumn: vscode.ViewColumn.Beside });
}

async function analyzePerformance() {
    if (!performanceAnalyzer) return;
    
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await performanceAnalyzer.analyzeFile(editor.document.fileName);
}

async function benchmarkCode() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(`cursed-zig --benchmark "${filePath}"`, 'CURSED Benchmark');
}

async function profileCode() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(`cursed-zig --profile "${filePath}"`, 'CURSED Profile');
}

async function memoryCheck() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(`valgrind --leak-check=full cursed-zig "${filePath}"`, 'CURSED Memory Check');
}

async function crossCompile() {
    const targets = vscode.workspace.getConfiguration('cursed').get<string[]>('crossCompile.targets', []);
    
    const target = await vscode.window.showQuickPick(targets, {
        placeHolder: 'Select cross-compilation target'
    });
    
    if (!target) return;
    
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'cursed') {
        vscode.window.showErrorMessage('Please open a CURSED file first');
        return;
    }
    
    await editor.document.save();
    const filePath = editor.document.fileName;
    
    terminalManager.executeCommand(
        `cursed-zig --compile --target=${target} "${filePath}"`, 
        'CURSED Cross Compile'
    );
}

async function addDependency() {
    const name = await vscode.window.showInputBox({
        prompt: 'Enter dependency name',
        placeHolder: 'e.g., httpz, jsonz, cryptz'
    });
    
    if (!name) return;
    
    const version = await vscode.window.showInputBox({
        prompt: 'Enter version (optional)',
        placeHolder: 'e.g., 1.0.0, latest'
    });
    
    const command = version 
        ? `cursed-pkg add "${name}@${version}"`
        : `cursed-pkg add "${name}"`;
    
    terminalManager.executeCommand(command, 'CURSED Add Dependency');
}

async function updateDependencies() {
    terminalManager.executeCommand('cursed-pkg update', 'CURSED Update Dependencies');
}

// Utility functions
async function checkCompilerAvailability(compilerPath: string): Promise<boolean> {
    try {
        const result = await terminalManager.executeCommandSilent(`${compilerPath} --version`);
        return result.includes('CURSED');
    } catch {
        return false;
    }
}

async function fileExists(path: string): Promise<boolean> {
    try {
        await vscode.workspace.fs.stat(vscode.Uri.file(path));
        return true;
    } catch {
        return false;
    }
}

async function commandExists(command: string): Promise<boolean> {
    try {
        await terminalManager.executeCommandSilent(`which ${command}`);
        return true;
    } catch {
        return false;
    }
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
