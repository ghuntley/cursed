import * as vscode from 'vscode';
import { CursedOutputChannels } from './outputChannels';

export class CursedTerminalManager implements vscode.Disposable {
    private terminals: Map<string, vscode.Terminal> = new Map();
    private disposables: vscode.Disposable[] = [];

    constructor(private outputChannels: CursedOutputChannels) {
        // Listen for terminal disposal
        this.disposables.push(
            vscode.window.onDidCloseTerminal((terminal) => {
                // Clean up our terminal references
                for (const [name, term] of this.terminals.entries()) {
                    if (term === terminal) {
                        this.terminals.delete(name);
                        break;
                    }
                }
            })
        );
    }

    public async runTask(taskName: string, args: string[] = [], message?: string): Promise<void> {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            throw new Error('No workspace folder found');
        }

        if (message) {
            this.outputChannels.appendLine(taskName, message);
        }

        return new Promise((resolve, reject) => {
            const terminal = this.getOrCreateTerminal(taskName);
            
            // Build command
            const command = this.buildCommand(taskName, args);
            
            // Show terminal and execute command
            terminal.show();
            terminal.sendText(command);

            // Listen for task completion (simplified - in real implementation, you'd want more sophisticated tracking)
            const timeout = setTimeout(() => {
                this.outputChannels.appendLine(taskName, `Task '${taskName}' completed`);
                resolve();
            }, 1000); // Basic timeout - real implementation would track actual completion

            this.disposables.push({
                dispose: () => clearTimeout(timeout)
            });
        });
    }

    public openRepl(): void {
        const terminal = this.getOrCreateTerminal('repl');
        terminal.show();
        terminal.sendText('cursed repl');
    }

    public openBuildTerminal(): vscode.Terminal {
        return this.getOrCreateTerminal('build');
    }

    public openTestTerminal(): vscode.Terminal {
        return this.getOrCreateTerminal('test');
    }

    public runCommand(command: string, terminalName: string = 'cursed'): void {
        const terminal = this.getOrCreateTerminal(terminalName);
        terminal.show();
        terminal.sendText(command);
    }

    public runInBackground(command: string, outputChannel?: string): Promise<void> {
        return new Promise((resolve, reject) => {
            const terminal = this.getOrCreateTerminal('background');
            
            if (outputChannel) {
                this.outputChannels.appendLine(outputChannel, `Running: ${command}`);
            }

            terminal.sendText(command);

            // Basic completion tracking
            setTimeout(() => {
                if (outputChannel) {
                    this.outputChannels.appendLine(outputChannel, 'Command completed');
                }
                resolve();
            }, 2000);
        });
    }

    private getOrCreateTerminal(name: string): vscode.Terminal {
        // Check if terminal exists and is still active
        let terminal = this.terminals.get(name);
        
        if (!terminal || terminal.exitStatus !== undefined) {
            // Create new terminal
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            
            terminal = vscode.window.createTerminal({
                name: `CURSED: ${name}`,
                cwd: workspaceFolder?.uri.fsPath,
                iconPath: new vscode.ThemeIcon('fire'),
                color: new vscode.ThemeColor('terminal.ansiRed')
            });
            
            this.terminals.set(name, terminal);
            
            // Add welcome message for new terminals
            if (name !== 'background') {
                this.sendWelcomeMessage(terminal, name);
            }
        }
        
        return terminal;
    }

    private sendWelcomeMessage(terminal: vscode.Terminal, taskName: string): void {
        const messages = {
            build: 'echo "🔥 CURSED Build Terminal - Ready to compile some fire code!"',
            test: 'echo "🧪 CURSED Test Terminal - Time to validate that the code is absolutely sending!"',
            repl: 'echo "💬 CURSED REPL - Interactive coding session about to be legendary!"',
            package: 'echo "📦 CURSED Package Terminal - Managing dependencies like a boss!"'
        };

        const message = messages[taskName as keyof typeof messages] || 
                       `echo "🔥 CURSED ${taskName} Terminal - Let's get this bread!"`;
        
        terminal.sendText(message);
    }

    private buildCommand(taskName: string, args: string[]): string {
        const baseCommand = this.getBaseCommand(taskName);
        
        if (args.length > 0) {
            return `${baseCommand} ${args.join(' ')}`;
        }
        
        return baseCommand;
    }

    private getBaseCommand(taskName: string): string {
        const commands: Record<string, string> = {
            build: 'cursed build',
            run: 'cursed run',
            test: 'cursed test',
            clean: 'cursed clean',
            lint: 'cursed lint',
            format: 'cursed format',
            install: 'cursed install',
            update: 'cursed update',
            docs: 'cursed docs',
            benchmark: 'cursed benchmark',
            check: 'cursed check',
            package: 'cursed package'
        };

        return commands[taskName] || `cursed ${taskName}`;
    }

    public killTerminal(name: string): void {
        const terminal = this.terminals.get(name);
        if (terminal) {
            terminal.dispose();
            this.terminals.delete(name);
        }
    }

    public killAllTerminals(): void {
        for (const terminal of this.terminals.values()) {
            terminal.dispose();
        }
        this.terminals.clear();
    }

    public listActiveTerminals(): string[] {
        return Array.from(this.terminals.keys()).filter(name => {
            const terminal = this.terminals.get(name);
            return terminal && terminal.exitStatus === undefined;
        });
    }

    public showTerminal(name: string): boolean {
        const terminal = this.terminals.get(name);
        if (terminal && terminal.exitStatus === undefined) {
            terminal.show();
            return true;
        }
        return false;
    }

    public sendText(name: string, text: string): boolean {
        const terminal = this.terminals.get(name);
        if (terminal && terminal.exitStatus === undefined) {
            terminal.sendText(text);
            return true;
        }
        return false;
    }

    public dispose(): void {
        // Dispose all terminals
        this.killAllTerminals();
        
        // Dispose all event listeners
        for (const disposable of this.disposables) {
            disposable.dispose();
        }
        this.disposables = [];
    }
}
