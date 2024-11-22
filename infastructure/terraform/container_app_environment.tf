# Azure Container App Environment (ACAE)
resource "azurerm_container_app_environment" "example" {
  name                = var.container_app_env_name
  location            = azurerm_resource_group.example.location
  resource_group_name = azurerm_resource_group.example.name
  tags = {
    environment = "dev"
  }
}
