/**
 * Fluxx Data Source Config Editor for Grafana
 */

import React, { ChangeEvent } from 'react';
import { DataSourcePluginOptionsEditorProps } from '@grafana/data';
import { InlineField, Input, SecretInput } from '@grafana/ui';

interface FluxxDataSourceOptions {
  url: string;
  apiKey?: string;
}

interface FluxxSecureJsonData {
  apiKey?: string;
}

type Props = DataSourcePluginOptionsEditorProps<FluxxDataSourceOptions, FluxxSecureJsonData>;

export function ConfigEditor(props: Props) {
  const { onOptionsChange, options } = props;
  const { jsonData, secureJsonFields } = options;

  const onURLChange = (event: ChangeEvent<HTMLInputElement>) => {
    onOptionsChange({
      ...options,
      jsonData: { ...jsonData, url: event.target.value },
    });
  };

  const onAPIKeyChange = (event: ChangeEvent<HTMLInputElement>) => {
    onOptionsChange({
      ...options,
      secureJsonData: { apiKey: event.target.value },
    });
  };

  const onResetAPIKey = () => {
    onOptionsChange({
      ...options,
      secureJsonFields: { ...secureJsonFields, apiKey: false },
      secureJsonData: { apiKey: '' },
    });
  };

  return (
    <div className="gf-form-group">
      <InlineField label="Fluxx Agent URL" labelWidth={20} tooltip="URL of your Fluxx agent (e.g., http://localhost:3000)">
        <Input
          value={jsonData.url || ''}
          placeholder="http://localhost:3000"
          width={40}
          onChange={onURLChange}
        />
      </InlineField>

      <InlineField label="API Key" labelWidth={20} tooltip="Optional API key for authentication">
        <SecretInput
          isConfigured={!!secureJsonFields?.apiKey}
          value={options.secureJsonData?.apiKey || ''}
          placeholder="Optional"
          width={40}
          onReset={onResetAPIKey}
          onChange={onAPIKeyChange}
        />
      </InlineField>
    </div>
  );
}
