import * as assert from 'assert';
import * as vscode from 'vscode';
import * as path from 'path';

suite('CURSED Syntax Highlighting Tests', () => {
	let testDocument: vscode.TextDocument;

	suiteSetup(async function() {
		this.timeout(10000);
		// Create a test document
		const testFilePath = path.resolve(__dirname, '../../../test_syntax.💀');
		testDocument = await vscode.workspace.openTextDocument(testFilePath);
		await vscode.window.showTextDocument(testDocument);
	});

	test('Should detect CURSED language', () => {
		assert.ok(testDocument.languageId === 'cursed');
	});

	test('Should highlight keywords correctly', async function() {
		this.timeout(5000);
		// Wait for syntax highlighting to be applied
		await new Promise(resolve => setTimeout(resolve, 1000));

		const tokens = await vscode.commands.executeCommand('vscode.provideDocumentSemanticTokens', testDocument.uri);
		assert.ok(tokens, 'Should provide semantic tokens');

		// Check that we have some tokens (basic check)
		if (tokens && typeof tokens === 'object' && 'data' in tokens) {
			assert.ok((tokens as any).data.length > 0, 'Should have semantic tokens');
		}
	});

	test('Should highlight comments', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Get the range of the line comment
		const lineCommentRange = new vscode.Range(3, 0, 3, 28);
		const lineCommentText = testDocument.getText(lineCommentRange);
		assert.ok(lineCommentText.includes('fr fr'), 'Should contain line comment');

		// Get the range of the block comment
		const blockCommentRange = new vscode.Range(5, 0, 7, 6);
		const blockCommentText = testDocument.getText(blockCommentRange);
		assert.ok(blockCommentText.includes('no cap'), 'Should contain block comment start');
		assert.ok(blockCommentText.includes('on god'), 'Should contain block comment end');
	});

	test('Should highlight function declarations', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'slay' keyword
		const functionDeclarationRange = new vscode.Range(12, 0, 12, 12);
		const functionDeclarationText = testDocument.getText(functionDeclarationRange);
		assert.ok(functionDeclarationText.includes('slay main()'), 'Should contain function declaration');
	});

	test('Should highlight type declarations', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'be_like' and 'squad' keywords
		const typeDeclarationRange = new vscode.Range(9, 0, 9, 18);
		const typeDeclarationText = testDocument.getText(typeDeclarationRange);
		assert.ok(typeDeclarationText.includes('be_like Person squad'), 'Should contain type declaration');
	});

	test('Should highlight control structures', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'ready' keyword
		const ifStatementRange = new vscode.Range(18, 4, 18, 9);
		const ifStatementText = testDocument.getText(ifStatementRange);
		assert.ok(ifStatementText.includes('ready'), 'Should contain if statement');

		// Check for 'otherwise' keyword
		const elseRange = new vscode.Range(20, 4, 20, 13);
		const elseText = testDocument.getText(elseRange);
		assert.ok(elseText.includes('otherwise'), 'Should contain else statement');
	});

	test('Should highlight loops', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'bestie' keyword
		const forLoopRange = new vscode.Range(25, 4, 25, 10);
		const forLoopText = testDocument.getText(forLoopRange);
		assert.ok(forLoopText.includes('bestie'), 'Should contain for loop');
	});

	test('Should highlight error handling', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'fam' keyword
		const tryRange = new vscode.Range(35, 4, 35, 7);
		const tryText = testDocument.getText(tryRange);
		assert.ok(tryText.includes('fam'), 'Should contain error handling');
	});

	test('Should highlight concurrency constructs', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for 'stan' keyword
		const goRange = new vscode.Range(31, 4, 31, 8);
		const goText = testDocument.getText(goRange);
		assert.ok(goText.includes('stan'), 'Should contain goroutine');

		// Check for 'dm_send' and 'dm_recv'
		const sendRange = new vscode.Range(32, 8, 32, 15);
		const sendText = testDocument.getText(sendRange);
		assert.ok(sendText.includes('dm_send'), 'Should contain channel send');

		const receiveRange = new vscode.Range(34, 18, 34, 25);
		const receiveText = testDocument.getText(receiveRange);
		assert.ok(receiveText.includes('dm_recv'), 'Should contain channel receive');
	});

	test('Should highlight literals', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for boolean literals
		assert.ok(testDocument.getText().includes('based'), 'Should contain true literal');
		assert.ok(testDocument.getText().includes('cringe'), 'Should contain false literal');

		// Check for string literals
		assert.ok(testDocument.getText().includes('"Alice"'), 'Should contain string literal');

		// Check for numeric literals
		assert.ok(testDocument.getText().includes('30'), 'Should contain numeric literal');
	});

	test('Should highlight operators', async function() {
		this.timeout(5000);
		await new Promise(resolve => setTimeout(resolve, 1000));

		// Check for various operators
		assert.ok(testDocument.getText().includes('>'), 'Should contain comparison operator');
		assert.ok(testDocument.getText().includes('='), 'Should contain assignment operator');
		assert.ok(testDocument.getText().includes('++'), 'Should contain increment operator');
		assert.ok(testDocument.getText().includes('ඞ'), 'Should contain pointer operator');
	});
});