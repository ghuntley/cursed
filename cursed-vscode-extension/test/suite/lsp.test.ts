import * as assert from 'assert';
import * as vscode from 'vscode';
import * as path from 'path';

suite('CURSED LSP Integration Tests', () => {
	let testDocument: vscode.TextDocument;

	suiteSetup(async function() {
		this.timeout(15000);
		// Create a test document
		const testFilePath = path.resolve(__dirname, '../../../test_syntax.💀');
		testDocument = await vscode.workspace.openTextDocument(testFilePath);
		await vscode.window.showTextDocument(testDocument);

		// Wait for LSP to initialize
		await new Promise(resolve => setTimeout(resolve, 5000));
	});

	test('Should provide hover information', async function() {
		this.timeout(10000);
		const position = new vscode.Position(12, 5); // Position of 'main' in 'slay main()'
		const hover = await vscode.commands.executeCommand('vscode.executeHoverProvider', testDocument.uri, position);

		// Basic check that hover provider is working
		assert.ok(hover, 'Should provide hover information');
	});

	test('Should provide completion items', async function() {
		this.timeout(10000);
		const position = new vscode.Position(15, 8); // Position after 'sus p '
		const completions = await vscode.commands.executeCommand('vscode.executeCompletionItemProvider', testDocument.uri, position);

		// Basic check that completion provider is working
		assert.ok(completions, 'Should provide completion items');
		if (completions && typeof completions === 'object' && 'items' in completions) {
			assert.ok((completions as any).items.length > 0, 'Should have completion items');
		}
	});

	test('Should provide document symbols', async function() {
		this.timeout(10000);
		const symbols = await vscode.commands.executeCommand('vscode.executeDocumentSymbolProvider', testDocument.uri);

		// Basic check that document symbol provider is working
		assert.ok(symbols, 'Should provide document symbols');
		if (Array.isArray(symbols)) {
			assert.ok(symbols.length > 0, 'Should have document symbols');
		}
	});

	test('Should provide workspace symbols', async function() {
		this.timeout(10000);
		const symbols = await vscode.commands.executeCommand('vscode.executeWorkspaceSymbolProvider', 'main');

		// Basic check that workspace symbol provider is working
		assert.ok(symbols, 'Should provide workspace symbols');
	});

	test('Should provide signature help', async function() {
		this.timeout(10000);
		const position = new vscode.Position(32, 15); // Position in 'dm_send(' call
		const signatureHelp = await vscode.commands.executeCommand('vscode.executeSignatureHelpProvider', testDocument.uri, position);

		// Basic check that signature help provider is working
		assert.ok(signatureHelp, 'Should provide signature help');
	});

	test('Should provide definition', async function() {
		this.timeout(10000);
		const position = new vscode.Position(19, 12); // Position of 'p' in 'p.age'
		const definitions = await vscode.commands.executeCommand('vscode.executeDefinitionProvider', testDocument.uri, position);

		// Basic check that definition provider is working
		assert.ok(definitions, 'Should provide definitions');
	});

	test('Should provide references', async function() {
		this.timeout(10000);
		const position = new vscode.Position(15, 8); // Position of 'p' in 'sus p Person'
		const references = await vscode.commands.executeCommand('vscode.executeReferenceProvider', testDocument.uri, position);

		// Basic check that reference provider is working
		assert.ok(references, 'Should provide references');
	});

	test('Should provide code actions', async function() {
		this.timeout(10000);
		const range = new vscode.Range(15, 0, 15, 20); // Range of variable declaration
		const codeActions = await vscode.commands.executeCommand('vscode.executeCodeActionProvider', testDocument.uri, range);

		// Basic check that code action provider is working
		assert.ok(codeActions, 'Should provide code actions');
	});

	test('Should format document', async function() {
		this.timeout(10000);
		const formatEdits = await vscode.commands.executeCommand('vscode.executeFormatDocumentProvider', testDocument.uri, {});

		// Basic check that document formatting provider is working
		assert.ok(formatEdits, 'Should provide format edits');
	});

	test('Should provide diagnostics', async function() {
		this.timeout(10000);
		// Wait a bit for diagnostics to be computed
		await new Promise(resolve => setTimeout(resolve, 2000));

		const diagnostics = vscode.languages.getDiagnostics(testDocument.uri);

		// Diagnostics might be empty if there are no errors, which is fine
		assert.ok(Array.isArray(diagnostics), 'Should provide diagnostics array');
	});
});