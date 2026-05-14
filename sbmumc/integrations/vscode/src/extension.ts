// SBMUMC OmniDev VSCode Extension
// Instantaneous AI coding assistant

import * as vscode from 'vscode';
import { LanguageClient, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
    console.log('SBMUMC OmniDev AGI extension activating...');

    const config = vscode.workspace.getConfiguration('sbmumc');
    const endpoint = config.get<string>('endpoint', 'http://localhost:8080');
    const mode = config.get<string>('mode', 'hybrid');

    // Register all SBMUMC commands
    const commands = [
        { command: 'sbmumc.omnidev', handler: showOmniDevPanel },
        { command: 'sbmumc.refactor', handler: handleRefactor },
        { command: 'sbmumc.search', handler: handleSemanticSearch },
        { command: 'sbmumc.commit', handler: handleCommitMessage },
        { command: 'sbmumc.test', handler: handleTestGeneration },
        { command: 'sbmumc.verify', handler: handleFormalVerification },
        { command: 'sbmumc.status', handler: showSystemStatus }
    ];

    commands.forEach(({ command, handler }) => {
        const disposable = vscode.commands.registerCommand(command, handler);
        context.subscriptions.push(disposable);
    });

    // Start Language Server
    startLanguageServer(context, endpoint);

    // Register providers
    vscode.languages.registerCodeLensProvider(
        { scheme: 'file', language: '*' },
        new OmniDevCodeLensProvider()
    );

    if (config.get<boolean>('autocomplete', true)) {
        registerInlineCompletionProvider();
    }

    console.log(`SBMUMC OmniDev AGI activated (mode: ${mode})`);
}

async function showOmniDevPanel() {
    const panel = vscode.window.createWebviewPanel(
        'sbmumc-omnidev', 'SBMUMC OmniDev',
        vscode.ViewColumn.Beside,
        { enableScripts: true }
    );
    panel.webview.html = getWebviewContent();
}

async function handleRefactor() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const code = editor.document.getText(editor.selection);
    const result = await callOmniDevAPI('/api/v1/omnidev', { action: 'refactor', code });
    if (result?.suggestions) {
        const choice = await vscode.window.showQuickPick(
            result.suggestions.map((s: any) => s.description)
        );
        if (choice) {
            const suggestion = result.suggestions.find((s: any) => s.description === choice);
            if (suggestion) {
                editor.edit(builder => builder.replace(editor.selection, suggestion.code));
            }
        }
    }
}

async function handleSemanticSearch() {
    const query = await vscode.window.showInputBox({ prompt: 'Enter search query' });
    if (query) {
        const results = await callOmniDevAPI('/api/v1/graph/search', { query });
        const items = results.slice(0, 10).map((r: any) => ({
            label: r.file, detail: r.context, resourceUri: vscode.Uri.file(r.file)
        }));
        const selected = await vscode.window.showQuickPick(items);
        if (selected?.resourceUri) {
            const doc = await vscode.workspace.openTextDocument(selected.resourceUri);
            await vscode.window.showTextDocument(doc);
        }
    }
}

async function handleCommitMessage() {
    const diffs = await getGitDiffs();
    const commit = await callOmniDevAPI('/api/v1/commit', { diffs, conventional: true });
    if (commit) {
        vscode.window.showInformationMessage(`Commit: ${commit.message}`);
    }
}

async function handleTestGeneration() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const code = editor.document.getText();
    const tests = await callOmniDevAPI('/api/v1/test/generate', { code, language: editor.document.languageId });
    if (tests) {
        const testUri = editor.document.uri.with({ path: editor.document.uri.path + '.test.ts' });
        await vscode.workspace.writeFile(testUri, Buffer.from(tests.content));
        await vscode.window.showTextDocument(testUri);
    }
}

