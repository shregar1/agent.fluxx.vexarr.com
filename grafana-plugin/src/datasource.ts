/**
 * Fluxx Grafana Data Source Plugin
 *
 * Translates Grafana queries into Fluxx PromQL API calls.
 * Supports: metrics, logs, annotations (deploy markers).
 */

import {
  DataQueryRequest,
  DataQueryResponse,
  DataSourceApi,
  DataSourceInstanceSettings,
  MutableDataFrame,
  FieldType,
  AnnotationQueryRequest,
  AnnotationEvent,
} from '@grafana/data';

interface FluxxQuery {
  refId: string;
  queryText: string;
  queryType: 'metrics' | 'logs';
}

interface FluxxDataSourceOptions {
  url: string;
  apiKey?: string;
}

interface FluxxApiResponse {
  transactionUrn: string;
  status: 'SUCCESS' | 'FAILED';
  responseMessage: string;
  data: any;
}

export class FluxxDataSource extends DataSourceApi<FluxxQuery, FluxxDataSourceOptions> {
  url: string;
  apiKey?: string;

  constructor(instanceSettings: DataSourceInstanceSettings<FluxxDataSourceOptions>) {
    super(instanceSettings);
    this.url = instanceSettings.jsonData.url || instanceSettings.url || 'http://localhost:7007';
    this.apiKey = instanceSettings.jsonData.apiKey;
  }

  /**
   * Main query method — called by Grafana for each panel refresh.
   */
  async query(options: DataQueryRequest<FluxxQuery>): Promise<DataQueryResponse> {
    const { range } = options;
    const from = range.from.valueOf() * 1000; // Convert ms to µs
    const to = range.to.valueOf() * 1000;

    const promises = options.targets
      .filter((t) => !t.hide && t.queryText)
      .map(async (target) => {
        if (target.queryType === 'logs') {
          return this.queryLogs(target, from, to);
        }
        return this.queryMetrics(target, from, to, options.maxDataPoints || 1000);
      });

    const results = await Promise.all(promises);
    return { data: results.flat() };
  }

  /**
   * Query metrics via the PromQL endpoint.
   */
  private async queryMetrics(
    target: FluxxQuery,
    fromUs: number,
    toUs: number,
    maxPoints: number,
  ): Promise<MutableDataFrame[]> {
    const lookback = Math.floor((toUs - fromUs) / 1_000_000); // seconds
    const params = new URLSearchParams({
      q: target.queryText,
      lookback: lookback.toString(),
    });

    const resp = await this.doRequest<FluxxApiResponse>(`/v1/query?${params}`);
    if (resp.status !== 'SUCCESS') {
      throw new Error(resp.responseMessage);
    }

    const data = resp.data;
    const frame = new MutableDataFrame({
      refId: target.refId,
      fields: [
        { name: 'Time', type: FieldType.time },
        { name: target.queryText, type: FieldType.number },
      ],
    });

    if (data.resultType === 'vector' && Array.isArray(data.result)) {
      for (const sample of data.result) {
        const [tsUs, value] = sample.value;
        frame.add({ Time: tsUs / 1000, [target.queryText]: value }); // µs → ms
      }
    } else if (data.resultType === 'scalar') {
      const [tsUs, value] = data.result;
      frame.add({ Time: tsUs / 1000, [target.queryText]: value });
    }

    return [frame];
  }

  /**
   * Query logs via the logs endpoint.
   */
  private async queryLogs(
    target: FluxxQuery,
    fromUs: number,
    toUs: number,
  ): Promise<MutableDataFrame[]> {
    const params = new URLSearchParams({
      q: target.queryText,
      from: fromUs.toString(),
      to: toUs.toString(),
      limit: '500',
    });

    const resp = await this.doRequest<FluxxApiResponse>(`/v1/logs?${params}`);
    if (resp.status !== 'SUCCESS') {
      throw new Error(resp.responseMessage);
    }

    const logs = Array.isArray(resp.data) ? resp.data : [];
    const frame = new MutableDataFrame({
      refId: target.refId,
      meta: { preferredVisualisationType: 'logs' },
      fields: [
        { name: 'Time', type: FieldType.time },
        { name: 'Level', type: FieldType.string },
        { name: 'Message', type: FieldType.string },
        { name: 'Service', type: FieldType.string },
        { name: 'TraceID', type: FieldType.string },
      ],
    });

    for (const log of logs) {
      frame.add({
        Time: (log.timestamp_us || 0) / 1000,
        Level: log.level,
        Message: log.message,
        Service: log.service || '',
        TraceID: log.trace_id || '',
      });
    }

    return [frame];
  }

  /**
   * Annotations — fetch deploy markers for chart overlay.
   */
  async annotationQuery(options: AnnotationQueryRequest): Promise<AnnotationEvent[]> {
    const from = options.range.from.valueOf() * 1000;
    const to = options.range.to.valueOf() * 1000;

    const params = new URLSearchParams({
      from: from.toString(),
      to: to.toString(),
    });

    try {
      const resp = await this.doRequest<FluxxApiResponse>(`/v1/annotations?${params}`);
      if (resp.status !== 'SUCCESS' || !Array.isArray(resp.data)) {
        return [];
      }

      return resp.data.map((a: any) => ({
        time: a.timestamp_us / 1000,
        title: a.title,
        text: a.description || '',
        tags: [a.annotation_type || 'deploy', a.service || ''].filter(Boolean),
      }));
    } catch {
      return [];
    }
  }

  /**
   * Health check — tests connection to Fluxx agent.
   */
  async testDatasource(): Promise<{ status: string; message: string }> {
    try {
      const resp = await this.doRequest<FluxxApiResponse>('/v1/stats');
      if (resp.status === 'SUCCESS') {
        return { status: 'success', message: 'Connected to Fluxx agent' };
      }
      return { status: 'error', message: resp.responseMessage };
    } catch (err) {
      return { status: 'error', message: `Connection failed: ${err}` };
    }
  }

  /**
   * Metric name suggestions for autocomplete.
   */
  async metricFindQuery(query: string): Promise<Array<{ text: string }>> {
    try {
      const resp = await this.doRequest<FluxxApiResponse>('/v1/metrics/names');
      if (resp.status === 'SUCCESS' && Array.isArray(resp.data)) {
        return resp.data
          .filter((name: string) => !query || name.includes(query))
          .map((name: string) => ({ text: name }));
      }
    } catch {}
    return [];
  }

  private async doRequest<T>(path: string): Promise<T> {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (this.apiKey) {
      headers['X-API-Key'] = this.apiKey;
    }

    const resp = await fetch(`${this.url}${path}`, { headers });
    if (!resp.ok) {
      throw new Error(`HTTP ${resp.status}`);
    }
    return resp.json();
  }
}
