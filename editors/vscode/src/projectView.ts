import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export class CursedProjectTreeDataProvider implements vscode.TreeDataProvider<ProjectItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<ProjectItem | undefined | null | void> = new vscode.EventEmitter<ProjectItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<ProjectItem | undefined | null | void> = this._onDidChangeTreeData.event;

    constructor(private context: vscode.ExtensionContext) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: ProjectItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: ProjectItem): Thenable<ProjectItem[]> {
        if (!element) {
            return Promise.resolve(this.getProjectStructure());
        }
        
        if (element.children) {
            return Promise.resolve(element.children);
        }
        
        return Promise.resolve([]);
    }

    private getProjectStructure(): ProjectItem[] {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return [
                new ProjectItem(
                    'No CURSED project found',
                    vscode.TreeItemCollapsibleState.None,
                    'info',
                    undefined,
                    new vscode.ThemeIcon('info')
                )
            ];
        }

        const items: ProjectItem[] = [];
        
        for (const workspaceFolder of workspaceFolders) {
            const rootPath = workspaceFolder.uri.fsPath;
            
            // Check for CursedPackage.toml
            const packageFile = path.join(rootPath, 'CursedPackage.toml');
            const hasCursedFiles = this.findCursedFiles(rootPath).length > 0;
            
            if (fs.existsSync(packageFile) || hasCursedFiles) {
                items.push(...this.buildProjectItems(rootPath, workspaceFolder.name));
            }
        }

        if (items.length === 0) {
            return [
                new ProjectItem(
                    'No CURSED files found',
                    vscode.TreeItemCollapsibleState.None,
                    'info',
                    undefined,
                    new vscode.ThemeIcon('warning')
                )
            ];
        }

        return items;
    }

    private buildProjectItems(rootPath: string, projectName: string): ProjectItem[] {
        const items: ProjectItem[] = [];
        
        // Project info
        const packageFile = path.join(rootPath, 'CursedPackage.toml');
        let projectInfo = 'CURSED Project';
        
        if (fs.existsSync(packageFile)) {
            try {
                const packageContent = fs.readFileSync(packageFile, 'utf8');
                const versionMatch = packageContent.match(/version\s*=\s*"([^"]+)"/);
                if (versionMatch) {
                    projectInfo = `${projectName} v${versionMatch[1]}`;
                }
            } catch {
                // Ignore parsing errors
            }
        }

        const projectItem = new ProjectItem(
            projectInfo,
            vscode.TreeItemCollapsibleState.Expanded,
            'project',
            rootPath,
            new vscode.ThemeIcon('folder-library')
        );

        // Source files
        const sourceFiles = this.findCursedFiles(rootPath);
        if (sourceFiles.length > 0) {
            const sourceItems = sourceFiles.map(file => 
                new ProjectItem(
                    path.basename(file),
                    vscode.TreeItemCollapsibleState.None,
                    'file',
                    file,
                    new vscode.ThemeIcon('file-code'),
                    {
                        command: 'vscode.open',
                        title: 'Open File',
                        arguments: [vscode.Uri.file(file)]
                    }
                )
            );

            const sourceFolderItem = new ProjectItem(
                `Source Files (${sourceFiles.length})`,
                vscode.TreeItemCollapsibleState.Expanded,
                'folder',
                undefined,
                new vscode.ThemeIcon('file-directory'),
                undefined,
                sourceItems
            );

            projectItem.children = projectItem.children || [];
            projectItem.children.push(sourceFolderItem);
        }

        // Test files
        const testFiles = this.findTestFiles(rootPath);
        if (testFiles.length > 0) {
            const testItems = testFiles.map(file => 
                new ProjectItem(
                    path.basename(file),
                    vscode.TreeItemCollapsibleState.None,
                    'test',
                    file,
                    new vscode.ThemeIcon('beaker'),
                    {
                        command: 'vscode.open',
                        title: 'Open Test File',
                        arguments: [vscode.Uri.file(file)]
                    }
                )
            );

            const testFolderItem = new ProjectItem(
                `Tests (${testFiles.length})`,
                vscode.TreeItemCollapsibleState.Collapsed,
                'folder',
                undefined,
                new vscode.ThemeIcon('test-view-icon'),
                undefined,
                testItems
            );

            projectItem.children = projectItem.children || [];
            projectItem.children.push(testFolderItem);
        }

        // Configuration files
        const configFiles = this.findConfigFiles(rootPath);
        if (configFiles.length > 0) {
            const configItems = configFiles.map(file => 
                new ProjectItem(
                    path.basename(file),
                    vscode.TreeItemCollapsibleState.None,
                    'config',
                    file,
                    new vscode.ThemeIcon('settings-gear'),
                    {
                        command: 'vscode.open',
                        title: 'Open Config File',
                        arguments: [vscode.Uri.file(file)]
                    }
                )
            );

            const configFolderItem = new ProjectItem(
                `Configuration (${configFiles.length})`,
                vscode.TreeItemCollapsibleState.Collapsed,
                'folder',
                undefined,
                new vscode.ThemeIcon('gear'),
                undefined,
                configItems
            );

            projectItem.children = projectItem.children || [];
            projectItem.children.push(configFolderItem);
        }

        // Build artifacts
        const buildDir = path.join(rootPath, 'target');
        if (fs.existsSync(buildDir)) {
            const buildItem = new ProjectItem(
                'Build Artifacts',
                vscode.TreeItemCollapsibleState.Collapsed,
                'build',
                buildDir,
                new vscode.ThemeIcon('tools'),
                {
                    command: 'revealInExplorer',
                    title: 'Reveal in Explorer',
                    arguments: [vscode.Uri.file(buildDir)]
                }
            );

            projectItem.children = projectItem.children || [];
            projectItem.children.push(buildItem);
        }

        items.push(projectItem);
        return items;
    }

    private findCursedFiles(rootPath: string): string[] {
        const files: string[] = [];
        
        try {
            // Check root directory
            const rootFiles = fs.readdirSync(rootPath);
            for (const file of rootFiles) {
                if (file.endsWith('.csd')) {
                    files.push(path.join(rootPath, file));
                }
            }

            // Check src directory
            const srcDir = path.join(rootPath, 'src');
            if (fs.existsSync(srcDir)) {
                const srcFiles = this.findCursedFilesRecursive(srcDir);
                files.push(...srcFiles);
            }
        } catch {
            // Ignore errors
        }

        return files;
    }

    private findTestFiles(rootPath: string): string[] {
        const files: string[] = [];
        
        try {
            const testDirs = ['tests', 'test'];
            for (const testDir of testDirs) {
                const testPath = path.join(rootPath, testDir);
                if (fs.existsSync(testPath)) {
                    const testFiles = this.findCursedFilesRecursive(testPath);
                    files.push(...testFiles);
                }
            }
        } catch {
            // Ignore errors
        }

        return files;
    }

    private findConfigFiles(rootPath: string): string[] {
        const files: string[] = [];
        const configFiles = [
            'CursedPackage.toml',
            'CursedBuild.toml',
            '.cursed-doc.toml',
            '.cursed-lint.toml'
        ];
        
        try {
            for (const configFile of configFiles) {
                const filePath = path.join(rootPath, configFile);
                if (fs.existsSync(filePath)) {
                    files.push(filePath);
                }
            }
        } catch {
            // Ignore errors
        }

        return files;
    }

    private findCursedFilesRecursive(dirPath: string): string[] {
        const files: string[] = [];
        
        try {
            const entries = fs.readdirSync(dirPath, { withFileTypes: true });
            for (const entry of entries) {
                const fullPath = path.join(dirPath, entry.name);
                
                if (entry.isDirectory()) {
                    files.push(...this.findCursedFilesRecursive(fullPath));
                } else if (entry.name.endsWith('.csd')) {
                    files.push(fullPath);
                }
            }
        } catch {
            // Ignore errors
        }

        return files;
    }
}

class ProjectItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly itemType: string,
        public readonly resourcePath?: string,
        public readonly iconPath?: vscode.ThemeIcon | vscode.Uri,
        public readonly command?: vscode.Command,
        public children?: ProjectItem[]
    ) {
        super(label, collapsibleState);
        
        this.tooltip = this.resourcePath ? this.resourcePath : this.label;
        this.contextValue = itemType;
        
        if (this.iconPath) {
            this.iconPath = iconPath;
        }
        
        if (this.command) {
            this.command = command;
        }
    }
}
