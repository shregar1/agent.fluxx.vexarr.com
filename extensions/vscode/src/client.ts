export interface HealthResponse {
    [key: string]: {
        status: string;
        last_check: string;
        message?: string;
        latency_ms?: number;
    };
}

export interface MetricResponse {
    timestamp: number;
    values: Array<{
        metric: string;
        value: number;
        labels?: Record<string, string>;
    }>;
}

export interface EventResponse {
    id: string;
    timestamp: string;
    severity: 'critical' | 'error' | 'warning' | 'info';
    title: string;
    source: string;
    details: Record<string, any>;
}

export class FluxxAgentClient {
    private baseUrl: string;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }

    private async fetch<T>(path: string): Promise<T> {
        const response = await fetch(`${this.baseUrl}${path}`);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        return response.json();
    }

    async getHealth(): Promise<HealthResponse> {
        return this.fetch('/health');
    }

    async getMetrics(): Promise<MetricResponse[]> {
        return this.fetch('/metrics');
    }

    async getMetricsForIntegration(name: string): Promise<MetricResponse> {
        return this.fetch(`/metrics/${name}`);
    }

    async getEvents(limit: number = 10): Promise<EventResponse[]> {
        return this.fetch(`/events?limit=${limit}`);
    }

    async getIntegrations(): Promise<string[]> {
        return this.fetch('/integrations');
    }

    async runDiagnostic(integration: string, checkName?: string): Promise<any> {
        const url = checkName 
            ? `/diagnostic/${integration}/${checkName}`
            : `/diagnostic/${integration}`;
        return this.fetch(url);
    }

    async remediate(integration: string, issue: string): Promise<any> {
        const response = await fetch(`${this.baseUrl}/remediate`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ integration, issue })
        });
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        return response.json();
    }
}
