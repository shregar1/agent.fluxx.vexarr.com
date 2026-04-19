// Terraform Provider for Fluxx Observability Platform
//
// Manages Fluxx resources as Infrastructure-as-Code:
// - fluxx_alert_rule — metric alert rules
// - fluxx_slo — service level objectives
// - fluxx_notebook — investigation notebooks
// - fluxx_workflow — automation workflows
// - fluxx_annotation — deploy annotations
// - fluxx_synthetic_check — uptime/API checks
//
// Usage:
//   terraform {
//     required_providers {
//       fluxx = {
//         source = "fluxx/fluxx"
//       }
//     }
//   }
//
//   provider "fluxx" {
//     endpoint = "http://localhost:3000"
//     api_key  = var.fluxx_api_key
//   }

package main

import (
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
	"github.com/hashicorp/terraform-plugin-sdk/v2/plugin"
)

func main() {
	plugin.Serve(&plugin.ServeOpts{
		ProviderFunc: Provider,
	})
}

// Provider returns the Fluxx Terraform provider schema.
func Provider() *schema.Provider {
	return &schema.Provider{
		Schema: map[string]*schema.Schema{
			"endpoint": {
				Type:        schema.TypeString,
				Required:    true,
				DefaultFunc: schema.EnvDefaultFunc("FLUXX_ENDPOINT", "http://localhost:3000"),
				Description: "Fluxx agent HTTP endpoint URL",
			},
			"api_key": {
				Type:        schema.TypeString,
				Optional:    true,
				Sensitive:   true,
				DefaultFunc: schema.EnvDefaultFunc("FLUXX_API_KEY", ""),
				Description: "API key for authentication",
			},
		},
		ResourcesMap: map[string]*schema.Resource{
			"fluxx_alert_rule":     resourceAlertRule(),
			"fluxx_slo":           resourceSLO(),
			"fluxx_annotation":    resourceAnnotation(),
			"fluxx_notebook":      resourceNotebook(),
			"fluxx_workflow":      resourceWorkflow(),
			"fluxx_synthetic":     resourceSyntheticCheck(),
		},
		DataSourcesMap: map[string]*schema.Resource{
			"fluxx_metrics":       dataSourceMetrics(),
			"fluxx_service_map":   dataSourceServiceMap(),
		},
		ConfigureContextFunc: providerConfigure,
	}
}
