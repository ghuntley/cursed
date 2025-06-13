import * as vscode from 'vscode';

export class CursedOutputChannels implements vscode.Disposable {
    private channels: Map<string, vscode.OutputChannel> = new Map();

    constructor() {
        this.createDefaultChannels();
    }

    private createDefaultChannels(): void {
        // Core channels
        this.createChannel('lsp', 'CURSED Language Server');
        this.createChannel('build', 'CURSED Build');
        this.createChannel('test', 'CURSED Tests');
        this.createChannel('run', 'CURSED Run');
        this.createChannel('package', 'CURSED Package Manager');
        this.createChannel('lint', 'CURSED Linter');
        this.createChannel('format', 'CURSED Formatter');
        this.createChannel('benchmark', 'CURSED Benchmarks');
        this.createChannel('debug', 'CURSED Debug');
    }

    public createChannel(name: string, displayName?: string): vscode.OutputChannel {
        if (this.channels.has(name)) {
            return this.channels.get(name)!;
        }

        const channel = vscode.window.createOutputChannel(displayName || `CURSED ${name}`);
        this.channels.set(name, channel);
        return channel;
    }

    public getChannel(name: string): vscode.OutputChannel | undefined {
        return this.channels.get(name);
    }

    public show(name: string, preserveFocus: boolean = false): void {
        const channel = this.channels.get(name);
        if (channel) {
            channel.show(preserveFocus);
        }
    }

    public hide(name: string): void {
        const channel = this.channels.get(name);
        if (channel) {
            channel.hide();
        }
    }

    public append(name: string, value: string): void {
        const channel = this.channels.get(name);
        if (channel) {
            channel.append(value);
        }
    }

    public appendLine(name: string, value: string): void {
        const channel = this.channels.get(name);
        if (channel) {
            const timestamp = new Date().toLocaleTimeString();
            channel.appendLine(`[${timestamp}] ${value}`);
        }
    }

    public clear(name: string): void {
        const channel = this.channels.get(name);
        if (channel) {
            channel.clear();
        }
    }

    public clearAll(): void {
        for (const channel of this.channels.values()) {
            channel.clear();
        }
    }

    public logError(name: string, error: Error | string): void {
        const errorMessage = error instanceof Error ? error.message : error;
        const errorStack = error instanceof Error ? error.stack : undefined;
        
        this.appendLine(name, `❌ ERROR: ${errorMessage}`);
        if (errorStack) {
            this.appendLine(name, `Stack trace:\n${errorStack}`);
        }
    }

    public logWarning(name: string, message: string): void {
        this.appendLine(name, `⚠️ WARNING: ${message}`);
    }

    public logInfo(name: string, message: string): void {
        this.appendLine(name, `ℹ️ INFO: ${message}`);
    }

    public logSuccess(name: string, message: string): void {
        this.appendLine(name, `✅ SUCCESS: ${message}`);
    }

    public logDebug(name: string, message: string): void {
        const config = vscode.workspace.getConfiguration('cursed.languageServer');
        if (config.get('debug', false)) {
            this.appendLine(name, `🐛 DEBUG: ${message}`);
        }
    }

    public logBuildStart(message: string = 'Build started'): void {
        this.clear('build');
        this.appendLine('build', `🔨 ${message}`);
        this.show('build', true);
    }

    public logBuildSuccess(message: string = 'Build completed successfully'): void {
        this.appendLine('build', `✅ ${message}`);
    }

    public logBuildError(error: string): void {
        this.appendLine('build', `❌ Build failed: ${error}`);
        this.show('build', false);
    }

    public logTestStart(testName?: string): void {
        this.clear('test');
        const message = testName ? `Running test: ${testName}` : 'Running tests';
        this.appendLine('test', `🧪 ${message}`);
        this.show('test', true);
    }

    public logTestSuccess(message: string = 'All tests passed'): void {
        this.appendLine('test', `✅ ${message}`);
    }

    public logTestFailure(message: string): void {
        this.appendLine('test', `❌ Test failed: ${message}`);
        this.show('test', false);
    }

    public logPackageOperation(operation: string, packageName?: string): void {
        const message = packageName ? 
            `${operation} package: ${packageName}` : 
            operation;
        this.appendLine('package', `📦 ${message}`);
    }

    public logLspMessage(level: 'info' | 'warn' | 'error' | 'debug', message: string): void {
        const icons = {
            info: 'ℹ️',
            warn: '⚠️',
            error: '❌',
            debug: '🐛'
        };

        if (level === 'debug') {
            this.logDebug('lsp', message);
        } else {
            this.appendLine('lsp', `${icons[level]} ${message}`);
        }

        if (level === 'error') {
            this.show('lsp', false);
        }
    }

    public showChannelQuickPick(): void {
        const channelNames = Array.from(this.channels.keys());
        const items = channelNames.map(name => ({
            label: name,
            description: `CURSED ${name}`,
            detail: `Show ${name} output channel`
        }));

        vscode.window.showQuickPick(items, {
            placeHolder: 'Select output channel to show'
        }).then(selected => {
            if (selected) {
                this.show(selected.label);
            }
        });
    }

    public getChannels(): vscode.OutputChannel[] {
        return Array.from(this.channels.values());
    }

    public dispose(): void {
        for (const channel of this.channels.values()) {
            channel.dispose();
        }
        this.channels.clear();
    }
}