async function handleFormalVerification() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const result = await callOmniDevAPI('/api/v1/verify', {
        code: editor.document.getText(),
        claims: ['safety', 'termination']
    });
    if (result) {
        vscode.window.showQuickPick(
            result.proofs.map((p: any) => ({
                label: p.claim, detail: p.proven ? 'Proven' : 'Not proven'
            }))
        );
    }
}

async function showSystemStatus() {
    const status = await callOmniDevAPI('/api/v1/status', {});
    vscode.window.showInformationMessage(
        `SBMUMC: ${status.mode} | Latency: ${status.latency_p99_ms}ms | Nodes: ${status.nodes_indexed}`
    );
}

function startLanguageServer(context: vscode.ExtensionContext, endpoint: string) {
    const serverOptions = {
        run: { command: 'sbmumc-lsp', args: ['--endpoint', endpoint], transport: TransportKind.stdio },
        debug: { command: 'sbmumc-lsp', args: ['--endpoint', endpoint, '--debug'] }
    };
    client = new LanguageClient('sbmumc-lsp', 'SBMUMC LSP', serverOptions, true);
    client.start();
}

async function callOmniDevAPI(endpoint: string, body: any): Promise<any> {
    const config = vscode.workspace.getConfiguration('sbmumc');
    const baseUrl = config.get<string>('endpoint', 'http://localhost:8080');
    try {
        const response = await fetch(`${baseUrl}${endpoint}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(body)
        });
        return await response.json();
    } catch (error) {
        console.error('SBMUMC API error:', error);
        return null;
    }
}

async function getGitDiffs(): Promise<string[]> { return []; }

function getWebviewContent(): string {
    return `<!DOCTYPE html><html><head><style>
        body { font-family: system-ui; padding: 20px; background: #1e1e1e; color: #fff; }
        .chat-container { max-width: 800px; margin: 0 auto; }
        .message { margin: 10px 0; padding: 10px; border-radius: 5px; }
        .user { background: #0066cc; }
        .assistant { background: #2d2d2d; }
        input { width: 100%; padding: 10px; margin-top: 10px; }
    </style></head><body>
    <div class="chat-container">
        <h1>SBMUMC OmniDev Assistant</h1>
        <div id="messages"></div>
        <input type="text" id="input" placeholder="Ask me anything..." />
    </div>
    <script>
        const input = document.getElementById('input');
        const messages = document.getElementById('messages');
        input.addEventListener('keypress', async (e) => {
            if (e.key === 'Enter') {
                const query = input.value;
                messages.innerHTML += '<div class="message user">' + query + '</div>';
                input.value = '';
                const response = await fetch('/api/v1/omnidev', {
                    method: 'POST', headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ query })
                });
                const data = await response.json();
                messages.innerHTML += '<div class="message assistant">' + data.response + '</div>';
            }
        });
    </script>
</body></html>`;
}

class OmniDevCodeLensProvider implements vscode.CodeLensProvider {
    provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
        const lenses: vscode.CodeLens[] = [];
        for (let i = 0; i < document.lineCount; i++) {
            const line = document.lineAt(i);
            if (line.text.includes('TODO') || line.text.includes('FIXME')) {
                lenses.push(new vscode.CodeLens(line.range, {
                    title: 'SBMUMC Fix', command: 'sbmumc.omnidev'
                }));
            }
        }
        return lenses;
    }
}

function registerInlineCompletionProvider() {
    vscode.languages.registerInlineCompletionItemProvider(
        { scheme: 'file', language: '*' },
        {
            provideInlineCompletionItems: async (document, position) => {
                const config = vscode.workspace.getConfiguration('sbmumc');
                if (!config.get<boolean>('autocomplete', true)) return [];
                const prefix = document.lineAt(position.line).text.substring(0, position.character);
                const completion = await callOmniDevAPI('/api/v1/complete', { prefix, language: document.languageId });
                if (completion) {
                    return [{ insertText: completion.text, range: new vscode.Range(position, position) }];
                }
                return [];
            }
        }
    );
}

export function deactivate() { if (client) client.stop(); }