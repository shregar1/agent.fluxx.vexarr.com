/**
 * Fluxx Grafana Data Source Plugin — Module Entry Point
 *
 * Usage:
 * 1. Build: npm run build
 * 2. Copy to Grafana plugins directory: /var/lib/grafana/plugins/fluxx-datasource/
 * 3. Restart Grafana
 * 4. Add "Fluxx" data source, enter your Fluxx agent URL
 */

import { DataSourcePlugin } from '@grafana/data';
import { FluxxDataSource } from './datasource';

export const plugin = new DataSourcePlugin(FluxxDataSource);
