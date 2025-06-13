import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export class CursedDependencyProvider implements vscode.TreeDataProvider<DependencyItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<DependencyItem | undefined | null | void> = new vscode.EventEmitter<DependencyItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<DependencyItem | undefined | null | void> = this._onDidChangeTreeData.event;

    constructor(private context: vscode.ExtensionContext) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: DependencyItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: DependencyItem): Thenable<DependencyItem[]> {
        if (!element) {
            return Promise.resolve(this.getDependencies());
        }
        
        if (element.children) {
            return Promise.resolve(element.children);
        }
        
        return Promise.resolve([]);
    }

    private getDependencies(): DependencyItem[] {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return [
                new DependencyItem(
                    'No workspace found',
                    '',
                    vscode.TreeItemCollapsibleState.None,
                    'info',
                    new vscode.ThemeIcon('info')
                )
            ];
        }

        const items: DependencyItem[] = [];
        
        for (const workspaceFolder of workspaceFolders) {
            const rootPath = workspaceFolder.uri.fsPath;
            const packageFile = path.join(rootPath, 'CursedPackage.toml');
            
            if (fs.existsSync(packageFile)) {
                const dependencies = this.parseDependencies(packageFile);
                items.push(...dependencies);
            }
        }

        if (items.length === 0) {
            return [
                new DependencyItem(
                    'No dependencies found',
                    'Add dependencies to CursedPackage.toml',
                    vscode.TreeItemCollapsibleState.None,
                    'info',
                    new vscode.ThemeIcon('package')
                )
            ];
        }

        return items;
    }

    private parseDependencies(packageFile: string): DependencyItem[] {
        const items: DependencyItem[] = [];
        
        try {
            const content = fs.readFileSync(packageFile, 'utf8');
            const lines = content.split('\n');
            
            let inDependenciesSection = false;
            let inDevDependenciesSection = false;
            let inBuildDependenciesSection = false;
            
            const dependencies: Dependency[] = [];
            const devDependencies: Dependency[] = [];
            const buildDependencies: Dependency[] = [];
            
            for (const line of lines) {
                const trimmedLine = line.trim();
                
                // Check section headers
                if (trimmedLine === '[dependencies]') {
                    inDependenciesSection = true;
                    inDevDependenciesSection = false;
                    inBuildDependenciesSection = false;
                    continue;
                } else if (trimmedLine === '[dev-dependencies]') {
                    inDependenciesSection = false;
                    inDevDependenciesSection = true;
                    inBuildDependenciesSection = false;
                    continue;
                } else if (trimmedLine === '[build-dependencies]') {
                    inDependenciesSection = false;
                    inDevDependenciesSection = false;
                    inBuildDependenciesSection = true;
                    continue;
                } else if (trimmedLine.startsWith('[') && trimmedLine.endsWith(']')) {
                    inDependenciesSection = false;
                    inDevDependenciesSection = false;
                    inBuildDependenciesSection = false;
                    continue;
                }
                
                // Skip comments and empty lines
                if (trimmedLine.startsWith('#') || trimmedLine === '') {
                    continue;
                }
                
                // Parse dependency line
                const match = trimmedLine.match(/^([^=]+)\s*=\s*"([^"]+)"/);
                if (match) {
                    const [, name, version] = match;
                    const dependency: Dependency = {
                        name: name.trim(),
                        version: version.trim(),
                        type: inDevDependenciesSection ? 'dev' : inBuildDependenciesSection ? 'build' : 'normal'
                    };
                    
                    if (inDependenciesSection) {
                        dependencies.push(dependency);
                    } else if (inDevDependenciesSection) {
                        devDependencies.push(dependency);
                    } else if (inBuildDependenciesSection) {
                        buildDependencies.push(dependency);
                    }
                }
            }
            
            // Create dependency items
            if (dependencies.length > 0) {
                const depItems = dependencies.map(dep => this.createDependencyItem(dep));
                const depsGroup = new DependencyItem(
                    `Dependencies (${dependencies.length})`,
                    'Runtime dependencies',
                    vscode.TreeItemCollapsibleState.Expanded,
                    'group',
                    new vscode.ThemeIcon('package'),
                    undefined,
                    depItems
                );
                items.push(depsGroup);
            }
            
            if (devDependencies.length > 0) {
                const devDepItems = devDependencies.map(dep => this.createDependencyItem(dep));
                const devDepsGroup = new DependencyItem(
                    `Dev Dependencies (${devDependencies.length})`,
                    'Development dependencies',
                    vscode.TreeItemCollapsibleState.Collapsed,
                    'group',
                    new vscode.ThemeIcon('tools'),
                    undefined,
                    devDepItems
                );
                items.push(devDepsGroup);
            }
            
            if (buildDependencies.length > 0) {
                const buildDepItems = buildDependencies.map(dep => this.createDependencyItem(dep));
                const buildDepsGroup = new DependencyItem(
                    `Build Dependencies (${buildDependencies.length})`,
                    'Build-time dependencies',
                    vscode.TreeItemCollapsibleState.Collapsed,
                    'group',
                    new vscode.ThemeIcon('gear'),
                    undefined,
                    buildDepItems
                );
                items.push(buildDepsGroup);
            }
            
        } catch (error) {
            items.push(new DependencyItem(
                'Error parsing dependencies',
                `Failed to parse CursedPackage.toml: ${error}`,
                vscode.TreeItemCollapsibleState.None,
                'error',
                new vscode.ThemeIcon('error')
            ));
        }
        
        return items;
    }

    private createDependencyItem(dependency: Dependency): DependencyItem {
        const icon = this.getDependencyIcon(dependency);
        const tooltip = this.getDependencyTooltip(dependency);
        
        return new DependencyItem(
            dependency.name,
            dependency.version,
            vscode.TreeItemCollapsibleState.None,
            'dependency',
            icon,
            {
                command: 'cursed.showDependencyInfo',
                title: 'Show Dependency Info',
                arguments: [dependency]
            }
        );
    }

    private getDependencyIcon(dependency: Dependency): vscode.ThemeIcon {
        // Check if it's a standard library dependency
        if (dependency.name.startsWith('stdlib::')) {
            return new vscode.ThemeIcon('library');
        }
        
        // Check if it's a crypto dependency
        if (dependency.name.includes('crypto')) {
            return new vscode.ThemeIcon('lock');
        }
        
        // Check if it's a web dependency
        if (dependency.name.includes('web') || dependency.name.includes('http')) {
            return new vscode.ThemeIcon('globe');
        }
        
        // Check if it's a database dependency
        if (dependency.name.includes('db') || dependency.name.includes('sql')) {
            return new vscode.ThemeIcon('database');
        }
        
        // Default package icon
        return new vscode.ThemeIcon('package');
    }

    private getDependencyTooltip(dependency: Dependency): string {
        const typeText = dependency.type === 'dev' ? ' (dev)' : dependency.type === 'build' ? ' (build)' : '';
        return `${dependency.name} v${dependency.version}${typeText}`;
    }
}

class DependencyItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly version: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly itemType: string,
        public readonly iconPath?: vscode.ThemeIcon | vscode.Uri,
        public readonly command?: vscode.Command,
        public children?: DependencyItem[]
    ) {
        super(label, collapsibleState);
        
        this.description = version;
        this.tooltip = `${label} ${version}`;
        this.contextValue = itemType;
        
        if (iconPath) {
            this.iconPath = iconPath;
        }
        
        if (command) {
            this.command = command;
        }
    }
}

interface Dependency {
    name: string;
    version: string;
    type: 'normal' | 'dev' | 'build';
}
