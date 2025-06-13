import * as vscode from 'vscode';

export class CursedStatusBar implements vscode.Disposable {
    private statusBarItem: vscode.StatusBarItem;
    private timeoutHandle?: NodeJS.Timeout;

    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            100
        );
        
        this.statusBarItem.text = '🔥 CURSED';
        this.statusBarItem.tooltip = 'CURSED Language Support - Ready to slay!';
        this.statusBarItem.command = 'cursed.showProjectStructure';
        this.statusBarItem.show();
    }

    public updateStatus(message: string, timeoutMs?: number): void {
        // Clear any existing timeout
        if (this.timeoutHandle) {
            clearTimeout(this.timeoutHandle);
            this.timeoutHandle = undefined;
        }

        // Update status
        this.statusBarItem.text = `🔥 ${message}`;
        this.statusBarItem.tooltip = `CURSED: ${message}`;

        // Set timeout to revert to default if specified
        if (timeoutMs && timeoutMs > 0) {
            this.timeoutHandle = setTimeout(() => {
                this.resetToDefault();
                this.timeoutHandle = undefined;
            }, timeoutMs);
        }
    }

    public updateWithProgress(message: string, progress?: number): void {
        let progressText = message;
        
        if (progress !== undefined) {
            const progressBar = this.createProgressBar(progress);
            progressText = `${message} ${progressBar}`;
        }
        
        this.statusBarItem.text = `🔥 ${progressText}`;
        this.statusBarItem.tooltip = `CURSED: ${message}`;
    }

    public showError(message: string, timeoutMs: number = 5000): void {
        this.statusBarItem.text = `🚨 ${message}`;
        this.statusBarItem.tooltip = `CURSED Error: ${message}`;
        this.statusBarItem.color = new vscode.ThemeColor('errorForeground');

        if (this.timeoutHandle) {
            clearTimeout(this.timeoutHandle);
        }

        this.timeoutHandle = setTimeout(() => {
            this.resetToDefault();
            this.timeoutHandle = undefined;
        }, timeoutMs);
    }

    public showWarning(message: string, timeoutMs: number = 4000): void {
        this.statusBarItem.text = `⚠️ ${message}`;
        this.statusBarItem.tooltip = `CURSED Warning: ${message}`;
        this.statusBarItem.color = new vscode.ThemeColor('warningForeground');

        if (this.timeoutHandle) {
            clearTimeout(this.timeoutHandle);
        }

        this.timeoutHandle = setTimeout(() => {
            this.resetToDefault();
            this.timeoutHandle = undefined;
        }, timeoutMs);
    }

    public showSuccess(message: string, timeoutMs: number = 3000): void {
        this.statusBarItem.text = `✅ ${message}`;
        this.statusBarItem.tooltip = `CURSED Success: ${message}`;
        this.statusBarItem.color = new vscode.ThemeColor('terminalCommandDecorationSuccessForeground');

        if (this.timeoutHandle) {
            clearTimeout(this.timeoutHandle);
        }

        this.timeoutHandle = setTimeout(() => {
            this.resetToDefault();
            this.timeoutHandle = undefined;
        }, timeoutMs);
    }

    public updateConfiguration(): void {
        const config = vscode.workspace.getConfiguration('cursed');
        
        // Update based on current configuration
        const lspEnabled = config.get('languageServer.enabled', true);
        
        if (!lspEnabled) {
            this.statusBarItem.text = '🔥 CURSED (LSP Disabled)';
            this.statusBarItem.tooltip = 'CURSED Language Support - LSP is disabled';
            this.statusBarItem.color = new vscode.ThemeColor('warningForeground');
        } else {
            this.resetToDefault();
        }
    }

    public updateProjectInfo(projectName?: string, version?: string): void {
        let text = '🔥 CURSED';
        let tooltip = 'CURSED Language Support';

        if (projectName) {
            text = `🔥 ${projectName}`;
            tooltip = `CURSED Project: ${projectName}`;
            
            if (version) {
                text += ` v${version}`;
                tooltip += ` v${version}`;
            }
        }

        this.statusBarItem.text = text;
        this.statusBarItem.tooltip = tooltip;
    }

    public showLspStatus(status: 'starting' | 'ready' | 'error' | 'stopped'): void {
        const statusIcons = {
            starting: '⏳',
            ready: '✅',
            error: '❌',
            stopped: '⏹️'
        };

        const statusMessages = {
            starting: 'LSP Starting...',
            ready: 'LSP Ready',
            error: 'LSP Error',
            stopped: 'LSP Stopped'
        };

        this.statusBarItem.text = `🔥 CURSED ${statusIcons[status]}`;
        this.statusBarItem.tooltip = `CURSED Language Support - ${statusMessages[status]}`;

        if (status === 'error') {
            this.statusBarItem.color = new vscode.ThemeColor('errorForeground');
        } else if (status === 'starting') {
            this.statusBarItem.color = new vscode.ThemeColor('warningForeground');
        } else {
            this.statusBarItem.color = undefined;
        }
    }

    private resetToDefault(): void {
        this.statusBarItem.text = '🔥 CURSED';
        this.statusBarItem.tooltip = 'CURSED Language Support - Ready to slay!';
        this.statusBarItem.color = undefined;
        this.statusBarItem.command = 'cursed.showProjectStructure';
    }

    private createProgressBar(progress: number): string {
        const barLength = 10;
        const filledLength = Math.round((progress / 100) * barLength);
        const emptyLength = barLength - filledLength;
        
        const filled = '█'.repeat(filledLength);
        const empty = '░'.repeat(emptyLength);
        
        return `[${filled}${empty}] ${progress}%`;
    }

    public dispose(): void {
        if (this.timeoutHandle) {
            clearTimeout(this.timeoutHandle);
        }
        this.statusBarItem.dispose();
    }
}
