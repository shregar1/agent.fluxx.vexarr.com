import * as vscode from 'vscode';
import { FluxxAgentClient } from './client';

export class IntegrationsProvider implements vscode.TreeDataProvider<IntegrationItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<IntegrationItem | undefined>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private client: FluxxAgentClient) {
        // Refresh every 30 seconds
        setInterval(() => this.refresh(), 30000);
    }

    refresh(): void {
        this._onDidChangeTreeData.fire(undefined);
    }

    getTreeItem(element: IntegrationItem): vscode.TreeItem {
        return element;
    }

    async getChildren(): Promise<IntegrationItem[]> {
        try {
            const health = await this.client.getHealth();
            return Object.entries(health).map(([name, status]) => {
                const healthStatus = status as any;
                return new IntegrationItem(
                    name,
                    healthStatus.status || 'Unknown',
                    healthStatus.last_check,
                    healthStatus.latency_ms,
                    vscode.TreeItemCollapsibleState.None
                );
            });
        } catch {
            return [new IntegrationItem('Connection failed', 'Error', '', undefined)];
        }
    }
}

class IntegrationItem extends vscode.TreeItem {
    constructor(
        public readonly name: string,
        public readonly status: string,
        public readonly lastCheck: string,
        public readonly latency?: number
    ) {
        super(name, vscode.TreeItemCollapsibleState.None);
        
        this.tooltip = this.getTooltip();
        this.description = this.getDescription();
        this.iconPath = this.getIcon();
        this.contextValue = 'integration';
        
        this.command = {
            command: 'fluxx.showIntegrationDetails',
            title: 'Show Details',
            arguments: [this]
        };
    }

    private getDescription(): string {
        const latencyText = this.latency ? `${this.latency}ms` : '';
        return `${this.status} ${latencyText}`;
    }

    private getTooltip(): string {
        return `${this.name}\nStatus: ${this.status}\nLast Check: ${this.lastCheck}`;
    }

    private getIcon(): vscode.ThemeIcon {
        switch (this.status.toLowerCase()) {
            case 'healthy':
                return new vscode.ThemeIcon('check', new vscode.ThemeColor('testing.iconPassed'));
            case 'degraded':
                return new vscode.ThemeIcon('warning', new vscode.ThemeColor('testing.iconQueued'));
            case 'unhealthy':
            case 'error':
            case 'failed':
                return new vscode.ThemeIcon('error', new vscode.ThemeColor('testing.iconFailed'));
            default:
                return new vscode.ThemeIcon('question');
        }
    }
}
