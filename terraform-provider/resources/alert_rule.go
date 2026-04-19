package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
	"github.com/hashicorp/terraform-plugin-sdk/v2/diag"
	"context"
)

func resourceAlertRule() *schema.Resource {
	return &schema.Resource{
		Description:   "Manages a Fluxx metric alert rule.",
		CreateContext: resourceAlertRuleCreate,
		ReadContext:   resourceAlertRuleRead,
		DeleteContext: resourceAlertRuleDelete,
		Schema: map[string]*schema.Schema{
			"name": {
				Type:        schema.TypeString,
				Required:    true,
				ForceNew:    true,
				Description: "Alert rule name",
			},
			"condition": {
				Type:        schema.TypeString,
				Required:    true,
				ForceNew:    true,
				Description: "PromQL-style condition (e.g., 'cpu.usage > 80')",
			},
			"severity": {
				Type:        schema.TypeString,
				Optional:    true,
				Default:     "warning",
				ForceNew:    true,
				Description: "Alert severity: info, warning, critical",
			},
			"channels": {
				Type:     schema.TypeList,
				Optional: true,
				ForceNew: true,
				Elem:     &schema.Schema{Type: schema.TypeString},
				Description: "Notification channel names",
			},
			"cooldown_seconds": {
				Type:        schema.TypeInt,
				Optional:    true,
				Default:     300,
				ForceNew:    true,
				Description: "Cooldown period between repeated alerts",
			},
		},
	}
}

func resourceAlertRuleCreate(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics {
	// In production: POST to /v1/alert-rules
	d.SetId(d.Get("name").(string))
	return nil
}

func resourceAlertRuleRead(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics {
	return nil
}

func resourceAlertRuleDelete(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics {
	d.SetId("")
	return nil
}

func resourceSLO() *schema.Resource {
	return &schema.Resource{
		Description:   "Manages a Fluxx Service Level Objective.",
		CreateContext: resourceSLOCreate,
		ReadContext:   resourceSLORead,
		DeleteContext: resourceSLODelete,
		Schema: map[string]*schema.Schema{
			"name": {Type: schema.TypeString, Required: true, ForceNew: true},
			"description": {Type: schema.TypeString, Optional: true, ForceNew: true},
			"target_percent": {Type: schema.TypeFloat, Required: true, ForceNew: true, Description: "SLO target (e.g., 99.9)"},
			"window_days": {Type: schema.TypeInt, Optional: true, Default: 30, ForceNew: true},
			"total_query": {Type: schema.TypeString, Required: true, ForceNew: true, Description: "PromQL for total events"},
			"good_query": {Type: schema.TypeString, Required: true, ForceNew: true, Description: "PromQL for good events"},
			"service": {Type: schema.TypeString, Optional: true, ForceNew: true},
		},
	}
}

func resourceSLOCreate(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics {
	d.SetId(d.Get("name").(string))
	return nil
}
func resourceSLORead(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { return nil }
func resourceSLODelete(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(""); return nil }

func resourceAnnotation() *schema.Resource {
	return &schema.Resource{
		Description: "Creates a deploy annotation marker.",
		CreateContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(d.Get("title").(string)); return nil },
		ReadContext:   func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { return nil },
		DeleteContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(""); return nil },
		Schema: map[string]*schema.Schema{
			"title": {Type: schema.TypeString, Required: true, ForceNew: true},
			"description": {Type: schema.TypeString, Optional: true, ForceNew: true},
			"service": {Type: schema.TypeString, Optional: true, ForceNew: true},
			"environment": {Type: schema.TypeString, Optional: true, ForceNew: true},
			"source": {Type: schema.TypeString, Optional: true, Default: "terraform", ForceNew: true},
			"reference": {Type: schema.TypeString, Optional: true, ForceNew: true, Description: "Commit SHA or build ID"},
		},
	}
}

func resourceNotebook() *schema.Resource {
	return &schema.Resource{
		Description: "Manages a Fluxx investigation notebook.",
		CreateContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(d.Get("title").(string)); return nil },
		ReadContext:   func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { return nil },
		DeleteContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(""); return nil },
		Schema: map[string]*schema.Schema{
			"title": {Type: schema.TypeString, Required: true, ForceNew: true},
			"author": {Type: schema.TypeString, Required: true, ForceNew: true},
			"tags": {Type: schema.TypeList, Optional: true, ForceNew: true, Elem: &schema.Schema{Type: schema.TypeString}},
			"shared": {Type: schema.TypeBool, Optional: true, Default: false, ForceNew: true},
		},
	}
}

func resourceWorkflow() *schema.Resource {
	return &schema.Resource{
		Description: "Manages a Fluxx workflow automation.",
		CreateContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(d.Get("name").(string)); return nil },
		ReadContext:   func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { return nil },
		DeleteContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(""); return nil },
		Schema: map[string]*schema.Schema{
			"name": {Type: schema.TypeString, Required: true, ForceNew: true},
			"description": {Type: schema.TypeString, Optional: true, ForceNew: true},
			"enabled": {Type: schema.TypeBool, Optional: true, Default: true, ForceNew: true},
			"trigger_type": {Type: schema.TypeString, Required: true, ForceNew: true, Description: "alert_fired, metric_threshold, schedule"},
		},
	}
}

func resourceSyntheticCheck() *schema.Resource {
	return &schema.Resource{
		Description: "Manages a Fluxx synthetic monitoring check.",
		CreateContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(d.Get("name").(string)); return nil },
		ReadContext:   func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { return nil },
		DeleteContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId(""); return nil },
		Schema: map[string]*schema.Schema{
			"name": {Type: schema.TypeString, Required: true, ForceNew: true},
			"url": {Type: schema.TypeString, Required: true, ForceNew: true},
			"method": {Type: schema.TypeString, Optional: true, Default: "GET", ForceNew: true},
			"expected_status": {Type: schema.TypeInt, Optional: true, Default: 200, ForceNew: true},
			"interval_secs": {Type: schema.TypeInt, Optional: true, Default: 60, ForceNew: true},
			"timeout_secs": {Type: schema.TypeInt, Optional: true, Default: 10, ForceNew: true},
		},
	}
}

func dataSourceMetrics() *schema.Resource {
	return &schema.Resource{
		Description: "Reads metric names from Fluxx.",
		ReadContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId("metrics"); return nil },
		Schema: map[string]*schema.Schema{
			"names": {Type: schema.TypeList, Computed: true, Elem: &schema.Schema{Type: schema.TypeString}},
		},
	}
}

func dataSourceServiceMap() *schema.Resource {
	return &schema.Resource{
		Description: "Reads the service dependency map from Fluxx.",
		ReadContext: func(ctx context.Context, d *schema.ResourceData, meta interface{}) diag.Diagnostics { d.SetId("service-map"); return nil },
		Schema: map[string]*schema.Schema{
			"services": {Type: schema.TypeList, Computed: true, Elem: &schema.Schema{Type: schema.TypeString}},
		},
	}
}

func providerConfigure(ctx context.Context, d *schema.ResourceData) (interface{}, diag.Diagnostics) {
	endpoint := d.Get("endpoint").(string)
	apiKey := d.Get("api_key").(string)
	return map[string]string{"endpoint": endpoint, "api_key": apiKey}, nil
}
