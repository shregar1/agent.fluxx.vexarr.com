# Fluxx Agent Helm chart

A Helm chart that deploys the Fluxx agent as a DaemonSet across a
Kubernetes cluster.

> **What this is — and isn't.** This is a Helm chart, not a Kubernetes
> operator. There is no CRD or custom reconciler. If you need
> declarative CRD-driven management (`FluxxAgent` resources, automatic
> upgrades, drift detection), open an issue — that's on the roadmap as
> a separate `fluxx-operator/` package.

## Install

```bash
helm repo add fluxx https://charts.fluxx.dev
helm install fluxx fluxx/fluxx-agent \
  --namespace fluxx --create-namespace \
  --set apiKey=flx_live_...
```

## Local install (from this directory)

```bash
helm install fluxx ./charts/fluxx-agent \
  --namespace fluxx --create-namespace \
  --set apiKey=flx_live_...
```

## Values

See [`charts/fluxx-agent/values.yaml`](./charts/fluxx-agent/values.yaml)
for the full schema. Most-used:

| Key | Default | Notes |
|---|---|---|
| `apiKey` | (required) | Your `flx_live_…` key |
| `image.repository` | `ghcr.io/fluxx/fluxx-agent` | Set for private registries |
| `image.tag` | chart appVersion | Pin for reproducible rollouts |
| `resources.limits.cpu` | `500m` | Bump for high-traffic nodes |
| `resources.limits.memory` | `512Mi` | |
| `tolerations` | `[]` | Add `node-role.kubernetes.io/control-plane` to include masters |
| `mesh.enabled` | `false` | Enables gossip-based federation |
| `mesh.peers` | `[]` | Static peer list, when DNS-SD isn't available |

## Uninstall

```bash
helm uninstall fluxx -n fluxx
```

## See also

- [Install guide](https://docs.fluxx.dev/install) — non-k8s installs
- [Self-hosted setup](https://docs.fluxx.dev/self-hosted) — control plane
- [Configuration reference](https://docs.fluxx.dev/configuration) — agent options
