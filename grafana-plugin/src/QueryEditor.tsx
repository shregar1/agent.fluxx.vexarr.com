/**
 * Fluxx Query Editor for Grafana panels
 */

import React, { ChangeEvent } from 'react';
import { QueryEditorProps, SelectableValue } from '@grafana/data';
import { InlineField, Input, Select } from '@grafana/ui';

interface FluxxQuery {
  refId: string;
  queryText: string;
  queryType: 'metrics' | 'logs';
}

interface FluxxDataSourceOptions {
  url: string;
}

type Props = QueryEditorProps<any, FluxxQuery, FluxxDataSourceOptions>;

const QUERY_TYPES: Array<SelectableValue<string>> = [
  { label: 'Metrics (PromQL)', value: 'metrics' },
  { label: 'Logs', value: 'logs' },
];

export function QueryEditor({ query, onChange, onRunQuery }: Props) {
  const onQueryTextChange = (event: ChangeEvent<HTMLInputElement>) => {
    onChange({ ...query, queryText: event.target.value });
  };

  const onQueryTypeChange = (value: SelectableValue<string>) => {
    onChange({ ...query, queryType: (value.value || 'metrics') as 'metrics' | 'logs' });
    onRunQuery();
  };

  const onKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter') {
      onRunQuery();
    }
  };

  return (
    <div className="gf-form-inline">
      <InlineField label="Type" labelWidth={8}>
        <Select
          options={QUERY_TYPES}
          value={QUERY_TYPES.find((t) => t.value === (query.queryType || 'metrics'))}
          onChange={onQueryTypeChange}
          width={20}
        />
      </InlineField>

      <InlineField label="Query" labelWidth={8} grow>
        <Input
          value={query.queryText || ''}
          onChange={onQueryTextChange}
          onKeyDown={onKeyDown}
          placeholder={
            query.queryType === 'logs'
              ? 'Search logs by message...'
              : 'avg(cpu.usage) or http_requests_total{method="GET"}'
          }
        />
      </InlineField>
    </div>
  );
}
