import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind,
	ExecutableOptions,
	Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;
let outputChannel: vscode.OutputChannel;
let extensionContext: vscode.ExtensionContext;

function getBundledServerPath(context: vscode.ExtensionContext): string | null {
	const platform = process.platform;
	const arch = process.arch;

	// Map Node.js platform/arch to our binary naming
	let binaryName: string;
	switch (platform) {
		case 'win32':
			binaryName = arch === 'x64' ? 'quickmark-server-x86_64-pc-windows-msvc.exe' : 'quickmark-server-i686-pc-windows-msvc.exe';
			break;
		case 'darwin':
			binaryName = arch === 'arm64' ? 'quickmark-server-aarch64-apple-darwin' : 'quickmark-server-x86_64-apple-darwin';
			break;
		case 'linux':
			binaryName = arch === 'arm64' ? 'quickmark-server-aarch64-unknown-linux-gnu' : 'quickmark-server-x86_64-unknown-linux-gnu';
			break;
		default:
			outputChannel.appendLine(`Unsupported platform: ${platform}-${arch}`);
			return null;
	}

	const serverPath = path.join(context.extensionPath, 'bin', binaryName);

	// Check if the bundled binary exists and is executable
	if (fs.existsSync(serverPath)) {
		try {
			fs.accessSync(serverPath, fs.constants.F_OK | fs.constants.X_OK);
			return serverPath;
		} catch (error) {
			outputChannel.appendLine(`Bundled server binary is not executable: ${serverPath}`);
		}
	} else {
		outputChannel.appendLine(`Bundled server binary not found: ${serverPath}`);
	}

	return null;
}

export function activate(context: vscode.ExtensionContext) {
	extensionContext = context;
	outputChannel = vscode.window.createOutputChannel('QuickMark');

	// Register commands
	const restartCommand = vscode.commands.registerCommand('quickmark.restartServer', restartServer);
	const showOutputCommand = vscode.commands.registerCommand('quickmark.showOutput', () => {
		outputChannel.show();
	});

	context.subscriptions.push(restartCommand, showOutputCommand, outputChannel);

	// Start the language server
	startLanguageServer(context);
}

function startLanguageServer(context: vscode.ExtensionContext) {
	const config = vscode.workspace.getConfiguration('quickmark');
	const customServerPath = config.get<string>('serverPath');

	// Determine server path - prefer bundled binary, fallback to custom path
	const serverPath = customServerPath || getBundledServerPath(context);

	if (!serverPath) {
		const errorMsg = 'QuickMark server binary not found. Please install quickmark-server or configure the server path.';
		outputChannel.appendLine(errorMsg);
		vscode.window.showErrorMessage(errorMsg);
		return;
	}

	outputChannel.appendLine(`Using QuickMark server at: ${serverPath}`);

	// Create executable options
	const executableOptions: ExecutableOptions = {
		cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
	};

	// Configure server executable
	const serverExecutable: Executable = {
		command: serverPath,
		transport: TransportKind.stdio,
		options: executableOptions
	};

	const serverOptions: ServerOptions = serverExecutable;

	// Configure client options
	const clientOptions: LanguageClientOptions = {
		documentSelector: [
			{ scheme: 'file', language: 'markdown' },
			{ scheme: 'untitled', language: 'markdown' }
		],
		synchronize: {
			// Synchronize configuration changes
			configurationSection: 'quickmark',
			// Watch for quickmark.toml config file changes
			fileEvents: [
				vscode.workspace.createFileSystemWatcher('**/quickmark.toml'),
				vscode.workspace.createFileSystemWatcher('**/.quickmark.toml')
			]
		},
		outputChannel,
		traceOutputChannel: outputChannel,
		initializationOptions: {
			// Pass any initialization options here
		},
		middleware: {
			// Add any middleware here if needed
		}
	};

	// Create and start the language client
	client = new LanguageClient(
		'quickmark',
		'QuickMark Language Server',
		serverOptions,
		clientOptions
	);

	// Configure trace level from settings
	const traceLevel = config.get<string>('trace.server', 'off');
	client.setTrace(traceLevel as any);

	// Handle server startup
	client.start().then(() => {
		outputChannel.appendLine('QuickMark language server started successfully');

		// Listen for configuration changes
		const configChangeListener = vscode.workspace.onDidChangeConfiguration((event) => {
			if (event.affectsConfiguration('quickmark')) {
				handleConfigurationChange();
			}
		});

		context.subscriptions.push(configChangeListener);

	}).catch((error) => {
		outputChannel.appendLine(`Failed to start QuickMark language server: ${error.message}`);
		vscode.window.showErrorMessage(
			`Failed to start QuickMark language server. Please ensure quickmark-server is installed and accessible. Error: ${error.message}`
		);
	});

	context.subscriptions.push(client);
}

function handleConfigurationChange() {
	const config = vscode.workspace.getConfiguration('quickmark');

	// Update trace level
	const traceLevel = config.get<string>('trace.server', 'off');
	if (client) {
		client.setTrace(traceLevel as any);
	}

	// For server path changes, we need to restart the server
	// This is handled by the restart command for now
	outputChannel.appendLine('QuickMark configuration changed');
}

async function restartServer() {
	if (client) {
		outputChannel.appendLine('Stopping QuickMark language server...');
		await client.stop();
	}

	outputChannel.appendLine('Starting QuickMark language server...');

	const config = vscode.workspace.getConfiguration('quickmark');
	const customServerPath = config.get<string>('serverPath');

	// Determine server path - prefer bundled binary, fallback to custom path
	const serverPath = customServerPath || getBundledServerPath(extensionContext);

	if (!serverPath) {
		const errorMsg = 'QuickMark server binary not found. Please install quickmark-server or configure the server path.';
		outputChannel.appendLine(errorMsg);
		vscode.window.showErrorMessage(errorMsg);
		return;
	}

	outputChannel.appendLine(`Using QuickMark server at: ${serverPath}`);

	const executableOptions: ExecutableOptions = {
		cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
	};

	const serverExecutable: Executable = {
		command: serverPath,
		transport: TransportKind.stdio,
		options: executableOptions
	};

	const serverOptions: ServerOptions = serverExecutable;

	const clientOptions: LanguageClientOptions = {
		documentSelector: [
			{ scheme: 'file', language: 'markdown' },
			{ scheme: 'untitled', language: 'markdown' }
		],
		synchronize: {
			configurationSection: 'quickmark',
			fileEvents: [
				vscode.workspace.createFileSystemWatcher('**/quickmark.toml'),
				vscode.workspace.createFileSystemWatcher('**/.quickmark.toml')
			]
		},
		outputChannel,
		traceOutputChannel: outputChannel
	};

	client = new LanguageClient(
		'quickmark',
		'QuickMark Language Server',
		serverOptions,
		clientOptions
	);

	// Configure trace level
	const traceLevel = config.get<string>('trace.server', 'off');
	client.setTrace(traceLevel as any);

	try {
		await client.start();
		outputChannel.appendLine('QuickMark language server restarted successfully');
		vscode.window.showInformationMessage('QuickMark language server restarted');
	} catch (error: any) {
		outputChannel.appendLine(`Failed to restart QuickMark language server: ${error.message}`);
		vscode.window.showErrorMessage(`Failed to restart QuickMark language server: ${error.message}`);
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}

	outputChannel.appendLine('Stopping QuickMark language server...');
	return client.stop();
}
