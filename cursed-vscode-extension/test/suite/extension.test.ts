import * as assert from 'assert';
import { after } from 'mocha';

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from 'vscode';
// import * as myExtension from '../../src/extension';

suite('CURSED Extension Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('Extension should be present', () => {
		assert.ok(vscode.extensions.getExtension('cursed-lang.cursed-language'));
	});

	test('Extension should activate', function () {
		this.timeout(10000);
		return vscode.extensions.getExtension('cursed-lang.cursed-language')!.activate().then((api) => {
			assert.ok(true, "Extension activated");
		});
	});

	test('Should register CURSED language', () => {
		const languages = vscode.languages.getLanguages();
		return languages.then(langs => {
			assert.ok(langs.includes('cursed'), 'CURSED language should be registered');
		});
	});

	test('Should have CURSED file extensions', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.languages);
		const cursedLang = packageJSON.contributes.languages.find((lang: any) => lang.id === 'cursed');
		assert.ok(cursedLang);
		assert.ok(cursedLang.extensions.includes('.💀'));
		assert.ok(cursedLang.extensions.includes('.cursed'));
	});

	test('Should have language configuration', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.languages);
		const cursedLang = packageJSON.contributes.languages.find((lang: any) => lang.id === 'cursed');
		assert.ok(cursedLang.configuration);
	});

	test('Should have syntax highlighting', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.grammars);
		const cursedGrammar = packageJSON.contributes.grammars.find((g: any) => g.language === 'cursed');
		assert.ok(cursedGrammar);
		assert.ok(cursedGrammar.scopeName === 'source.cursed');
	});

	test('Should have snippets', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.snippets);
		const cursedSnippets = packageJSON.contributes.snippets.find((s: any) => s.language === 'cursed');
		assert.ok(cursedSnippets);
	});

	test('Should have themes', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.themes);
		assert.ok(packageJSON.contributes.themes.length >= 2);
	});

	test('Should have commands', () => {
		const cursedExtension = vscode.extensions.getExtension('cursed-lang.cursed-language');
		assert.ok(cursedExtension);
		const packageJSON = cursedExtension!.packageJSON;
		assert.ok(packageJSON.contributes.commands);
		assert.ok(packageJSON.contributes.commands.length > 0);
		const commands = packageJSON.contributes.commands.map((c: any) => c.command);
		assert.ok(commands.includes('cursed.run'));
		assert.ok(commands.includes('cursed.build'));
		assert.ok(commands.includes('cursed.format'));
		assert.ok(commands.includes('cursed.lint'));
	});
});