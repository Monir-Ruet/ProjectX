# Virtual network for the project
resource "azurerm_virtual_network" "example" {
  name                = var.vnet_name
  location            = azurerm_resource_group.example.location
  resource_group_name = azurerm_resource_group.example.name
  address_space       = ["10.3.0.0/16"]
}

# Create the Subnet for Azure Container Apps
resource "azurerm_subnet" "example" {
  name                 = var.vnet_subnet_name
  resource_group_name  = azurerm_resource_group.example.name
  virtual_network_name = azurerm_virtual_network.example.name
  address_prefixes     = ["10.3.1.0/24"]
}
