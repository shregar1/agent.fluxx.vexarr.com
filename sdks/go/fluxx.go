// Package fluxx provides a lightweight client for sending traces, metrics,
// and logs to a Fluxx observability agent.
//
// Usage:
//
//	client := fluxx.New("http://localhost:3000")
//	client.Log("info", "User logged in", fluxx.LogOpts{Service: "auth"})
//	client.Metric("request.count", 1, "counter", map[string]string{"env": "prod"})
package fluxx

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"time"

	"github.com/google/uuid"
)

// Client sends telemetry to a Fluxx agent.
type Client struct {
	Endpoint      string
	StatsdAddr    string
	ProjectID     string
	EnvironmentID string
	Service       string
	httpClient    *http.Client
	udpConn       *net.UDPConn
}

// New creates a Fluxx client pointing at the given agent endpoint.
func New(endpoint string, opts ...Option) *Client {
	c := &Client{
		Endpoint:   endpoint,
		StatsdAddr: "localhost:8125",
		httpClient: &http.Client{Timeout: 5 * time.Second},
	}
	for _, o := range opts {
		o(c)
	}
	return c
}

// Option configures the client.
type Option func(*Client)

func WithProject(id string) Option        { return func(c *Client) { c.ProjectID = id } }
func WithEnvironment(id string) Option     { return func(c *Client) { c.EnvironmentID = id } }
func WithService(name string) Option       { return func(c *Client) { c.Service = name } }
func WithStatsdAddr(addr string) Option    { return func(c *Client) { c.StatsdAddr = addr } }

// ── Logs ──

// LogOpts are optional log parameters.
type LogOpts struct {
	Service    string
	TraceID    string
	SpanID     string
	Attributes map[string]interface{}
}

// Log sends a structured log entry.
func (c *Client) Log(level, message string, opts ...LogOpts) error {
	var o LogOpts
	if len(opts) > 0 {
		o = opts[0]
	}
	svc := o.Service
	if svc == "" {
		svc = c.Service
	}

	entry := map[string]interface{}{
		"id":             uuid.New().String(),
		"timestamp_us":   nowUs(),
		"level":          level,
		"message":        message,
		"service":        svc,
		"trace_id":       o.TraceID,
		"span_id":        o.SpanID,
		"attributes":     o.Attributes,
		"resource":       map[string]string{},
		"project_id":     c.ProjectID,
		"environment_id": c.EnvironmentID,
	}
	return c.post("/v1/logs", []interface{}{entry})
}

// ── Metrics ──

// Metric sends a metric via StatsD UDP.
func (c *Client) Metric(name string, value float64, metricType string, tags map[string]string) error {
	typeMap := map[string]string{
		"counter": "c", "gauge": "g", "histogram": "h",
		"timing": "ms", "set": "s", "distribution": "d",
	}
	code, ok := typeMap[metricType]
	if !ok {
		code = "g"
	}

	line := fmt.Sprintf("%s:%g|%s", name, value, code)
	if len(tags) > 0 {
		tagStr := ""
		for k, v := range tags {
			if tagStr != "" {
				tagStr += ","
			}
			tagStr += fmt.Sprintf("%s:%s", k, v)
		}
		line += "|#" + tagStr
	}

	return c.sendUDP(line)
}

// ── Events ──

// Event sends an event to the agent.
func (c *Client) Event(source, protocol string, latencyUs int64, severity string) error {
	evt := map[string]interface{}{
		"id":             uuid.New().String(),
		"timestamp_us":   nowUs(),
		"protocol":       protocol,
		"latency_us":     latencyUs,
		"severity":       severity,
		"service_id":     source,
		"project_id":     c.ProjectID,
		"environment_id": c.EnvironmentID,
		"metadata":       map[string]string{},
	}
	return c.post("/v1/events", evt)
}

// ── Internal ──

func (c *Client) post(path string, data interface{}) error {
	body, err := json.Marshal(data)
	if err != nil {
		return err
	}
	resp, err := c.httpClient.Post(c.Endpoint+path, "application/json", bytes.NewReader(body))
	if err != nil {
		return err
	}
	resp.Body.Close()
	if resp.StatusCode != 200 {
		return fmt.Errorf("fluxx: HTTP %d", resp.StatusCode)
	}
	return nil
}

func (c *Client) sendUDP(line string) error {
	if c.udpConn == nil {
		addr, err := net.ResolveUDPAddr("udp", c.StatsdAddr)
		if err != nil {
			return err
		}
		conn, err := net.DialUDP("udp", nil, addr)
		if err != nil {
			return err
		}
		c.udpConn = conn
	}
	_, err := c.udpConn.Write([]byte(line))
	return err
}

// Close releases resources.
func (c *Client) Close() {
	if c.udpConn != nil {
		c.udpConn.Close()
	}
}

func nowUs() int64 {
	return time.Now().UnixMicro()
}
