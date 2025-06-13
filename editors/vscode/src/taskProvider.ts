import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export class CursedTaskProvider implements vscode.TaskProvider {
    static readonly CursedType = 'cursed';
    private tasks: vscode.Task[] | undefined;

    constructor() {}

    public provideTasks(): Thenable<vscode.Task[]> | undefined {
        return this.getCursedTasks();
    }

    public resolveTask(_task: vscode.Task): vscode.Task | undefined {
        const task = _task.definition.task;
        if (task) {
            const definition: CursedTaskDefinition = <any>_task.definition;
            return this.createTask(definition, _task.scope);
        }
        return undefined;
    }

    private async getCursedTasks(): Promise<vscode.Task[]> {
        if (this.tasks !== undefined) {
            return this.tasks;
        }

        const workspaceFolders = vscode.workspace.workspaceFolders;
        const result: vscode.Task[] = [];

        if (!workspaceFolders || workspaceFolders.length === 0) {
            return result;
        }

        for (const workspaceFolder of workspaceFolders) {
            const folderString = workspaceFolder.uri.fsPath;
            if (!folderString) {
                continue;
            }

            // Check for CursedPackage.toml or .csd files
            const packageFile = path.join(folderString, 'CursedPackage.toml');
            const hasCursedFiles = this.hasCursedFiles(folderString);

            if (fs.existsSync(packageFile) || hasCursedFiles) {
                result.push(...this.createCursedTasks(workspaceFolder));
            }
        }

        this.tasks = result;
        return result;
    }

    private hasCursedFiles(folderPath: string): boolean {
        try {
            const files = fs.readdirSync(folderPath);
            return files.some(file => file.endsWith('.csd'));
        } catch {
            return false;
        }
    }

    private createCursedTasks(workspaceFolder: vscode.WorkspaceFolder): vscode.Task[] {
        const tasks: vscode.Task[] = [];

        // Build task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'build'
        }, workspaceFolder, 'Build', 'cursed build', ['build']));

        // Run task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'run'
        }, workspaceFolder, 'Run', 'cursed run', ['run']));

        // Test task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'test'
        }, workspaceFolder, 'Test', 'cursed test', ['test']));

        // Clean task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'clean'
        }, workspaceFolder, 'Clean', 'cursed clean', ['clean']));

        // Lint task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'lint'
        }, workspaceFolder, 'Lint', 'cursed lint', ['lint']));

        // Format task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'format'
        }, workspaceFolder, 'Format', 'cursed format', ['format']));

        // Install dependencies task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'install'
        }, workspaceFolder, 'Install Dependencies', 'cursed install', ['install']));

        // Update dependencies task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'update'
        }, workspaceFolder, 'Update Dependencies', 'cursed update', ['update']));

        // Documentation task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'docs'
        }, workspaceFolder, 'Generate Documentation', 'cursed docs', ['docs']));

        // Benchmark task
        tasks.push(this.createTask({
            type: CursedTaskProvider.CursedType,
            task: 'benchmark'
        }, workspaceFolder, 'Run Benchmarks', 'cursed benchmark', ['benchmark']));

        return tasks;
    }

    private createTask(
        definition: CursedTaskDefinition, 
        scope: vscode.WorkspaceFolder | vscode.TaskScope | undefined,
        name?: string,
        command?: string,
        args?: string[]
    ): vscode.Task {
        const actualName = name || definition.task;
        const actualCommand = command || `cursed ${definition.task}`;
        const actualArgs = args || [definition.task];

        const execution = new vscode.ShellExecution(actualCommand, {
            cwd: scope && scope !== vscode.TaskScope.Global && scope !== vscode.TaskScope.Workspace 
                ? scope.uri.fsPath 
                : undefined
        });

        const task = new vscode.Task(
            definition,
            scope ?? vscode.TaskScope.Workspace,
            actualName,
            CursedTaskProvider.CursedType,
            execution,
            '$cursed'
        );

        // Set task group
        switch (definition.task) {
            case 'build':
                task.group = vscode.TaskGroup.Build;
                break;
            case 'test':
                task.group = vscode.TaskGroup.Test;
                break;
            case 'clean':
                task.group = vscode.TaskGroup.Clean;
                break;
            default:
                task.group = undefined;
        }

        // Set as build task for build command
        if (definition.task === 'build') {
            task.group = {
                kind: vscode.TaskGroup.Build,
                isDefault: true
            };
        }

        // Set as test task for test command
        if (definition.task === 'test') {
            task.group = {
                kind: vscode.TaskGroup.Test,
                isDefault: true
            };
        }

        return task;
    }
}

interface CursedTaskDefinition extends vscode.TaskDefinition {
    task: string;
    args?: string[];
    cwd?: string;
}
