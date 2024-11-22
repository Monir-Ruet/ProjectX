# Azure Container App (ACA)
resource "azurerm_container_app" "example" {
  name                         = var.container_app_name
  container_app_environment_id = azurerm_container_app_environment.example.id
  resource_group_name          = azurerm_resource_group.example.name
  revision_mode                = "Single"
  max_inactive_revisions       = 5
  template {
    container {
      name   = var.container_app_name
      image  = "${azurerm_container_registry.example.login_server}/${var.container_image}:latest"
      cpu    = 0.25
      memory = "0.5Gi"
    }
    min_replicas = 0
    max_replicas = 10
  }

  registry {
    server               = azurerm_container_registry.example.login_server
    username             = azurerm_container_registry.example.name
    password_secret_name = random_uuid.example.result
  }

  secret {
    name  = random_uuid.example.result
    value = azurerm_container_registry.example.admin_password
  }

  dapr {
    app_id       = var.dapr_app_id
    app_protocol = "http"
    app_port     = 8080
  }

  ingress {
    allow_insecure_connections = false
    external_enabled           = var.container_app_external_enabled
    target_port                = 8080
    transport                  = "auto"
    traffic_weight {
      latest_revision = true
      percentage      = 100
    }
  }
}
