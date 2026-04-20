/**
 * Fluxx Node.js SDK
 *
 * Lightweight client for sending traces, metrics, and logs to a Fluxx agent.
 *
 * Usage:
 *   const { Fluxx } = require('./fluxx');
 *   const client = new Fluxx('http://localhost:7007');
 *   client.log('info', 'User logged in', { service: 'auth-service' });
 *   client.metric('request.count', 1, 'counter', { env: 'prod' });
 */

const dgram = require('dgram');
const http = require('http');
const https = require('https');
const { randomUUID } = require('crypto');

class Fluxx {
  /**
   * @param {string} endpoint - Fluxx agent HTTP endpoint
   * @param {Object} options
   * @param {string} [options.statsdHost='localhost']
   * @param {number} [options.statsdPort=8125]
   * @param {string} [options.projectId]
   * @param {string} [options.environmentId]
   * @param {string} [options.service]
   */
  constructor(endpoint = 'http://localhost:7007', options = {}) {
    this.endpoint = endpoint.replace(/\/$/, '');
    this.statsdHost = options.statsdHost || 'localhost';
    this.statsdPort = options.statsdPort || 8125;
    this.projectId = options.projectId || null;
    this.environmentId = options.environmentId || null;
    this.service = options.service || null;
    this._udpSocket = null;
  }

  // ── Logs ──

  /**
   * Send a structured log entry.
   * @param {'trace'|'debug'|'info'|'warn'|'error'|'fatal'} level
   * @param {string} message
   * @param {Object} [options]
   * @returns {Promise<boolean>}
   */
  async log(level, message, options = {}) {
    const entry = {
      id: randomUUID(),
      timestamp_us: nowUs(),
      level,
      message,
      service: options.service || this.service,
      trace_id: options.traceId || null,
      span_id: options.spanId || null,
      attributes: options.attributes || {},
      resource: {},
      project_id: this.projectId,
      environment_id: this.environmentId,
    };
    return this._post('/v1/logs', [entry]);
  }

  // ── Metrics (StatsD UDP) ──

  /**
   * Send a metric via StatsD UDP.
   * @param {string} name
   * @param {number} value
   * @param {'counter'|'gauge'|'histogram'|'timing'|'set'|'distribution'} [type='gauge']
   * @param {Object.<string,string>} [tags={}]
   * @param {number} [sampleRate=1.0]
   */
  metric(name, value, type = 'gauge', tags = {}, sampleRate = 1.0) {
    const typeMap = {
      counter: 'c', gauge: 'g', histogram: 'h',
      timing: 'ms', set: 's', distribution: 'd',
    };
    const typeCode = typeMap[type] || 'g';
    let line = `${name}:${value}|${typeCode}`;
    if (sampleRate !== 1.0) line += `|@${sampleRate}`;
    const tagPairs = Object.entries(tags).map(([k, v]) => `${k}:${v}`).join(',');
    if (tagPairs) line += `|#${tagPairs}`;
    return this._sendUdp(line);
  }

  // ── Events ──

  /**
   * Send an event.
   * @param {string} source
   * @param {Object} [options]
   * @returns {Promise<boolean>}
   */
  async event(source, options = {}) {
    const evt = {
      id: randomUUID(),
      timestamp_us: nowUs(),
      protocol: options.protocol || 'http',
      latency_us: options.latencyUs || 0,
      severity: options.severity || 'info',
      payload: options.payload || null,
      service_id: source,
      project_id: this.projectId,
      environment_id: this.environmentId,
      metadata: options.metadata || {},
    };
    return this._post('/v1/events', evt);
  }

  // ── Internal ──

  /** @private */
  _post(path, data) {
    return new Promise((resolve) => {
      const url = new URL(path, this.endpoint);
      const mod = url.protocol === 'https:' ? https : http;
      const body = JSON.stringify(data);

      const req = mod.request(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Content-Length': Buffer.byteLength(body) },
        timeout: 5000,
      }, (res) => {
        res.resume();
        resolve(res.statusCode === 200);
      });

      req.on('error', () => resolve(false));
      req.on('timeout', () => { req.destroy(); resolve(false); });
      req.write(body);
      req.end();
    });
  }

  /** @private */
  _sendUdp(line) {
    try {
      if (!this._udpSocket) {
        this._udpSocket = dgram.createSocket('udp4');
      }
      const buf = Buffer.from(line);
      this._udpSocket.send(buf, 0, buf.length, this.statsdPort, this.statsdHost);
      return true;
    } catch {
      return false;
    }
  }

  close() {
    if (this._udpSocket) {
      this._udpSocket.close();
      this._udpSocket = null;
    }
  }
}

function nowUs() {
  return Math.floor(Date.now() * 1000);
}

module.exports = { Fluxx };
