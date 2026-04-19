{{- define "fluxx-agent.fullname" -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "fluxx-agent.labels" -}}
app.kubernetes.io/name: fluxx-agent
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
{{- end }}

{{- define "fluxx-agent.selectorLabels" -}}
app.kubernetes.io/name: fluxx-agent
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "fluxx-agent.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "fluxx-agent.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
