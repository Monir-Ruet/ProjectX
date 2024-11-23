resource "azurerm_container_app_environment_dapr_component" "rabbitmq" {
  name                         = "pubsub"
  container_app_environment_id = azurerm_container_app_environment.example.name
  component_type               = "pubsub.rabbitmq"
  version                      = "v1"

  metadata {
    name  = "host"
    value = var.rabbitmq_host
  }
  metadata {
    name  = "durable"
    value = false
  }
  metadata {
    name  = "username"
    value = "guest"
  }
  metadata {
    name  = "password"
    value = "guest"
  }
  metadata {
    name = "deletedWhenUnused"
    value = "false"
  }
  metadata {
    name = "autoAck"
    value = "false"
  }
  metadata {
    name = "deliveryMode"
    value = "0"
  }
  metadata {
    name = "requeueInFailure"
    value = "false"
  }
  metadata {
    name = "prefetchCount"
    value = "0"
  }
  metadata {
    name = "reconnectWait"
    value = "0"
  }
  metadata {
    name = "concurrencyMode"
    value = "parallel"
  }
  metadata {
    name = "publisherConfirm"
    value = "false"
  }
  metadata {
    name = "enableDeadLetter"
    value = "true"
  }

  scopes = ["projectx-identity"]
}
