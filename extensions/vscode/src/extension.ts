import * as vscode from 'vscode';
import { FluxxAgentClient } from './client';
import { IntegrationsProvider } from './integrationsProvider';

let statusBarItem: vscode.StatusBarItem;
let refreshInterval: NodeJS.Timer;

export function activate(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('fluxx');
    const apiUrl = config.get<string>('apiUrl') || 'http://localhost:8080';
    const client = new FluxxAgentClient(apiUrl);

    // Status bar item
    if (config.get<boolean>('showStatusBar')) {
        statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Right,
            100
        );
        statusBarItem.command = 'fluxx.showOverview';
        context.subscriptions.push(statusBarItem);
        
        // Initial update
        updateStatusBar(client);
        
        // Periodic updates
        const interval = (config.get<number>('refreshInterval') || 30) * 1000;
        refreshInterval = setInterval(() => updateStatusBar(client), interval);
    }

    // Tree view for integrations
    const integrationsProvider = new IntegrationsProvider(client);
    vscode.window.registerTreeDataProvider('fluxx.integrations', integrationsProvider);

    // Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('fluxx.showOverview', async () => {
            const panel = vscode.window.createWebviewPanel(
                'fluxxOverview',
                'Fluxx Infrastructure Overview',
                vscode.ViewColumn.One,
                { enableScripts: true }
            );

            try {
                const health = await client.getHealth();
                const metrics = await client.getMetrics();
                const events = await client.getEvents(5);
                panel.webview.html = generateOverviewHtml(health, metrics, events);
            } catch (e) {
                panel.webview.html = `<h1>Error</h1><p>Failed to connect to Fluxx Agent: ${e}</p>`;
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('fluxx.healthCheck', async () => {
            try {
                const health = await client.getHealth();
                const healthy = Object.values(health).filter((h: any) => 
                    JSON.stringify(h).includes('Healthy')
                ).length;
                const total = Object.keys(health).length;
                
                vscode.window.showInformationMessage(
                    `Fluxx Health: ${healthy}/${total} integrations healthy`
                );
            } catch (e) {
                vscode.window.showErrorMessage(`Failed to check health: ${e}`);
            }
        })
    );

    // Set context
    vscode.commands.executeCommand('setContext', 'fluxx.enabled', true);
}

async function updateStatusBar(client: FluxxAgentClient) {
    try {
        const health = await client.getHealth();
        const healthy = Object.values(health).filter((h: any) => 
            JSON.stringify(h).includes('Healthy')
        ).length;
        const total = Object.keys(health).length;
        
        const allHealthy = healthy === total;
        statusBarItem.text = allHealthy 
            ? `$(check) Fluxx: ${healthy}/${total}` 
            : `$(alert) Fluxx: ${healthy}/${total}`;
        statusBarItem.tooltip = `Fluxx Agent: ${healthy}/${total} integrations healthy`;
        statusBarItem.show();
    } catch (e) {
        statusBarItem.text = '$(error) Fluxx: Disconnected';
        statusBarItem.tooltip = 'Cannot connect to Fluxx Agent';
        statusBarItem.show();
    }
}

function generateOverviewHtml(health: any, metrics: any, events: any): string {
    const healthyCount = Object.values(health).filter((h: any) => 
        JSON.stringify(h).includes('Healthy')
    ).length;
    const totalCount = Object.keys(health).length;
    
    return `<!DOCTYPE html>
    <html>
    <body>
        <h1>Infrastructure Overview</h1>
        <p>${healthyCount}/${totalCount} integrations healthy</p>
        <h2>Events</h2>
        ${events.length === 0 ? '<p>No events</p>' : events.map((e: any) => 
            `<div>${e.severity}: ${e.title}</div>`
        ).join('')}
    </body>
    </html>`;
}

export function deactivate() {
    if (refreshInterval) {
        clearInterval(refreshInterval);
    }
}
