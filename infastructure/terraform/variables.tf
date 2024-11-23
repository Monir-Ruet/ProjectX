# The container log analytics workspace
variable "subscription_id" {
  description = "Azure Subscription Id"
  type        = string
}

# The Azure region where resources will be created
variable "location" {
  description = "The Azure region where resources will be deployed"
  type        = string
}

# The name of the resource group to create or use
variable "resource_group_name" {
  description = "The name of the resource group"
  type        = string
}

# The name for the Azure Container Registry
variable "acr_name" {
  description = "The name of the Azure Container Registry"
  type        = string
}

# The name for the Azure Container App environment
variable "container_app_env_name" {
  description = "The name for the Azure Container App environment"
  type        = string
}

# The name for the Azure Container App itself
variable "container_app_name" {
  description = "The name of the Azure Container App"
  type        = string
}

# The container image to deploy
variable "container_image" {
  description = "The container image URL to be deployed"
  type        = string
}

# Optionally, configure the Dapr app ID
variable "dapr_app_id" {
  description = "The Dapr app ID"
  type        = string
}

# If container app external enabled or not
variable "container_app_external_enabled" {
  description = "Container app external enabled or not"
  type        = bool
}

variable "vnet_name" {
  description = "Virtual Network for container apps"
  type        = string
}

variable "vnet_subnet_name" {
  description = "Subnet for container apps"
  type        = string
}

variable "rabbitmq_host" {
  description = "Rabbitmq Host for dapr pubsub"
  type        = string
}

variable "dapr_pubsub_component_name" {
  description = "Dapr pubsub component name"
  type        = string
}