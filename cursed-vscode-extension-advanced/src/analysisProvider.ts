/**
 * Advanced Analysis Provider for CURSED
 * Provides security, performance, memory, and concurrency analysis
 */

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class CursedAnalysisProvider implements vscode.Disposable {
    private disposables: vscode.Disposable[] = [];
    private analysisResults: Map<string, AnalysisResult[]> = new Map();
    
    constructor(
        private client: LanguageClient,
        private outputChannel: vscode.OutputChannel
    ) {
        this.registerAnalysisCommands();
        this.setupAnalysisDecorations();
    }
    
    dispose() {
        this.disposables.forEach(d => d.dispose());
        this.analysisResults.clear();
    }
    
    private registerAnalysisCommands() {
        // Security analysis
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.security.run', async () => {
                await this.runSecurityAnalysis();
            })
        );
        
        // Performance analysis
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.performance.run', async () => {
                await this.runPerformanceAnalysis();
            })
        );
        
        // Memory analysis
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.memory.run', async () => {
                await this.runMemoryAnalysis();
            })
        );
        
        // Concurrency analysis
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.concurrency.run', async () => {
                await this.runConcurrencyAnalysis();
            })
        );
        
        // Comprehensive analysis
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.comprehensive', async () => {
                await this.runComprehensiveAnalysis();
            })
        );
        
        // Clear analysis results
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.clear', async () => {
                await this.clearAnalysisResults();
            })
        );
        
        // Export analysis report
        this.disposables.push(
            vscode.commands.registerCommand('cursed.analysis.export', async () => {
                await this.exportAnalysisReport();
            })
        );
    }
    
    private setupAnalysisDecorations() {
        // Create decoration types for different analysis results
        this.createDecorationTypes();
        
        // Listen for document changes to update decorations
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument((event) => {
                if (event.document.languageId === 'cursed') {
                    this.updateDecorations(event.document);
                }
            })
        );
        
        // Listen for active editor changes
        this.disposables.push(
            vscode.window.onDidChangeActiveTextEditor((editor) => {
                if (editor && editor.document.languageId === 'cursed') {
                    this.updateDecorations(editor.document);
                }
            })
        );
    }
    
    async runSecurityAnalysis(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        this.outputChannel.show();
        this.outputChannel.appendLine('🔒 Running security analysis...');
        
        try {
            const result = await this.client.sendRequest('cursed/securityAnalysis', {
                textDocument: { uri: editor.document.uri.toString() }
            });
            
            if (result && result.vulnerabilities) {
                const vulnerabilities = result.vulnerabilities as SecurityVulnerability[];
                
                this.outputChannel.appendLine(`Found ${vulnerabilities.length} security issues:`);
                
                for (const vuln of vulnerabilities) {
                    this.outputChannel.appendLine(`  ⚠️  ${vuln.category}: ${vuln.message}`);
                    if (vuln.cwe_id) {
                        this.outputChannel.appendLine(`      CWE-${vuln.cwe_id}: https://cwe.mitre.org/data/definitions/${vuln.cwe_id}.html`);
                    }
                    this.outputChannel.appendLine(`      Location: Line ${vuln.location.range.start.line + 1}`);
                    this.outputChannel.appendLine('');
                }
                
                // Store results and update decorations
                this.storeAnalysisResults(editor.document.uri.toString(), 'security', vulnerabilities);
                this.updateDecorations(editor.document);
                
                if (vulnerabilities.length > 0) {
                    const action = await vscode.window.showWarningMessage(
                        `Found ${vulnerabilities.length} security issues. View details?`,
                        'View Details',
                        'Dismiss'
                    );
                    
                    if (action === 'View Details') {
                        this.showSecurityReport(vulnerabilities);
                    }
                }
            } else {
                this.outputChannel.appendLine('✅ No security issues found');
                vscode.window.showInformationMessage('No security issues found');
            }
        } catch (error) {
            this.outputChannel.appendLine(`❌ Security analysis failed: ${error}`);
            vscode.window.showErrorMessage(`Security analysis failed: ${error}`);
        }
    }
    
    async runPerformanceAnalysis(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        this.outputChannel.show();
        this.outputChannel.appendLine('⚡ Running performance analysis...');
        
        try {
            const result = await this.client.sendRequest('cursed/performanceHints', {
                textDocument: { uri: editor.document.uri.toString() }
            });
            
            if (result && result.hints) {
                const hints = result.hints as PerformanceHint[];
                
                this.outputChannel.appendLine(`Found ${hints.length} performance suggestions:`);
                
                for (const hint of hints) {
                    this.outputChannel.appendLine(`  🚀 ${hint.category}: ${hint.message}`);
                    this.outputChannel.appendLine(`      Suggestion: ${hint.suggestion}`);
                    this.outputChannel.appendLine(`      Location: Line ${hint.location.range.start.line + 1}`);
                    this.outputChannel.appendLine('');
                }
                
                // Store results and update decorations
                this.storeAnalysisResults(editor.document.uri.toString(), 'performance', hints);
                this.updateDecorations(editor.document);
                
                if (hints.length > 0) {
                    vscode.window.showInformationMessage(`Found ${hints.length} performance optimization opportunities`);
                }
            } else {
                this.outputChannel.appendLine('✅ No performance issues found');
                vscode.window.showInformationMessage('No performance issues found');
            }
        } catch (error) {
            this.outputChannel.appendLine(`❌ Performance analysis failed: ${error}`);
            vscode.window.showErrorMessage(`Performance analysis failed: ${error}`);
        }
    }
    
    async runMemoryAnalysis(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        this.outputChannel.show();
        this.outputChannel.appendLine('🧠 Running memory analysis...');
        
        try {
            // For now, implement basic memory analysis patterns
            const memoryIssues = await this.analyzeMemoryPatterns(editor.document);
            
            this.outputChannel.appendLine(`Found ${memoryIssues.length} memory-related issues:`);
            
            for (const issue of memoryIssues) {
                this.outputChannel.appendLine(`  💾 ${issue.type}: ${issue.message}`);
                this.outputChannel.appendLine(`      Location: Line ${issue.line + 1}`);
                this.outputChannel.appendLine('');
            }
            
            if (memoryIssues.length === 0) {
                this.outputChannel.appendLine('✅ No memory issues found');
                vscode.window.showInformationMessage('No memory issues found');
            } else {
                vscode.window.showInformationMessage(`Found ${memoryIssues.length} memory optimization opportunities`);
            }
        } catch (error) {
            this.outputChannel.appendLine(`❌ Memory analysis failed: ${error}`);
            vscode.window.showErrorMessage(`Memory analysis failed: ${error}`);
        }
    }
    
    async runConcurrencyAnalysis(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        this.outputChannel.show();
        this.outputChannel.appendLine('🔄 Running concurrency analysis...');
        
        try {
            const concurrencyIssues = await this.analyzeConcurrencyPatterns(editor.document);
            
            this.outputChannel.appendLine(`Found ${concurrencyIssues.length} concurrency-related issues:`);
            
            for (const issue of concurrencyIssues) {
                this.outputChannel.appendLine(`  ⚙️  ${issue.type}: ${issue.message}`);
                this.outputChannel.appendLine(`      Severity: ${issue.severity}`);
                this.outputChannel.appendLine(`      Location: Line ${issue.line + 1}`);
                this.outputChannel.appendLine('');
            }
            
            if (concurrencyIssues.length === 0) {
                this.outputChannel.appendLine('✅ No concurrency issues found');
                vscode.window.showInformationMessage('No concurrency issues found');
            } else {
                vscode.window.showInformationMessage(`Found ${concurrencyIssues.length} concurrency issues`);
            }
        } catch (error) {
            this.outputChannel.appendLine(`❌ Concurrency analysis failed: ${error}`);
            vscode.window.showErrorMessage(`Concurrency analysis failed: ${error}`);
        }
    }
    
    async runComprehensiveAnalysis(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        this.outputChannel.show();
        this.outputChannel.appendLine('🔍 Running comprehensive analysis...');
        this.outputChannel.appendLine('');
        
        // Run all analyses
        await this.runSecurityAnalysis();
        this.outputChannel.appendLine('');
        await this.runPerformanceAnalysis();
        this.outputChannel.appendLine('');
        await this.runMemoryAnalysis();
        this.outputChannel.appendLine('');
        await this.runConcurrencyAnalysis();
        
        this.outputChannel.appendLine('');
        this.outputChannel.appendLine('✅ Comprehensive analysis complete');
        
        // Show summary
        const results = this.analysisResults.get(editor.document.uri.toString()) || [];
        const totalIssues = results.length;
        
        if (totalIssues > 0) {
            const action = await vscode.window.showInformationMessage(
                `Comprehensive analysis found ${totalIssues} total issues. Generate report?`,
                'Generate Report',
                'Dismiss'
            );
            
            if (action === 'Generate Report') {
                await this.exportAnalysisReport();
            }
        } else {
            vscode.window.showInformationMessage('Comprehensive analysis found no issues! 🎉');
        }
    }
    
    async clearAnalysisResults(): Promise<void> {
        this.analysisResults.clear();
        
        // Clear decorations from all open editors
        for (const editor of vscode.window.visibleTextEditors) {
            if (editor.document.languageId === 'cursed') {
                this.clearDecorations(editor);
            }
        }
        
        this.outputChannel.appendLine('🗑️  Analysis results cleared');
        vscode.window.showInformationMessage('Analysis results cleared');
    }
    
    async exportAnalysisReport(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'cursed') {
            vscode.window.showErrorMessage('No CURSED file is currently active');
            return;
        }
        
        const results = this.analysisResults.get(editor.document.uri.toString()) || [];
        if (results.length === 0) {
            vscode.window.showInformationMessage('No analysis results to export');
            return;
        }
        
        const report = this.generateAnalysisReport(editor.document, results);
        
        // Save report to file
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (workspaceFolder) {
            const fileName = `cursed-analysis-report-${new Date().toISOString().split('T')[0]}.md`;
            const reportPath = vscode.Uri.joinPath(workspaceFolder.uri, fileName);
            
            try {
                await vscode.workspace.fs.writeFile(reportPath, Buffer.from(report));
                const document = await vscode.workspace.openTextDocument(reportPath);
                await vscode.window.showTextDocument(document);
                
                vscode.window.showInformationMessage(`Analysis report exported to ${fileName}`);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to export report: ${error}`);
            }
        }
    }
    
    // Helper methods
    
    private async analyzeMemoryPatterns(document: vscode.TextDocument): Promise<MemoryIssue[]> {
        const issues: MemoryIssue[] = [];
        const text = document.getText();
        const lines = text.split('\n');
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            
            // Check for potential memory leaks
            if (line.includes('malloc') || line.includes('alloc')) {
                if (!this.hasCorrespondingFree(lines, i)) {
                    issues.push({
                        type: 'Memory Leak',
                        message: 'Allocated memory without corresponding free',
                        line: i,
                        severity: 'high'
                    });
                }
            }
            
            // Check for large array allocations
            if (line.includes('[]') && line.includes('new')) {
                const sizeMatch = line.match(/\[(\d+)\]/);
                if (sizeMatch && parseInt(sizeMatch[1]) > 10000) {
                    issues.push({
                        type: 'Large Allocation',
                        message: `Large array allocation (${sizeMatch[1]} elements)`,
                        line: i,
                        severity: 'medium'
                    });
                }
            }
            
            // Check for potential buffer overruns
            if (line.includes('[') && line.includes('+') && !line.includes('len()')) {
                issues.push({
                    type: 'Buffer Access',
                    message: 'Array access without bounds checking',
                    line: i,
                    severity: 'high'
                });
            }
        }
        
        return issues;
    }
    
    private async analyzeConcurrencyPatterns(document: vscode.TextDocument): Promise<ConcurrencyIssue[]> {
        const issues: ConcurrencyIssue[] = [];
        const text = document.getText();
        const lines = text.split('\n');
        
        let hasGoroutines = false;
        let hasChannels = false;
        let hasSharedState = false;
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            
            // Check for goroutines
            if (line.includes('go {')) {
                hasGoroutines = true;
            }
            
            // Check for channels
            if (line.includes('chan<') || line.includes('<-') || line.includes('->')) {
                hasChannels = true;
            }
            
            // Check for shared mutable state
            if (line.includes('sus ') && this.isSharedVariable(line)) {
                hasSharedState = true;
            }
            
            // Check for potential race conditions
            if (hasGoroutines && hasSharedState && !hasChannels) {
                issues.push({
                    type: 'Race Condition',
                    message: 'Potential race condition: shared state without proper synchronization',
                    line: i,
                    severity: 'high'
                });
            }
            
            // Check for deadlock potential
            if (line.includes('<-') && line.includes('bestie')) {
                issues.push({
                    type: 'Deadlock Risk',
                    message: 'Channel operation in loop may cause deadlock',
                    line: i,
                    severity: 'medium'
                });
            }
            
            // Check for channel leaks
            if (line.includes('make_channel()') && !this.hasChannelClose(lines, i)) {
                issues.push({
                    type: 'Resource Leak',
                    message: 'Channel created but never closed',
                    line: i,
                    severity: 'low'
                });
            }
        }
        
        return issues;
    }
    
    private hasCorrespondingFree(lines: string[], allocLine: number): boolean {
        for (let i = allocLine + 1; i < lines.length; i++) {
            if (lines[i].includes('free') || lines[i].includes('dealloc')) {
                return true;
            }
        }
        return false;
    }
    
    private isSharedVariable(line: string): boolean {
        // Simple heuristic - variables that might be shared across goroutines
        return !line.includes('local') && !line.includes('temp') && line.includes('sus ');
    }
    
    private hasChannelClose(lines: string[], channelLine: number): boolean {
        for (let i = channelLine + 1; i < lines.length; i++) {
            if (lines[i].includes('close(') || lines[i].includes('.close()')) {
                return true;
            }
        }
        return false;
    }
    
    private storeAnalysisResults(uri: string, type: string, results: any[]): void {
        const existing = this.analysisResults.get(uri) || [];
        const newResults = results.map(r => ({ ...r, type }));
        this.analysisResults.set(uri, [...existing, ...newResults]);
    }
    
    private createDecorationTypes(): void {
        // Implementation for creating VS Code decoration types
        // for highlighting analysis results in the editor
    }
    
    private updateDecorations(document: vscode.TextDocument): void {
        // Implementation for updating editor decorations
        // based on analysis results
    }
    
    private clearDecorations(editor: vscode.TextEditor): void {
        // Implementation for clearing decorations
    }
    
    private showSecurityReport(vulnerabilities: SecurityVulnerability[]): void {
        // Create a webview panel to show detailed security report
        const panel = vscode.window.createWebviewPanel(
            'cursedSecurityReport',
            'CURSED Security Analysis Report',
            vscode.ViewColumn.Two,
            {
                enableScripts: true
            }
        );
        
        panel.webview.html = this.generateSecurityReportHtml(vulnerabilities);
    }
    
    private generateSecurityReportHtml(vulnerabilities: SecurityVulnerability[]): string {
        return `
        <!DOCTYPE html>
        <html>
        <head>
            <title>CURSED Security Analysis Report</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 20px; }
                .vulnerability { margin: 20px 0; padding: 15px; border-left: 4px solid #f39c12; background: #fdf2e9; }
                .high { border-left-color: #e74c3c; background: #fdedec; }
                .medium { border-left-color: #f39c12; background: #fdf2e9; }
                .low { border-left-color: #f1c40f; background: #fefbea; }
                .category { font-weight: bold; color: #2c3e50; }
                .message { margin: 10px 0; }
                .location { font-size: 0.9em; color: #7f8c8d; }
            </style>
        </head>
        <body>
            <h1>🔒 CURSED Security Analysis Report</h1>
            <p>Found ${vulnerabilities.length} security issues:</p>
            ${vulnerabilities.map(v => `
                <div class="vulnerability ${v.severity}">
                    <div class="category">${v.category}</div>
                    <div class="message">${v.message}</div>
                    <div class="location">Line ${v.location.range.start.line + 1}</div>
                    ${v.cwe_id ? `<div class="cwe">CWE-${v.cwe_id}</div>` : ''}
                </div>
            `).join('')}
        </body>
        </html>
        `;
    }
    
    private generateAnalysisReport(document: vscode.TextDocument, results: AnalysisResult[]): string {
        const fileName = document.fileName.split('/').pop() || 'unknown';
        const date = new Date().toISOString().split('T')[0];
        
        let report = `# CURSED Analysis Report\n\n`;
        report += `**File:** ${fileName}\n`;
        report += `**Date:** ${date}\n`;
        report += `**Total Issues:** ${results.length}\n\n`;
        
        const groupedResults = this.groupResultsByType(results);
        
        for (const [type, typeResults] of Object.entries(groupedResults)) {
            report += `## ${type.charAt(0).toUpperCase() + type.slice(1)} Analysis\n\n`;
            report += `Found ${typeResults.length} issues:\n\n`;
            
            for (const result of typeResults) {
                report += `### ${result.category || result.type}\n\n`;
                report += `**Message:** ${result.message}\n\n`;
                if (result.location) {
                    report += `**Location:** Line ${result.location.range.start.line + 1}\n\n`;
                }
                if (result.severity) {
                    report += `**Severity:** ${result.severity}\n\n`;
                }
                if (result.suggestion) {
                    report += `**Suggestion:** ${result.suggestion}\n\n`;
                }
                report += '---\n\n';
            }
        }
        
        return report;
    }
    
    private groupResultsByType(results: AnalysisResult[]): Record<string, AnalysisResult[]> {
        const grouped: Record<string, AnalysisResult[]> = {};
        
        for (const result of results) {
            const type = result.type || 'other';
            if (!grouped[type]) {
                grouped[type] = [];
            }
            grouped[type].push(result);
        }
        
        return grouped;
    }
}

// Type definitions

interface SecurityVulnerability {
    category: string;
    message: string;
    severity: string;
    location: {
        range: {
            start: { line: number; character: number };
            end: { line: number; character: number };
        };
    };
    cwe_id?: number;
}

interface PerformanceHint {
    category: string;
    message: string;
    suggestion: string;
    location: {
        range: {
            start: { line: number; character: number };
            end: { line: number; character: number };
        };
    };
}

interface MemoryIssue {
    type: string;
    message: string;
    line: number;
    severity: string;
}

interface ConcurrencyIssue {
    type: string;
    message: string;
    line: number;
    severity: string;
}

interface AnalysisResult {
    type?: string;
    category?: string;
    message: string;
    severity?: string;
    suggestion?: string;
    location?: {
        range: {
            start: { line: number; character: number };
            end: { line: number; character: number };
        };
    };
}
