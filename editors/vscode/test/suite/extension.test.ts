import * as assert from 'assert';
import * as vscode from 'vscode';

suite('CURSED Extension Test Suite', () => {
    vscode.window.showInformationMessage('Start all tests.');

    test('Extension should be present', () => {
        assert.ok(vscode.extensions.getExtension('cursed-lang.cursed-language-support'));
    });

    test('Extension should activate', async () => {
        const extension = vscode.extensions.getExtension('cursed-lang.cursed-language-support');
        if (extension) {
            await extension.activate();
            assert.strictEqual(extension.isActive, true);
        }
    });

    test('CURSED language should be registered', () => {
        const languages = vscode.languages.getLanguages();
        assert.ok(languages.then(langs => langs.includes('cursed')));
    });

    test('Commands should be registered', async () => {
        const commands = await vscode.commands.getCommands();
        
        const expectedCommands = [
            'cursed.restartLanguageServer',
            'cursed.newProject',
            'cursed.build',
            'cursed.run',
            'cursed.test',
            'cursed.clean',
            'cursed.openRepl',
            'cursed.formatDocument',
            'cursed.runLinter'
        ];

        for (const expectedCommand of expectedCommands) {
            assert.ok(
                commands.includes(expectedCommand),
                `Command ${expectedCommand} should be registered`
            );
        }
    });
});
