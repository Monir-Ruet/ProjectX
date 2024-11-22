# Azure Container Registry (ACR)
resource "azurerm_container_registry" "example" {
  name                = var.acr_name
  location            = azurerm_resource_group.example.location
  resource_group_name = azurerm_resource_group.example.name
  sku                 = "Basic"
  admin_enabled       = true
}
