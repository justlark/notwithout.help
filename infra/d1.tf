resource "cloudflare_d1_database" "matchdrop" {
  account_id = var.cloudflare_account_id
  name       = "matchdrop"
}

resource "cloudflare_d1_database" "matchdrop_dev" {
  account_id = var.cloudflare_account_id
  name       = "matchdrop-dev"
}
